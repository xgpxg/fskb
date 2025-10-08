use crate::db::model::knowledge_base::KnowledgeBase;
use crate::db::model::knowledge_base_import_record::{
    KnowledgeBaseImportFileContentExtractType, KnowledgeBaseImportFileContentType,
    KnowledgeBaseImportRecord, KnowledgeBaseImportSource,
};
use crate::db::model::model::Model;
use crate::db::Pool;
use crate::utils::file_util::make_kb_ref_file;
use anyhow::{anyhow, bail};
use common::temp_dir;
use embedding::{Embedding, EmbeddingInput, Embeddings};
use engine::db::ContentRef;
use engine::{AddRecordRequest, Engine, TableEngine};
use input::csv::CsvInput;
use input::md::MdInput;
use input::pdf::PdfInput;
use input::txt::TxtInput;
use input::xlsx::XlsxInput;
use input::{Input, Split};
use rbs::value;
use std::fs;
use std::path::Path;

impl KnowledgeBaseImportRecord {
    pub(crate) async fn parse(&mut self) -> anyhow::Result<()> {
        let source = KnowledgeBaseImportSource::try_from(self.source.unwrap())?;
        match source {
            KnowledgeBaseImportSource::LocalFile => {
                let file_path = &self.file_path.clone().unwrap();
                let ext = Path::new(file_path).extension().unwrap().to_str().unwrap();
                match ext {
                    "txt" => parse_txt(self).await?,
                    "pdf" => parse_pdf(self).await?,
                    "md" => parse_md(self).await?,
                    "doc" | "docx" => parse_docx(self).await?,
                    "xls" | "xlsx" => parse_xlsx(self).await?,
                    "csv" => parse_csv(self).await?,
                    "png" | "jpg" | "jpeg" | "bmp" => parse_image(self).await?,
                    _ => bail!(format!("不支持的文件类型：{}", ext)),
                };
            }
            KnowledgeBaseImportSource::Url => {
                bail!("暂不支持导入网页");
            }
            KnowledgeBaseImportSource::CustomText => {
                bail!("暂不支持导入自定义文本");
            }
        }

        Ok(())
    }
}

pub(crate) async fn get_table_name(knowledge_base_id: i64) -> anyhow::Result<String> {
    // 获取知识库
    let kb = KnowledgeBase::select_by_map(
        Pool::get()?,
        value! {
            "id": knowledge_base_id
        },
    )
    .await?;
    if kb.is_empty() {
        bail!("知识库不存在");
    }
    let kb = kb.first().unwrap();
    Ok(kb.table_name.clone().unwrap())
}
async fn parse_txt(record: &KnowledgeBaseImportRecord) -> anyhow::Result<()> {
    // 导入记录ID
    let id = record.id.unwrap();
    // 知识库ID
    let kb_id = record.knowledge_base_id.unwrap();
    // 文件路径
    let file_path = &record.file_path.clone().unwrap();
    // 知识库对应的向量数据库表名
    let table_name = &get_table_name(kb_id).await?;

    // 读取文件
    let output = TxtInput::read(file_path).map_err(|e| anyhow!(e.to_string()))?;

    // 分段处理
    let mut data = Vec::new();
    for segment in TxtInput::split(output) {
        data.push(convert_to_vector_record(id, segment, None)?);
    }

    // 添加数据
    Engine::add_data(table_name, data).await?;

    Ok(())
}

pub async fn parse_pdf(record: &KnowledgeBaseImportRecord) -> anyhow::Result<()> {
    // 导入记录ID
    let id = record.id.unwrap();
    // 知识库ID
    let kb_id = record.knowledge_base_id.unwrap();
    // 文件路径
    let file_path = &record.file_path.clone().unwrap();
    // 知识库对应的向量数据库表名
    let table_name = &get_table_name(kb_id).await?;

    // 读取文件
    let mut output = PdfInput::read(file_path).map_err(|e| anyhow!(e.to_string()))?;

    // 遍历每一页
    for item in &mut output.pages {
        log::debug!("Processing pdf snapshot image: {}", item.snapshot);
        let mut text = Some(String::new());
        let extract_type = record.file_content_extract_type.clone().unwrap();
        match extract_type {
            KnowledgeBaseImportFileContentExtractType::Text => {
                text = Some(item.text.clone());
            }
            KnowledgeBaseImportFileContentExtractType::Ocr => {
                text = Some(ocr::run(&item.snapshot)?);
            }
            KnowledgeBaseImportFileContentExtractType::VisionModel { model_id } => {
                let model = get_model(model_id).await?;
                text = image_to_text::extra(
                    &item.snapshot,
                    &model.base_url.unwrap(),
                    &model.name.unwrap(),
                    &model.api_key.unwrap_or_default(),
                )
                .await?;
            }
        }

        if text.is_none() || text.as_ref().unwrap().is_empty() {
            continue;
        }

        item.text = text.unwrap_or_default();
        let file_name = Path::new(&item.snapshot)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap();

        let (_, save_file, download_file) =
            make_kb_ref_file(table_name, record.id.unwrap(), file_name)?;

        // 移动图片到refs目录下
        fs::copy(&item.snapshot, &save_file)?;

        // 删除临时图片
        fs::remove_file(&item.snapshot)?;

        item.snapshot = download_file;
    }

    // 最终需要添加到向量库的数据
    let mut data = Vec::new();

    let split_res = output.split();
    log::debug!("Text splitting result: {:?}", split_res);

    // 文本拆分
    for item in split_res {
        data.push(convert_to_vector_record(
            id,
            item.text,
            Some(item.snapshot),
        )?);
    }

    // 添加数据
    Engine::add_data(table_name, data)
        .await
        .map_err(|e| anyhow!(e.to_string()))?;

    Ok(())
}

pub async fn parse_md(record: &KnowledgeBaseImportRecord) -> anyhow::Result<()> {
    // 导入记录ID
    let id = record.id.unwrap();
    // 知识库ID
    let kb_id = record.knowledge_base_id.unwrap();
    // 文件路径
    let file_path = &record.file_path.clone().unwrap();
    // 知识库对应的向量数据库表名
    let table_name = &get_table_name(kb_id).await?;

    // 读取文件
    let output = MdInput::read(file_path).map_err(|e| anyhow!(e.to_string()))?;

    // 分段处理
    let mut data = Vec::new();
    for segment in MdInput::split(output) {
        data.push(convert_to_vector_record(id, segment, None)?);
    }

    // 添加数据
    Engine::add_data(table_name, data).await?;

    Ok(())
}

/// 解析 doc 和 docx 文件
pub async fn parse_docx(record: &KnowledgeBaseImportRecord) -> anyhow::Result<()> {
    // 文件路径
    let file_path = &record.file_path.clone().unwrap();
    // 创建临时 pdf 文件
    //let temp_pdf = format!("data/temp/{}.pdf", uuid::Uuid::new_v4().to_string());
    let temp_pdf = temp_dir!(format!("{}.pdf", uuid::Uuid::new_v4().to_string()))
        .to_string_lossy()
        .into_owned();

    // 将 docx 转换为 pdf
    doc_to_pdf::convert(&file_path, &temp_pdf)?;
    // 解析 pdf
    let mut record = record.clone();
    record.file_path = Some(temp_pdf.clone());
    parse_pdf(&record).await?;

    // 删除临时 pdf 文件
    fs::remove_file(temp_pdf)?;

    Ok(())
}

/// 解析 xlsx 文件
pub(crate) async fn parse_xlsx(record: &KnowledgeBaseImportRecord) -> anyhow::Result<()> {
    // 文件路径
    let file_path = &record.file_path.clone().unwrap();
    // 创建临时 pdf 文件
    //let temp_pdf = format!("data/temp/{}.pdf", uuid::Uuid::new_v4().to_string());
    let temp_pdf = temp_dir!(format!("{}.pdf", uuid::Uuid::new_v4().to_string()))
        .to_string_lossy()
        .into_owned();

    // 将 xlsx 转换为 pdf
    doc_to_pdf::convert(&file_path, &temp_pdf)?;

    // 解析 pdf
    let mut record = record.clone();
    record.file_path = Some(temp_pdf.clone());
    parse_pdf(&record).await?;

    // 删除临时 pdf 文件
    fs::remove_file(temp_pdf)?;

    let file_content_type =
        KnowledgeBaseImportFileContentType::try_from(record.file_content_type.unwrap())?;
    match file_content_type {
        KnowledgeBaseImportFileContentType::Document => {
            // nothing
        }
        KnowledgeBaseImportFileContentType::Table => {
            // 知识库ID
            let kb_id = record.knowledge_base_id.unwrap();
            // 知识库对应的向量数据库表名
            let table_name = &get_table_name(kb_id).await?;

            // 读取文件
            let output = XlsxInput::read(file_path).map_err(|e| anyhow!(e.to_string()))?;

            let headers = output.headers;
            let rows = output.rows;

            // 创建数据表
            TableEngine::new_table_with_rows(table_name, &record.title.unwrap(), headers, rows)
                .await?;
        }
    }

    Ok(())
}

/// 解析 CSV 文件
pub(crate) async fn parse_csv(record: &KnowledgeBaseImportRecord) -> anyhow::Result<()> {
    // 文件路径
    let file_path = &record.file_path.clone().unwrap();
    // 创建临时 pdf 文件
    //let temp_pdf = format!("data/temp/{}.pdf", uuid::Uuid::new_v4().to_string());
    let temp_pdf = temp_dir!(format!("{}.pdf", uuid::Uuid::new_v4().to_string()))
        .to_string_lossy()
        .into_owned();

    // 将 csv 转换为 pdf
    doc_to_pdf::convert(&file_path, &temp_pdf)?;

    // 解析 pdf
    let mut record = record.clone();
    record.file_path = Some(temp_pdf.clone());
    parse_pdf(&record).await?;

    // 删除临时 pdf 文件
    fs::remove_file(temp_pdf)?;

    let file_content_type =
        KnowledgeBaseImportFileContentType::try_from(record.file_content_type.unwrap())?;
    match file_content_type {
        KnowledgeBaseImportFileContentType::Document => {
            // nothing
        }
        KnowledgeBaseImportFileContentType::Table => {
            // 知识库ID
            let kb_id = record.knowledge_base_id.unwrap();
            // 知识库对应的向量数据库表名
            let table_name = &get_table_name(kb_id).await?;

            // 读取文件
            let output = CsvInput::read(file_path).map_err(|e| anyhow!(e.to_string()))?;

            let headers = output.headers;
            let rows = output.rows;

            // 创建数据表
            TableEngine::new_table_with_rows(table_name, &record.title.unwrap(), headers, rows)
                .await?;
        }
    }

    Ok(())
}

/// 解析PPT文件
pub(crate) async fn parse_pptx(record: KnowledgeBaseImportRecord) -> anyhow::Result<()> {
    // 文件路径
    let file_path = &record.file_path.clone().unwrap();
    // 创建临时 pdf 文件
    //let temp_pdf = format!("data/temp/{}.pdf", uuid::Uuid::new_v4().to_string());
    let temp_pdf = temp_dir!(format!("{}.pdf", uuid::Uuid::new_v4().to_string()))
        .to_string_lossy()
        .into_owned();

    // 将 docx 转换为 pdf
    doc_to_pdf::convert(&file_path, &temp_pdf)?;
    // 解析 pdf
    // 解析 pdf
    let mut record = record.clone();
    record.file_path = Some(temp_pdf.clone());
    parse_pdf(&record).await?;

    // 删除临时 pdf 文件
    fs::remove_file(temp_pdf)?;

    Ok(())
}

/// 解析图片
pub(crate) async fn parse_image(record: &KnowledgeBaseImportRecord) -> anyhow::Result<()> {
    // 导入记录ID
    let id = record.id.unwrap();
    // 知识库ID
    let kb_id = record.knowledge_base_id.unwrap();
    // 文件路径
    let file_path = &record.file_path.clone().unwrap();
    // 知识库对应的向量数据库表名
    let table_name = &get_table_name(kb_id).await?;

    let extract_type = record.file_content_extract_type.clone().unwrap();
    let mut text = Some(String::new());
    match extract_type {
        KnowledgeBaseImportFileContentExtractType::Text => {
            bail!("仅抽取文本模式下无法解析图片");
        }
        KnowledgeBaseImportFileContentExtractType::Ocr => {
            text = Some(ocr::run(file_path)?);
        }
        KnowledgeBaseImportFileContentExtractType::VisionModel { model_id } => {
            let model = get_model(model_id).await?;
            text = image_to_text::extra(
                file_path,
                &model.base_url.unwrap(),
                &model.name.unwrap(),
                &model.api_key.unwrap_or_default(),
            )
            .await?;
        }
    }
    if text.is_none() || text.as_ref().unwrap().is_empty() {
        bail!("文本提取失败");
    }
    let mut data = Vec::new();
    for segment in TxtInput::split(text.unwrap()) {
        data.push(convert_to_vector_record(id, segment, None)?);
    }

    // let file_name = Path::new(&file_path).file_name().unwrap().to_str().unwrap();
    //
    // let (_, save_file, download_file) =
    //     make_kb_ref_file(table_name, record.id.unwrap(), file_name)?;
    //
    // // 移动图片到refs目录下
    // fs::copy(&file_path, &save_file)?;

    // 添加数据
    Engine::add_data(table_name, data).await?;

    Ok(())
}

/// 转换为向量库需要的数据格式
///
/// - knowledge_import_record_id: 知识库导入记录 id
/// - segment: 分段内容
/// - segment_ref_image: 分段引用的快照图片

fn convert_to_vector_record(
    knowledge_import_record_id: i64,
    segment: String,
    segment_ref_image: Option<Vec<String>>,
) -> anyhow::Result<AddRecordRequest> {
    let vector = Embeddings::embedding(EmbeddingInput::Text(segment.clone()))
        .map_err(|e| anyhow!(e.to_string()))?;
    Ok(AddRecordRequest {
        batch_id: knowledge_import_record_id.to_string(),
        vector,
        content: segment,
        content_type: "text".to_string(),
        content_ref: Some(ContentRef {
            images: segment_ref_image,
            urls: None,
        }),
        payload: None,
    })
}

async fn get_model(model_id: i64) -> anyhow::Result<Model> {
    let model = Model::select_by_map(Pool::get()?, value! {"id": model_id}).await?;
    if model.is_empty() {
        bail!("model not found");
    }
    Ok(model[0].clone())
}

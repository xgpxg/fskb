use arrow_array::types::Float32Type;
use arrow_array::{
    Array, ArrayRef, FixedSizeListArray, Float32Array, Int64Array, RecordBatch,
    RecordBatchIterator, StringArray,
};
use chrono::Duration;
use dashmap::DashMap;
use derive_builder::Builder;
use futures_util::TryStreamExt;
use itertools::Itertools;
use lancedb::arrow::arrow_schema::{ArrowError, DataType, Field, Schema, SchemaRef};
use lancedb::database::CreateTableMode;
use lancedb::query::{ExecutableQuery, QueryBase, Select};
use lancedb::table::{CompactionOptions, OptimizeAction};
use lancedb::{Connection, DistanceType, Table, connect};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, LazyLock};
use std::{fs, io};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("database error: {0}")]
    LancedbError(#[from] lancedb::Error),
    #[error("arrow error: {0}")]
    ArrowError(#[from] ArrowError),
    #[error("invalid parameter: {0}")]
    InvalidParameter(String),
    #[error("io error: {0}")]
    IoError(#[from] io::Error),
    #[error("unknown error")]
    Unknown,
}

type Result<T> = std::result::Result<T, Error>;
static TABLE_INSERT_COUNT: LazyLock<DashMap<String, i32>> = LazyLock::new(|| DashMap::new());
/// 表优化阈值，超过这个值时，执行一次优化
const TABLE_OPTIMIZE_THRESHOLD: i32 = 20;

pub struct Database {
    connection: Connection,
}

impl Database {
    /// 新建一个数据库
    /// - uri：数据存储路径
    pub async fn new(uri: &str) -> Result<Database> {
        let connection = connect(uri).execute().await?;
        Ok(Database { connection })
    }

    /// 创建一张空表
    ///
    /// 创建表时，LanceDB不支持在空表上建立索引，而是需要在达到一定数据量后需要手动建立索引，
    /// 但是目前测试来看，创建索引会很慢，所以暂时不创建索引，即使没有索引，再100万以下的数据量查询性能也在维持在1秒以内，
    /// 对于大部分场景应该是够的。
    /// - table_name：表名
    pub async fn create_empty_table(&self, table_name: &str) -> Result<Table> {
        let table_names = self.connection.table_names().execute().await?;
        if table_names.contains(&table_name.to_string()) {
            let table = self.connection.open_table(table_name).execute().await?;
            Ok(table)
        } else {
            let table = self
                .connection
                .create_empty_table(table_name, self.default_schema().await?)
                .mode(CreateTableMode::ExistOk(Box::new(|x| x)))
                .execute()
                .await?;
            Ok(table)
        }
    }

    /// 默认的schema
    async fn default_schema(&self) -> Result<Arc<Schema>> {
        let schema = Arc::new(Schema::new(vec![
            // 唯一标识
            Field::new("id", DataType::Int64, false),
            Field::new("prev", DataType::Int64, true),
            Field::new("next", DataType::Int64, true),
            // 特征向量：目前使用Clip模型，文本和图片的输出维度均为512
            Field::new(
                "vector",
                DataType::new_fixed_size_list(DataType::Float32, 512, false),
                false,
            ),
            // 原始内容：文本 或 图片地址 或 base64编码的图片
            Field::new("content", DataType::Utf8, false),
            // 内容类型：text | image
            Field::new("content_type", DataType::Utf8, false),
            // 参考(引用)内容
            Field::new("content_ref", DataType::Utf8, true),
            // 自定义数据
            Field::new("payload", DataType::Utf8, true),
            // 批次id：通常为UUID，用于标识同一个批次的数据，可按批次删除
            Field::new("batch_id", DataType::Utf8, false),
            // 创建时间：毫秒时间戳
            Field::new("create_time", DataType::Int64, false),
        ]));
        Ok(schema)
    }

    pub async fn drop_table(&self, table_name: &str) -> Result<()> {
        self.connection.drop_table(table_name).await?;
        Ok(())
    }

    /// 添加记录
    /// - table_name：表名
    /// - records：批量数据
    pub async fn add_records(
        &self,
        table_name: &str,
        records: Vec<AddRecordRequest>,
    ) -> Result<()> {
        let table = self.connection.open_table(table_name).execute().await?;
        let schema = table.schema().await?;

        let batch = self.convert_records(schema, records)?;
        table.add(batch).execute().await?;

        // 满足阈值时执行一次优化
        self.optimize_table(&table).await?;

        Ok(())
    }

    /// 优化表
    ///
    /// 在多次调用`table::add`后会产生大量小文件，导致磁盘写入缓慢，
    /// 需要执行优化，将小文件合并，提高写入性能。
    ///
    /// - table：表
    /// - threshold：阈值，当调用`table::add`次数超过该值时，执行一次优化。如果为0或负数则每次添加数据时都执行优化。
    async fn optimize_table(&self, table: &Table) -> Result<()> {
        let key = format!("{}:{}", self.connection.uri(), table.name());
        let count_ref = TABLE_INSERT_COUNT.get_mut(&key);
        if count_ref.is_some() {
            *count_ref.unwrap() += 1;
        } else {
            TABLE_INSERT_COUNT.insert(key.clone(), 1);
        }

        if *TABLE_INSERT_COUNT.get(&key).unwrap() < TABLE_OPTIMIZE_THRESHOLD {
            return Ok(());
        }

        *TABLE_INSERT_COUNT.get_mut(&key).unwrap() = 0;

        let compact = OptimizeAction::Compact {
            options: CompactionOptions::default(),
            remap_options: None,
        };
        // 清理文件
        let prune = OptimizeAction::Prune {
            // 立即清理合并前的文件
            older_than: Some(Duration::seconds(0)),
            delete_unverified: None,
            error_if_tagged_old_versions: None,
        };
        table.optimize(compact).await?;
        table.optimize(prune).await?;
        Ok(())
    }

    /// 检查指定的id是否存在
    /// - table_name：表名
    /// - id：唯一标识
    async fn exists(&self, table_name: &str, id: &str) -> Result<bool> {
        let exists = self.exists_ids(table_name, vec![id]).await?.len() > 0;
        Ok(exists)
    }

    /// 给定一组id列表，返回表中已存在的id列表
    /// - table_name：表名
    /// - ids：唯一标识
    async fn exists_ids(&self, table_name: &str, ids: Vec<&str>) -> Result<Vec<String>> {
        if ids.is_empty() {
            return Ok(vec![]);
        }
        let table = self.connection.open_table(table_name).execute().await?;
        let records = table
            .query()
            .only_if(&format!(
                "id in ({})",
                ids.iter()
                    .map(|x| format!("'{}'", x))
                    .collect::<Vec<_>>()
                    .join(",")
            ))
            .select(Select::columns(&["id"]))
            .execute()
            .await?
            .try_collect::<Vec<_>>()
            .await?;

        // 提取结果
        let record = if let Some(result) = records.get(0) {
            result
        } else {
            return Ok(vec![]);
        };

        // 结果数量
        let rows = record.num_rows();
        // id 列格式
        let ids = record.column_by_name("id").unwrap();

        // 提取id
        let mut exists_ids = Vec::new();
        for i in 0..rows {
            let id = ids.as_any().downcast_ref::<StringArray>().unwrap();
            let id = id.value(i);
            exists_ids.push(id.to_string());
        }

        Ok(exists_ids)
    }

    /// 删除记录
    ///
    /// 从表中删除指定id的数据，如果id不存在则忽略，仍然返回成功
    /// - table_name：表名
    /// - ids：唯一标识
    pub async fn delete_records(&self, table_name: &str, ids: Vec<String>) -> Result<()> {
        if ids.is_empty() {
            return Ok(());
        }
        let table = self.connection.open_table(table_name).execute().await?;
        table
            .delete(&format!(
                "id in ({})",
                ids.iter()
                    .map(|x| format!("'{}'", x))
                    .collect::<Vec<_>>()
                    .join(",")
            ))
            .await?;
        Ok(())
    }

    /// 删除记录
    ///
    /// 从表中删除指定id的数据，如果id不存在则忽略，仍然返回成功
    /// - table_name：表名
    /// - batch_ids：批次ID
    pub async fn delete_records_with_batch_id(
        &self,
        table_name: &str,
        batch_ids: Vec<String>,
    ) -> Result<()> {
        if batch_ids.is_empty() {
            return Ok(());
        }
        let table = self.connection.open_table(table_name).execute().await?;
        table
            .delete(&format!(
                "batch_id in ({})",
                batch_ids
                    .iter()
                    .map(|x| format!("'{}'", x))
                    .collect::<Vec<_>>()
                    .join(",")
            ))
            .await?;
        Ok(())
    }

    /// 更新单条数据
    ///
    /// 更新指定id的数据，如果id不存在则返回错误
    /// - table_name：表名
    /// - id：唯一标识
    /// - vector：特征向量，如果为None则不更新
    /// - payload：自定义数据，如果为None则不更新
    pub async fn update_record(
        &self,
        table_name: &str,
        id: String,
        vector: Option<Vec<f32>>,
        payload: Option<String>,
    ) -> Result<()> {
        let table = self.connection.open_table(table_name).execute().await?;

        if vector.is_none() && payload.is_none() {
            // 不存在对应的id，则返回错误
            if !self.exists(table_name, &id).await? {
                return Err(Error::InvalidParameter(format!(
                    "data not found with id: {}",
                    id
                )));
            }
            return Ok(());
        }

        let mut builder = table.update().only_if(&format!("id = '{}'", id));

        // 更新特征向量
        if let Some(vector) = vector {
            builder = builder.column(
                "vector",
                format!(
                    "[{}]",
                    vector
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                        .join(",")
                ),
            );
        }
        // 更新自定义数据
        if let Some(payload) = payload {
            builder = builder.column("payload", payload);
        }
        let update_rows = builder.execute().await?.rows_updated;
        if update_rows == 0 {
            return Err(Error::InvalidParameter(format!(
                "data not found with id: {}",
                id
            )));
        }
        Ok(())
    }

    /// 将记录转换为RecordBatch
    /// - schema：表的schema
    /// - records：原始数据
    fn convert_records(
        &self,
        schema: SchemaRef,
        records: Vec<AddRecordRequest>,
    ) -> Result<Box<RecordBatchIterator<Vec<std::result::Result<RecordBatch, ArrowError>>>>> {
        // 校验records
        if records.is_empty() {
            return Err(Error::InvalidParameter("records is empty".to_string()));
        }
        let first_vector_len = records[0].vector.len() as i32;
        if records
            .iter()
            .any(|x| x.vector.len() as i32 != first_vector_len)
        {
            return Err(Error::InvalidParameter(
                "vector length must be same".to_string(),
            ));
        }
        let mut p = (None, 1, Some(1));
        let db_records = records
            .iter()
            .enumerate()
            .map(|(index, record)| {
                let r = Record {
                    id: p.1,
                    prev: p.0,
                    next: p.2,
                    vector: record.vector.clone(),
                    content: record.content.clone(),
                    content_type: record.content_type.clone(),
                    content_ref: if let Some(ref content_ref) = record.content_ref {
                        Some(serde_json::to_string(content_ref).unwrap())
                    } else {
                        None
                    },
                    payload: record.payload.clone(),
                    batch_id: record.batch_id.clone(),
                };
                if index == records.len() - 1 {
                    p.2 = None;
                }
                p = (Some(r.id), r.next.unwrap(), Some(r.next.unwrap() + 1));
                r
            })
            .collect::<Vec<_>>();

        let batch = RecordBatch::try_new(
            schema,
            vec![
                // id
                Arc::new(Int64Array::from(
                    db_records.iter().map(|x| x.id).collect::<Vec<_>>(),
                )),
                // prev
                Arc::new(Int64Array::from(
                    db_records.iter().map(|x| x.prev).collect::<Vec<_>>(),
                )),
                // next
                Arc::new(Int64Array::from(
                    db_records.iter().map(|x| x.next).collect::<Vec<_>>(),
                )),
                // vector
                Arc::new(
                    FixedSizeListArray::from_iter_primitive::<Float32Type, _, _>(
                        db_records
                            .iter()
                            .map(|x| Some(x.vector.iter().map(|v| Some(*v)).collect::<Vec<_>>()))
                            .collect::<Vec<_>>(),
                        first_vector_len,
                    ),
                ),
                // content
                Arc::new(StringArray::from(
                    db_records
                        .iter()
                        .map(|x| x.content.as_str())
                        .collect::<Vec<_>>(),
                )),
                // content_type
                Arc::new(StringArray::from(
                    db_records
                        .iter()
                        .map(|x| x.content_type.as_str())
                        .collect::<Vec<_>>(),
                )),
                // content_ref
                Arc::new(StringArray::from(
                    db_records
                        .iter()
                        .map(|x| x.content_ref.clone())
                        .collect::<Vec<_>>(),
                )),
                // payload
                Arc::new(StringArray::from(
                    db_records
                        .iter()
                        .map(|x| x.payload.clone())
                        .collect::<Vec<_>>(),
                )),
                // batch_id
                Arc::new(StringArray::from(
                    db_records
                        .iter()
                        .map(|x| x.batch_id.as_str())
                        .collect::<Vec<_>>(),
                )),
                // create_time
                Arc::new(Int64Array::from(
                    db_records
                        .iter()
                        .map(|_| chrono::Local::now().timestamp_millis())
                        .collect::<Vec<_>>(),
                )),
            ],
        )?;
        let schema = batch.schema().clone();
        let batch = Ok(batch);
        let batch = Box::new(RecordBatchIterator::new(vec![batch], schema));
        Ok(batch)
    }

    /// 搜索
    /// - table_name：表名
    /// - id：唯一标识，如果指定，则只搜索该记录
    /// - vector：特征向量
    /// - limit：搜索结果数量，默认10
    /// - offset：搜索结果偏移量
    /// - min_score：最小分数，如果指定，则只返回分数大于等于该值的记录，取值范围`[0,1]`
    pub async fn search(&self, search_request: SearchRequest) -> Result<Vec<SearchResult>> {
        let table_name = search_request.table_name;
        let table = self
            .connection
            .open_table(table_name.clone())
            .execute()
            .await?;
        let is_vector_search = search_request.vector.is_some();
        let min_score = if let Some(min_score) = search_request.min_score {
            Some((1. - min_score) * 2.)
        } else {
            None
        };
        let mut query = table.query();
        let limit = search_request.limit.unwrap_or(10);
        let offset = search_request.offset.unwrap_or(0);
        query = query.limit(limit);
        query = query.offset(offset);
        // 这应该是LanceDB的一个bug，不使用向量查询和使用向量查询时的offset表现不一致
        // 使用向量查询时：offset需要加上limit，否则限制的limit数量不会返回
        // 不使用向量查询时：offset和limit表现符合预期
        if is_vector_search {
            query = query.offset(offset).limit(offset + limit);
        }

        if let Some(id) = search_request.id {
            query = query.only_if(format!("id = '{}'", id));
        }
        if let Some(batch_id) = search_request.batch_id {
            query = query.only_if(format!("batch_id = '{}'", batch_id));
        }
        let record_batches = match search_request.vector {
            None => query.execute().await?.try_collect::<Vec<_>>().await?,
            Some(vector) => {
                query
                    // 临近搜索
                    .nearest_to(vector)?
                    .column("vector")
                    // 使用余弦相似度
                    .distance_type(DistanceType::Cosine)
                    .distance_range(None, min_score)
                    .with_row_id()
                    .execute()
                    .await?
                    .try_collect::<Vec<_>>()
                    .await?
            }
        };

        let records =
            Self::convert_record_batch_to_search_result(record_batches, is_vector_search)?;

        if search_request.context_size.is_none() || search_request.context_size.unwrap() == 0 {
            return Ok(records);
        }

        let mut result = vec![];
        let group = records
            .into_iter()
            .into_group_map_by(|x| x.batch_id.clone());
        for (batch_id, records) in group.into_iter() {
            let list = self
                .extend_context(
                    &table_name,
                    &batch_id,
                    &records,
                    search_request.context_size.unwrap_or(0),
                )
                .await?;
            result.extend(list);
        }

        result.sort_by(|a, b| {
            return if a.score == b.score {
                a.id.cmp(&b.id)
            } else {
                b.score.partial_cmp(&a.score).unwrap()
            };
        });

        Ok(result)
    }

    fn convert_record_batch_to_search_result(
        record_batches: Vec<RecordBatch>,
        is_vector_search: bool,
    ) -> Result<Vec<SearchResult>> {
        let mut records = Vec::new();

        // 遍历每一个batch，每个batch中包含多行记录
        // 调用add时会产生一个batch
        for record in record_batches {
            // 结果数量
            let rows = record.num_rows();
            // id 列格式
            let ids = record.column_by_name("id").unwrap();
            let prev_s = record.column_by_name("prev").unwrap();
            let next_s = record.column_by_name("next").unwrap();
            // batch_ids 列格式
            let batch_ids = record.column_by_name("batch_id").unwrap();
            let contents = record.column_by_name("content").unwrap();
            let content_types = record.column_by_name("content_type").unwrap();
            let content_refs = record.column_by_name("content_ref").unwrap();
            // payload 列格式
            let payloads = record.column_by_name("payload").unwrap();
            // create_times 列格式
            let create_times = record.column_by_name("create_time").unwrap();

            // distance 列格式
            let default_distances = &(Arc::new(Float32Array::new_null(rows)) as ArrayRef);
            let distances = record
                .column_by_name("_distance")
                .unwrap_or(default_distances);

            for i in 0..rows {
                let id = ids.as_any().downcast_ref::<Int64Array>().unwrap();
                let prev = prev_s.as_any().downcast_ref::<Int64Array>().unwrap();
                let next = next_s.as_any().downcast_ref::<Int64Array>().unwrap();
                let batch_id = batch_ids.as_any().downcast_ref::<StringArray>().unwrap();
                let content = contents.as_any().downcast_ref::<StringArray>().unwrap();
                let content_type = content_types
                    .as_any()
                    .downcast_ref::<StringArray>()
                    .unwrap();
                let content_ref = content_refs.as_any().downcast_ref::<StringArray>().unwrap();
                let payload = payloads.as_any().downcast_ref::<StringArray>().unwrap();
                let create_time = create_times.as_any().downcast_ref::<Int64Array>().unwrap();
                let distance = distances.as_any().downcast_ref::<Float32Array>().unwrap();

                let id = id.value(i);
                let prev = prev.value(i);
                let next = next.value(i);
                let batch_id = batch_id.value(i);
                let content = content.value(i);
                let content_type = content_type.value(i);
                let content_ref = content_ref.value(i);
                let payload = payload.value(i);
                let create_time = create_time.value(i);
                let score = if is_vector_search {
                    Some(1. - distance.value(i) / 2.)
                } else {
                    None
                };

                records.push(SearchResult {
                    id,
                    prev: Some(prev),
                    next: Some(next),
                    payload: Some(payload.to_string()),
                    score,
                    batch_id: batch_id.to_string(),
                    content: content.to_string(),
                    content_type: content_type.to_string(),
                    content_ref: serde_json::from_str(content_ref).unwrap(),
                    create_time,
                });
            }
        }

        Ok(records)
    }

    /// 扩展上下文。数据使用链式存储，在此向前和向后寻找context_size条数据用于扩展上下文
    async fn extend_context(
        &self,
        table_name: &str,
        batch_id: &String,
        records: &Vec<SearchResult>,
        context_size: usize,
    ) -> Result<Vec<SearchResult>> {
        let table = self.connection.open_table(table_name).execute().await?;
        let context_records_batches = table
            .query()
            .only_if(format!(
                "batch_id = '{}' and id in ({})",
                batch_id,
                records
                    .iter()
                    .flat_map(|r| {
                        let mut ids = vec![];
                        for i in 1..=context_size {
                            ids.push((r.id - i as i64).to_string());
                            ids.push((r.id + i as i64).to_string());
                        }
                        ids
                    })
                    .join(",")
            ))
            .execute()
            .await?
            .try_collect::<Vec<_>>()
            .await?;

        let context_records =
            Self::convert_record_batch_to_search_result(context_records_batches, false)?;

        let mapping: HashMap<i64, &SearchResult> =
            HashMap::from_iter(context_records.iter().map(|r| (r.id, r)));

        let list = records
            .into_iter()
            .map(|record| {
                let id = record.id;
                let mut record = record.clone();
                for i in 1..=context_size {
                    if let Some(prev) = mapping.get(&(id - i as i64)) {
                        record.content = format!("{}{}", prev.content, record.content);
                    }
                    if let Some(next) = mapping.get(&(id + i as i64)) {
                        record.content = format!("{}{}", record.content, next.content);
                    }
                }
                record
            })
            .collect::<Vec<_>>();

        Ok(list)
    }
    /// 获取表的总行数
    /// - table_name：表名
    pub async fn total_rows(&self, table_name: &str) -> Result<usize> {
        let table = self.connection.open_table(table_name).execute().await?;
        let count = table.count_rows(None).await?;
        Ok(count)
    }

    pub async fn disk_usage(&self, table_name: &str) -> Result<usize> {
        let table = self.connection.open_table(table_name).execute().await?;
        let uri = table.dataset_uri();
        // 统计uri目录占用空间大小
        let usage = fs::metadata(uri)?.len();
        Ok(usage as usize)
    }

    /// 获取数据库所有表名
    pub async fn table_names(&self) -> Result<Vec<String>> {
        let table_names = self.connection.table_names().execute().await?;
        Ok(table_names)
    }

    /// 获取数据库信息
    pub async fn database_info(&self) -> Result<DatabaseInfo> {
        let table_names = self.connection.table_names().execute().await?;
        let mut tables = Vec::new();
        for table_name in &table_names {
            let table = self.connection.open_table(table_name).execute().await?;
            let count = table.count_rows(None).await?;
            tables.push(TableInfo {
                table_name: table_name.clone(),
                total_rows: count,
                disk_usage: self.disk_usage(table_name).await?,
            });
        }
        Ok(DatabaseInfo {
            uri: self.connection.uri().to_string(),
            table_names,
            total_rows: tables.iter().map(|x| x.total_rows).sum(),
            tables,
        })
    }

    pub async fn table_info(&self, table_name: &str) -> Result<TableInfo> {
        let table = self.connection.open_table(table_name).execute().await?;
        let count = table.count_rows(None).await?;
        Ok(TableInfo {
            table_name: table_name.to_string(),
            total_rows: count,
            disk_usage: self.disk_usage(table_name).await?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Record {
    id: i64,
    prev: Option<i64>,
    next: Option<i64>,
    /// 特征向量
    pub vector: Vec<f32>,
    /// 原始内容
    pub content: String,
    /// 内容类型：text | image
    pub content_type: String,
    /// 内容引用
    pub content_ref: Option<String>,
    /// 自定义数据
    pub payload: Option<String>,
    /// 批次id
    pub batch_id: String,
}

#[derive(Debug, Clone)]
pub struct AddRecordRequest {
    /// 批次id
    pub batch_id: String,
    /// 特征向量
    pub vector: Vec<f32>,
    /// 原始内容
    pub content: String,
    /// 内容类型：text | image
    pub content_type: String,
    /// 内容引用
    pub content_ref: Option<ContentRef>,
    /// 自定义数据
    pub payload: Option<String>,
}

/// 内容引用
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentRef {
    /// 引用的图片
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<String>>,
    /// 引用的链接
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urls: Option<String>,
}

#[derive(Debug, Clone, Builder, Default)]
#[builder(default)]
pub struct SearchRequest {
    /// 表名
    pub table_name: String,
    /// 唯一标识
    pub id: Option<i64>,
    /// 批次id，对应业务ID，同一个文件的批次ID应该保持一致
    pub batch_id: Option<String>,
    /// 搜索关键词的特征向量
    pub vector: Option<Vec<f32>>,
    /// 上下文扩充长度，默认为0
    pub context_size: Option<usize>,
    /// 返回条数，默认为10
    pub limit: Option<usize>,
    /// 偏移量，默认为0，注意不是页数
    pub offset: Option<usize>,
    /// 最小匹配度，为空时不限制
    pub min_score: Option<f32>,
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    /// 唯一标识
    pub id: i64,
    /// 前一条数据
    pub prev: Option<i64>,
    /// 后一条数据
    pub next: Option<i64>,
    /// 自定义数据
    pub payload: Option<String>,
    /// 匹配度
    pub score: Option<f32>,
    /// 批次id
    pub batch_id: String,
    /// 原始内容
    pub content: String,
    /// 内容类型：text | image
    pub content_type: String,
    /// 内容引用
    pub content_ref: Option<ContentRef>,
    /// 创建时间
    pub create_time: i64,
}

pub struct DatabaseInfo {
    pub uri: String,
    pub table_names: Vec<String>,
    pub total_rows: usize,
    pub tables: Vec<TableInfo>,
}

pub struct TableInfo {
    pub table_name: String,
    pub total_rows: usize,
    pub disk_usage: usize,
}

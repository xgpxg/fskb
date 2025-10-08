use common::resources_dir;
use fastembed::{
    ImageEmbedding, ImageInitOptionsUserDefined, InitOptionsUserDefined,
    RerankInitOptionsUserDefined, TextEmbedding, TextRerank, TokenizerFiles,
    UserDefinedEmbeddingModel, UserDefinedImageEmbeddingModel, UserDefinedRerankingModel,
};
use image::DynamicImage;
use std::sync::{LazyLock, Mutex};
use std::{env, fs};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub enum EmbeddingInput {
    Text(String),
    Image(DynamicImage),
}
pub trait Embedding {
    fn embedding(input: EmbeddingInput) -> Result<Vec<f32>>;
}

#[allow(unused)]
static TEXT_TO_IMAGE_MODEL: LazyLock<TextEmbedding> = LazyLock::new(|| {
    let current_dir = env::current_exe().unwrap();
    let model_path = &resources_dir!("model", "clip-ViT-B-32-text")
        .to_string_lossy()
        .into_owned(); //"resources/model/clip-ViT-B-32-text";
    let tokenizer_files = TokenizerFiles {
        tokenizer_file: fs::read(format!("{}/tokenizer.json", model_path))
            .expect("tokenizer file load fail"),
        config_file: fs::read(format!("{}/config.json", model_path))
            .expect("config file load fail"),
        special_tokens_map_file: fs::read(format!("{}/special_tokens_map.json", model_path))
            .expect("special tokens map file load fail"),
        tokenizer_config_file: fs::read(format!("{}/tokenizer_config.json", model_path))
            .expect("tokenizer config file load fail"),
    };
    let model = UserDefinedEmbeddingModel::new(
        fs::read(format!("{}/model.onnx", model_path)).expect("model load fail"),
        tokenizer_files,
    );
    let model = TextEmbedding::try_new_from_user_defined(model, InitOptionsUserDefined::default())
        .expect("Failed to initialize text embedding model");
    model
});

static TEXT_MODEL: LazyLock<Mutex<TextEmbedding>> = LazyLock::new(|| {
    let current_dir = env::current_exe().unwrap();
    let model_path = &resources_dir!("model", "bge-small-zh-v1_5")
        .to_string_lossy()
        .into_owned(); //"resources/model/bge-small-zh-v1_5";
    let tokenizer_files = TokenizerFiles {
        tokenizer_file: fs::read(format!("{}/tokenizer.json", model_path))
            .expect("tokenizer file load fail"),
        config_file: fs::read(format!("{}/config.json", model_path))
            .expect("config file load fail"),
        special_tokens_map_file: fs::read(format!("{}/special_tokens_map.json", model_path))
            .expect("special tokens map file load fail"),
        tokenizer_config_file: fs::read(format!("{}/tokenizer_config.json", model_path))
            .expect("tokenizer config file load fail"),
    };
    let model = UserDefinedEmbeddingModel::new(
        fs::read(format!("{}/model.onnx", model_path)).expect("model load fail"),
        tokenizer_files,
    );

    let model = TextEmbedding::try_new_from_user_defined(model, InitOptionsUserDefined::default())
        .expect("Failed to initialize text embedding model");
    Mutex::new(model)
});

static IMAGE_MODEL: LazyLock<Mutex<ImageEmbedding>> = LazyLock::new(|| {
    let current_dir = env::current_exe().unwrap();
    let model_path = &resources_dir!("model", "clip-ViT-B-32-vision")
        .to_string_lossy()
        .into_owned(); //"resources/model/clip-ViT-B-32-vision";
    let model = UserDefinedImageEmbeddingModel::new(
        fs::read(format!("{}/model.onnx", model_path)).expect("model load fail"),
        fs::read(format!("{}/preprocessor_config.json", model_path))
            .expect("preprocessor config file load fail"),
    );
    let model =
        ImageEmbedding::try_new_from_user_defined(model, ImageInitOptionsUserDefined::default())
            .expect("Failed to initialize image embedding model");
    Mutex::new(model)
});

static TEXT_RERANK_MODEL: LazyLock<Mutex<TextRerank>> = LazyLock::new(|| {
    let current_dir = env::current_exe().unwrap();
    let model_path = &resources_dir!("model", "jina-reranker-v2-base-multilingual")
        .to_string_lossy()
        .into_owned(); // "resources/model/jina-reranker-v2-base-multilingual";

    let tokenizer_files = TokenizerFiles {
        tokenizer_file: fs::read(format!("{}/tokenizer.json", model_path))
            .expect("tokenizer file load fail"),
        config_file: fs::read(format!("{}/config.json", model_path))
            .expect("config file load fail"),
        special_tokens_map_file: fs::read(format!("{}/special_tokens_map.json", model_path))
            .expect("special tokens map file load fail"),
        tokenizer_config_file: fs::read(format!("{}/tokenizer_config.json", model_path))
            .expect("tokenizer config file load fail"),
    };

    let model = UserDefinedRerankingModel::new(
        fs::read(format!("{}/model.onnx", model_path)).expect("model load fail"),
        tokenizer_files,
    );
    let model =
        TextRerank::try_new_from_user_defined(model, RerankInitOptionsUserDefined::default())
            .expect("Failed to initialize text rerank model");
    Mutex::new(model)
});

pub async fn init() {
    // let _ = TEXT_MODEL;
    // let _ = IMAGE_MODEL;
    // let _ = TEXT_RERANK_MODEL;

    let dylib_dir = resources_dir!("dylib");
    #[cfg(target_os = "windows")]
    unsafe {
        //env::set_var("ORT_DYLIB_PATH", "resources/dylib/windows/onnxruntime.dll");
        env::set_var(
            "ORT_DYLIB_PATH",
            dylib_dir
                .join("windows")
                .join("onnxruntime.dll")
                .to_string_lossy()
                .as_ref(),
        );
    }
}

pub struct Embeddings;
impl Embedding for Embeddings {
    fn embedding(input: EmbeddingInput) -> Result<Vec<f32>> {
        match input {
            EmbeddingInput::Text(text) => {
                let embeddings = TEXT_MODEL.lock()?.embed(vec![text], None)?;
                Ok(embeddings[0].clone())
            }
            EmbeddingInput::Image(image) => {
                let embeddings = IMAGE_MODEL.lock()?.embed_images(vec![image])?;
                Ok(embeddings[0].clone())
            }
        }
    }
}

pub struct Reranker;
impl Reranker {
    pub fn rerank<S: AsRef<str> + Send + Sync>(
        question: S,
        documents: Vec<S>,
    ) -> Result<Vec<(usize, f32)>> {
        let results = TEXT_RERANK_MODEL
            .lock()?
            .rerank(question, documents, false, None)?;
        Ok(results
            .into_iter()
            .map(|result| (result.index, result.score))
            .collect::<Vec<(usize, f32)>>())
    }
}

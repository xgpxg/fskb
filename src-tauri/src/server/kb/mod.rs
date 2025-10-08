use crate::constant;
use crate::db::model::knowledge_base::{KnowledgeBase, KnowledgeBaseConfig};
use embedding::{Embedding, EmbeddingInput, Embeddings, Reranker};
use engine::Engine;

pub(crate) mod commands;
mod parse;
pub(crate) mod request;
mod response;
mod search;
mod service;

pub(crate) use search::search;

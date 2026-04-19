use fastembed::SparseTextEmbedding;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::config::Config;
use crate::retrieval::qdrant::VectorStore;

#[derive(Clone)]
pub struct AppState {
    pub cfg: Arc<Config>,
    pub http: reqwest::Client,
    pub sparse: Arc<Mutex<SparseTextEmbedding>>,
    pub store: Arc<dyn VectorStore>,
}

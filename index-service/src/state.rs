use fastembed::SparseTextEmbedding;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct AppState {
    pub sparse: Arc<Mutex<SparseTextEmbedding>>,
}

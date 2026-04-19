use fastembed::SparseTextEmbedding;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::errors::AppError;
use crate::retrieval::qdrant::SparseQuery;

pub async fn embed(
    model: Arc<Mutex<SparseTextEmbedding>>,
    text: &str,
) -> Result<SparseQuery, AppError> {
    let mut m = model.lock().await;
    let mut result = m
        .embed(vec![text.to_string()], None)
        .map_err(|e| AppError::Sparse(e.to_string()))?;
    let first = result
        .pop()
        .ok_or_else(|| AppError::Sparse("empty result".into()))?;
    Ok(SparseQuery {
        indices: first.indices.into_iter().map(|i| i as u32).collect(),
        values: first.values,
    })
}

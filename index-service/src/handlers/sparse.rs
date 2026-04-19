use axum::{
    extract::{Json, State},
    response::IntoResponse,
};
use shared::{SparseEmbeddingRequest, SparseEmbeddingResponse, SparseVector};

use crate::errors::AppError;
use crate::state::AppState;

pub async fn handle_sparse_embedding(
    State(state): State<AppState>,
    Json(payload): Json<SparseEmbeddingRequest>,
) -> Result<impl IntoResponse, AppError> {
    let mut model = state.sparse.lock().await;
    let embeddings = model
        .embed(payload.texts, None)
        .map_err(|e| AppError::SparseEmbedding(e.to_string()))?;

    let vectors = embeddings
        .into_iter()
        .map(|e| SparseVector {
            indices: e.indices.into_iter().map(|i| i as u32).collect(),
            values: e.values,
        })
        .collect();
    Ok(Json(SparseEmbeddingResponse { vectors }))
}

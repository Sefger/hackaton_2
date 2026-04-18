use axum::{extract::State, Json, response::IntoResponse};
use std::sync::Arc;
use tokio::sync::Mutex; // Убедись, что это Tokio Mutex!
use fastembed::{SparseTextEmbedding, SparseModel};
use shared::{
    IndexAPIRequest, IndexAPIResponse,
    SparseEmbeddingRequest, SparseEmbeddingResponse // Добавь импорт явно
};
use crate::chunker;

pub fn init_sparse_model() -> SparseTextEmbedding {
    SparseTextEmbedding::try_new(
        fastembed::SparseInitOptions::new(SparseModel::BGEM3)
            .with_show_download_progress(true),
    ).expect("Failed to load BGE-M3 model")
}

// 1. Обязательно добавь PUB, чтобы main.rs его видел
pub async fn handle_index(
    Json(payload): Json<IndexAPIRequest>,
) -> impl IntoResponse {
    let chunks = chunker::process_to_chunks(payload.data);
    Json(IndexAPIResponse { results: chunks })
}

// 2. Исправляем работу с Mutex
pub async fn handle_sparse_embedding(
    State(model): State<Arc<Mutex<SparseTextEmbedding>>>,
    Json(payload): Json<SparseEmbeddingRequest>,
) -> impl IntoResponse {
    // lock() у Tokio Mutex асинхронный, поэтому .await нужен
    let mut model_lock = model.lock().await;

    let embeddings = model_lock.embed(payload.texts, None)
        .expect("Failed to generate sparse embeddings");

    let vectors = embeddings.into_iter().map(|item| {
        shared::SparseVector {
            indices: item.indices.into_iter().map(|i| i as u32).collect(),
            values: item.values,
        }
    }).collect();

    // Теперь SparseEmbeddingResponse должен быть виден
    Json(SparseEmbeddingResponse { vectors })
}
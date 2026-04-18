use axum::{
    extract::{Json, State},
    response::IntoResponse,
};
use std::sync::Arc;
use tokio::sync::Mutex;

use fastembed::{SparseInitOptions, SparseModel, SparseTextEmbedding}; // Импортируем именно SparseModel
use shared::{
    IndexAPIRequest, IndexAPIResponse, IndexAPIResultItem, SparseEmbeddingRequest,
    SparseEmbeddingResponse, SparseVector,
};
use std::path::PathBuf;
pub fn init_sparse_model() -> SparseTextEmbedding {
    let model_path = PathBuf::from("/app/models");

    // Ошибка E0639 лечится использованием методов-сеттеров или Default + мутация,
    // так как структуру с #[non_exhaustive] нельзя инициализировать через литерал.
    let mut options = SparseInitOptions::default();
    options.model_name = SparseModel::BGEM3; // Используем SparseModel вместо EmbeddingModel
    options.cache_dir = model_path;
    options.show_download_progress = false;

    SparseTextEmbedding::try_new(options)
        .expect("Failed to initialize Sparse Model. Ensure weights are in the image.")
}

pub async fn handle_index(Json(payload): Json<IndexAPIRequest>) -> impl IntoResponse {
    let chat = &payload.data.chat;
    let messages = &payload.data.new_messages;

    let mut results = Vec::new();
    // Группируем сообщения для сохранения контекста (Chunking)
    for chunk in messages.chunks(10) {
        let message_ids: Vec<String> = chunk.iter().map(|m| m.id.clone()).collect();
        let combined_text: String = chunk
            .iter()
            .map(|m| format!("{}: {}", m.sender_id, m.text))
            .collect::<Vec<_>>()
            .join("\n");

        let context = format!("Чат: {} | {}", chat.name, combined_text);

        results.push(IndexAPIResultItem {
            page_content: combined_text,
            dense_content: context.clone(),
            sparse_content: context,
            message_ids,
        });
    }

    Json(IndexAPIResponse { results })
}

pub async fn handle_sparse_embedding(
    State(model): State<Arc<Mutex<SparseTextEmbedding>>>,
    Json(payload): Json<SparseEmbeddingRequest>,
) -> impl IntoResponse {
    let mut model_lock = model.lock().await;

    // Используем метод .embed() из того кода, что ты прислал
    match model_lock.embed(payload.texts, None) {
        Ok(embeddings) => {
            let vectors = embeddings
                .into_iter()
                .map(|e| SparseVector {
                    // В твоем lib.rs indices: Vec<u32>, а в fastembed это Vec<usize>
                    indices: e.indices.into_iter().map(|i| i as u32).collect(),
                    values: e.values,
                })
                .collect();
            Json(SparseEmbeddingResponse { vectors }).into_response()
        }
        Err(_) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            "Embedding Error",
        )
            .into_response(),
    }
}

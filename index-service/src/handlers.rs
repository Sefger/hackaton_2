use axum::{
    extract::State,
    Json,
    response::IntoResponse,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use fastembed::{SparseTextEmbedding, SparseInitOptions, SparseModel};
use shared::{
    IndexAPIRequest, IndexAPIResponse,
    SparseEmbeddingRequest, SparseEmbeddingResponse, SparseVector
};
use crate::chunker::{create_chunks, ChunkerConfig};

/// Инициализация модели.
/// Используем Splade_PP_En_Distil (Snake Case), так как это стандарт для версии 5.13.x
pub fn init_sparse_model() -> SparseTextEmbedding {
    // В версии 5.13.2 это наиболее вероятное имя для BGE-SVO v2
    // Если не сработает, попробуй: SparseModel::SpladePpEnDistil
    let model_info = SparseModel::BGEM3;

    SparseTextEmbedding::try_new(
        SparseInitOptions::new(model_info)
            .with_show_download_progress(true),
    ).expect("Не удалось загрузить модель Sparse-эмбеддингов")
}

/// Обработчик индексации: принимает сообщения и нарезает их на чанки для БД
pub async fn handle_index(
    Json(payload): Json<IndexAPIRequest>
) -> impl IntoResponse {
    let config = ChunkerConfig {
        window_size: 10,
        overlap: 3,
    };

    let chunks = create_chunks(
        &payload.data.new_messages,
        &payload.data.overlap_messages,
        &config
    );

    Json(IndexAPIResponse { results: chunks })
}

/// Обработчик генерации разреженных (sparse) эмбеддингов.
/// Использует Mutex, так как инференс модели требует эксклюзивного доступа (&mut self).
pub async fn handle_sparse_embedding(
    State(model): State<Arc<Mutex<SparseTextEmbedding>>>,
    Json(payload): Json<SparseEmbeddingRequest>,
) -> impl IntoResponse {

    // Блокируем доступ для текущего потока
    let mut model_lock = model.lock().await;

    // Генерируем векторы. batch_size установлен в Some(1) для простоты
    let output = model_lock.embed(payload.texts, Some(1))
        .expect("Ошибка при генерации векторов");

    let vectors = output.into_iter().map(|v| {
        SparseVector {
            // Конвертируем usize в i32 согласно контракту shared
            indices: v.indices.into_iter().map(|i| i as i32).collect(),
            values: v.values.into_iter().map(|f| f as f64).collect(),
        }
    }).collect();

    Json(SparseEmbeddingResponse { vectors })
}
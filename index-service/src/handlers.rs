use crate::chunker::{create_chunks, ChunkerConfig};
use axum::Json;
use shared::IndexAPIRequest;
use shared::IndexAPIResponse;
use axum::response::IntoResponse;
pub async fn handle_index(Json(payload): Json<IndexAPIRequest>) -> impl IntoResponse {
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
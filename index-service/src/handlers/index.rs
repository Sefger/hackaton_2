use axum::{extract::Json, response::IntoResponse};
use shared::{IndexAPIRequest, IndexAPIResponse};

use crate::chunker;

pub async fn handle_index(Json(payload): Json<IndexAPIRequest>) -> impl IntoResponse {
    let results = chunker::chunk(&payload.data);
    Json(IndexAPIResponse { results })
}

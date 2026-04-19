use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("qdrant error: {0}")]
    Qdrant(String),
    #[error("upstream http error ({status}): {body}")]
    Upstream { status: u16, body: String },
    #[error("response parsing: {0}")]
    Parse(String),
    #[error("sparse embedding failed: {0}")]
    Sparse(String),
    #[error("internal: {0}")]
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::error!(error = %self, "request failed");
        let code = StatusCode::INTERNAL_SERVER_ERROR;
        (code, Json(json!({ "error": self.to_string() }))).into_response()
    }
}

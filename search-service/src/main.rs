mod pipeline;
use axum::{
    routing::get,
    Router,
    response::IntoResponse,
    Json,
    http::StatusCode,
};
use std::net::SocketAddr;
use serde_json::json;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/health", get(health_handler))
        .route("/search", axum::routing::post(search_handler));

    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .unwrap_or(8080);

    let addr = format!("{}:{}", host, port).parse::<SocketAddr>().unwrap();

    println!("🚀 Search Service listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("Search Service running on http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}
pub struct AppState { // Добавь pub
    pub qdrant: qdrant_client::Qdrant,
    pub index_service_url: String,
    pub http_client: reqwest::Client,
}
async fn health_handler() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}

async fn search_handler() -> impl IntoResponse {
    // Пока заглушка
    Json(json!({
        "results": []
    }))
}
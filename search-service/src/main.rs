mod handlers;
mod pipeline;

use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;
// В версии 1.17 используем Qdrant вместо QdrantClient
use qdrant_client::Qdrant;

pub struct AppState {
    pub qdrant: Qdrant, // Тип теперь Qdrant
    pub index_service_url: String,
    pub http_client: reqwest::Client,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let qdrant_url = std::env::var("QDRANT_URL").unwrap_or("http://localhost:6334".into());
    let index_url = std::env::var("INDEX_SERVICE_URL").unwrap_or("http://localhost:3000".into());

    // Инициализация через Qdrant::from_url
    let qdrant = Qdrant::from_url(&qdrant_url)
        .build()
        .expect("Failed to connect to Qdrant");

    let state = Arc::new(AppState {
        qdrant,
        index_service_url: index_url,
        http_client: reqwest::Client::new(),
    });

    let app = Router::new()
        .route("/health", get(|| async { "OK" }))
        .route("/search", post(handlers::handle_search))
        .with_state(state);

    let addr = "0.0.0.0:3001";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("Search Service running on http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}
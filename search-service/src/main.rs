mod handlers;
mod pipeline;

use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use qdrant_client::Qdrant;

pub struct AppState {
    pub qdrant: Qdrant,
    pub index_service_url: String,
    pub http_client: reqwest::Client,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // Берем URL из переменных окружения (важно для Docker)
    let qdrant_url = std::env::var("QDRANT_URL").unwrap_or("http://localhost:6334".into());
    let index_url = std::env::var("INDEX_SERVICE_URL").unwrap_or("http://localhost:3000".into());

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
        // Фронтенд шлет данные сюда для поиска
        .route("/search", post(handlers::handle_search))
        // Фронтенд шлет данные сюда для добавления в базу (Pipeline)
        .route("/index", post(handlers::handle_index))
        .with_state(state);

    // Порт 3001 для поиска
    let addr = "0.0.0.0:3001";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("Search Service running on http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}
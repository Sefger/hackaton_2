mod chunker;
mod handlers;

use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    println!("Initializing Index Service (Port 3000)...");

    // Инициализация модели BGE-M3 (через fastembed)
    let model = Arc::new(Mutex::new(handlers::init_sparse_model()));

    let app = Router::new()
        .route("/health", get(|| async { "OK" }))
        .route("/index", post(handlers::handle_index))
        .route("/sparse_embedding", post(handlers::handle_sparse_embedding))
        .with_state(model);

    // ВАЖНО: Порт 3000 для индексации
    let addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("Index Service is running on http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}
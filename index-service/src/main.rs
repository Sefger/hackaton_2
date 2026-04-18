mod handlers;
mod chunker;

use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Инициализируем логирование
    tracing_subscriber::fmt::init();

    println!("🚀 Starting Index Service...");

    // 1. Инициализируем модель BGE-M3 (через твой handlers.rs)
    // Оборачиваем в Mutex, так как fastembed требует &mut self для генерации векторов
    let model = Arc::new(Mutex::new(handlers::init_sparse_model()));

    // 2. Настраиваем роутер
    let app = Router::new()
        .route("/health", get(health_handler))
        // Используем реальные обработчики из модуля handlers
        .route("/index", post(handlers::handle_index))
        .route("/sparse_embedding", post(handlers::handle_sparse_embedding))
        // Прокидываем состояние с моделью
        .with_state(model);

    // 3. Настройка портов (по умолчанию 3000 для Index Service)
    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .unwrap_or(3000);

    let addr = format!("{}:{}", host, port).parse::<SocketAddr>().unwrap();
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("🚀 Index Service listening on http://{}", addr);

    // Запуск сервера
    axum::serve(listener, app).await.unwrap();
}

// Простой хендлер проверки работоспособности
async fn health_handler() -> &'static str {
    "OK"
}       
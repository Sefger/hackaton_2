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
    // Инициализируем логирование, чтобы видеть ошибки в консоли
    tracing_subscriber::fmt::init();

    println!("Initializing Index Service...");

    // 1. Инициализируем модель. При первом запуске она начнет скачивание весов.
    // 2. Оборачиваем в Mutex, так как инференс в fastembed требует &mut self.
    // 3. Оборачиваем в Arc для безопасного использования между потоками Axum.
    let model = Arc::new(Mutex::new(handlers::init_sparse_model()));

    // Настраиваем роутер
    let app = Router::new()
        // Проверка работоспособности
        .route("/health", get(|| async { "OK" }))
        // Эндпоинт для нарезки чанков
        .route("/index", post(handlers::handle_index))
        // Эндпоинт для получения разреженных эмбеддингов (BM25/Splade)
        .route("/sparse_embedding", post(handlers::handle_sparse_embedding))
        // Передаем модель в состояние (State)
        .with_state(model);

    // Указываем адрес и порт
    let addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("Index Service (Axum 0.8) is running on http://{}", addr);
    println!("Ready to process requests.");

    // Запускаем сервер
    axum::serve(listener, app).await.unwrap();
}
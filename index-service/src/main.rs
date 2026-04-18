mod chunker;
mod handlers;

use axum::{
    routing::{get, post},
    Router,
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/health", get(|| async { "OK" }))
        // Добавляем обработчик индексации
        .route("/index", post(handlers::handle_index));

    let addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("Index Service listening on {}", addr);

    axum::serve(listener, app).await.unwrap();
}
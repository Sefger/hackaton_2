mod pipeline;

use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use qdrant_client::Qdrant;
use shared::{SearchAPIRequest, SearchAPIResponse};
use std::net::SocketAddr;
use std::sync::Arc;

// Единая структура состояния приложения
pub struct AppState {
    pub qdrant: Qdrant,
    pub http_client: reqwest::Client,
    pub api_key: String,
    pub dense_url: String,
    pub reranker_url: String,
    pub collection_name: String,
}

#[tokio::main]
async fn main() {
    // Инициализация логов (tracing)
    tracing_subscriber::fmt::init();

    // Читаем переменные окружения
    // Используем .expect(), так как без этих данных сервис бесполезен на хакатоне
    let api_key = std::env::var("API_KEY").expect("API_KEY not set");
    let qdrant_url = std::env::var("QDRANT_URL").expect("QDRANT_URL not set");
    let dense_url = std::env::var("EMBEDDINGS_DENSE_URL").expect("EMBEDDINGS_DENSE_URL not set");
    let reranker_url = std::env::var("RERANKER_URL").expect("RERANKER_URL not set");
    let collection_name =
        std::env::var("QDRANT_COLLECTION_NAME").expect("QDRANT_COLLECTION_NAME not set");

    // Инициализация клиента Qdrant
    // В версии qdrant-client 1.10+ используется .api_key()
    let qdrant = Qdrant::from_url(&qdrant_url)
        .api_key(api_key.clone())
        .build()
        .expect("Failed to create Qdrant client");

    let state = Arc::new(AppState {
        qdrant,
        http_client: reqwest::Client::new(),
        api_key,
        dense_url,
        reranker_url,
        collection_name,
    });

    // Настройка роутера
    let app = Router::new()
        .route("/health", get(|| async { "OK" }))
        .route("/search", post(search_handler))
        .with_state(state);

    // Настройка адреса запуска
    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");

    let addr: SocketAddr = format!("{}:{}", host, port)
        .parse()
        .expect("Invalid address format");

    println!("🚀 Search Service starting on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Исправленный хендлер: теперь он просто делегирует задачу пайплайну
async fn search_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<SearchAPIRequest>,
) -> Json<SearchAPIResponse> {
    // Вызываем логику из mod pipeline
    let results = pipeline::execute_search_pipeline(state, payload.question).await;
    Json(results)
}

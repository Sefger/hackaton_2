mod pipeline;

use axum::{
    routing::{get, post},
    Router,
    extract::State,
    Json,
};
use std::net::SocketAddr;
use std::sync::Arc;
use shared::{SearchAPIRequest, SearchAPIResponse};
use qdrant_client::Qdrant;

pub struct AppState {
    pub qdrant: Qdrant,
    pub index_service_url: String,
    pub http_client: reqwest::Client,
    pub api_key: String,
    pub dense_url: String,
    pub reranker_url: String,
    pub collection_name: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // Читаем переменные окружения согласно ТЗ хакатона
    let api_key = std::env::var("API_KEY").expect("API_KEY not set");
    let qdrant_url = std::env::var("QDRANT_URL").expect("QDRANT_URL not set");

    // Исправлено: метод .api_key() вместо .with_api_key()
    let qdrant = Qdrant::from_url(&qdrant_url)
        .api_key(api_key.clone())
        .build()
        .expect("Failed to create Qdrant client");

    let state = Arc::new(AppState {
        qdrant,
        // Ссылка на Index Service (обычно в одном поде/сети)
        index_service_url: format!("http://localhost:{}", std::env::var("INDEX_PORT").unwrap_or("3000".into())),
        http_client: reqwest::Client::new(),
        api_key: api_key.clone(),
        dense_url: std::env::var("EMBEDDINGS_DENSE_URL").expect("EMBEDDINGS_DENSE_URL not set"),
        reranker_url: std::env::var("RERANKER_URL").expect("RERANKER_URL not set"),
        collection_name: std::env::var("QDRANT_COLLECTION_NAME").expect("QDRANT_COLLECTION_NAME not set"),
    });

    let app = Router::new()
        .route("/health", get(|| async { "OK" }))
        .route("/search", post(search_handler))
        .with_state(state);

    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port: u16 = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string()).parse().unwrap();
    let addr = format!("{}:{}", host, port).parse::<SocketAddr>().unwrap();

    println!("🚀 Search Service starting on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn search_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<SearchAPIRequest>,
) -> Json<SearchAPIResponse> {
    let results = pipeline::execute_search_pipeline(state, payload.question).await;
    Json(results)
}
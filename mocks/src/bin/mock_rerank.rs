use axum::{routing::post, Json, Router};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
struct Req {
    #[serde(default)]
    query: String,
    #[serde(default)]
    documents: Vec<String>,
}

async fn rerank(Json(r): Json<Req>) -> Json<serde_json::Value> {
    let n = r.documents.len();
    let results: Vec<serde_json::Value> = (0..n)
        .map(|i| json!({ "index": i, "score": 1.0 - (i as f32) * 0.01 }))
        .collect();
    let _ = r.query;
    Json(json!({ "results": results }))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_env_filter("info").init();
    let app = Router::new().route("/rerank", post(rerank));
    let addr = std::env::var("ADDR").unwrap_or_else(|_| "0.0.0.0:9002".into());
    let listener = tokio::net::TcpListener::bind(&addr).await.expect("bind");
    tracing::info!(%addr, "mock_rerank listening");
    axum::serve(listener, app).await.unwrap();
}

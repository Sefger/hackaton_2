use axum::{routing::post, Json, Router};
use mocks::hash_embedding;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
struct Req {
    #[serde(default)]
    input: Option<String>,
    #[serde(default)]
    texts: Option<Vec<String>>,
}

async fn embeddings(Json(r): Json<Req>) -> Json<serde_json::Value> {
    let texts: Vec<String> = if let Some(t) = r.input {
        vec![t]
    } else if let Some(ts) = r.texts {
        ts
    } else {
        vec!["".into()]
    };
    let data: Vec<serde_json::Value> = texts
        .iter()
        .map(|t| json!({ "embedding": hash_embedding(t, 1024) }))
        .collect();
    Json(json!({ "data": data }))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_env_filter("info").init();
    let app = Router::new().route("/embeddings", post(embeddings));
    let addr = std::env::var("ADDR").unwrap_or_else(|_| "0.0.0.0:9001".into());
    let listener = tokio::net::TcpListener::bind(&addr).await.expect("bind");
    tracing::info!(%addr, "mock_dense listening");
    axum::serve(listener, app).await.unwrap();
}

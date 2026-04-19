use std::net::SocketAddr;
use std::sync::Arc;

use axum::{routing::{get, post}, Router};
use fastembed::{SparseInitOptions, SparseModel, SparseTextEmbedding};
use qdrant_client::Qdrant;
use tokio::sync::Mutex;

use search_service::config::Config;
use search_service::handlers;
use search_service::retrieval::qdrant::{QdrantStore, VectorStore};
use search_service::state::AppState;

fn init_sparse_model(model_dir: std::path::PathBuf) -> SparseTextEmbedding {
    let mut options = SparseInitOptions::default();
    options.model_name = SparseModel::BGEM3;
    options.cache_dir = model_dir;
    options.show_download_progress = false;
    SparseTextEmbedding::try_new(options)
        .expect("Failed to init sparse model; ensure MODEL_DIR has weights")
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
        )
        .init();

    let cfg = Arc::new(Config::from_env());
    tracing::info!(?cfg, "starting search-service");

    let http = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(55))
        .build()
        .expect("http client");

    let sparse_model = init_sparse_model(cfg.model_dir.clone());
    let sparse = Arc::new(Mutex::new(sparse_model));

    let qdrant = Qdrant::from_url(&cfg.qdrant_url)
        .api_key(cfg.api_key.clone())
        .build()
        .expect("qdrant client");

    let store: Arc<dyn VectorStore> = Arc::new(QdrantStore {
        client: qdrant,
        collection: cfg.collection.clone(),
        dense_vec_name: cfg.dense_vec_name.clone(),
        sparse_vec_name: cfg.sparse_vec_name.clone(),
    });

    let state = AppState { cfg: cfg.clone(), http, sparse, store };

    let app = Router::new()
        .route("/health", get(handlers::health::health))
        .route("/search", post(handlers::search::handle_search))
        .with_state(state);

    let addr: SocketAddr = format!("{}:{}", cfg.host, cfg.port).parse().expect("addr");
    tracing::info!(%addr, "listening");
    let listener = tokio::net::TcpListener::bind(addr).await.expect("bind");
    axum::serve(listener, app).await.expect("serve");
}

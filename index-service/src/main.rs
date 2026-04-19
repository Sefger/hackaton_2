use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;

use axum::{routing::{get, post}, Router};
use fastembed::{SparseInitOptions, SparseModel, SparseTextEmbedding};

use index_service::config::Config;
use index_service::handlers;
use index_service::state::AppState;

fn init_sparse_model(model_dir: std::path::PathBuf) -> SparseTextEmbedding {
    let mut options = SparseInitOptions::default();
    options.model_name = SparseModel::BGEM3;
    options.cache_dir = model_dir;
    options.show_download_progress = false;
    SparseTextEmbedding::try_new(options)
        .expect("Failed to initialize Sparse Model. Ensure weights are in MODEL_DIR.")
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
        )
        .init();

    let cfg = Config::from_env();
    tracing::info!(?cfg, "starting index-service");

    let model = init_sparse_model(cfg.model_dir.clone());
    let state = AppState { sparse: Arc::new(Mutex::new(model)) };

    let app = Router::new()
        .route("/health", get(handlers::health::health))
        .route("/index", post(handlers::index::handle_index))
        .route("/sparse_embedding", post(handlers::sparse::handle_sparse_embedding))
        .with_state(state);

    let addr: SocketAddr = format!("{}:{}", cfg.host, cfg.port).parse().expect("bad addr");
    tracing::info!(%addr, "listening");
    let listener = tokio::net::TcpListener::bind(addr).await.expect("bind");
    axum::serve(listener, app).await.expect("serve");
}

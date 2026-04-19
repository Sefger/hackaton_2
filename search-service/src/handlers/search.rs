use axum::{
    extract::{Json, State},
};
use shared::{SearchAPIRequest, SearchAPIResponse};

use crate::errors::AppError;
use crate::pipeline::{self, PipelineDeps};
use crate::retrieval::{dense, rerank, sparse};
use crate::state::AppState;

pub async fn handle_search(
    State(state): State<AppState>,
    Json(payload): Json<SearchAPIRequest>,
) -> Result<Json<SearchAPIResponse>, AppError> {
    let http = state.http.clone();
    let cfg = state.cfg.clone();
    let sparse_model = state.sparse.clone();
    let store = state.store.clone();

    let api_key = cfg.api_key.clone();
    let dense_url = cfg.dense_url.clone();
    let reranker_url = cfg.reranker_url.clone();

    let http_dense = http.clone();
    let http_rerank = http.clone();
    let api_key_d = api_key.clone();
    let api_key_r = api_key.clone();

    let deps = PipelineDeps {
        store: &store,
        dense_embed: Box::new(move |q: &str| {
            let http = http_dense.clone();
            let url = dense_url.clone();
            let api_key = api_key_d.clone();
            let q = q.to_string();
            Box::pin(async move { dense::embed(&http, &url, &api_key, &q).await })
        }),
        sparse_embed: Box::new(move |q: &str| {
            let model = sparse_model.clone();
            let q = q.to_string();
            Box::pin(async move { sparse::embed(model, &q).await })
        }),
        rerank: Box::new(move |q: &str, docs: &[String]| {
            let http = http_rerank.clone();
            let url = reranker_url.clone();
            let api_key = api_key_r.clone();
            let q = q.to_string();
            let docs = docs.to_vec();
            Box::pin(async move { rerank::rerank(&http, &url, &api_key, &q, &docs).await })
        }),
    };

    let resp = pipeline::execute(&deps, &payload.question).await?;
    Ok(Json(resp))
}

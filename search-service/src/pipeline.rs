use crate::AppState;
use shared::{Question, SearchAPIResponse, SearchAPIResultItem, SparseEmbeddingRequest, SparseEmbeddingResponse};
use std::sync::Arc;
use std::collections::HashSet;
use serde_json::json;
use qdrant_client::qdrant::{SearchPoints, value::Kind};

pub async fn execute_search_pipeline(state: Arc<AppState>, question: Question) -> SearchAPIResponse {
    // В shared/lib.rs доступно поле search_text
    let query_text = &question.search_text;

    // 1. Получаем Dense вектор (через внешний API)
    let dense_vec = fetch_dense_vector(&state, query_text).await;

    // 2. Поиск в Qdrant
    // Используем переменную окружения для имени вектора (из ТЗ)
    let dense_vector_name = std::env::var("QDRANT_DENSE_VECTOR_NAME").unwrap_or_default();

    let search_result = state.qdrant.search_points(SearchPoints {
        collection_name: state.collection_name.clone(),
        vector: dense_vec,
        vector_name: Some(dense_vector_name),
        limit: 100, // Запас для реранкера
        with_payload: Some(true.into()),
        ..Default::default()
    }).await.expect("Qdrant search failed");

    // 3. Reranking
    let final_ids = rerank_points(&state, query_text, search_result.result).await;

    SearchAPIResponse {
        results: vec![SearchAPIResultItem { message_ids: final_ids }]
    }
}

async fn rerank_points(state: &AppState, query: &str, points: Vec<qdrant_client::qdrant::ScoredPoint>) -> Vec<String> {
    let mut candidates = Vec::new();

    for p in points {
        // Безопасное извлечение текста чанка из Protobuf Payload
        let text = p.payload.get("page_content")
            .and_then(|v| v.kind.as_ref())
            .and_then(|k| match k {
                Kind::StringValue(s) => Some(s.clone()),
                _ => None
            }).unwrap_or_default();

        // Извлечение списка ID сообщений из чанка
        let mut ids = Vec::new();
        if let Some(val) = p.payload.get("message_ids") {
            if let Some(Kind::ListValue(list)) = &val.kind {
                for item in &list.values {
                    if let Some(Kind::StringValue(s)) = &item.kind {
                        ids.push(s.clone());
                    }
                }
            }
        }
        candidates.push((text, ids));
    }

    if candidates.is_empty() { return vec![]; }

    // Запрос к RERANKER_URL
    let rerank_payload = json!({
        "query": query,
        "documents": candidates.iter().map(|c| &c.0).collect::<Vec<_>>()
    });

    let resp = state.http_client.post(&state.reranker_url)
        .header("Authorization", format!("Bearer {}", state.api_key))
        .json(&rerank_payload)
        .send().await.unwrap()
        .json::<serde_json::Value>().await.unwrap();

    let mut final_message_ids = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();

    // Обработка результатов реранкера (обычно возвращает массив индексов)
    if let Some(results) = resp["results"].as_array() {
        for res in results {
            let idx = res["index"].as_u64().unwrap() as usize;
            if idx < candidates.len() {
                for id in &candidates[idx].1 {
                    if seen.insert(id.clone()) {
                        final_message_ids.push(id.clone());
                    }
                }
            }
        }
    }

    // Возвращаем ТОП-50 согласно критериям оценки
    final_message_ids.into_iter().take(50).collect()
}

async fn fetch_dense_vector(state: &AppState, text: &str) -> Vec<f32> {
    let resp = state.http_client.post(&state.dense_url)
        .header("Authorization", format!("Bearer {}", state.api_key))
        .json(&json!({ "input": text }))
        .send().await.unwrap()
        .json::<serde_json::Value>().await.unwrap();

    // Извлекаем массив f32
    resp["data"][0]["embedding"].as_array()
        .expect("Invalid dense embedding response")
        .iter()
        .map(|v| v.as_f64().unwrap() as f32)
        .collect()
}
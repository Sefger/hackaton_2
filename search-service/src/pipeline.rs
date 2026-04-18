use crate::AppState;
use shared::{Question, SearchAPIResponse, SearchAPIResultItem};
use std::sync::Arc;
use std::collections::HashSet;
use serde_json::json;
use qdrant_client::qdrant::{SearchPoints, value::Kind, Vector};

pub async fn execute_search_pipeline(state: Arc<AppState>, question: Question) -> SearchAPIResponse {
    let query_text = &question.search_text;

    // 1. Получаем Dense вектор (через внешний API)
    let dense_vec = fetch_dense_vector(&state, query_text).await;

    // 2. Поиск в Qdrant
    let dense_vector_name = std::env::var("QDRANT_DENSE_VECTOR_NAME")
        .unwrap_or_else(|_| "dense".to_string());

    // Формируем запрос к Qdrant (исправлено под Protobuf структуры)
    let search_points = SearchPoints {
        collection_name: state.collection_name.clone(),
        vector: dense_vec,
        vector_name: Some(dense_vector_name),
        limit: 100, // Берем с запасом для последующего реранкинга
        with_payload: Some(true.into()),
        ..Default::default()
    };

    let search_result = state.qdrant
        .search_points(search_points)
        .await
        .expect("Qdrant search failed");

    // 3. Reranking (переранжирование полученных кандидатов)
    let final_ids = rerank_points(&state, query_text, search_result.result).await;

    SearchAPIResponse {
        results: vec![SearchAPIResultItem { message_ids: final_ids }]
    }
}

async fn rerank_points(
    state: &AppState,
    query: &str,
    points: Vec<qdrant_client::qdrant::ScoredPoint>
) -> Vec<String> {
    let mut candidates = Vec::new();

    for p in points {
        // Безопасное извлечение текста чанка
        let text = p.payload.get("page_content")
            .and_then(|v| v.kind.as_ref())
            .and_then(|k| match k {
                Kind::StringValue(s) => Some(s.clone()),
                _ => None
            })
            .unwrap_or_default();

        // Извлечение message_ids (может быть списком или строкой)
        let mut ids = Vec::new();
        if let Some(val) = p.payload.get("message_ids") {
            match &val.kind {
                Some(Kind::ListValue(list)) => {
                    for item in &list.values {
                        if let Some(Kind::StringValue(s)) = &item.kind {
                            ids.push(s.clone());
                        }
                    }
                },
                Some(Kind::StringValue(s)) => {
                    ids.push(s.clone());
                },
                _ => {}
            }
        }
        candidates.push((text, ids));
    }

    if candidates.is_empty() { return vec![]; }

    // Запрос к Reranker API
    let rerank_payload = json!({
        "query": query,
        "documents": candidates.iter().map(|c| &c.0).collect::<Vec<_>>()
    });

    let resp = state.http_client.post(&state.reranker_url)
        .header("Authorization", format!("Bearer {}", state.api_key))
        .json(&rerank_payload)
        .send()
        .await
        .expect("Reranker request failed")
        .json::<serde_json::Value>()
        .await
        .expect("Failed to parse reranker response");

    let mut final_message_ids = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();

    // Обработка ответа реранкера
    if let Some(results) = resp.as_array().or(resp["results"].as_array()) {
        for res in results {
            // Реранкеры часто возвращают либо индекс напрямую, либо объект с полем "index"
            let idx = if res.is_number() {
                res.as_u64().unwrap() as usize
            } else {
                res["index"].as_u64().unwrap_or(0) as usize
            };

            if idx < candidates.len() {
                for id in &candidates[idx].1 {
                    if seen.insert(id.clone()) {
                        final_message_ids.push(id.clone());
                    }
                }
            }
        }
    }

    // Возвращаем ТОП-50 согласно ТЗ
    final_message_ids.into_iter().take(50).collect()
}

async fn fetch_dense_vector(state: &AppState, text: &str) -> Vec<f32> {
    let resp = state.http_client.post(&state.dense_url)
        .header("Authorization", format!("Bearer {}", state.api_key))
        .json(&json!({ "input": text }))
        .send()
        .await
        .expect("Embedding request failed")
        .json::<serde_json::Value>()
        .await
        .expect("Failed to parse embedding response");

    // Обработка разных форматов ответа (OpenAI-style или прямой массив)
    let embedding_json = if let Some(data) = resp["data"].as_array() {
        data[0]["embedding"].as_array()
    } else {
        resp.as_array()
    }.expect("Invalid dense embedding format");

    embedding_json
        .iter()
        .map(|v| v.as_f64().expect("Not a float") as f32)
        .collect()
}
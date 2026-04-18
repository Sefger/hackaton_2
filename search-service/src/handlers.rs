use axum::{extract::State, Json, response::IntoResponse};
use std::sync::Arc;
use crate::AppState;
use shared::{IndexAPIRequest, SearchAPIRequest, SearchAPIResponse, SearchAPIResultItem, SparseEmbeddingRequest, SparseEmbeddingResponse};
// Импортируем типы для работы с результатами и gRPC-значениями
use qdrant_client::qdrant::{
    SearchPoints,
    SearchResponse as QdrantSearchResponse,
    value::Kind
};
use crate::pipeline::IndexingPipeline;

pub async fn handle_search(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<SearchAPIRequest>,
) -> impl IntoResponse {

    let search_query = &payload.question.search_text;
    let index_url = format!("{}/sparse_embedding", state.index_service_url);

    // 1. Получаем разреженные эмбеддинги
    let index_res = state.http_client
        .post(&index_url)
        .json(&SparseEmbeddingRequest { texts: vec![search_query.clone()] })
        .send()
        .await
        .expect("Index-service connection error");

    let sparse_data: SparseEmbeddingResponse = index_res.json().await.expect("Failed to parse vector");
    let vector = &sparse_data.vectors[0];

    // 2. Формируем запрос к Qdrant
    let search_points = SearchPoints {
        collection_name: "messages".to_string(),
        sparse_indices: Some(vector.indices.iter().map(|&i| i as u32).collect::<Vec<_>>().into()),
        vector: vector.values.iter().map(|&v| v as f32).collect(),
        limit: 10,
        with_payload: Some(true.into()),
        ..Default::default()
    };

    // 3. Выполняем поиск (теперь компилятор поймет тип из-за явного указания переменной)
    let response: QdrantSearchResponse = state.qdrant
        .search_points(search_points)
        .await
        .expect("Qdrant search error");

    // 4. Разбираем Payload (используем Kind, как в предоставленных тобой тестах)
    let mut all_message_ids = Vec::new();

    for point in response.result {
        if let Some(value) = point.payload.get("message_ids") {
            // Идем по цепочке: Value -> Kind -> ListValue -> Values
            if let Some(Kind::ListValue(list)) = &value.kind {
                for item in &list.values {
                    if let Some(Kind::StringValue(id_str)) = &item.kind {
                        all_message_ids.push(id_str.clone());
                    }
                }
            }
        }
    }

    Json(SearchAPIResponse {
        results: vec![SearchAPIResultItem {
            message_ids: all_message_ids
        }]
    })
}



pub async fn handle_index(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<IndexAPIRequest>,
) -> impl IntoResponse {
    // Инициализируем пайплайн
    let pipeline = IndexingPipeline::new(state);

    // Запускаем процесс: Эмбеддинг -> Qdrant
    match pipeline.run(payload.data).await {
        Ok(_) => (axum::http::StatusCode::OK, "Data indexed successfully").into_response(),
        Err(e) => {
            eprintln!("Indexing error: {}", e);
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        }
    }
}
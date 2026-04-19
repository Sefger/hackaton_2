use axum::{
    body::Body,
    http::{Method, Request, StatusCode},
    Router,
};
use http_body_util::BodyExt;
use shared::IndexAPIResponse;
use tower::ServiceExt;

fn router() -> Router {
    use axum::routing::post;
    Router::new().route("/index", post(index_service::handlers::index::handle_index))
}

#[tokio::test]
async fn post_index_returns_one_chunk_per_new_message() {
    let body = serde_json::json!({
        "data": {
            "chat": {"id":"c","name":"General","sn":"sn","type":"group"},
            "overlap_messages": [],
            "new_messages": [
                {"id":"m1","time":1,"text":"hello","sender_id":"u1","is_system":false,"is_hidden":false,"is_forward":false,"is_quote":false},
                {"id":"m2","time":2,"text":"world","sender_id":"u2","is_system":false,"is_hidden":false,"is_forward":false,"is_quote":false}
            ]
        }
    })
    .to_string();

    let resp = router()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/index")
                .header("content-type", "application/json")
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
    let bytes = resp.into_body().collect().await.unwrap().to_bytes();
    let parsed: IndexAPIResponse = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(parsed.results.len(), 2);
    assert_eq!(parsed.results[0].message_ids, vec!["m1"]);
}

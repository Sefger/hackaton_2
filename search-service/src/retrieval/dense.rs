use serde_json::Value;

use crate::errors::AppError;

pub async fn embed(
    http: &reqwest::Client,
    url: &str,
    api_key: &str,
    text: &str,
) -> Result<Vec<f32>, AppError> {
    let body = serde_json::json!({ "input": text });
    let resp = http
        .post(url)
        .bearer_auth(api_key)
        .json(&body)
        .send()
        .await
        .map_err(|e| AppError::Upstream {
            status: 0,
            body: e.to_string(),
        })?;
    let status = resp.status();
    if !status.is_success() {
        let body = resp.text().await.unwrap_or_default();
        return Err(AppError::Upstream { status: status.as_u16(), body });
    }
    let json: Value = resp
        .json()
        .await
        .map_err(|e| AppError::Parse(format!("dense json: {e}")))?;
    parse_embedding(&json)
}

pub fn parse_embedding(v: &Value) -> Result<Vec<f32>, AppError> {
    // 1. OpenAI-like: { "data": [ { "embedding": [...] } ] }
    if let Some(arr) = v
        .get("data")
        .and_then(|d| d.get(0))
        .and_then(|d0| d0.get("embedding"))
        .and_then(|e| e.as_array())
    {
        return arr_to_f32(arr);
    }
    // 2. Direct: { "embedding": [...] }
    if let Some(arr) = v.get("embedding").and_then(|e| e.as_array()) {
        return arr_to_f32(arr);
    }
    // 3. Raw array of numbers: [f32, f32, ...]
    if let Some(arr) = v.as_array() {
        if arr.first().map(|x| x.is_number()).unwrap_or(false) {
            return arr_to_f32(arr);
        }
        // 4. Array of arrays: [[f32, ...]]
        if let Some(inner) = arr.first().and_then(|x| x.as_array()) {
            return arr_to_f32(inner);
        }
    }
    Err(AppError::Parse(format!("unknown dense format: {v:?}")))
}

fn arr_to_f32(arr: &[Value]) -> Result<Vec<f32>, AppError> {
    arr.iter()
        .map(|v| {
            v.as_f64()
                .map(|f| f as f32)
                .ok_or_else(|| AppError::Parse(format!("not a number: {v:?}")))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn parses_openai_format() {
        let v = json!({ "data": [ { "embedding": [1.0, 2.5, -0.5] } ] });
        let out = parse_embedding(&v).unwrap();
        assert_eq!(out, vec![1.0, 2.5, -0.5]);
    }

    #[test]
    fn parses_direct_embedding() {
        let v = json!({ "embedding": [0.0, 1.0] });
        assert_eq!(parse_embedding(&v).unwrap(), vec![0.0, 1.0]);
    }

    #[test]
    fn parses_raw_array() {
        let v = json!([0.1, 0.2, 0.3]);
        assert_eq!(parse_embedding(&v).unwrap(), vec![0.1, 0.2, 0.3]);
    }

    #[test]
    fn parses_array_of_arrays() {
        let v = json!([[0.5, 0.6]]);
        assert_eq!(parse_embedding(&v).unwrap(), vec![0.5, 0.6]);
    }

    #[test]
    fn errors_on_unknown_format() {
        let v = json!({ "weird": true });
        assert!(parse_embedding(&v).is_err());
    }

    #[tokio::test]
    async fn http_embed_happy_path() {
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/v1/embeddings"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [{ "embedding": [1.0, 2.0, 3.0] }]
            })))
            .mount(&server)
            .await;
        let http = reqwest::Client::new();
        let url = format!("{}/v1/embeddings", server.uri());
        let v = embed(&http, &url, "k", "hello").await.unwrap();
        assert_eq!(v, vec![1.0, 2.0, 3.0]);
    }

    #[tokio::test]
    async fn http_embed_upstream_error() {
        use wiremock::matchers::method;
        use wiremock::{Mock, MockServer, ResponseTemplate};
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(500).set_body_string("nope"))
            .mount(&server)
            .await;
        let http = reqwest::Client::new();
        let err = embed(&http, &server.uri(), "k", "x").await.unwrap_err();
        match err {
            AppError::Upstream { status, .. } => assert_eq!(status, 500),
            other => panic!("wrong variant: {other:?}"),
        }
    }
}

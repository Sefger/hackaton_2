use serde_json::Value;

/// Given the original candidate order (length N), returns a permutation of
/// indices sorted by rerank score descending. Falls back to identity on
/// unknown response shapes.
pub fn parse_rerank_order(resp: &Value, n: usize) -> Vec<usize> {
    // 1. { "results": [ { "index": i, "score": s? }, ... ] }
    if let Some(arr) = resp.get("results").and_then(|r| r.as_array()) {
        let mut pairs: Vec<(usize, f64)> = arr
            .iter()
            .filter_map(|r| {
                let idx = r.get("index").and_then(|x| x.as_u64())? as usize;
                let score = r.get("score").and_then(|x| x.as_f64()).unwrap_or(0.0);
                Some((idx, score))
            })
            .filter(|(i, _)| *i < n)
            .collect();
        let has_score = arr.iter().any(|r| r.get("score").is_some());
        if has_score {
            pairs.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        }
        if !pairs.is_empty() {
            return pairs.into_iter().map(|(i, _)| i).collect();
        }
    }
    // 2. { "scores": [f, f, f, ...] } — same-order array
    if let Some(arr) = resp.get("scores").and_then(|s| s.as_array()) {
        if arr.len() == n {
            let mut idx: Vec<usize> = (0..n).collect();
            let scores: Vec<f64> = arr.iter().map(|v| v.as_f64().unwrap_or(0.0)).collect();
            idx.sort_by(|a, b| {
                scores[*b].partial_cmp(&scores[*a]).unwrap_or(std::cmp::Ordering::Equal)
            });
            return idx;
        }
    }
    // 3. Raw array of objects with index
    if let Some(arr) = resp.as_array() {
        let pairs: Vec<(usize, f64)> = arr
            .iter()
            .filter_map(|r| {
                let idx = r.get("index").and_then(|x| x.as_u64())? as usize;
                let score = r.get("score").and_then(|x| x.as_f64()).unwrap_or(0.0);
                Some((idx, score))
            })
            .filter(|(i, _)| *i < n)
            .collect();
        if !pairs.is_empty() {
            let mut p = pairs;
            p.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
            return p.into_iter().map(|(i, _)| i).collect();
        }
    }
    // 4. Raw array of numbers (= indices permutation)
    if let Some(arr) = resp.as_array() {
        let idxs: Vec<usize> = arr
            .iter()
            .filter_map(|v| v.as_u64().map(|x| x as usize))
            .filter(|i| *i < n)
            .collect();
        if idxs.len() == arr.len() && !idxs.is_empty() {
            return idxs;
        }
    }
    // Fallback: identity
    tracing::warn!(response = %resp, "rerank response unparseable, using identity order");
    (0..n).collect()
}

pub async fn rerank(
    http: &reqwest::Client,
    url: &str,
    api_key: &str,
    query: &str,
    documents: &[String],
) -> Vec<usize> {
    let body = serde_json::json!({ "query": query, "documents": documents });
    let resp = match http.post(url).bearer_auth(api_key).json(&body).send().await {
        Ok(r) => r,
        Err(e) => {
            tracing::warn!(error = %e, "rerank HTTP failed, identity order");
            return (0..documents.len()).collect();
        }
    };
    if !resp.status().is_success() {
        tracing::warn!(status = %resp.status(), "rerank non-2xx, identity order");
        return (0..documents.len()).collect();
    }
    let json: Value = match resp.json().await {
        Ok(j) => j,
        Err(e) => {
            tracing::warn!(error = %e, "rerank json parse failed, identity order");
            return (0..documents.len()).collect();
        }
    };
    parse_rerank_order(&json, documents.len())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn parses_results_with_score() {
        let r = json!({ "results": [ { "index": 2, "score": 0.9 }, { "index": 0, "score": 0.5 } ] });
        assert_eq!(parse_rerank_order(&r, 3), vec![2, 0]);
    }

    #[test]
    fn parses_scores_parallel_array() {
        let r = json!({ "scores": [0.1, 0.9, 0.3] });
        assert_eq!(parse_rerank_order(&r, 3), vec![1, 2, 0]);
    }

    #[test]
    fn parses_raw_array_of_objects() {
        let r = json!([{ "index": 1, "score": 1.0 }, { "index": 0, "score": 0.5 }]);
        assert_eq!(parse_rerank_order(&r, 2), vec![1, 0]);
    }

    #[test]
    fn parses_raw_indices_array() {
        let r = json!([2, 0, 1]);
        assert_eq!(parse_rerank_order(&r, 3), vec![2, 0, 1]);
    }

    #[test]
    fn falls_back_to_identity_on_unknown() {
        let r = json!({ "wtf": true });
        assert_eq!(parse_rerank_order(&r, 4), vec![0, 1, 2, 3]);
    }

    #[test]
    fn drops_out_of_range_indices() {
        let r = json!({ "results": [ { "index": 99 }, { "index": 0 } ] });
        assert_eq!(parse_rerank_order(&r, 2), vec![0]);
    }

    #[tokio::test]
    async fn rerank_upstream_error_gives_identity() {
        use wiremock::matchers::method;
        use wiremock::{Mock, MockServer, ResponseTemplate};
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;
        let http = reqwest::Client::new();
        let order = rerank(&http, &server.uri(), "k", "q", &["a".into(), "b".into(), "c".into()]).await;
        assert_eq!(order, vec![0, 1, 2]);
    }
}

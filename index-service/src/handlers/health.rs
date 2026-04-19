use axum::http::StatusCode;

pub async fn health() -> (StatusCode, &'static str) {
    (StatusCode::OK, "OK")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn returns_ok() {
        let (status, body) = health().await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, "OK");
    }
}

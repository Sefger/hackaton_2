use axum::http::StatusCode;

pub async fn health() -> (StatusCode, &'static str) {
    (StatusCode::OK, "OK")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn returns_ok() {
        assert_eq!(health().await, (StatusCode::OK, "OK"));
    }
}

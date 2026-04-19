use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub api_key: String,
    pub dense_url: String,
    pub reranker_url: String,
    pub qdrant_url: String,
    pub collection: String,
    pub dense_vec_name: String,
    pub sparse_vec_name: String,
    pub model_dir: PathBuf,
}

impl Config {
    pub fn from_env() -> Self {
        let get = |k: &str| std::env::var(k).unwrap_or_else(|_| panic!("{} not set", k));
        let opt = |k: &str, d: &str| std::env::var(k).unwrap_or_else(|_| d.to_string());
        Self {
            host: opt("HOST", "0.0.0.0"),
            port: opt("PORT", "8080").parse().expect("PORT must be u16"),
            api_key: get("API_KEY"),
            dense_url: get("EMBEDDINGS_DENSE_URL"),
            reranker_url: get("RERANKER_URL"),
            qdrant_url: get("QDRANT_URL"),
            collection: get("QDRANT_COLLECTION_NAME"),
            dense_vec_name: opt("QDRANT_DENSE_VECTOR_NAME", "dense"),
            sparse_vec_name: opt("QDRANT_SPARSE_VECTOR_NAME", "sparse"),
            model_dir: opt("MODEL_DIR", "/app/models").into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn set_required() {
        std::env::set_var("API_KEY", "k");
        std::env::set_var("EMBEDDINGS_DENSE_URL", "http://d");
        std::env::set_var("RERANKER_URL", "http://r");
        std::env::set_var("QDRANT_URL", "http://q");
        std::env::set_var("QDRANT_COLLECTION_NAME", "c");
    }

    #[test]
    fn parses_required_and_defaults() {
        set_required();
        std::env::remove_var("QDRANT_DENSE_VECTOR_NAME");
        let c = Config::from_env();
        assert_eq!(c.dense_vec_name, "dense");
        assert_eq!(c.sparse_vec_name, "sparse");
        assert_eq!(c.port, 8080);
    }
}

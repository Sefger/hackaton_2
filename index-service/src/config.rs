use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub model_dir: PathBuf,
}

impl Config {
    pub fn from_env() -> Self {
        let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let port: u16 = std::env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()
            .expect("PORT must be a u16");
        let model_dir = std::env::var("MODEL_DIR")
            .unwrap_or_else(|_| "/app/models".to_string())
            .into();
        Self { host, port, model_dir }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_when_env_missing() {
        std::env::remove_var("HOST");
        std::env::remove_var("PORT");
        std::env::remove_var("MODEL_DIR");
        let c = Config::from_env();
        assert_eq!(c.host, "0.0.0.0");
        assert_eq!(c.port, 8080);
        assert_eq!(c.model_dir, PathBuf::from("/app/models"));
    }
}

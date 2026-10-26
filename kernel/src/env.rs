use std::str::FromStr;

#[derive(thiserror::Error, Debug)]
pub enum EnvError {
    #[error("missing environment variable: {0}")]
    Missing(String),
    #[error("failed to parse environment variable: {0}")]
    Parse(String),
}

pub fn extract_env<T: FromStr>(key: &str) -> Result<T, EnvError> {
    let raw = std::env::var(key).map_err(|err| {
        tracing::error!("error fetching env {}: {}", key, err);
        EnvError::Missing(key.to_string())
    })?;

    raw.parse::<T>().map_err(|_| {
        tracing::error!("error parsing env {}", key);
        EnvError::Parse(key.to_string())
    })
}

pub fn extract_optional_env<T: FromStr>(key: &str) -> Option<T> {
    std::env::var(key).ok().and_then(|raw| raw.parse().ok())
}

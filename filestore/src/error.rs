/// Everything that can go wrong while configuring or using a file store.
#[derive(Debug, thiserror::Error)]
pub enum FileStoreError {
    #[error("invalid file store configuration: {0}")]
    Config(String),

    #[error("failed to read upload: {0}")]
    Io(#[from] std::io::Error),

    #[error("upload request failed: {0}")]
    Request(#[from] reqwest::Error),

    #[error("invalid upload header: {0}")]
    InvalidHeader(#[from] reqwest::header::InvalidHeaderValue),

    #[error("could not build the object URL: {0}")]
    Url(#[from] url::ParseError),

    #[error("upload provider failed: {0}")]
    Provider(String),
}

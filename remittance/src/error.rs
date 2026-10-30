use thiserror::Error;

#[derive(Debug, Error)]
pub enum RemittanceError {
    #[error("configuration error: {0}")]
    Config(String),

    #[error("provider error: {0}")]
    Provider(#[from] PaymentError),

    #[error("database error: {0}")]
    Database(#[from] sea_orm::DbErr),
}

#[derive(Debug, Error)]
pub enum PaymentError {
    #[error("invalid request: {0}")]
    InvalidRequest(String),

    #[error("payment not found")]
    NotFound,

    #[error("duplicate reference: {0}")]
    DuplicateReference(String),

    #[error("provider returned unauthorized")]
    Unauthorized,

    #[error("provider unavailable: {0}")]
    ProviderUnavailable(String),

    #[error("verification failed: {0}")]
    VerificationFailed(String),

    #[error("unexpected error: {0}")]
    Unexpected(String),
}

impl PaymentError {
    pub fn is_permanent(&self) -> bool {
        matches!(
            self,
            Self::InvalidRequest(_)
                | Self::NotFound
                | Self::DuplicateReference(_)
                | Self::Unauthorized
        )
    }
}

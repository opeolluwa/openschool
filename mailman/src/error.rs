/// Everything that can go wrong while preparing or publishing a message.
#[derive(thiserror::Error, Debug)]
pub enum EmailServiceError {
    #[error("failed to send email")]
    SendEmailError,
    #[error("failed due to {0}")]
    OperationFailed(String),
    #[error("failed due to {0}")]
    ProviderError(String),
    #[error("failed due to {0}")]
    DeliveryError(String),
}

impl EmailServiceError {
    /// Whether retrying could ever change the outcome.
    pub fn is_permanent(&self) -> bool {
        matches!(self, Self::OperationFailed(_) | Self::ProviderError(_))
    }
}

/// Everything that can go wrong while starting the mailman service.
#[derive(thiserror::Error, Debug)]
pub enum MailmanError {
    #[error("invalid mailman configuration: {0}")]
    Config(String),

    #[error(transparent)]
    Queue(#[from] openschool_event_bus::QueueError),

    #[error(transparent)]
    Email(#[from] EmailServiceError),
}

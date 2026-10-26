use std::{str::FromStr, time::Duration};

use openschool_event_bus::{QueueConfig, QueueDriver};
use secrecy::SecretString;
use serde::{Deserialize, Serialize};

use crate::{
    contract::{EMAIL_EVENT, SenderIdentities},
    error::MailmanError,
};

/// Which provider actually delivers a message.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EmailProvider {
    #[default]
    Smtp,
    Mailtrap,
}

impl FromStr for EmailProvider {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_lowercase().as_str() {
            "smtp" | "lettre" => Ok(Self::Smtp),
            "mailtrap" | "api" => Ok(Self::Mailtrap),
            other => Err(format!("invalid email provider: {other}")),
        }
    }
}

impl std::fmt::Display for EmailProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Smtp => f.write_str("smtp"),
            Self::Mailtrap => f.write_str("mailtrap"),
        }
    }
}

/// How the SMTP transport negotiates TLS.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SmtpTlsMode {
    None,
    #[default]
    Starttls,
    Opportunistic,
    Implicit,
}

impl FromStr for SmtpTlsMode {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_lowercase().as_str() {
            "none" | "plain" | "plaintext" => Ok(Self::None),
            "starttls" | "required" => Ok(Self::Starttls),
            "opportunistic" => Ok(Self::Opportunistic),
            "implicit" | "wrapper" | "tls" => Ok(Self::Implicit),
            other => Err(format!("invalid SMTP TLS mode: {other}")),
        }
    }
}

/// Provider settings needed by [`crate::engine::EmailEngine`] and the probe.
#[derive(Debug, Clone)]
pub struct MailTransportConfig {
    pub provider: EmailProvider,
    pub smtp_host: SecretString,
    pub smtp_port: u16,
    pub smtp_username: SecretString,
    pub smtp_password: SecretString,
    pub smtp_tls_mode: SmtpTlsMode,
    pub mailtrap_api_token: SecretString,
    pub mailtrap_base_url: String,
}

impl Default for MailTransportConfig {
    fn default() -> Self {
        Self {
            provider: EmailProvider::default(),
            smtp_host: SecretString::from(String::new()),
            smtp_port: 587,
            smtp_username: SecretString::from(String::new()),
            smtp_password: SecretString::from(String::new()),
            smtp_tls_mode: SmtpTlsMode::default(),
            mailtrap_api_token: SecretString::from(String::new()),
            mailtrap_base_url: "https://send.api.mailtrap.io/api/send".to_string(),
        }
    }
}

/// Everything the mailman needs: where mail goes, which provider sends it, and
/// which transport carries the event.
#[derive(Debug, Clone)]
pub struct MailmanConfig {
    pub queue: QueueConfig,
    pub transport: MailTransportConfig,
    pub identities: SenderIdentities,
    pub event: String,
}

impl MailmanConfig {
    /// Builds a configuration from environment variables.
    ///
    /// Root applications usually construct this from their own config type
    /// instead; this exists so the standalone binary has the same inputs as
    /// every other service.
    pub fn from_env() -> Result<Self, MailmanError> {
        let mut queue = QueueConfig::default();

        if let Some(value) = optional_env("QUEUE_DRIVER")? {
            queue.driver = value;
        }
        if let Some(value) = optional_env("REDIS_CONNECTION_URL")?.or(optional_env("REDIS_URL")?) {
            queue.redis_url = value;
        }
        if let Some(value) = optional_env("KAFKA_BROKERS")? {
            queue.kafka_brokers = value;
        }
        if let Some(value) = optional_env("KAFKA_CONSUMER_GROUP")? {
            queue.consumer_group = value;
        }
        if let Some(value) = optional_env("MAILMAN_MAX_ATTEMPTS")? {
            queue.max_attempts = value;
        }
        if let Some(value) = optional_env("MAILMAN_BACKOFF_MILLIS")? {
            queue.backoff_millis = value;
        }
        if let Some(value) = optional_env("MAILMAN_LOCK_DURATION_SECS")? {
            queue.lock_duration = Duration::from_secs(value);
        }
        if let Some(value) = optional_env("MAILMAN_SEND_DEADLINE_SECS")? {
            queue.send_deadline = Duration::from_secs(value);
        }
        if let Some(value) = optional_env("MAILMAN_CONCURRENCY")? {
            queue.concurrency = value;
        }

        let mut transport = MailTransportConfig::default();
        if let Some(value) = optional_env("EMAIL_PROVIDER")? {
            transport.provider = value;
        }
        if let Some(value) = optional_env::<String>("SMTP_HOST")? {
            transport.smtp_host = SecretString::from(value);
        }
        if let Some(value) = optional_env("SMTP_PORT")? {
            transport.smtp_port = value;
        }
        if let Some(value) = optional_env::<String>("SMTP_USERNAME")? {
            transport.smtp_username = SecretString::from(value);
        }
        if let Some(value) = optional_env::<String>("SMTP_PASSWORD")? {
            transport.smtp_password = SecretString::from(value);
        }
        if let Some(value) = optional_env("SMTP_TLS_MODE")? {
            transport.smtp_tls_mode = value;
        }
        if let Some(value) = optional_env::<String>("MAILTRAP_API_TOKEN")? {
            transport.mailtrap_api_token = SecretString::from(value);
        }
        if let Some(value) = optional_env("MAILTRAP_BASE_URL")? {
            transport.mailtrap_base_url = value;
        }

        let mut identities = SenderIdentities::default();
        if let Some(value) = optional_env("EMAIL_FROM_NAME")? {
            identities.account.name = value;
        }
        if let Some(value) = optional_env("EMAIL_FROM_ADDRESS")? {
            identities.account.address = value;
        }
        if let Some(value) = optional_env("EMAIL_TRANSACTIONS_FROM_NAME")? {
            identities.transactions.name = value;
        }
        if let Some(value) = optional_env("EMAIL_TRANSACTIONS_FROM_ADDRESS")? {
            identities.transactions.address = value;
        }
        if let Some(value) = optional_env("EMAIL_SUPPORT_FROM_NAME")? {
            identities.support.name = value;
        }
        if let Some(value) = optional_env("EMAIL_SUPPORT_FROM_ADDRESS")? {
            identities.support.address = value;
        }

        let event = optional_env("MAILMAN_EVENT")?.unwrap_or_else(|| EMAIL_EVENT.to_string());

        let config = Self {
            queue,
            transport,
            identities,
            event,
        };
        config.validate()?;
        Ok(config)
    }

    fn validate(&self) -> Result<(), MailmanError> {
        use secrecy::ExposeSecret;

        match self.transport.provider {
            EmailProvider::Smtp if self.transport.smtp_host.expose_secret().trim().is_empty() => {
                Err(MailmanError::Config(
                    "SMTP_HOST is required when EMAIL_PROVIDER=smtp".to_string(),
                ))
            }
            EmailProvider::Mailtrap
                if self
                    .transport
                    .mailtrap_api_token
                    .expose_secret()
                    .trim()
                    .is_empty() =>
            {
                Err(MailmanError::Config(
                    "MAILTRAP_API_TOKEN is required when EMAIL_PROVIDER=mailtrap".to_string(),
                ))
            }
            _ if self.queue.driver == QueueDriver::Redis
                && self.queue.redis_url.trim().is_empty() =>
            {
                Err(MailmanError::Config(
                    "REDIS_CONNECTION_URL is required when QUEUE_DRIVER=redis".to_string(),
                ))
            }
            _ if self.queue.driver == QueueDriver::Kafka
                && self.queue.kafka_brokers.trim().is_empty() =>
            {
                Err(MailmanError::Config(
                    "KAFKA_BROKERS is required when QUEUE_DRIVER=kafka".to_string(),
                ))
            }
            _ => Ok(()),
        }
    }
}

fn optional_env<T>(key: &str) -> Result<Option<T>, MailmanError>
where
    T: FromStr,
    T::Err: std::fmt::Display,
{
    match std::env::var(key) {
        Ok(raw) => raw
            .parse()
            .map(Some)
            .map_err(|err| MailmanError::Config(format!("invalid {key}: {err}"))),
        Err(std::env::VarError::NotPresent) => Ok(None),
        Err(err) => Err(MailmanError::Config(format!("invalid {key}: {err}"))),
    }
}

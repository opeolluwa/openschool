use std::{str::FromStr, time::Duration};

use openschool_mailman::{
    EMAIL_EVENT, EmailProvider as MailmanEmailProvider, MailTransportConfig, MailmanConfig,
    QueueConfig, QueueDriver, SenderIdentities, SenderIdentity, SmtpTlsMode as MailmanSmtpTlsMode,
};
use secrecy::ExposeSecret;

use crate::{
    config::env::{AppConfig, EmailProvider, SmtpTlsMode},
    errors::app_error::AppError,
};

/// Builds the mailman's configuration from the application's existing env.
///
/// The mailman crate deliberately knows nothing about `AppConfig`, so this is
/// the one place the two config shapes meet.
pub fn mailman_config(config: &AppConfig) -> Result<MailmanConfig, AppError> {
    let mut queue = QueueConfig::default();

    if let Some(driver) = env_optional::<QueueDriver>("QUEUE_DRIVER")? {
        queue.driver = driver;
    }
    queue.redis_url = config.redis_url.expose_secret().to_string();

    if let Some(brokers) = env_optional::<String>("KAFKA_BROKERS")? {
        queue.kafka_brokers = brokers;
    }
    if let Some(group) = env_optional::<String>("KAFKA_CONSUMER_GROUP")? {
        queue.consumer_group = group;
    }
    if let Some(attempts) = env_optional::<u32>("MAILMAN_MAX_ATTEMPTS")? {
        queue.max_attempts = attempts;
    }
    if let Some(millis) = env_optional::<u64>("MAILMAN_BACKOFF_MILLIS")? {
        queue.backoff_millis = millis;
    }
    if let Some(secs) = env_optional::<u64>("MAILMAN_LOCK_DURATION_SECS")? {
        queue.lock_duration = Duration::from_secs(secs);
    }
    if let Some(secs) = env_optional::<u64>("MAILMAN_SEND_DEADLINE_SECS")? {
        queue.send_deadline = Duration::from_secs(secs);
    }
    if let Some(concurrency) = env_optional::<usize>("MAILMAN_CONCURRENCY")? {
        queue.concurrency = concurrency;
    }

    let transport = MailTransportConfig {
        provider: match config.email_provider {
            EmailProvider::Smtp => MailmanEmailProvider::Smtp,
            EmailProvider::Mailtrap => MailmanEmailProvider::Mailtrap,
        },
        smtp_host: config.smtp_host.clone(),
        smtp_port: config.smtp_port,
        smtp_username: config.smtp_username.clone(),
        smtp_password: config.smtp_password.clone(),
        smtp_tls_mode: match config.smtp_tls_mode {
            SmtpTlsMode::None => MailmanSmtpTlsMode::None,
            SmtpTlsMode::Starttls => MailmanSmtpTlsMode::Starttls,
            SmtpTlsMode::Opportunistic => MailmanSmtpTlsMode::Opportunistic,
            SmtpTlsMode::Implicit => MailmanSmtpTlsMode::Implicit,
        },
        mailtrap_api_token: config.mailtrap_api_token.clone(),
        mailtrap_base_url: config.mailtrap_base_url.clone(),
    };

    let identities = SenderIdentities {
        account: SenderIdentity {
            address: config.email_from_address.clone(),
            name: config.email_from_name.clone(),
        },
        transactions: SenderIdentity {
            address: config.email_transactions_from_address.clone(),
            name: config.email_transactions_from_name.clone(),
        },
        support: SenderIdentity {
            address: config.email_support_from_address.clone(),
            name: config.email_support_from_name.clone(),
        },
    };

    let event = env_optional::<String>("MAILMAN_EVENT")?.unwrap_or_else(|| EMAIL_EVENT.to_string());

    Ok(MailmanConfig {
        queue,
        transport,
        identities,
        event,
    })
}

fn env_optional<T>(key: &str) -> Result<Option<T>, AppError>
where
    T: FromStr,
    T::Err: std::fmt::Display,
{
    match std::env::var(key) {
        Ok(raw) => raw
            .parse()
            .map(Some)
            .map_err(|err| AppError::EnvError(format!("invalid {key}: {err}"))),
        Err(std::env::VarError::NotPresent) => Ok(None),
        Err(err) => Err(AppError::EnvError(format!("invalid {key}: {err}"))),
    }
}

//! Driver-agnostic mail delivery for openschool.
//!
//! The crate owns the email contract, the provider transports, and the
//! event-bus consumer. Producers only see [`EmailSender`], so they do not know
//! whether a message is delivered inline by [`EmailEngine`] or handed to the
//! queue by [`EventEmailSender`].
//!
//! [`start`] is the single entry point: it connects the event bus, builds the
//! configured provider, subscribes the consumer, and returns a [`MailmanHandle`]
//! that must stay alive for the process lifetime (the Redis worker stops when
//! the engine is dropped).

#![allow(async_fn_in_trait)]

pub mod adapter;
pub mod config;
pub mod contract;
pub mod engine;
pub mod error;
pub mod health;
pub mod mailtrap;
pub mod processor;
pub mod publisher;
pub mod sanitize;
pub mod smtp;

use openschool_event_bus::QueueEngine;

pub use config::{EmailProvider, MailTransportConfig, MailmanConfig, SmtpTlsMode};
pub use contract::{
    EMAIL_EVENT, EmailAttachment, EmailCategory, EmailContext, EmailSender, SenderIdentities,
    SenderIdentity,
};
pub use engine::EmailEngine;
pub use error::{EmailServiceError, MailmanError};
pub use openschool_event_bus::{HandlerError, JobContext, QueueConfig, QueueDriver, QueueError};
pub use processor::{process_event, start_processor};
pub use publisher::EventEmailSender;

/// A running mailman: the event bus, the producer handle, and subscription state.
///
/// The engine field is deliberately kept even though it is not read: dropping
/// the Redis backend stops its worker, so the handle is what keeps mail delivery
/// alive for the lifetime of the caller.
pub struct MailmanHandle {
    engine: QueueEngine,
    sender: EventEmailSender,
    ready: bool,
}

impl MailmanHandle {
    /// The queue-backed sender producers should hold.
    pub fn sender(&self) -> EventEmailSender {
        self.sender.clone()
    }

    /// The underlying event bus, for callers that also publish other events.
    pub fn engine(&self) -> &QueueEngine {
        &self.engine
    }

    /// The active queue driver.
    pub fn driver(&self) -> QueueDriver {
        self.engine.driver()
    }

    /// Whether the consumer subscription was established.
    pub fn is_ready(&self) -> bool {
        self.ready
    }
}

/// Starts the mailman and subscribes it to its configured event.
///
/// Fails fast on a bad queue/provider configuration: a process that cannot
/// deliver mail should not report itself healthy.
pub async fn start(config: MailmanConfig) -> Result<MailmanHandle, MailmanError> {
    let engine = QueueEngine::connect(config.queue.clone()).await?;
    start_with(engine, config).await
}

/// Starts the mailman against an already connected engine.
///
/// Useful for embedding the consumer in a process that owns one shared bus.
pub async fn start_with(
    engine: QueueEngine,
    config: MailmanConfig,
) -> Result<MailmanHandle, MailmanError> {
    let deadline = config.queue.send_deadline;
    let sender_engine = EmailEngine::from_config(&config.transport)?;
    let sender = EventEmailSender::new(
        engine.clone(),
        config.identities.clone(),
        config.event.clone(),
    );

    start_processor(&engine, sender_engine, &config.event, deadline).await?;

    Ok(MailmanHandle {
        engine,
        sender,
        ready: true,
    })
}

/// Runs the startup provider probe, logging the outcome.
///
/// Intended to be spawned as a detached task so a dead relay is reported
/// quickly without delaying or failing the rest of startup.
pub async fn check_health(config: MailmanConfig) {
    health::check_email_provider(config.transport).await;
}

use openschool_event_bus::QueueEngine;

use crate::{
    contract::{EmailContext, EmailSender, SenderIdentities},
    error::EmailServiceError,
};

/// Hands emails to the event bus instead of delivering them inline.
///
/// The mailman consumer is what actually talks to the provider, so a slow or
/// briefly unreachable server no longer blocks the request that triggered the
/// mail. The sender is independent of whether the bus is backed by Redis or
/// Kafka.
#[derive(Clone)]
pub struct EventEmailSender {
    engine: QueueEngine,
    identities: SenderIdentities,
    event: String,
}

impl EventEmailSender {
    pub fn new(
        engine: QueueEngine,
        identities: SenderIdentities,
        event: impl Into<String>,
    ) -> Self {
        Self {
            engine,
            identities,
            event: event.into(),
        }
    }

    /// The event name this sender publishes under.
    pub fn event(&self) -> &str {
        &self.event
    }
}

impl EmailSender for EventEmailSender {
    /// Stamps the deployment's configured identity onto the message before it
    /// becomes an event payload.
    ///
    /// The dispatch layer runs without access to the config, so it fills in
    /// [`crate::contract::SenderIdentity::default`] and the real identity is applied
    /// here. This matters for the API provider, which rejects any sending domain
    /// the account has not verified. The identity is chosen by the message's
    /// [`crate::contract::EmailCategory`]: account, transactions, or support.
    async fn send_email(&self, mut message: EmailContext) -> Result<(), EmailServiceError> {
        let identity = self.identities.get(&message.category);
        message.from_address = identity.address.clone();
        message.from_name = identity.name.clone();

        self.engine
            .publish(&self.event, &message)
            .await
            .map_err(|err| {
                EmailServiceError::OperationFailed(format!("failed to enqueue email: {err}"))
            })
    }
}

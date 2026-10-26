use std::{sync::Arc, time::Duration};

use openschool_event_bus::{HandlerError, JobContext, QueueEngine, QueueError};

use crate::{
    contract::{EMAIL_EVENT, EmailContext, EmailSender},
    engine::EmailEngine,
    error::EmailServiceError,
};

/// Turns one event payload into one delivery attempt.
///
/// Parsing failures are fatal: an old payload that no longer matches the
/// contract cannot become valid on another attempt. Transport failures are
/// classified the same way as the reference worker, so a bad address is not
/// retried while a temporarily unreachable relay is.
pub async fn process_event<S>(
    context: JobContext,
    payload: serde_json::Value,
    sender: &S,
    deadline: Duration,
) -> Result<(), HandlerError>
where
    S: EmailSender + ?Sized,
{
    let message: EmailContext = serde_json::from_value(payload).map_err(|err| {
        HandlerError::Fatal(format!("invalid {EMAIL_EVENT} event payload: {err}"))
    })?;

    tracing::info!(
        event = %context.event,
        job = %context.id,
        attempt = context.attempt + 1,
        recipient = %message.to_address,
        "delivering mail"
    );

    send_within(sender, message, deadline).await
}

/// Subscribes the provider engine to `event` and runs until the bus stops.
pub async fn start_processor(
    engine: &QueueEngine,
    sender: EmailEngine,
    event: &str,
    deadline: Duration,
) -> Result<(), QueueError> {
    let sender = Arc::new(sender);
    let events = [event.to_string()];

    engine
        .run(&events, move |context, payload| {
            let sender = Arc::clone(&sender);

            async move { process_event(context, payload, sender.as_ref(), deadline).await }
        })
        .await
}

/// Sends the mail under a wall-clock deadline.
///
/// Each provider bounds a single command round trip, so a server that accepts a
/// connection and then stalls can consume several of those budgets back to back.
/// Without an outer bound the send can outrun the job's lock, and the worker
/// loses the right to book a retry. Timing out here reports a retryable error
/// while the lock is still held.
async fn send_within<S>(
    sender: &S,
    message: EmailContext,
    deadline: Duration,
) -> Result<(), HandlerError>
where
    S: EmailSender + ?Sized,
{
    let send = sender.send_email(message);

    match tokio::time::timeout(deadline, send).await {
        Ok(Ok(())) => Ok(()),
        Ok(Err(err)) => Err(classify(err)),
        Err(_) => Err(HandlerError::Retry(format!(
            "delivery exceeded its {deadline:?} budget"
        ))),
    }
}

fn classify(err: EmailServiceError) -> HandlerError {
    if err.is_permanent() {
        HandlerError::Fatal(err.to_string())
    } else {
        HandlerError::Retry(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contract::{EmailCategory, MockEmailSender};

    fn message() -> EmailContext {
        EmailContext {
            from_address: "no-reply@example.com".to_string(),
            from_name: "Example".to_string(),
            to_address: "member@example.com".to_string(),
            to_name: "Member".to_string(),
            subject: "Welcome".to_string(),
            html_body: "<p>hi</p>".to_string(),
            notification_delivery_identifier: None,
            category: EmailCategory::default(),
            attachments: Vec::new(),
        }
    }

    fn context() -> JobContext {
        JobContext {
            id: "job-1".to_string(),
            event: EMAIL_EVENT.to_string(),
            attempt: 0,
        }
    }

    #[tokio::test]
    async fn delivers_a_valid_payload() {
        let mut sender = MockEmailSender::new();
        sender.expect_send_email().times(1).returning(|_| Ok(()));

        process_event(
            context(),
            serde_json::to_value(message()).unwrap(),
            &sender,
            Duration::from_secs(1),
        )
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn malformed_payload_is_fatal() {
        let sender = MockEmailSender::new();

        let result = process_event(
            context(),
            serde_json::json!({ "subject": "missing recipient" }),
            &sender,
            Duration::from_secs(1),
        )
        .await;

        assert!(matches!(result, Err(HandlerError::Fatal(_))));
    }

    #[tokio::test]
    async fn permanent_provider_error_is_fatal() {
        let mut sender = MockEmailSender::new();
        sender
            .expect_send_email()
            .times(1)
            .returning(|_| Err(EmailServiceError::ProviderError("bad address".to_string())));

        let result = process_event(
            context(),
            serde_json::to_value(message()).unwrap(),
            &sender,
            Duration::from_secs(1),
        )
        .await;

        assert!(matches!(result, Err(HandlerError::Fatal(_))));
    }

    #[tokio::test]
    async fn transient_provider_error_is_retryable() {
        let mut sender = MockEmailSender::new();
        sender
            .expect_send_email()
            .times(1)
            .returning(|_| Err(EmailServiceError::DeliveryError("relay down".to_string())));

        let result = process_event(
            context(),
            serde_json::to_value(message()).unwrap(),
            &sender,
            Duration::from_secs(1),
        )
        .await;

        assert!(matches!(result, Err(HandlerError::Retry(_))));
    }

    struct StallingSender;

    impl EmailSender for StallingSender {
        async fn send_email(&self, _: EmailContext) -> Result<(), EmailServiceError> {
            std::future::pending::<Result<(), EmailServiceError>>().await
        }
    }

    #[tokio::test]
    async fn a_stalled_provider_times_out_as_retryable() {
        let result = process_event(
            context(),
            serde_json::to_value(message()).unwrap(),
            &StallingSender,
            Duration::from_millis(20),
        )
        .await;

        assert!(matches!(result, Err(HandlerError::Retry(_))));
    }
}

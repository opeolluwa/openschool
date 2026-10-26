use serde::{Deserialize, Serialize};

use crate::error::EmailServiceError;

/// The event name used by every producer and by the mailman consumer.
pub const EMAIL_EVENT: &str = "send-email";

/// The provider-agnostic message contract carried by [`EMAIL_EVENT`].
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EmailContext {
    pub from_address: String,
    pub from_name: String,
    pub to_address: String,
    pub to_name: String,
    pub subject: String,
    pub html_body: String,
    /// The `notification_deliveries` row this message closes out, when the mail
    /// is the email rendering of a notification.
    ///
    /// `#[serde(default)]` because jobs enqueued before this field existed are
    /// still sitting in the queue after a deploy, and a payload that failed to
    /// deserialize would strand them as permanently unprocessable.
    #[serde(default)]
    pub notification_delivery_identifier: Option<String>,
    #[serde(default)]
    pub category: EmailCategory,
    #[serde(default)]
    pub attachments: Vec<EmailAttachment>,
}

/// Seeds the sender identity, leaves the per-message fields empty.
///
/// The dispatch layer runs without access to the config, so it builds messages
/// with `..EmailContext::default()` and lets the queue-backed sender swap in
/// the deployment's real identity. Defaulting the identity here — rather than
/// deriving `Default` and starting from empty strings — is what keeps a message
/// from ever reaching a provider with a blank `from_address`.
impl Default for EmailContext {
    fn default() -> Self {
        let from = SenderIdentity::default();
        Self {
            from_address: from.address,
            from_name: from.name,
            to_address: String::new(),
            to_name: String::new(),
            subject: String::new(),
            html_body: String::new(),
            notification_delivery_identifier: None,
            category: EmailCategory::default(),
            attachments: Vec::new(),
        }
    }
}

/// A file attached to an outbound message.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EmailAttachment {
    pub filename: String,
    pub content_type: String,
    /// Base64-encoded file content.
    pub data: String,
}

/// The sender identity used for a message category.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmailCategory {
    #[default]
    Account,
    Transactions,
    Support,
}

/// The From address and display name for outgoing mail.
///
/// [`EmailContext::default`] carries this identity onto every message the
/// dispatch layer builds, and the queue-backed sender rewrites those two fields
/// with the deployment's configured identity on its way into the job payload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SenderIdentity {
    pub address: String,
    pub name: String,
}

impl Default for SenderIdentity {
    fn default() -> Self {
        Self {
            address: "accounts@openschool.com".to_string(),
            name: "openschool Accounts".to_string(),
        }
    }
}

/// Service-wide sender identities, so every producer does not repeat them.
#[derive(Clone, Debug)]
pub struct SenderIdentities {
    pub account: SenderIdentity,
    pub transactions: SenderIdentity,
    pub support: SenderIdentity,
}

impl Default for SenderIdentities {
    fn default() -> Self {
        Self {
            account: SenderIdentity::default(),
            transactions: SenderIdentity {
                address: "transactions@openschool.com".to_string(),
                name: "openschool Transactions".to_string(),
            },
            support: SenderIdentity {
                address: "support@openschool.com".to_string(),
                name: "openschool Support".to_string(),
            },
        }
    }
}

impl SenderIdentities {
    pub fn get(&self, category: &EmailCategory) -> &SenderIdentity {
        match category {
            EmailCategory::Account => &self.account,
            EmailCategory::Transactions => &self.transactions,
            EmailCategory::Support => &self.support,
        }
    }
}

/// Anything that can deliver an [`EmailContext`].
///
/// The queue-backed [`crate::publisher::EventEmailSender`] and the provider-backed
/// [`crate::engine::EmailEngine`] both implement this, so call sites do not know
/// whether a message is sent inline or enqueued.
#[cfg_attr(test, mockall::automock)]
pub trait EmailSender {
    async fn send_email(&self, message: EmailContext) -> Result<(), EmailServiceError>;
}

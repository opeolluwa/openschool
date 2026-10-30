use sea_orm::DatabaseConnection;

use openschool_models::sea_orm_active_enums::{OrderStatus, PaymentChannel, ProductCurrency};

use crate::{
    config::{PaymentBackend, RemittanceConfig},
    contract::{InitializeRequest, InitializeResponse, PaymentProvider, PaymentStatus, VerifyResponse},
    error::{PaymentError, RemittanceError},
    paystack::PaystackProvider,
    repository::{InvoiceRepository, PaymentTransactionRepository},
};

/// The configured payment provider, dispatching to the active backend.
///
/// Follows the same enum-dispatch pattern as `FileStoreEngine` in the
/// filestore crate.
enum ProviderEngine {
    Paystack(PaystackProvider),
}

impl PaymentProvider for ProviderEngine {
    async fn initialize_transaction(
        &self,
        request: &InitializeRequest,
    ) -> Result<InitializeResponse, PaymentError> {
        match self {
            Self::Paystack(p) => p.initialize_transaction(request).await,
        }
    }

    async fn verify_transaction(
        &self,
        reference: &str,
    ) -> Result<VerifyResponse, PaymentError> {
        match self {
            Self::Paystack(p) => p.verify_transaction(reference).await,
        }
    }
}

pub struct RemittanceEngine {
    provider: ProviderEngine,
    backend: PaymentBackend,
    transactions: PaymentTransactionRepository,
    invoices: InvoiceRepository,
}

impl RemittanceEngine {
    pub fn from_config(
        config: &RemittanceConfig,
        db: &DatabaseConnection,
    ) -> Result<Self, RemittanceError> {
        let backend = config.backend.clone();

        let provider = match &config.backend {
            PaymentBackend::Paystack => ProviderEngine::Paystack(
                PaystackProvider::new(
                    config.paystack_api_key.clone(),
                    config.paystack_base_url.clone(),
                    config.paystack_verify_base_url.clone(),
                )
                .map_err(RemittanceError::Provider)?,
            ),
        };

        let transactions = PaymentTransactionRepository::new(db);
        let invoices = InvoiceRepository::new(db);

        Ok(Self {
            provider,
            backend,
            transactions,
            invoices,
        })
    }

    pub fn backend(&self) -> &PaymentBackend {
        &self.backend
    }

    pub fn transactions(&self) -> &PaymentTransactionRepository {
        &self.transactions
    }

    pub fn invoices(&self) -> &InvoiceRepository {
        &self.invoices
    }

    /// Creates a payment transaction record and initializes it with the provider.
    ///
    /// Returns the full transaction record with `authorization_url` populated.
    pub async fn create_payment(
        &self,
        request: &InitializeRequest,
        currency: ProductCurrency,
        institution_identifier: Vec<u8>,
    ) -> Result<(openschool_models::payment_transaction::Model, InitializeResponse), RemittanceError>
    {
        let transaction = self
            .transactions
            .create(
                &request.reference,
                request.amount_minor,
                currency,
                &request.email,
                &self.backend.to_string(),
                institution_identifier,
                request.metadata.clone(),
            )
            .await?;

        let init_response = self
            .provider
            .initialize_transaction(request)
            .await
            .map_err(RemittanceError::Provider)?;

        let transaction = self
            .transactions
            .update_authorization_url(
                &transaction.identifier,
                &init_response.authorization_url,
            )
            .await?;

        Ok((transaction, init_response))
    }

    /// Verifies a payment with the provider and updates the transaction status.
    ///
    /// Idempotent: returns the existing record if already paid.
    pub async fn handle_webhook(
        &self,
        reference: &str,
    ) -> Result<openschool_models::payment_transaction::Model, RemittanceError> {
        let transaction = self
            .transactions
            .find_by_reference(reference)
            .await?
            .ok_or(RemittanceError::Provider(PaymentError::NotFound))?;

        if transaction.status == OrderStatus::Paid {
            return Ok(transaction);
        }

        let verification = self
            .provider
            .verify_transaction(reference)
            .await
            .map_err(RemittanceError::Provider)?;

        let (status, channel) = match verification.status {
            PaymentStatus::Success => {
                let channel = verification
                    .channel
                    .as_deref()
                    .map(parse_channel)
                    .unwrap_or(None);
                (OrderStatus::Paid, channel)
            }
            _ => (OrderStatus::Cancelled, None),
        };

        let updated = self
            .transactions
            .update_status(
                &transaction.identifier,
                status,
                channel,
                Some(verification.provider_reference),
                verification.paid_at,
            )
            .await?;

        Ok(updated)
    }

    /// Creates an invoice linked to a payment transaction.
    pub async fn create_invoice(
        &self,
        transaction_identifier: Vec<u8>,
        invoice_number: &str,
        recipient_name: &str,
        recipient_email: &str,
        recipient_phone: Option<String>,
        description: Option<String>,
        amount: i64,
        currency: ProductCurrency,
        due_date: Option<String>,
        institution_identifier: Vec<u8>,
    ) -> Result<openschool_models::invoice::Model, RemittanceError> {
        self.invoices
            .create(
                invoice_number,
                transaction_identifier,
                recipient_name,
                recipient_email,
                recipient_phone,
                description,
                amount,
                currency,
                due_date,
                institution_identifier,
            )
            .await
    }

    /// Finds a transaction by its unique reference.
    pub async fn find_transaction(
        &self,
        reference: &str,
    ) -> Result<Option<openschool_models::payment_transaction::Model>, RemittanceError> {
        self.transactions.find_by_reference(reference).await
    }

    /// Verifies a transaction with the provider without updating the DB.
    pub async fn verify_transaction(
        &self,
        reference: &str,
    ) -> Result<VerifyResponse, RemittanceError> {
        self.provider
            .verify_transaction(reference)
            .await
            .map_err(RemittanceError::Provider)
    }
}

fn parse_channel(channel: &str) -> Option<PaymentChannel> {
    match channel {
        "card" => Some(PaymentChannel::Card),
        "bank" => Some(PaymentChannel::Bank),
        "ussd" => Some(PaymentChannel::Ussd),
        "qr" => Some(PaymentChannel::Qr),
        "mobile_money" => Some(PaymentChannel::MobileMoney),
        "bank_transfer" => Some(PaymentChannel::BankTransfer),
        _ => Some(PaymentChannel::Other),
    }
}

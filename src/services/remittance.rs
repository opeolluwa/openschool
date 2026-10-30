use openschool_remittance::RemittanceConfig;
use openschool_remittance::config::PaymentBackend;

use crate::{config::env::AppConfig, errors::app_error::AppError};

/// Builds the remittance engine's configuration from the application's env.
pub fn remittance_config(config: &AppConfig) -> Result<RemittanceConfig, AppError> {
    Ok(RemittanceConfig {
        backend: PaymentBackend::Paystack,
        paystack_api_key: config.paystack_api_key.clone(),
        paystack_api_secret: config.paystack_api_secret.clone(),
        paystack_base_url: config.paystack_base_url.clone(),
        paystack_verify_base_url: config.paystack_verify_base_url.clone(),
    })
}

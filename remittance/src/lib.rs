//! Provider-neutral remittance engine for openschool.
//!
//! Handles invoices, payment transactions, and payment link generation,
//! abstracting the underlying payment provider (Paystack, Stripe, etc.)
//! behind [`PaymentProvider`]. Callers interact with [`RemittanceEngine`],
//! which orchestrates provider HTTP calls and database persistence.
//!
//! The engine receives a `DatabaseConnection` at construction and owns
//! its repositories internally. Entity types are imported from the shared
//! `openschool_models` crate.

#![allow(async_fn_in_trait)]

pub mod config;
pub mod contract;
pub mod engine;
pub mod error;
pub mod paystack;
pub mod repository;

pub use config::{PaymentBackend, RemittanceConfig};
pub use contract::{
    InitializeRequest, InitializeResponse, PaymentProvider, PaymentStatus, VerifyResponse,
};
pub use engine::RemittanceEngine;
pub use error::{PaymentError, RemittanceError};
pub use paystack::PaystackProvider;

use sea_orm::DatabaseConnection;

/// Builds and returns a configured [`RemittanceEngine`].
///
/// Fails fast on misconfiguration so a process that cannot collect
/// payments reports the problem at startup.
pub fn start(
    config: &RemittanceConfig,
    db: &DatabaseConnection,
) -> Result<RemittanceEngine, RemittanceError> {
    RemittanceEngine::from_config(config, db)
}

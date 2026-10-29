//! Provider-neutral file storage for openschool.
//!
//! Callers depend on [`FileStoreProvider`] and receive a URL. The provider in
//! use — ImageKit or S3 — is selected by configuration and constructed through
//! [`FileStoreEngine::from_config`].

#![allow(async_fn_in_trait)]

pub mod config;
pub mod contract;
pub mod engine;
pub mod error;
pub mod imagekit;
mod key;
pub mod s3;
pub use config::{FileStoreConfig, ImagekitConfig, S3Config, StorageBackend};
pub use contract::FileStoreProvider;
pub use engine::FileStoreEngine;
pub use error::FileStoreError;
pub use imagekit::ImagekitProvider;
pub use s3::S3Provider;

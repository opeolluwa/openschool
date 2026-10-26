use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};

use crate::error::FileStoreError;

/// The provider that owns uploaded files.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StorageBackend {
    #[default]
    Imagekit,
    S3,
}

impl std::fmt::Display for StorageBackend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Imagekit => f.write_str("imagekit"),
            Self::S3 => f.write_str("s3"),
        }
    }
}

/// Configuration for the provider-neutral file store.
#[derive(Debug, Clone)]
pub struct FileStoreConfig {
    pub backend: StorageBackend,
    pub imagekit: ImagekitConfig,
    pub s3: S3Config,
}

/// ImageKit credentials and upload endpoint.
#[derive(Debug, Clone)]
pub struct ImagekitConfig {
    pub public_key: SecretString,
    pub private_key: SecretString,
    pub upload_url: String,
    /// Optional ImageKit folder every upload is written to.
    pub folder: Option<String>,
}

/// AWS S3 (or an S3-compatible service such as MinIO) configuration.
#[derive(Debug, Clone)]
pub struct S3Config {
    pub bucket: String,
    pub region: String,
    pub access_key_id: SecretString,
    pub secret_access_key: SecretString,
    pub session_token: Option<SecretString>,
    /// Custom endpoint for S3-compatible services; leave unset for AWS S3.
    pub endpoint: Option<String>,
    pub force_path_style: bool,
    /// Public base URL, typically a CloudFront distribution or CDN domain.
    ///
    /// When unset, a provider URL is constructed from the endpoint/bucket/region.
    pub public_base_url: Option<String>,
    /// Optional key prefix, for example `uploads/students`.
    pub key_prefix: Option<String>,
}

impl S3Config {
    pub(crate) fn validate(&self) -> Result<(), FileStoreError> {
        if self.bucket.trim().is_empty() {
            return Err(FileStoreError::Config("S3 bucket is required".into()));
        }

        if self.region.trim().is_empty() {
            return Err(FileStoreError::Config("S3 region is required".into()));
        }

        if self.access_key_id.expose_secret().trim().is_empty() {
            return Err(FileStoreError::Config("S3 access key ID is required".into()));
        }

        if self.secret_access_key.expose_secret().trim().is_empty() {
            return Err(FileStoreError::Config(
                "S3 secret access key is required".into(),
            ));
        }

        if let Some(endpoint) = &self.endpoint {
            url::Url::parse(endpoint).map_err(|err| {
                FileStoreError::Config(format!("S3 endpoint is not a valid URL: {err}"))
            })?;
        }

        if let Some(base_url) = &self.public_base_url {
            url::Url::parse(base_url).map_err(|err| {
                FileStoreError::Config(format!("S3 public base URL is not valid: {err}"))
            })?;
        }

        Ok(())
    }
}

impl ImagekitConfig {
    pub(crate) fn validate(&self) -> Result<(), FileStoreError> {
        if self.private_key.expose_secret().trim().is_empty() {
            return Err(FileStoreError::Config(
                "ImageKit private key is required".into(),
            ));
        }

        url::Url::parse(&self.upload_url).map_err(|err| {
            FileStoreError::Config(format!("ImageKit upload URL is not valid: {err}"))
        })?;

        Ok(())
    }
}

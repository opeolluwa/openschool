use std::path::Path;

use crate::{
    config::{FileStoreConfig, StorageBackend},
    contract::FileStoreProvider,
    error::FileStoreError,
    imagekit::ImagekitProvider,
    s3::S3Provider,
};

/// The configured file storage provider.
///
/// Consumers only see [`FileStoreProvider`], so switching `FILE_STORAGE_PROVIDER`
/// changes where uploads land without changing call sites.
#[derive(Debug, Clone)]
pub enum FileStoreEngine {
    Imagekit(ImagekitProvider),
    S3(S3Provider),
}

impl FileStoreEngine {
    /// Builds the provider named by the configuration, failing at startup rather
    /// than at the first upload.
    pub fn from_config(config: &FileStoreConfig) -> Result<Self, FileStoreError> {
        match config.backend {
            StorageBackend::Imagekit => ImagekitProvider::new(&config.imagekit).map(Self::Imagekit),
            StorageBackend::S3 => S3Provider::new(&config.s3).map(Self::S3),
        }
    }

    /// The active backend, useful for startup logging.
    pub fn backend(&self) -> StorageBackend {
        match self {
            Self::Imagekit(_) => StorageBackend::Imagekit,
            Self::S3(_) => StorageBackend::S3,
        }
    }
}

impl FileStoreProvider for FileStoreEngine {
    async fn upload_file(
        &self,
        path: &Path,
        file_name: &str,
        content_type: Option<&str>,
    ) -> Result<String, FileStoreError> {
        match self {
            Self::Imagekit(provider) => provider.upload_file(path, file_name, content_type).await,
            Self::S3(provider) => provider.upload_file(path, file_name, content_type).await,
        }
    }
}

#[cfg(test)]
mod tests {
    use secrecy::SecretString;

    use crate::config::{ImagekitConfig, S3Config};

    use super::*;

    fn config(backend: StorageBackend) -> FileStoreConfig {
        FileStoreConfig {
            backend,
            imagekit: ImagekitConfig {
                public_key: SecretString::from("public"),
                private_key: SecretString::from("private"),
                upload_url: "https://upload.imagekit.io/api/v1/files/upload".to_string(),
                folder: None,
            },
            s3: S3Config {
                bucket: "openschool".to_string(),
                region: "eu-west-1".to_string(),
                access_key_id: SecretString::from("key"),
                secret_access_key: SecretString::from("secret"),
                session_token: None,
                endpoint: None,
                force_path_style: false,
                public_base_url: None,
                key_prefix: None,
            },
        }
    }

    #[test]
    fn builds_the_configured_backend() {
        let imagekit = FileStoreEngine::from_config(&config(StorageBackend::Imagekit)).unwrap();
        let s3 = FileStoreEngine::from_config(&config(StorageBackend::S3)).unwrap();

        assert_eq!(imagekit.backend(), StorageBackend::Imagekit);
        assert_eq!(s3.backend(), StorageBackend::S3);
    }

    #[test]
    fn fails_when_the_selected_provider_is_misconfigured() {
        let mut config = config(StorageBackend::S3);
        config.s3.bucket.clear();

        let error = FileStoreEngine::from_config(&config).err().unwrap();

        assert!(matches!(error, FileStoreError::Config(_)));
    }
}

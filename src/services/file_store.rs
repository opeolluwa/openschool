use openschool_filestore::{
    FileStoreConfig, FileStoreEngine, FileStoreError, ImagekitConfig, S3Config, StorageBackend,
};

use crate::config::env::{AppConfig, FileStorageProvider};

/// Maps the application's environment configuration onto the file store crate.
///
/// The crate deliberately knows nothing about `AppConfig`, mirroring the
/// mailman boundary, so this is the one place the two shapes meet.
pub fn file_store_config(config: &AppConfig) -> FileStoreConfig {
    let backend = match config.file_storage_provider {
        FileStorageProvider::Imagekit => StorageBackend::Imagekit,
        FileStorageProvider::S3 => StorageBackend::S3,
    };

    FileStoreConfig {
        backend,
        imagekit: ImagekitConfig {
            public_key: config.imagekit_public_key.clone(),
            private_key: config.imagekit_private_key.clone(),
            upload_url: config.imagekit_upload_url.clone(),
            folder: config.imagekit_folder.clone(),
        },
        s3: S3Config {
            bucket: config.s3_bucket.clone(),
            region: config.s3_region.clone(),
            access_key_id: config.s3_access_key_id.clone(),
            secret_access_key: config.s3_secret_access_key.clone(),
            session_token: config.s3_session_token.clone(),
            endpoint: config.s3_endpoint.clone(),
            force_path_style: config.s3_force_path_style,
            public_base_url: config.s3_public_base_url.clone(),
            key_prefix: config.s3_key_prefix.clone(),
        },
    }
}

/// Builds the configured provider once, so invalid storage credentials fail at
/// startup rather than at the first upload.
pub fn file_store_engine(config: &AppConfig) -> Result<FileStoreEngine, FileStoreError> {
    FileStoreEngine::from_config(&file_store_config(config))
}

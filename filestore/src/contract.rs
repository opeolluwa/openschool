use std::path::Path;

use crate::error::FileStoreError;

/// Anything that can upload a local file and return a public URL.
pub trait FileStoreProvider: Send + Sync {
    async fn upload_file(
        &self,
        path: &Path,
        file_name: &str,
        content_type: Option<&str>,
    ) -> Result<String, FileStoreError>;
}

use std::path::Path;

use base64::{Engine, engine::general_purpose::STANDARD};
use reqwest::{
    Client, Method,
    header::{AUTHORIZATION, HeaderMap, HeaderValue},
    multipart,
};
use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;

use crate::{
    config::ImagekitConfig,
    contract::FileStoreProvider,
    error::FileStoreError,
    key::sanitize_file_name,
};

/// ImageKit's upload response, narrowed to the fields the file store needs.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ImagekitUploadResponse {
    url: String,
}

/// ImageKit's upload API.
#[derive(Debug, Clone)]
pub struct ImagekitProvider {
    client: Client,
    upload_url: String,
    private_key: SecretString,
    folder: Option<String>,
    _public_key: SecretString,
}

impl ImagekitProvider {
    /// Builds an ImageKit provider from the application configuration.
    pub fn new(config: &ImagekitConfig) -> Result<Self, FileStoreError> {
        config.validate()?;

        Ok(Self {
            client: Client::builder().build()?,
            upload_url: config.upload_url.clone(),
            private_key: config.private_key.clone(),
            folder: config.folder.clone(),
            _public_key: config.public_key.clone(),
        })
    }
}

impl FileStoreProvider for ImagekitProvider {
    /// Uploads a local file and returns the URL ImageKit reports for it.
    async fn upload_file(
        &self,
        path: &Path,
        file_name: &str,
        content_type: Option<&str>,
    ) -> Result<String, FileStoreError> {
        let file_name = sanitize_file_name(file_name);
        let file_bytes = tokio::fs::read(path).await?;

        let credentials = STANDARD.encode(format!("{}:", self.private_key.expose_secret()));
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Basic {credentials}"))?,
        );

        let mut part = multipart::Part::bytes(file_bytes).file_name(file_name.clone());

        if let Some(content_type) = content_type {
            part = part.mime_str(content_type)?;
        }

        let mut form = multipart::Form::new().part("file", part).text("fileName", file_name);

        if let Some(folder) = &self.folder {
            form = form.text("folder", folder.clone());
        }

        let response = self
            .client
            .request(Method::POST, &self.upload_url)
            .headers(headers)
            .multipart(form)
            .send()
            .await?;

        let status = response.status();

        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(FileStoreError::Provider(format!(
                "ImageKit rejected the upload with {status}: {}",
                truncate_body(&body)
            )));
        }

        let response = response.json::<ImagekitUploadResponse>().await?;

        Ok(response.url)
    }
}

/// Keeps a provider error body useful without dumping a large response into a
/// log line.
fn truncate_body(body: &str) -> String {
    const MAX_BODY_LEN: usize = 512;

    body.chars().take(MAX_BODY_LEN).collect()
}

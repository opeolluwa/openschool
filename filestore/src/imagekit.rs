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
    config::ImagekitConfig, contract::FileStoreProvider, error::FileStoreError,
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

        let mut form = multipart::Form::new()
            .part("file", part)
            .text("fileName", file_name);

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

#[cfg(test)]
mod tests {
    use std::io::Write;

    use secrecy::SecretString;
    use tempfile::NamedTempFile;
    use tokio::{
        io::{AsyncReadExt, AsyncWriteExt},
        net::TcpListener,
    };

    use super::*;

    fn provider(upload_url: String) -> ImagekitProvider {
        ImagekitProvider::new(&ImagekitConfig {
            public_key: SecretString::from("test-public"),
            private_key: SecretString::from("test-private"),
            upload_url,
            folder: None,
        })
        .unwrap()
    }

    /// Serves one HTTP request and returns both the request text and the body
    /// ImageKit would send back.
    async fn serve_once(
        status: &'static str,
        response_body: &'static str,
    ) -> (String, tokio::task::JoinHandle<String>) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();

        let handle = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            let mut request = Vec::new();
            let mut buffer = [0_u8; 4096];
            let mut header_end = None;
            let mut content_length = 0_usize;

            loop {
                let read = socket.read(&mut buffer).await.unwrap();

                if read == 0 {
                    break;
                }

                request.extend_from_slice(&buffer[..read]);

                if header_end.is_none()
                    && let Some(position) = find_header_end(&request)
                {
                    header_end = Some(position);

                    let headers = String::from_utf8_lossy(&request[..position]);
                    for line in headers.lines() {
                        if let Some((name, value)) = line.split_once(':')
                            && name.eq_ignore_ascii_case("content-length")
                        {
                            content_length = value.trim().parse().unwrap_or_default();
                        }
                    }
                }

                if let Some(header_end) = header_end
                    && request.len() >= header_end + content_length
                {
                    break;
                }
            }

            let response = format!(
                "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                response_body.len(),
                response_body
            );
            socket.write_all(response.as_bytes()).await.unwrap();

            String::from_utf8_lossy(&request).to_string()
        });

        (format!("http://{address}/upload"), handle)
    }

    fn find_header_end(request: &[u8]) -> Option<usize> {
        request
            .windows(4)
            .position(|window| window == b"\r\n\r\n")
            .map(|position| position + 4)
    }

    #[tokio::test]
    async fn uploads_with_basic_auth_and_returns_the_url() {
        let (upload_url, request) = serve_once(
            "200 OK",
            r#"{"url":"https://ik.imagekit.io/acme/photo.png"}"#,
        )
        .await;

        let mut file = NamedTempFile::new().unwrap();
        file.write_all(b"\x89PNG\r\n\x1a\nimage-bytes").unwrap();
        file.flush().unwrap();

        let url = provider(upload_url)
            .upload_file(file.path(), "photo.png", Some("image/png"))
            .await
            .unwrap();

        assert_eq!(url, "https://ik.imagekit.io/acme/photo.png");

        let request = request.await.unwrap();
        let authorization = request
            .lines()
            .find(|line| line.to_ascii_lowercase().starts_with("authorization:"))
            .unwrap();
        assert!(authorization.ends_with("Basic dGVzdC1wcml2YXRlOg=="));
        assert!(request.contains("name=\"fileName\""));
        assert!(request.contains("photo.png"));
    }

    #[tokio::test]
    async fn reports_a_provider_error_body() {
        let (upload_url, request) =
            serve_once("401 Unauthorized", r#"{"message":"bad key"}"#).await;

        let mut file = NamedTempFile::new().unwrap();
        file.write_all(b"bytes").unwrap();
        file.flush().unwrap();

        let error = provider(upload_url)
            .upload_file(file.path(), "photo.png", None)
            .await
            .unwrap_err();

        request.await.unwrap();
        assert!(matches!(error, FileStoreError::Provider(_)));
        assert!(error.to_string().contains("bad key"));
    }
}

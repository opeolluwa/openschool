use std::path::Path;

use aws_sdk_s3::{
    Client,
    config::{BehaviorVersion, Credentials, Region},
    primitives::ByteStream,
};
use secrecy::ExposeSecret;

use crate::{
    config::S3Config, contract::FileStoreProvider, error::FileStoreError, key::build_object_key,
};

/// AWS S3, or an S3-compatible service reached through a custom endpoint.
#[derive(Debug, Clone)]
pub struct S3Provider {
    client: Client,
    config: S3Config,
}

impl S3Provider {
    /// Builds an S3 provider using the static credentials in the configuration.
    pub fn new(config: &S3Config) -> Result<Self, FileStoreError> {
        config.validate()?;

        let credentials = Credentials::new(
            config.access_key_id.expose_secret().to_string(),
            config.secret_access_key.expose_secret().to_string(),
            config
                .session_token
                .as_ref()
                .map(|token| token.expose_secret().to_string()),
            None,
            "openschool-env",
        );

        let mut builder = aws_sdk_s3::config::Builder::new()
            .behavior_version(BehaviorVersion::latest())
            .region(Region::new(config.region.clone()))
            .credentials_provider(credentials);

        if let Some(endpoint) = &config.endpoint {
            builder = builder.endpoint_url(endpoint);
        }

        if config.force_path_style || config.endpoint.is_some() {
            builder = builder.force_path_style(true);
        }

        Ok(Self {
            client: Client::from_conf(builder.build()),
            config: config.clone(),
        })
    }

    /// The URL a client should use to read the object after upload.
    fn public_url(&self, key: &str) -> Result<String, FileStoreError> {
        build_public_url(&self.config, key)
    }
}

impl FileStoreProvider for S3Provider {
    async fn upload_file(
        &self,
        path: &Path,
        file_name: &str,
        content_type: Option<&str>,
    ) -> Result<String, FileStoreError> {
        let file_bytes = tokio::fs::read(path).await?;
        let key = build_object_key(self.config.key_prefix.as_deref(), file_name);

        self.client
            .put_object()
            .bucket(&self.config.bucket)
            .key(&key)
            .body(ByteStream::from(file_bytes))
            .content_type(content_type.unwrap_or("application/octet-stream"))
            .send()
            .await
            .map_err(|err| FileStoreError::Provider(format!("S3 rejected the upload: {err}")))?;

        self.public_url(&key)
    }
}

fn build_public_url(config: &S3Config, key: &str) -> Result<String, FileStoreError> {
    if let Some(base_url) = &config.public_base_url {
        return join_url(base_url, key);
    }

    if let Some(endpoint) = &config.endpoint {
        let object_path = format!("{}/{}", config.bucket.trim_matches('/'), key);
        return join_url(endpoint, &object_path);
    }

    let base_url = format!(
        "https://{}.s3.{}.amazonaws.com",
        config.bucket, config.region
    );

    join_url(&base_url, key)
}

/// Appends each path segment to a base URL, letting `url` percent-encode it.
fn join_url(base_url: &str, path: &str) -> Result<String, FileStoreError> {
    let mut url = url::Url::parse(base_url)?;

    {
        let mut segments = url.path_segments_mut().map_err(|_| {
            FileStoreError::Config(format!("'{base_url}' cannot be used as a base URL"))
        })?;

        segments.pop_if_empty();

        for segment in path.split('/').filter(|segment| !segment.is_empty()) {
            segments.push(segment);
        }
    }

    Ok(url.to_string())
}

#[cfg(test)]
mod tests {
    use secrecy::SecretString;

    use super::*;

    fn config() -> S3Config {
        S3Config {
            bucket: "openschool".to_string(),
            region: "eu-west-1".to_string(),
            access_key_id: SecretString::from("key"),
            secret_access_key: SecretString::from("secret"),
            session_token: None,
            endpoint: None,
            force_path_style: false,
            public_base_url: None,
            key_prefix: None,
        }
    }

    #[test]
    fn uses_the_regional_virtual_host_url() {
        let url = build_public_url(&config(), "uploads/photo.png").unwrap();

        assert_eq!(
            url,
            "https://openschool.s3.eu-west-1.amazonaws.com/uploads/photo.png"
        );
    }

    #[test]
    fn prefers_the_public_base_url() {
        let mut config = config();
        config.public_base_url = Some("https://cdn.example.com/files".to_string());

        let url = build_public_url(&config, "uploads/photo.png").unwrap();

        assert_eq!(url, "https://cdn.example.com/files/uploads/photo.png");
    }

    #[test]
    fn uses_a_path_style_url_for_custom_endpoints() {
        let mut config = config();
        config.endpoint = Some("http://localhost:9000".to_string());

        let url = build_public_url(&config, "uploads/photo.png").unwrap();

        assert_eq!(url, "http://localhost:9000/openschool/uploads/photo.png");
    }

    #[test]
    fn percent_encodes_object_keys() {
        let mut config = config();
        config.public_base_url = Some("https://cdn.example.com".to_string());

        let url = build_public_url(&config, "uploads/holiday photo.png").unwrap();

        assert_eq!(url, "https://cdn.example.com/uploads/holiday%20photo.png");
    }

    #[test]
    fn rejects_missing_credentials() {
        let mut config = config();
        config.access_key_id = SecretString::from("");

        let error = S3Provider::new(&config).err().unwrap();

        assert!(matches!(error, FileStoreError::Config(_)));
    }
}

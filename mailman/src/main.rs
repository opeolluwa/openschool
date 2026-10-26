use openschool_mailman::{EMAIL_EVENT, MailmanConfig, check_health, start};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .compact()
        .init();

    let config = MailmanConfig::from_env()?;
    let health_config = config.clone();

    let handle = start(config).await?;
    tracing::info!(
        driver = %handle.driver(),
        event = EMAIL_EVENT,
        "mailman started"
    );

    tokio::spawn(check_health(health_config));

    shutdown_signal().await;
    tracing::info!("mailman shutting down");

    drop(handle);
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

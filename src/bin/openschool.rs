use std::net::{Ipv4Addr, SocketAddrV4};
use std::num::NonZeroU32;

use axum::{
    Router,
    extract::DefaultBodyLimit,
    http::{HeaderName, HeaderValue, Method, StatusCode},
};
use axum_governor::{
    BodyPreset, GovernorConfigBuilder, GovernorLayer, Quota, extractor::PeerIp, nz,
};
use lib_openschool::config::db::init_db_pool;
use lib_openschool::config::{
    cors::init_cors, env::load_config, logger::init_tracing, shutdown::shutdown_signal,
};
use lib_openschool::errors::app_error::AppError;
use lib_openschool::middlewares::csrf::CsrfLayer;
use lib_openschool::services::mailer::mailman_config;

use tower_http::{
    compression::CompressionLayer, limit::RequestBodyLimitLayer,
    sensitive_headers::SetSensitiveHeadersLayer, timeout::TimeoutLayer,
};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let app_config = load_config()?;

    // A zero-valued env var would otherwise panic in NonZeroU32::new; fall back
    // to the hardened defaults instead of failing to start.
    let requests_per_second =
        NonZeroU32::new(app_config.rate_limit_requests_per_second).unwrap_or(nz!(20u32));
    let burst = NonZeroU32::new(app_config.rate_limit_burst).unwrap_or(nz!(40u32));
    let requests_per_minute =
        NonZeroU32::new(app_config.rate_limit_requests_per_minute).unwrap_or(nz!(300u32));

    let governor = GovernorConfigBuilder::default()
        .with_extractor(PeerIp::default())
        .expect_connect_info()
        // Primary per-second quota, with a longer sustained ceiling stacked on top.
        .quota_default(Quota::requests_per_second(requests_per_second).burst(burst))
        .stack(
            "per-minute",
            PeerIp::default(),
            Quota::requests_per_minute(requests_per_minute),
        )
        // Health probes must never be throttled, and preflights are handled by CORS.
        .whitelist_paths(["/health"])
        .whitelist_methods([Method::OPTIONS])
        // Cap the per-key sidecar so IP rotation cannot exhaust memory, and keep
        // peer IPs out of the logs.
        .max_keys(app_config.rate_limit_max_keys.max(1))
        .redact_keys(true)
        .body_preset(BodyPreset::Json)
        .finish()
        .map_err(|err| {
            tracing::warn!("failed to startup due to {} ", err.to_string());
            AppError::StartupError("internal server error".to_string())
        })?;

    init_tracing(&app_config);

    let _db_conn = init_db_pool(&app_config).await?;
    // let app_state = AppState::new(&db_conn).await?;

    let mailman_settings = mailman_config(&app_config)?;
    let mailman_health = mailman_settings.clone();
    let mailman = openschool_mailman::start(mailman_settings)
        .await
        .map_err(|err| AppError::StartupError(format!("mailman failed to start: {err}")))?;
    tracing::info!(driver = %mailman.driver(), "mailman started");
    tokio::spawn(openschool_mailman::check_health(mailman_health));

    let app = Router::new()
        // .merge(http_routes)
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(
            app_config.body_limit_megabytes * 1024 * 1024,
        ))
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            app_config.requests_time_out_secs,
        ))
        .layer(CompressionLayer::new())
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .layer(SetSensitiveHeadersLayer::new([
            axum::http::header::AUTHORIZATION,
            axum::http::header::COOKIE,
            axum::http::header::SET_COOKIE,
        ]))
        .layer(CsrfLayer::new(
            app_config.allowed_origins.clone(),
            app_config.is_production(),
        ))
        .layer(GovernorLayer::new(governor))
        .layer(init_cors(&app_config))
        .layer({
            let is_production = app_config.is_production();
            axum::middleware::map_response(move |mut response: axum::response::Response| async move {
                let headers = response.headers_mut();
                headers.insert(
                    HeaderName::from_static("x-content-type-options"),
                    HeaderValue::from_static("nosniff"),
                );
                headers.insert(
                    HeaderName::from_static("x-frame-options"),
                    HeaderValue::from_static("DENY"),
                );
                headers.insert(
                    HeaderName::from_static("referrer-policy"),
                    HeaderValue::from_static("no-referrer"),
                );
                headers.insert(
                    HeaderName::from_static("permissions-policy"),
                    HeaderValue::from_static("geolocation=(), microphone=(), camera=()"),
                );
                headers.insert(
                    HeaderName::from_static("cross-origin-opener-policy"),
                    HeaderValue::from_static("same-origin"),
                );
                headers.insert(
                    HeaderName::from_static("cross-origin-resource-policy"),
                    HeaderValue::from_static("same-origin"),
                );
                headers.insert(
                    HeaderName::from_static("content-security-policy"),
                    HeaderValue::from_static(
                        "default-src 'none'; frame-ancestors 'none'; base-uri 'none'; form-action 'none'",
                    ),
                );
                if is_production {
                    headers.insert(
                        HeaderName::from_static("strict-transport-security"),
                        HeaderValue::from_static("max-age=31536000; includeSubDomains"),
                    );
                }
                response
            })
        });

    let ip_address = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, app_config.port);

    tracing::info!("Service health check at http://{}/health", ip_address,);
    let listener = tokio::net::TcpListener::bind(ip_address).await?;

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await?;

    tracing::info!("Server shutdown completed");
    Ok(())
}

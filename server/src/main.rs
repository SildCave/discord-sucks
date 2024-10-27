#![feature(test)]
#![feature(trivial_bounds)]

use auth::JWTKeys;
use axum::middleware;
use axum_server::tls_rustls::RustlsConfig;
use cloudflare::cloudflare_validation_middleware;
use reqwest::Method;
use routes::configure_routes;
use tokio::sync::RwLock;
use tracing::info;


use std::{
    net::SocketAddr,
    path::PathBuf,
    sync::Arc,
    time::Duration
};
use tower_http::{
    cors::{
        Any, CorsLayer,
    },
    services::ServeDir,
    trace::{DefaultMakeSpan, TraceLayer},
};

use axum_client_ip::SecureClientIpSource;

mod database;
mod configuration;
mod logs;
mod cloudflare;
mod routes;
mod auth;
mod app_objects;
mod state;
mod credentials;

use server::{
    start_main_server,
    start_metrics_server
};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    logs::setup_logging()?;
    let config = configuration::Config::from_file("configuration/server/config.toml")?;

    let cloudflare_ips = cloudflare::CloudflareIpAddresses::new_from_cloudflare_api().await;
    let cloudflare_ips = Arc::new(RwLock::new(cloudflare_ips?));

    let cloudflare_validation_state = cloudflare::CloudflareValidationState {
        cloudflare_ips: cloudflare_ips.clone(),
        allow_non_cloudflare_ips: config.server.allow_non_cloudflare_ips,
    };

    let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");

    let jwt_keys = JWTKeys::new(&config)?;

    info!("Connecting to databases");
    let db_client = database::DatabaseClientWithCaching::new(
        &config.redis_database,
        &config.postgres_database
    ).await?;



    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any);

    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::default().include_headers(true));
    let app = configure_routes(
        &jwt_keys,
        &config.jwt_config
    ).await;
    let app = app
        .fallback_service(ServeDir::new(assets_dir).append_index_html_on_directories(true))
        .layer(
            trace_layer.clone()
        )
        .layer(cors)
        .layer(SecureClientIpSource::ConnectInfo.into_extension())
        .layer(middleware::from_fn_with_state(
            cloudflare_validation_state.clone(),
            cloudflare_validation_middleware
        ));
        // .with_state(db_client)
        // .with_state(cloudflare_ips);

    let server_tls_config: Option<RustlsConfig> = {
        if config.server.enable_https {
            Some(
                RustlsConfig::from_pem_file(
                    config.server.pem_cert_path.unwrap(),
                    config.server.pem_key_path.unwrap()
                ).await?
        )
        } else {
            None
        }
    };

    let server_addr = SocketAddr::new(
        config.server.host.parse().unwrap(),
        config.server.port,
    );


    let cloudflare_refresh_cron_job_enable = ! config.server.allow_non_cloudflare_ips;
    let (_main_server, _cloudflare_refresh_job) = tokio::join!(
        start_main_server(
            app,
            server_addr,
            server_tls_config.clone()
        ),
        cloudflare::cloudflare_ip_refresh_cron_job(
            cloudflare_ips,
            Duration::from_secs(config.server.cloudflare_ips_refresh_interval_s.unwrap_or(3600 * 24)),
            Duration::from_secs(config.server.cloudflare_ips_refresh_interval_jitter_s.unwrap_or(3600)),
            cloudflare_refresh_cron_job_enable
        )
    );
    Ok(())
}



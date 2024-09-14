#![feature(test)]


use auth::JWTKeys;
use axum::middleware;
use axum_server::tls_rustls::RustlsConfig;
use cloudflare::cloudflare_validation_middleware;
use reqwest::Method;
use routes::configure_routes;
use tokio::sync::RwLock;


use std::{net::SocketAddr, path::PathBuf, sync::Arc};
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

    let db_client = database::prepare_mongodb_client(
        config.mongo_database.username,
        config.mongo_database.password.unwrap(),
        config.mongo_database.source_db,
    )?;

    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any);

    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::default().include_headers(true));
    let app = configure_routes(
        jwt_keys
    ).await;
    let app = app
        .fallback_service(ServeDir::new(assets_dir).append_index_html_on_directories(true))
        .route_layer(
            middleware::from_fn(
                logs::metrics::track_metrics
            )
        )
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

    let metrics_app = logs::metrics::metrics_app(
        trace_layer
    );
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
    let metrics_server_addr = SocketAddr::new(
        config.metrics_server.host.parse().unwrap(),
        config.metrics_server.port,
    );



    let (_main_server, _metrics_server, _cloudflare_refresh_job) = tokio::join!(
        start_main_server(
            app,
            server_addr,
            server_tls_config.clone()
        ), start_metrics_server(
            metrics_app,
            metrics_server_addr
        ), cloudflare::cloudflare_ip_refresh_cron_job(
            cloudflare_ips,
            std::time::Duration::from_secs(config.server.cloudflare_ips_refresh_interval_s.unwrap_or(60) as u64)
        )
    );
    Ok(())
}



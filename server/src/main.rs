#![feature(test)]


use axum::{
    middleware, routing::get, Router
};
use axum_server::tls_rustls::RustlsConfig;
use cloudflare::cloudflare_validation_middleware;
use tokio::sync::RwLock;


use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tower_http::{
    services::ServeDir,
    trace::{DefaultMakeSpan, TraceLayer},
};


mod database;
mod configuration;
mod logs;
mod cloudflare;

use axum_client_ip::SecureClientIpSource;



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    logs::setup_logging()?;

    let cloudflare_ips = cloudflare::CloudflareIpAddresses::new_from_cloudflare_api().await;
    let cloudflare_ips = Arc::new(RwLock::new(cloudflare_ips?));
    let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");

    let config = configuration::Config::from_file("configuration/server/config.toml")?;
    let db_client = database::prepare_mongodb_client(
        config.database.username,
        config.database.password.unwrap(),
        config.database.source_db,
    )?;

    let app = Router::new()
        .fallback_service(ServeDir::new(assets_dir).append_index_html_on_directories(true))
        // logging so we can see whats going on
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        )
        .layer(SecureClientIpSource::ConnectInfo.into_extension())
        .layer(middleware::from_fn_with_state(
            cloudflare_ips.clone(),
            cloudflare_validation_middleware
        ))
        .with_state(db_client)
        .with_state(cloudflare_ips);

    let server_tls_config = RustlsConfig::from_pem_file(
        config.server.pem_cert_path,
        config.server.pem_key_path
    ).await?;


    let server_addr = SocketAddr::new(
        config.server.host.parse().unwrap(),
        config.server.port,
    );
    tracing::debug!("listening on {}", server_addr);


    axum_server::bind_rustls(
        server_addr,
        server_tls_config
    ).serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();

    Ok(())
}


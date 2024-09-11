#![feature(test)]


use axum::{
    middleware, Router
};
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


mod database;
mod configuration;
mod logs;
mod cloudflare;
mod routes;

use axum_client_ip::SecureClientIpSource;



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    logs::setup_logging()?;
    let config = configuration::Config::from_file("configuration/server/config.toml")?;

    let cloudflare_ips = cloudflare::CloudflareIpAddresses::new_from_cloudflare_api().await;
    let cloudflare_ips = Arc::new(RwLock::new(cloudflare_ips?));
    let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");

    let db_client = database::prepare_mongodb_client(
        config.database.username,
        config.database.password.unwrap(),
        config.database.source_db,
    )?;

    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any);

    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::default().include_headers(true));
    let app = configure_routes().await;
    let app = app
        .fallback_service(ServeDir::new(assets_dir).append_index_html_on_directories(true))
        // logging so we can see whats going on
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
            cloudflare_ips.clone(),
            cloudflare_validation_middleware
        ));
        // .with_state(db_client)
        // .with_state(cloudflare_ips);

    let metrics_app = logs::metrics::metrics_app(
        trace_layer
    );

    let server_tls_config = RustlsConfig::from_pem_file(
        config.server.pem_cert_path,
        config.server.pem_key_path
    ).await?;

    let server_addr = SocketAddr::new(
        config.server.host.parse().unwrap(),
        config.server.port,
    );
    let metrics_server_addr = SocketAddr::new(
        config.metrics_server.host.parse().unwrap(),
        config.metrics_server.port,
    );


    tracing::info!("server listening on {}", server_addr);
    tracing::info!("metrics server listening on {}", metrics_server_addr);

    let (_main_server, _metrics_server) = tokio::join!(
        start_main_server(
            app,
            server_addr,
            server_tls_config.clone()
        ), start_metrics_server(
            metrics_app,
            metrics_server_addr
        )
    );
    Ok(())
}


async fn start_main_server(
    app: Router,
    server_addr: SocketAddr,
    server_tls_config: RustlsConfig,
) {
    axum_server::bind_rustls(
        server_addr,
        server_tls_config
    ).serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}

async fn start_metrics_server(
    metrics_app: Router,
    server_addr: SocketAddr,
) {
    let listener = tokio::net::TcpListener::bind(server_addr)
        .await
        .unwrap();
    axum::serve(listener, metrics_app).await.unwrap();

}
use std::net::SocketAddr;

use axum::Router;
use axum_server::tls_rustls::RustlsConfig;



pub async fn start_main_server(
    app: Router,
    server_addr: SocketAddr,
    server_tls_config: Option<RustlsConfig>,
    domain: Option<String>
) {
    match server_tls_config {
        Some(server_tls_config) => {
            let base_url: String;
            if domain.is_some() {
                base_url = format!("https://{}:{}", domain.unwrap(), server_addr.port());
            } else {
                base_url = format!("https://{}:{}", server_addr, server_addr.port());
            }
            tracing::info!("server listening on {}", base_url);
            axum_server::bind_rustls(
                server_addr,
                server_tls_config
            ).serve(app.into_make_service_with_connect_info::<SocketAddr>())
                .await
                .unwrap();

        }
        None => {
            let base_url: String;
            if domain.is_some() {
                base_url = format!("http://{}:{}", domain.unwrap(), server_addr.port());
            } else {
                base_url = format!("http://{}:{}", server_addr, server_addr.port());
            }
            tracing::info!("server listening on {}", base_url);
            axum_server::bind(server_addr)
                .serve(app.into_make_service_with_connect_info::<SocketAddr>())
                .await
                .unwrap();
        }
    }

}

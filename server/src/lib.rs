use std::net::SocketAddr;

use axum::Router;
use axum_server::tls_rustls::RustlsConfig;



pub async fn start_main_server(
    app: Router,
    server_addr: SocketAddr,
    server_tls_config: Option<RustlsConfig>,
) {
    match server_tls_config {
        Some(server_tls_config) => {
            tracing::info!("server listening on https://{}", server_addr);
            axum_server::bind_rustls(
                server_addr,
                server_tls_config
            ).serve(app.into_make_service_with_connect_info::<SocketAddr>())
                .await
                .unwrap();

        }
        None => {
            tracing::info!("server listening on http://{}", server_addr);
            axum_server::bind(server_addr)
                .serve(app.into_make_service_with_connect_info::<SocketAddr>())
                .await
                .unwrap();
        }
    }

}

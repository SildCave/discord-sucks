
use axum::{
    extract::{
        ConnectInfo,
        Request, State
    },
    http::StatusCode,
    middleware::Next,
    response::{
        Html,
        IntoResponse,
        Response
    }
};

use tokio::sync::RwLock;
use tracing::{trace, warn};


use std::{
    net::SocketAddr,
    sync::Arc
};


use crate::cloudflare::CloudflareIpAddresses;

pub async fn cloudflare_validation_middleware(
    State(cloudflare_ips): State<Arc<RwLock<CloudflareIpAddresses>>>,
    connection_info: ConnectInfo<SocketAddr>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let ip = connection_info.ip();
    let cloudflare_ips = cloudflare_ips.read().await;
    trace!("Request from IP: {}", ip);
    if cloudflare_ips.is_cloudflare_ip(ip) {
        trace!("Request from Cloudflare IP: {}", ip);
        drop(cloudflare_ips);
        let response = next.run(request).await;
        return Ok(response);
    } else {
        warn!("Request from non-Cloudflare IP: {}, Request: {:?}", ip, request);
        let mut response = Html(
            r#"
                <!DOCTYPE html>
                <html lang="en">
                <head>
                    <meta charset="UTF-8">
                    <meta name="viewport" content="width=device-width, initial-scale=1.0">
                    <title>Full Screen Image</title>
                    <style>
                        html, body {
                            margin: 0;
                            height: 100%;
                        }
                        img {
                            display: block;
                            width: 100vw;
                            height: 100vh;
                        }
                    </style>
                </head>
                <body>
                    <img src="https://http.cat/403" alt="403 Forbidden">
                </body>
                </html>
            "#,
        ).into_response();
        *response.status_mut() = StatusCode::FORBIDDEN;
        return Ok(
            response
        );

    }
}
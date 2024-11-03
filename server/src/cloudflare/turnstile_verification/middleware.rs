
use axum::{
    body, extract::{
        ConnectInfo,
        Request, State
    }, http::StatusCode, middleware::Next, response::{
        Html,
        IntoResponse,
        Response
    }
};

use headers::Header;
use tokio::sync::RwLock;
use tracing::{info, trace, warn};


use std::{
    net::SocketAddr,
    sync::Arc
};

use super::{state::CloudflareTurnstileState, TurnstileError};


use axum_extra::TypedHeader;
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::request::Parts,
    RequestPartsExt,
};


use anyhow::Result;



#[derive(Debug, Clone, PartialEq)]
pub enum TurnstileResult {
    Allowed,
    Denied,
}




pub async fn turnstile_verification(
    State(turnstile_state): State<CloudflareTurnstileState>,
    body: body::Body,
    next: Next,
) -> impl IntoResponse {
    let body_str = body::to_bytes(body, 1024*10).await.unwrap();
    let body_str = String::from_utf8(body_str.to_vec()).unwrap();
    info!("body_str: {}", body_str);

    // do something with `request`...
    // zajebie sie 
    // let body = request.body();
    // let body = body.into_data_stream();

    // do something with `response`...

    
}
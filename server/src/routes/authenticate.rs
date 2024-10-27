// TODO: https://github.com/tokio-rs/axum/blob/main/examples/jwt/src/main.rs
use axum::{
    extract::State,
    http::{
        HeaderMap,
        HeaderValue
    },
    response::IntoResponse,
    Json
};
use jsonwebtoken::{
    encode,
    Header
};
use reqwest::header::SET_COOKIE;
use std::sync::Arc;
use crate::{
    auth::{
        AuthError,
        AuthenticationBody,
        AuthenticationPayload,
        ClaimType,
        Claims,
    },
    state::AuthenticationState
};


pub async fn authenticate(
    State(authentication_state): State<Arc<AuthenticationState>>,
    Json(payload): Json<AuthenticationPayload>,
) -> Result<impl IntoResponse, AuthError> {
    // Check if the user sent the credentials
    if payload.email.is_empty() || payload.password.is_empty() {
        return Err(AuthError::MissingCredentials);
    }
    // Here you can check the user credentials from a database
    if payload.email != "foo" || payload.password != "bar" {
        return Err(AuthError::WrongCredentials);
    }

    let claims: Claims = Claims::new_refresh(
        authentication_state.jwt_config.refresh_key_lifetime_s
    );
    // Create the authorization token
    let token = encode(
        &Header::default(),
        &claims,
        &authentication_state.jwt_keys.encoding
    )
        .map_err(|_| AuthError::TokenCreation)?;

    let cookie = cookie::Cookie::build(
        (ClaimType::Refresh.as_str(), format!("Bearer {}", token))
    )
        .max_age(cookie::time::Duration::seconds(
            authentication_state.jwt_config.refresh_key_lifetime_s
        ))
        //.secure(true)
        .http_only(false);

    let mut headers = HeaderMap::new();
    headers.insert(SET_COOKIE, HeaderValue::from_str(&cookie.to_string()).unwrap());


    // Send the authorized token
    Ok((headers, Json(AuthenticationBody::new(token))))
}
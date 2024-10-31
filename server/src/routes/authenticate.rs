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
use tracing::{error, info, instrument};
use uuid::Uuid;
use std::sync::Arc;
use crate::{
    auth::{
        AuthError,
        AuthenticationBody,
        AuthenticationPayload,
        ClaimType,
        Claims,
    }, credentials::Password, state::AuthenticationState
};

// FIX TRACING ON ASYNC

pub async fn authenticate(
    State(authentication_state): State<Arc<AuthenticationState>>,
    Json(payload): Json<AuthenticationPayload>,
) -> Result<impl IntoResponse, AuthError> {
    let request_id = Uuid::new_v4().to_string();
    let request_span = tracing::info_span!(
        "authenticate_user",
        %request_id,
        subscriber_email = %payload.email,
    );
    let _request_span_guard = request_span.enter();
    info!("authenticating user");

    // Check if email exists in the db
    let db_res = authentication_state.db_client.cached_get_user_id_by_email(&payload.email).await;
    //info!("user_id: {:?}", user_id);
    if db_res.is_err() {
        let db_error = db_res.unwrap_err();
        error!("db_error: {:?}", db_error);
        let error = db_error.to_auth_error();
        return Err(error);
    }

    let user_id = db_res.unwrap();
    if user_id.is_none() {
        return Err(AuthError::WrongCredentials);
    }
    let user_id = user_id.unwrap();
    let db_res = authentication_state.db_client.cached_get_password_hash_and_salt_by_user_id(user_id).await;
    if db_res.is_err() {
        let db_error = db_res.unwrap_err();
        error!("db_error: {:?}", db_error);
        let error = db_error.to_auth_error();
        return Err(error);
    }

    let (password_hash, salt) = db_res.unwrap();

    info!("salt: {:?}", salt);
    let user_imputed_password_hash = Password::new(
        &payload.password,
        &authentication_state.password_requirements
    );
    // Check if the password is correct
    let match_result = user_imputed_password_hash.check_if_password_matches_hash(
        &salt,
        &password_hash
    ).await;
    if match_result.is_err() {
        let error = match_result.as_ref().unwrap_err();
        error!("password error: {:?}", error);
        let error = error.to_auth_error();
        return Err(error);
    }

    let valid = match_result.unwrap();

    if !valid {
        return Err(AuthError::WrongCredentials);
    }

    // Get password hash from the db


    let claims: Claims = Claims::new_refresh(
        authentication_state.jwt_config.refresh_key_lifetime_s,
        user_id
    );
    // Create the authorization token
    let refresh_token = encode(
        &Header::default(),
        &claims,
        &authentication_state.jwt_keys.encoding
    )
        .map_err(|_| AuthError::TokenCreation)?;

    let cookie = cookie::Cookie::build(
        (ClaimType::Refresh.as_str(), format!("Bearer {}", refresh_token))
    )
        .max_age(cookie::time::Duration::seconds(
            authentication_state.jwt_config.refresh_key_lifetime_s
        ))
        //.secure(true)
        .http_only(false);

    let mut headers = HeaderMap::new();
    headers.insert(SET_COOKIE, HeaderValue::from_str(&cookie.to_string()).unwrap());

    // Update the refresh token in the db
    let db_res = authentication_state.db_client.cached_update_user_refresh_token(
        user_id,
        &refresh_token
    ).await;
    if db_res.is_err() {
        let db_error = db_res.unwrap_err();
        error!("db_error: {:?}", db_error);
        let error = db_error.to_auth_error();
        return Err(error);
    }

    // Send the authorized token
    Ok((headers, Json(AuthenticationBody::new(refresh_token))))
}



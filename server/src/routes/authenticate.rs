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
use tracing::info;
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


pub async fn authenticate(
    State(authentication_state): State<Arc<AuthenticationState>>,
    Json(payload): Json<AuthenticationPayload>,
) -> Result<impl IntoResponse, AuthError> {

    // Check if email exists in the db
    let user_id = authentication_state.db_client.get_user_id_by_email_with_cache(&payload.email).await.unwrap();
    info!("user_id: {:?}", user_id);
    if user_id.is_none() {
        return Err(AuthError::WrongCredentials);
    }
    let user_id = user_id.unwrap();
    let (password_hash, salt) = authentication_state.db_client.get_password_hash_and_salt_by_user_id_with_caching(user_id).await.unwrap();
    info!("salt: {:?}", salt);
    let user_imputed_password_hash = Password::new(
        &payload.password,
        &authentication_state.password_requirements
    );
    // Check if the password is correct
    let valid = user_imputed_password_hash.check_if_password_matches_hash(
        &salt,
        &password_hash
    ).await.unwrap();

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
    authentication_state.db_client.update_user_refresh_token_with_caching(
        user_id,
        &refresh_token
    ).await.unwrap();

    // Send the authorized token
    Ok((headers, Json(AuthenticationBody::new(refresh_token))))
}



// TODO: https://github.com/tokio-rs/axum/blob/main/examples/jwt/src/main.rs
use axum::{
    extract::State, Json,
};
use jsonwebtoken::{
    encode,
    Header
};

use crate::auth::{AuthenticationBody, AuthenticationPayload, AuthError, Claims, JWTKeys};


pub async fn authorize(
    State(state): State<JWTKeys>,
    Json(payload): Json<AuthenticationPayload>,
) -> Result<Json<AuthenticationBody>, AuthError> {
    // Check if the user sent the credentials
    if payload.client_id.is_empty() || payload.client_secret.is_empty() {
        return Err(AuthError::MissingCredentials);
    }
    // Here you can check the user credentials from a database
    if payload.client_id != "foo" || payload.client_secret != "bar" {
        return Err(AuthError::WrongCredentials);
    }
    let claims: Claims = Claims {
        iat: chrono::Utc::now().timestamp() as u64,
        exp: (chrono::Utc::now() + chrono::Duration::seconds(222)).timestamp() as u64, // SUBJECT TO CHANGE
    };
    // Create the authorization token
    let token = encode(&Header::default(), &claims, &state.encoding)
        .map_err(|_| AuthError::TokenCreation)?;

    // Send the authorized token
    Ok(Json(AuthenticationBody::new(token)))
}
mod authorization;
mod authentication;

mod jwt;

pub use authentication::{
    AuthClaims,
    JWTKeys,
    AuthenticationPayload,
    AuthenticationBody,
};

pub use jwt::{
    ClaimType,
    extract_token_from_cookie,
    VerificationError
};

use serde_json::json;
use axum::{
    http::StatusCode,
    response::{
        IntoResponse,
        Response
    },
    Json
};

#[derive(Debug)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
    ExpiredToken,
    NoToken,
    InternalError(&'static str),
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "1300"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
            AuthError::ExpiredToken => (StatusCode::UNAUTHORIZED, "Expired token"),
            AuthError::NoToken => (StatusCode::BAD_REQUEST, "No token"),
            AuthError::InternalError(error_message) => {
                (StatusCode::INTERNAL_SERVER_ERROR, error_message)
            },
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}
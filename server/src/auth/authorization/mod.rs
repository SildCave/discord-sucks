mod jwt;
mod authorization;

pub use jwt::{
    Claims,
    JWTKeys
};

pub use authorization::{
    AuthenticationBody,
    AuthenticationPayload
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
    ExpiredToken
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
            AuthError::ExpiredToken => (StatusCode::UNAUTHORIZED, "Expired token"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
} 
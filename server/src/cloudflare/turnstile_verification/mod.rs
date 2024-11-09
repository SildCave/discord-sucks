mod state;
mod extractor;
mod validation;
mod tests;

pub use state::TurnstileState;
pub use extractor::{
    TurnstileRequest,
    TurnstileResult,
    GetTurnstileCode
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

use tracing::error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TurnstileError {
    #[error("Invalid body")]
    InvalidBody,
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Request to turnstile failed, response: {0}")]
    RequestFailed(String),
    #[error("Turnstile response deserialization failed: {0}")]
    DeserializationFailed(#[from] serde_json::Error),
    #[error("Turnstile invalid input secret")]
    InvalidInputSecret,
    #[error("Turnstile invalid input response")]
    InvalidInputResponse,
    #[error("Invalid turnstile response, success field not found")]
    SuccessFieldNotFound,
}

impl IntoResponse for TurnstileError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            TurnstileError::InvalidBody => {
                (StatusCode::BAD_REQUEST, "Invalid body, cf-turnstile-response is required")
            },
            TurnstileError::ReqwestError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "1500")
            },
            TurnstileError::RequestFailed(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "1501")
            },
            TurnstileError::DeserializationFailed(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "1502")
            },
            TurnstileError::InvalidInputSecret => {
                (StatusCode::INTERNAL_SERVER_ERROR, "1503")
            },
            TurnstileError::InvalidInputResponse => {
                (StatusCode::INTERNAL_SERVER_ERROR, "1504")
            },
            TurnstileError::SuccessFieldNotFound => {
                (StatusCode::INTERNAL_SERVER_ERROR, "1505")
            },
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

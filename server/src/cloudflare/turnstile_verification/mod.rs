mod state;
mod middleware;

use serde_json::json;
use axum::{
    http::StatusCode,
    response::{
        IntoResponse,
        Response
    },
    Json
};


pub use middleware::{
    TurnstileResult,
    turnstile_verification
};
pub use state::CloudflareTurnstileState;


#[derive(Debug, Clone)]
pub enum TurnstileError {
    MissingTurnstileHeader,
    InvalidTurnstileToken,
    InternalError(&'static str),
}

impl IntoResponse for TurnstileError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            TurnstileError::MissingTurnstileHeader => (StatusCode::BAD_REQUEST, "Missing Turnstile header"),
            TurnstileError::InvalidTurnstileToken => (StatusCode::BAD_REQUEST, "Invalid Turnstile token"),
            TurnstileError::InternalError(error_message) => {
                (StatusCode::INTERNAL_SERVER_ERROR, error_message)
            },
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

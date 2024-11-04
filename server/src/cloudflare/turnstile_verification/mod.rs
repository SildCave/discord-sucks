mod state;
mod extractor;
mod validation;
mod tests;

pub use state::TurnstileState;
pub use extractor::TurnstileResult;

use serde_json::json;
use axum::{
    http::StatusCode,
    response::{
        IntoResponse,
        Response
    },
    Json
};


#[derive(Debug, Clone, PartialEq)]
pub enum TurnstileError {
    InvalidBody,
    InternalError(&'static str),
}

impl IntoResponse for TurnstileError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            TurnstileError::InvalidBody => (StatusCode::BAD_REQUEST, "Invalid body, cf-turnstile-response is required"),
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

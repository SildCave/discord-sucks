use axum::{
    http::StatusCode,
    response::{
        IntoResponse,
        Response
    },
    Json
};
use serde_json::json;

mod payload;
mod user_form_for_jwt;
mod extractor;

pub use payload::CredentialBasedRegistrationPayload;
pub use user_form_for_jwt::UserRegistrationFormJWT;



#[derive(Debug, Clone, PartialEq)]
pub enum RegistrationError {
    InvalidBody,
}

impl IntoResponse for RegistrationError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            RegistrationError::InvalidBody => (StatusCode::BAD_REQUEST, "Invalid body, cf-turnstile-response is required"),

        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

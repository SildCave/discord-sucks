mod email_verification;
mod email_handler;
mod email_handler_state;

mod tests;

use axum::response::IntoResponse;
pub use email_handler::EmailHandler;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum EmailHandlerError {
    #[error("Email sending error: {0}")]
    EmailCreationFailed(String),
    #[error("Email sending error: {0}")]
    EmailSendingFailed(String),
}

impl IntoResponse for EmailHandlerError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            EmailHandlerError::EmailCreationFailed(_) => {
                (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "1600")
            },
            EmailHandlerError::EmailSendingFailed(_) => {
                (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "1601")
            },
        };

        axum::http::Response::builder()
            .status(status)
            .body(error_message.into())
            .unwrap()
    }
}
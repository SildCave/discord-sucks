mod user;

use axum::response::IntoResponse;
use thiserror::Error;

use crate::auth::AuthError;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("An error occurred while interacting with the database: {0}")]
    SQLXError(#[from] sqlx::Error),
    #[error("An error occurred while interacting with the Redis database: {0}")]
    RedisError(#[from] redis::RedisError),
    #[error("UUID parsing error: {0}")]
    UUIDError(#[from] uuid::Error),
    #[error("Tokio error: {0}")]
    TokioError(#[from] tokio::task::JoinError),
    #[error("User with id: {0} not found")]
    UserNotFound(i64),
    #[error("User with id: {0} already exists")]
    UserAlreadyExists(i64),
}

impl IntoResponse for DatabaseError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            DatabaseError::SQLXError(_) => {
                (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "1200")
            },
            DatabaseError::RedisError(_) => {
                (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "1201")
            },
            DatabaseError::UUIDError(_) => {
                (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "1202")
            },
            DatabaseError::TokioError(_) => {
                (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "1203")
            },
            DatabaseError::UserNotFound(_) => {
                (axum::http::StatusCode::NOT_FOUND, "1204")
            },
            DatabaseError::UserAlreadyExists(_) => {
                (axum::http::StatusCode::BAD_REQUEST, "1205")
            },
        };

        axum::http::Response::builder()
            .status(status)
            .body(error_message.into())
            .unwrap()
    }
}

impl DatabaseError {
    pub fn into_internal_error_code(&self) -> &'static str {
        match self {
            DatabaseError::SQLXError(_) => "1200",
            DatabaseError::RedisError(_) => "1201",
            DatabaseError::UUIDError(_) => "1202",
            DatabaseError::TokioError(_) => "1203",
            DatabaseError::UserNotFound(_) => "1204",
            DatabaseError::UserAlreadyExists(_) => "1205",
        }
    }

    pub fn to_auth_error(&self) -> AuthError {
        AuthError::InternalError(
            self.into_internal_error_code()
        )
    }
}
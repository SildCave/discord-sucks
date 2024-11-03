mod user;

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
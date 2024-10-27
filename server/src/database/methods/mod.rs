mod message;
mod login;
mod user;

use thiserror::Error;

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


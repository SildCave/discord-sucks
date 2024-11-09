
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RegistrationError {
    #[error("Invalid body")]
    InvalidBody,
    #[error("Internal error: {0}")]
    InternalError(&'static str),
}

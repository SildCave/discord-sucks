mod email_verification;
mod email_handler;
mod email_handler_state;

mod tests;

pub use email_handler::EmailHandler;

#[derive(Debug, Clone)]
pub enum EmailHandlerError {
    EmailConstructionError(String),
    EmailCreationFailed(String),
    EmailSendingFailed(String),
}


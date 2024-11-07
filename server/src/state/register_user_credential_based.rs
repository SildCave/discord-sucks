use crate::{auth::JWTKeys, cloudflare::TurnstileState, email::EmailHandler};



#[derive(Clone)]
pub struct RegisterUserCredentialBasedState {
    pub email_handler: EmailHandler,
    pub turnstile_state: TurnstileState,
    pub jwt_keys: JWTKeys,
}
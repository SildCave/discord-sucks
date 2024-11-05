use crate::{cloudflare::TurnstileResult, registration::CredentialBasedRegistrationPayload};
use axum::http::StatusCode;
use tracing::info;

// takes user info and sends jwt through email
//#[axum::debug_handler]
pub async fn register_user(
    turnstile_result: TurnstileResult,
    //body: String,
    //registration_form: CredentialBasedRegistrationPayload
) -> (StatusCode, String) {
    todo!("axum 2 fromreq");
    //println!("form: {:?}", registration_form);
    // if turnstile_result == TurnstileResult::Allowed {
    //     return (StatusCode::OK, "User registered".to_string());
    // }
    info!("turnstile_result: {:?}", turnstile_result);
    if turnstile_result == TurnstileResult::Denied {
        return (StatusCode::FORBIDDEN, "Forbidden".to_string());
    }
    return (StatusCode::OK, "User registered".to_string());

}

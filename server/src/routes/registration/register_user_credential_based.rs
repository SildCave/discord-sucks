use std::sync::Arc;

use crate::{cloudflare::{TurnstileResult, TurnstileState}, email::{self, EmailHandler}, registration::CredentialBasedRegistrationPayload, state::RegisterUserCredentialBasedState};
use axum::{extract::State, http::StatusCode, Form};
use tracing::info;

// takes user info and sends jwt through email
pub async fn register_user(
    State(register_user_credential_based_state): State<Arc<RegisterUserCredentialBasedState>>,
    registration_form: Form<CredentialBasedRegistrationPayload>,
    //turnstile_result: TurnstileResult,
    //body: String,
    //registration_form: CredentialBasedRegistrationPayload
) -> (StatusCode, String) {

    let turnstile_result = register_user_credential_based_state.turnstile_state.verify_turnstile_from_request(
        &registration_form
    ).await.unwrap();

    //info!("turnstile_result: {:?}", turnstile_result);
    if turnstile_result == TurnstileResult::Denied {
        return (StatusCode::FORBIDDEN, "Forbidden".to_string());
    }

    let registration_form = registration_form.to_owned();
    // send email with jwt
    let email_handler = &register_user_credential_based_state.email_handler;
    let user_email = registration_form.email.clone();
    let jwt = registration_form.into_jwt_form(
        register_user_credential_based_state.email_handler.state.verification_email_state.email_verification_jwt_lifetime_s
    ).unwrap().into_jwt_token(
        &register_user_credential_based_state.jwt_keys
    ).unwrap();
    let email = email_handler.create_email_verification_email(
        user_email.parse().unwrap(),
        jwt
    ).unwrap();

    email_handler.send_email(email).await.unwrap();

    // let email = email_handler.send_email(
    //     user_email,
    //     jwt
    // ).await.unwrap();


    return (StatusCode::OK, "User registered".to_string());

}

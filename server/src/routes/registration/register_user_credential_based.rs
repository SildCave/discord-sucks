use std::sync::Arc;

use crate::{
    cloudflare::TurnstileResult,
    credentials::Password,
    registration::CredentialBasedRegistrationPayload,
    state::RegisterUserCredentialBasedState
};
use axum::{
    extract::State,
    http::StatusCode,
    response::{
        IntoResponse,
        Response
    },
    Form
};
use email_address::EmailAddress;
use tracing::{
    error,
    info
};

// takes user info and sends jwt through email, function with logging
pub async fn register_user(
    State(register_user_credential_based_state):
        State<Arc<RegisterUserCredentialBasedState>>,
    registration_form: Form<CredentialBasedRegistrationPayload>,
) -> Result<Response, Response> {
    let request_id = uuid::Uuid::new_v4();

    let turnstile_state = &register_user_credential_based_state.turnstile_state;
    let email_handler = &register_user_credential_based_state.email_handler;

    // chrome gamin 10000Gb ram usage
    let turnstile_result = turnstile_state.verify_turnstile_from_request(
        &registration_form
    ).await.map_err(
        |e| {
            error!("|{}| Error verifying turnstile: {:?}", request_id, e);
            e.into_response()
        }
    )?;

    
    info!("turnstile_result: {:?}", turnstile_result);
    if turnstile_result == TurnstileResult::Denied {
        return Ok(
            (StatusCode::FORBIDDEN, "Forbidden").into_response()
        );
    }

    let registration_form = registration_form.to_owned();
    let user_email = registration_form.email.clone();
    if EmailAddress::is_valid(&user_email) == false {
        return Ok(
            (StatusCode::BAD_REQUEST, "Invalid email").into_response()
        );
    }

    let password_requirements = register_user_credential_based_state.password_requirements.clone();
    let password = registration_form.password.clone();
    let password = Password::new(
        &password,
        &password_requirements
    );

    let valid = password.check_if_password_is_valid_based_on_requirements();
    if valid.is_err() {
        let password_error_code = valid.unwrap_err().into_internal_error_code();
        return Ok(
            (StatusCode::BAD_REQUEST, password_error_code).into_response()
        );
    };


    let jwt_encoded_registration_form = registration_form.into_jwt_form(
        email_handler.state.verification_email_state.email_verification_jwt_lifetime_s
        ).await.map_err(
            |e| {
                error!("Error creating jwt form: {:?}", e);
                e.into_response()
            }
        )?.into_jwt_token(
            &register_user_credential_based_state.jwt_keys
        ).map_err(
            |e| {
                error!("Error encoding jwt: {:?}", e);
                e.into_response()
            }
        )?;

    let email = email_handler.create_email_verification_email(
        user_email.parse().unwrap(),
        jwt_encoded_registration_form
    ).map_err(
        |e| {
            error!("Error creating email: {:?}", e);
            e.into_response()
        }
    )?;

    email_handler.send_email(email).await.unwrap();


    return Ok(
        (StatusCode::OK, "User registered").into_response()
    );

}

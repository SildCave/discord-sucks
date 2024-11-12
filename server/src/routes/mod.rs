mod hello_world;
mod secured;
mod authenticate;
mod refresh_token;
mod registration;

pub mod tests;

use std::sync::Arc;

use axum::{
    routing::{
        get,
        post
    }, Router
};

use hello_world::hello_world;
use secured::secured;
use authenticate::authenticate;

use crate::{
    auth::JWTKeys,
    cloudflare::TurnstileState,
    configuration::Config,
    credentials::PasswordRequirements,
    database::DatabaseClientWithCaching,
    email::EmailHandler,
    state::{
        AddUserFromJWTTokenState,
        ApiState,
        AuthenticationState,
        RefreshState,
        RegisterUserCredentialBasedState
    }
};

pub async fn configure_routes(
    jwt_keys: &JWTKeys,
    db_client: DatabaseClientWithCaching,
    password_requirements: PasswordRequirements,
    turnstile_state: &TurnstileState,
    email_handler: &EmailHandler,
    config: &Config
) -> Router {
    let authentication_state = AuthenticationState {
        jwt_keys: jwt_keys.clone(),
        jwt_config: config.jwt_config.clone(),
        db_client: db_client.clone(),
        password_requirements: password_requirements.clone(),
    };
    let refresh_state = RefreshState {
        jwt_keys: jwt_keys.clone(),
        jwt_config: config.jwt_config.clone(),
        db_client: db_client.clone(),
    };
    let register_user_credential_based_state = RegisterUserCredentialBasedState {
        email_handler: email_handler.clone(),
        turnstile_state: turnstile_state.clone(),
        jwt_keys: jwt_keys.clone(),
        password_requirements: password_requirements.clone(),
    };

    let add_user_from_jwt_token_state = AddUserFromJWTTokenState {
        db_client: db_client.clone(),
        jwt_keys: jwt_keys.clone(),
    };

    let api_state = ApiState {
        authentication: Arc::new(authentication_state),
        refresh: Arc::new(refresh_state),
        register_user_credential_based: Arc::new(register_user_credential_based_state),
        add_user_from_jwt: Arc::new(add_user_from_jwt_token_state),
    };

    Router::new()
        .route("/", get(hello_world))
        .route("/authenticate", post(authenticate))
            .with_state(api_state.clone())
        .route("/refresh_token", post(refresh_token::refresh_token))
            .with_state(api_state.clone())
        .route("/secured", get(secured))
            .with_state(jwt_keys.clone())
        .route("/register_user", post(registration::register_user))
            .with_state(api_state.clone())
        .route("/verify_email", get(registration::add_user_from_jwt_token))
            .with_state(api_state.clone())
}


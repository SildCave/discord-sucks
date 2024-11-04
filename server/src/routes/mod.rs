mod hello_world;
mod secured;
mod authenticate;
mod refresh_token;
mod registration;

mod tests;

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
    state::{
        ApiState,
        AuthenticationState,
        RefreshState
    }
};

pub async fn configure_routes(
    jwt_keys: &JWTKeys,
    db_client: DatabaseClientWithCaching,
    password_requirements: PasswordRequirements,
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

    let turnstile_state = TurnstileState::new(
        &config
    ).unwrap();

    let api_state = ApiState {
        authentication: Arc::new(authentication_state),
        refresh: Arc::new(refresh_state),
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
            .with_state(turnstile_state)
}

// curl -s \
//     -w '\n' \
//     -H 'Content-Type: application/json' \
//     -H 'Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJiQGIuY29tIiwiY29tcGFueSI6IkFDTUUiLCJleHAiOjEwMDAwMDAwMDAwfQ.M3LAZmrzUkXDC1q5mSzFAs_kJrwuKz3jOoDmjJ0G4gM' \
//     http://localhost:3000/protected
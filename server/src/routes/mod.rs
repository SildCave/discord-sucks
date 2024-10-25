mod hello_world;
mod secured;
mod authenticate;
mod refresh_token;

use std::sync::Arc;

use axum::{
    routing::{
        get,
        post
    },
    Router
};

use hello_world::hello_world;
use secured::secured;
use authenticate::authenticate;

use crate::{auth::JWTKeys, configuration::JWTConfig, state::{ApiState, AuthenticationState, RefreshState}};

pub async fn configure_routes(
    jwt_keys: &JWTKeys,
    jwt_config: &JWTConfig
) -> Router {
    let authentication_state = AuthenticationState {
        jwt_keys: jwt_keys.clone(),
        jwt_config: jwt_config.clone(),
    };
    let refresh_state = RefreshState {
        jwt_keys: jwt_keys.clone(),
        jwt_config: jwt_config.clone(),
    };


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
        //.route("/health", axum::handler::get(|| async { "OK" }))
}

// curl -s \
//     -w '\n' \
//     -H 'Content-Type: application/json' \
//     -H 'Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJiQGIuY29tIiwiY29tcGFueSI6IkFDTUUiLCJleHAiOjEwMDAwMDAwMDAwfQ.M3LAZmrzUkXDC1q5mSzFAs_kJrwuKz3jOoDmjJ0G4gM' \
//     http://localhost:3000/protected
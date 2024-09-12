mod hello_world;
mod authorize;
mod secured;
mod authenticate;

use axum::{
    routing::{
        get,
        post
    },
    Router
};

use hello_world::hello_world;
use authorize::authorize;
use secured::secured;

use crate::auth::JWTKeys;

pub async fn configure_routes(
    jwt_keys: JWTKeys
) -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/authorize", post(authorize)).with_state(jwt_keys.clone())
        .route("/secured", get(secured)).with_state(jwt_keys.clone())
        //.route("/health", axum::handler::get(|| async { "OK" }))
}

// curl -s \
//     -w '\n' \
//     -H 'Content-Type: application/json' \
//     -H 'Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJiQGIuY29tIiwiY29tcGFueSI6IkFDTUUiLCJleHAiOjEwMDAwMDAwMDAwfQ.M3LAZmrzUkXDC1q5mSzFAs_kJrwuKz3jOoDmjJ0G4gM' \
//     http://localhost:3000/protected
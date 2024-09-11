mod hello_world;

use axum::{routing::get, Router};

use hello_world::hello_world;

pub async fn configure_routes() -> Router {
    Router::new()
        .route("/", get(hello_world))
        //.route("/health", axum::handler::get(|| async { "OK" }))
}
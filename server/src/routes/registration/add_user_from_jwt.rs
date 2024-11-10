use axum::{extract::Path, response::IntoResponse, Form};

use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct AddUserFromJWTToken {
    token: String,
}

// work in progress

pub async fn add_user_from_jwt_token(
    Form(jwt_token): Form<AddUserFromJWTToken>,
) -> impl IntoResponse {
    format!("Hello, {:?}!", jwt_token)
}

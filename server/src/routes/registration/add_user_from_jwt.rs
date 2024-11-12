use std::sync::Arc;

use axum::{extract::{Path, State}, response::{IntoResponse, Response}, Form};

use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use tracing::error;

use crate::{registration::{CredentialBasedRegistrationPayload, UserRegistrationFormJWT}, state::AddUserFromJWTTokenState};


#[derive(Serialize, Deserialize, Debug)]
pub struct AddUserFromJWTToken {
    token: String,
}

// work in progress

pub async fn add_user_from_jwt_token(
    State(add_user_from_jwt_token_state): State<Arc<AddUserFromJWTTokenState>>,
    Form(jwt_token): Form<AddUserFromJWTToken>,
) -> Result<Response, Response> {
    let request_id = uuid::Uuid::new_v4();

    let db_client = &add_user_from_jwt_token_state.db_client;
    let jwt_keys = &add_user_from_jwt_token_state.jwt_keys;

    let registration_payload: UserRegistrationFormJWT = jwt_keys.verify_token_and_return_claims(
        &jwt_token.token,
    ).await.map_err(
        |e| {
            error!("|{}| Error verifying token: {:?}", request_id, e);
            e.into_response()
        }
    )?;

    let user_id_from_db = db_client.cached_get_user_id_by_email(&registration_payload.email).await
        .map_err(
            |e| {
                error!("|{}| Error getting user_id from db: {:?}", request_id, e);
                e.into_response()
            }
        )?;
    if user_id_from_db.is_some() {
        return Ok(
            (StatusCode::BAD_REQUEST, "User already exists").into_response()
        );
    }
    
    let user = registration_payload.into_user();
    todo!("id handling");
    db_client.cached_insert_user(&user).await
        .map_err(
            |e| {
                error!("|{}| Error inserting user into db: {:?}", request_id, e);
                e.into_response()
            }
        )?;

    Ok(format!(
        "User with email {} added to the database",
        registration_payload.email
    ).into_response())
}

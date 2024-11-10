
use axum::response::IntoResponse;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use time::OffsetDateTime;
use time::serde::rfc3339;
use axum::Form;
use crate::cloudflare::{GetTurnstileCode, TurnstileRequest};
use crate::credentials::{Password, PasswordRequirements, SaltMode};
use crate::registration::UserRegistrationFormJWT;
// TODO - Implement OTP 2fa and add date of birth field to the db
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CredentialBasedRegistrationPayload {
    pub email: String,
    pub password: String,
    pub date_of_birth: String,
    #[serde(rename = "cf-turnstile-response")]
    cf_turnstile_response: String,
}

impl GetTurnstileCode for Form<CredentialBasedRegistrationPayload> {
    fn get_turnstile_code(&self) -> String {
        self.cf_turnstile_response.clone()
    }
}

#[derive(Debug, Clone, Error)]
pub enum CredentialBasedRegistrationPayloadError {
    #[error("Invalid body, date of birth must be in the format YYYY-MM-DD")]
    InvalidBody,
}

impl IntoResponse for CredentialBasedRegistrationPayloadError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            CredentialBasedRegistrationPayloadError::InvalidBody => {
                (axum::http::StatusCode::BAD_REQUEST, "Invalid body, date of birth must be in the format YYYY-MM-DD")
            },
        };
        let body = axum::Json(serde_json::json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
    
}

impl CredentialBasedRegistrationPayload {
    pub async fn into_jwt_form(
        &self,
        email_jwt_lifetime: i64
    ) -> Result<UserRegistrationFormJWT, CredentialBasedRegistrationPayloadError> {
        let password = Password::new(
            &self.password,
            &PasswordRequirements::no_requirements()
        ).hash_and_salt_password(&SaltMode::Generate).await.unwrap();
        let date_of_birth: NaiveDate = self.date_of_birth.parse().map_err(
            |_| CredentialBasedRegistrationPayloadError::InvalidBody
        )?;
        Ok(UserRegistrationFormJWT::new(
            self.email.clone(),
            password.password_hash,
            password.salt,
            date_of_birth,
            chrono::Utc::now().timestamp() + email_jwt_lifetime
        ))
    }
}
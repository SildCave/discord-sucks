
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use time::serde::rfc3339;

use crate::cloudflare::{GetTurnstileCode, TurnstileRequest};
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

impl GetTurnstileCode for axum::Form<CredentialBasedRegistrationPayload> {
    fn get_turnstile_code(&self) -> String {
        self.cf_turnstile_response.clone()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CredentialBasedRegistrationPayloadError {
    InvalidBody,
    InternalError(&'static str),
}

impl CredentialBasedRegistrationPayload {
    pub fn into_jwt_form(
        &self,
        email_jwt_lifetime: i64
    ) -> Result<UserRegistrationFormJWT, CredentialBasedRegistrationPayloadError> {
        let date_of_birth: NaiveDate = self.date_of_birth.parse().map_err(|_| CredentialBasedRegistrationPayloadError::InvalidBody)?;
        Ok(UserRegistrationFormJWT::new(
            self.email.clone(),
            self.password.clone(),
            date_of_birth,
            chrono::Utc::now().timestamp() + email_jwt_lifetime
        ))
    }
    
}
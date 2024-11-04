use chrono::{
    serde::ts_seconds_option,
    DateTime,
    Utc
};
use serde::{Deserialize, Serialize};

// TODO - Implement OTP 2fa and add date of birth field to the db
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CredentialBasedRegistrationPayload {
    pub email: String,
    pub password: String,
    #[serde(with = "ts_seconds_option")]
    pub date_of_birth: Option<DateTime<Utc>>,
}
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::auth::{JWTKeys, VerificationError};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct UserRegistrationFormJWT {
    pub email: String,
    pub password: String,
    pub date_of_birth: NaiveDate,

    pub exp: i64,
}

impl UserRegistrationFormJWT {
    pub fn new(
        email: String,
        password: String,
        date_of_birth: NaiveDate,
        lifetime_s: i64,
    ) -> Self {
        Self {
            email,
            password,
            date_of_birth,
            exp: chrono::Utc::now().timestamp() + lifetime_s,
        }
    }

    pub fn from_jwt_token(
        token: &str,
        keys: &JWTKeys,
    ) -> Result<Self, VerificationError> {
        let data = jsonwebtoken::decode::<Self>(
            token,
            &keys.decoding,
            &jsonwebtoken::Validation::default(),
        )?;
        if data.claims.exp < chrono::Utc::now().timestamp() {
            return Err(VerificationError::ExpiredToken);
        }
        Ok(data.claims)
    }

}
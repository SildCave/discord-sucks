use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::{app_objects::User, auth::{JWTKeys, VerificationError}};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct UserRegistrationFormJWT {
    pub email: String,
    pub username: String,
    pub password_hash: String,
    pub password_salt: String,
    pub date_of_birth: NaiveDate,

    pub exp: i64,
}


impl UserRegistrationFormJWT {
    pub fn new(
        email: String,
        username: String,
        password_hash: String,
        password_salt: String,
        date_of_birth: NaiveDate,
        lifetime_s: i64,
    ) -> Self {
        Self {
            email,
            username,
            password_hash,
            password_salt,
            date_of_birth,
            exp: chrono::Utc::now().timestamp() + lifetime_s,
        }
    }

    pub fn into_user(
        &self,
    ) -> User {
        User {
            email: self.email.clone(),
            username: self.username.clone(),
            password_hash: self.password_hash.clone(),
            salt: self.password_salt.clone(),
            date_of_birth: self.date_of_birth,
            verified: false,
            banned: false,
            created_at: chrono::Utc::now().timestamp(),
            valid_refresh_token: None,
            id: {
                0
            },
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
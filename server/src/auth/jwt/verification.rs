use axum::{response::{IntoResponse, Response}, Json};
use axum::http::StatusCode;
use jsonwebtoken::{
  decode,
  Algorithm,
  Validation
};
use serde_json::json;

use crate::auth::AuthError;

use super::{ClaimType, AuthClaims, JWTKeys};


use thiserror::Error;

#[derive(Debug, Error)]
pub enum VerificationError {
    #[error("Invalid token")]
    InvalidToken,

    // The error in question
    #[error("Expired token")]
    ExpiredToken,

    // Generic error
    #[error(transparent)]
    JWTError {
        source: jsonwebtoken::errors::Error,
    }
}

impl IntoResponse for VerificationError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            VerificationError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid token"),
            VerificationError::ExpiredToken => (StatusCode::UNAUTHORIZED, "Expired token"),
            VerificationError::JWTError { source: _ } => (StatusCode::UNAUTHORIZED, "1402"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

impl VerificationError {
    pub fn into_internal_error_code(&self) -> &'static str {
        match self {
            VerificationError::InvalidToken => "1400",
            VerificationError::ExpiredToken => "1401",
            VerificationError::JWTError { source: _ } => "1402",
        }
    }

    // pub fn to_auth_error(&self) -> AuthError {
    //     match self {
    //         VerificationError::InvalidToken => AuthError::InvalidToken,
    //         VerificationError::ExpiredToken => AuthError::ExpiredToken,
    //         VerificationError::JWTError { source: _ } => AuthError::InvalidToken,
    //     }
    // }
}

impl From<VerificationError> for AuthError {
    fn from(err: VerificationError) -> Self {
        match err {
            VerificationError::InvalidToken => Self::InvalidToken,
            VerificationError::ExpiredToken => Self::ExpiredToken,
            VerificationError::JWTError { source: _ } => Self::InvalidToken,
        }
    }

}

impl From<jsonwebtoken::errors::Error> for VerificationError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        match err.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => Self::ExpiredToken,
            jsonwebtoken::errors::ErrorKind::InvalidToken => Self::InvalidToken,
            _ => Self::JWTError { source: err },
        }
    }
}

impl JWTKeys {
    /// Verifies the signature of the token and returns the claims if the token is valid.
    pub async fn verify_token_and_return_claims<T>(
        &self,
        token: &str,
    ) -> Result<T, VerificationError>
    where T: serde::de::DeserializeOwned,
    {
        let token_data = decode::<T>(
            token,
            &self.decoding,
            &Validation::new(Algorithm::HS256),
        )?;
        Ok(token_data.claims)
    }

}


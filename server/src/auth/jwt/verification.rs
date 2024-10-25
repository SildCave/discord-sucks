use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};

use crate::auth::AuthError;

use super::{ClaimType, Claims, JWTKeys};


use thiserror::Error;

#[derive(Debug, Error)]
pub enum VerificationError {
    #[error("Invalid token")]
    InvalidToken,

    // The error in question
    #[error("Expired token")]
    ExpiredToken,
    #[error("No token")]
    NoToken,

    // Generic error
    #[error(transparent)]
    JWTError {
        source: jsonwebtoken::errors::Error,
    }
}

impl From<VerificationError> for AuthError {
    fn from(err: VerificationError) -> Self {
        match err {
            VerificationError::InvalidToken => Self::InvalidToken,
            VerificationError::ExpiredToken => Self::ExpiredToken,
            VerificationError::NoToken => Self::NoToken,
            VerificationError::JWTError { source: _ } => Self::InvalidToken,
        }
    }

}

impl From<jsonwebtoken::errors::Error> for VerificationError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        match err.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => Self::ExpiredToken,
                _ => Self::JWTError { source: err },
            }
    }
}

/// Verifies the signature of the token and returns the claims if the token is valid.
pub async fn verify_token(
    token: &str,
    keys: &JWTKeys,
    expected_type: Option<ClaimType>
) -> Result<Claims, VerificationError> {
    let token_data = decode::<Claims>(
        token,
        &keys.decoding,
        &Validation::new(Algorithm::HS256),
    )?;
    println!("Token data: {:?}", token_data);
    if expected_type.is_none() {
        return Ok(token_data.claims);
    }
    if expected_type.unwrap() != token_data.claims.claim_type {
        return Err(VerificationError::InvalidToken);
    }

    Ok(token_data.claims)
}

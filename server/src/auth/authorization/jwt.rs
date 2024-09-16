
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::request::Parts,
    RequestPartsExt,
};

use jsonwebtoken::{
    decode, DecodingKey, EncodingKey, Validation
};
use serde::{Deserialize, Serialize};

use anyhow::Result;


use crate::configuration::Config;

#[derive(Clone)]
pub struct JWTKeys {
    pub encoding: EncodingKey,
    decoding: DecodingKey,
}

impl JWTKeys {
    pub fn new(config: &Config) -> Result<Self> {
        let secret_path = &config.server.jwt_secret_path;
        let secret = std::fs::read_to_string(secret_path)?;

        Ok(
            Self {
                encoding: EncodingKey::from_secret(secret.as_bytes()),
                decoding: DecodingKey::from_secret(secret.as_bytes()),
            }
        )
    }
}




use super::AuthError;
#[async_trait]
impl FromRequestParts<JWTKeys> for Claims
{
    type Rejection = AuthError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &JWTKeys
    ) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;
        let decoding_key = &state.decoding;

        let token_data = decode::<Claims>(bearer.token(), decoding_key, &Validation::default())
            .map_err(|_| AuthError::InvalidToken)?;

        if token_data.claims.exp < chrono::Utc::now().timestamp() as u64 {
            return Err(AuthError::ExpiredToken);
        }
        Ok(token_data.claims)
    }
}



#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub iat: u64,
    pub exp: u64,
}


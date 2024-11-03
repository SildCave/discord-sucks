use std::fmt::Debug;

use jsonwebtoken::{DecodingKey, EncodingKey};
use anyhow::Result;


use crate::configuration::Config;

#[derive(Clone)]
pub struct JWTKeys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Debug for JWTKeys {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JWTKeys")
            .field("encoding", &"EncodingKey")
            .field("decoding", &"DecodingKey")
            .finish()
    }
}

impl JWTKeys {
    pub fn new(config: &Config) -> Result<Self> {
        let secret_path = &config.jwt_config.jwt_secret_path;
        let secret = std::fs::read_to_string(secret_path)?;

        Ok(
            Self {
                encoding: EncodingKey::from_secret(secret.as_bytes()),
                decoding: DecodingKey::from_secret(secret.as_bytes()),
            }
        )
    }

}

use crate::{auth::JWTKeys, configuration::JWTConfig};

#[derive(Clone)]
pub struct AuthenticationState {
    pub jwt_keys: JWTKeys,
    pub jwt_config: JWTConfig,
}
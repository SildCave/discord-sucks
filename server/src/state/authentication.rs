use crate::{auth::JWTKeys, configuration::JWTConfig, credentials::PasswordRequirements, database::DatabaseClientWithCaching};

#[derive(Clone, Debug)]
pub struct AuthenticationState {
    pub jwt_keys: JWTKeys,
    pub jwt_config: JWTConfig,
    pub db_client: DatabaseClientWithCaching,
    pub password_requirements: PasswordRequirements,
}
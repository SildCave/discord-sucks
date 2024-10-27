use crate::{
    auth::JWTKeys,
    configuration::JWTConfig,
    database::DatabaseClientWithCaching
};

#[derive(Clone)]
pub struct RefreshState {
    pub jwt_keys: JWTKeys,
    pub jwt_config: JWTConfig,
    pub db_client: DatabaseClientWithCaching,
}
use crate::{auth::JWTKeys, database::DatabaseClientWithCaching};


#[derive(Clone, Debug)]
pub struct AddUserFromJWTTokenState {
    pub db_client: DatabaseClientWithCaching,
    pub jwt_keys: JWTKeys,
}
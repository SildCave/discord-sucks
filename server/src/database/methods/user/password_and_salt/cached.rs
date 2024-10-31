use std::sync::Arc;

use crate::database::{
    methods::DatabaseError,
    DatabaseClientWithCaching
};



impl DatabaseClientWithCaching {
    pub async fn cached_get_password_hash_and_salt_by_user_id(
        &self,
        user_id: i64
    ) -> Result<(String, String), DatabaseError> {
        let db_client = Arc::new(self.clone());

        // Check Redis first
        let password_hash = db_client.redis_get_password_hash_by_user_id(user_id).await?;
        let salt = db_client.redis_get_salt_by_user_id(user_id).await?;

        // If Redis has the password hash and salt, return them
        if !password_hash.is_none() && !salt.is_none() {
            return Ok((password_hash.unwrap(), salt.unwrap()));
        }

        // If Redis doesn't have the password hash and salt, check Postgres
        let (password_hash, salt) = db_client.postgres_get_password_hash_and_salt_by_user_id(user_id).await?;


        // If Postgres has the password hash and salt, set them in Redis
        db_client.redis_set_user_salt(user_id, &salt).await?;
        db_client.redis_set_user_password_hash(user_id, &password_hash).await?;

        return Ok((password_hash, salt));
    }

}
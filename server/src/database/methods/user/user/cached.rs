use std::sync::Arc;

use crate::{app_objects::User, database::{
    methods::DatabaseError,
    DatabaseClientWithCaching
}};



impl DatabaseClientWithCaching {

    pub async fn cached_insert_user(
        &self,
        user: &User
    ) -> Result<(), DatabaseError> {
        let db_client = Arc::new(self.clone());
        db_client.postgres_insert_user(user).await?;

       
        db_client.redis_set_email_by_user_id(
            &user.email,
            user.id
        ).await?;
        
        Ok(())
    }

    pub async fn cached_get_user_id_by_email(
        &self,
        email: &str
    ) -> Result<Option<i64>, DatabaseError> {
        let db_client = Arc::new(self.clone());

        // Check Redis first
        let user_id = db_client.redis_get_user_id_by_email(email).await?;

        // If Redis has the user_id, return it
        if !user_id.is_none() {
            return Ok(user_id);
        }

        // If Redis doesn't have the user_id, check Postgres
        let user_id = db_client.postgres_get_user_id_by_email(email).await?;

        // If Postgres doesn't have the user_id, return None
        if user_id.is_none() {
            return Ok(None);
        }

        // If Postgres has the user_id, set it in Redis
        let user_id = user_id.unwrap();
        db_client.redis_set_email_by_user_id(email, user_id).await?;

        return Ok(Some(user_id));
    }

}
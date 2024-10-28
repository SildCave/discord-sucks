use std::sync::Arc;

use crate::database::{
    methods::DatabaseError,
    DatabaseClientWithCaching
};



impl DatabaseClientWithCaching {
    pub async fn update_user_refresh_token_with_caching(
        &self,
        user_id: i64,
        refresh_token: &str
    ) -> Result<(), DatabaseError> {
        let db_client = Arc::new(self.clone());

        // Update the token in Postgres job
        let postgres_job = tokio::spawn({
            let user_id = user_id;
            let refresh_token = refresh_token.to_string();
            let db_client = db_client.clone();
            async move {
                db_client.set_user_refresh_token_in_postgres(user_id, &refresh_token).await
            }
        });

        // Update the token in Redis job
        let redis_job = tokio::spawn({
            let user_id = user_id;
            let refresh_token = refresh_token.to_string();
            let db_client = db_client.clone();
            async move {
                db_client.set_user_refresh_token_in_redis(user_id, &refresh_token).await
            }
        });

        let (postgres_job_res, redis_job_res) = tokio::join!(
            postgres_job,
            redis_job
        );

        match postgres_job_res {
            Ok(_) => {},
            Err(e) => {
                // If there was an error Joining Postgres job, delete the token from Redis
                self.delete_user_refresh_token_in_redis(user_id).await?;
                return Err(DatabaseError::TokioError(e));
            }
        }
        match redis_job_res {
            Ok(_) => {},
            Err(e) => {
                // If there was an error Joining Redis job, delete the token from Postgres
                self.delete_user_refresh_token_in_postgres(user_id).await?;
                return Err(DatabaseError::TokioError(e));
            }
        }

        // Return the error if there was any
        let postgres_job_res = postgres_job_res?;
        let redis_job_res = redis_job_res?;

        match postgres_job_res {
            Ok(_) => {},
            Err(e) => {
                // If there was an error Joining Postgres job, delete the token from Redis
                self.delete_user_refresh_token_in_redis(user_id).await?;
                return Err(e);
            }
        }

        match redis_job_res {
            Ok(_) => {},
            Err(e) => {
                // If there was an error Joining Redis job, delete the token from Postgres
                self.delete_user_refresh_token_in_postgres(user_id).await?;
                return Err(e);
            }
        }

        redis_job_res?;
        postgres_job_res?;

        Ok(())
    }

    pub async fn delete_user_refresh_token_with_caching(
        &self,
        user_id: i64
    ) -> Result<(), DatabaseError> {
        let db_client = Arc::new(self.clone());

        // Delete the token from Postgres job
        let postgres_job = tokio::spawn({
            let user_id = user_id;
            let db_client = db_client.clone();
            async move {
                db_client.delete_user_refresh_token_in_postgres(user_id).await
            }
        });

        // Delete the token from Redis job
        let redis_job = tokio::spawn({
            let user_id = user_id;
            let db_client = db_client.clone();
            async move {
                db_client.delete_user_refresh_token_in_redis(user_id).await
            }
        });

        // Wait for both jobs to finish
        let (postgres_job_res, redis_job_res) = tokio::join!(
            postgres_job,
            redis_job
        );

        // Check if there was an error in tokio::join!
        let postgres_job_res = postgres_job_res?;
        let redis_job_res = redis_job_res?;

        // Check if there was an error in the Postgres and Redis job
        // We don't need to do any fancy cache invalidation here because we're deleting the token
        postgres_job_res?;
        redis_job_res?;

        Ok(())
    }

    pub async fn get_user_id_by_email_with_cache(
        &self,
        email: &str
    ) -> Result<Option<i64>, DatabaseError> {
        let db_client = Arc::new(self.clone());

        // Check Redis first
        let user_id = db_client.get_user_id_by_email_from_redis(email).await?;

        // If Redis has the user_id, return it
        if !user_id.is_none() {
            return Ok(user_id);
        }

        // If Redis doesn't have the user_id, check Postgres
        let user_id = db_client.get_user_id_by_email_from_postgres(email).await?;

        // If Postgres doesn't have the user_id, return None
        if user_id.is_none() {
            return Ok(None);
        }

        // If Postgres has the user_id, set it in Redis
        let user_id = user_id.unwrap();
        db_client.set_email_id_in_redis(email, user_id).await?;

        return Ok(Some(user_id));
    }

    pub async fn get_password_hash_and_salt_by_user_id_with_caching(
        &self,
        user_id: i64
    ) -> Result<(String, String), DatabaseError> {
        let db_client = Arc::new(self.clone());

        // Check Redis first
        let password_hash = db_client.get_password_hash_by_user_id_from_redis(user_id).await?;
        let salt = db_client.get_salt_by_user_id_from_redis(user_id).await?;

        // If Redis has the password hash and salt, return them
        if !password_hash.is_none() && !salt.is_none() {
            return Ok((password_hash.unwrap(), salt.unwrap()));
        }

        // If Redis doesn't have the password hash and salt, check Postgres
        let (password_hash, salt) = db_client.get_password_hash_and_salt_from_postgres_by_user_id(user_id).await?;


        // If Postgres has the password hash and salt, set them in Redis
        db_client.set_salt_in_redis(user_id, &salt).await?;
        db_client.set_password_hash_in_redis(user_id, &password_hash).await?;

        return Ok((password_hash, salt));
    }
    pub async fn get_user_refresh_token_with_caching(
        &self,
        user_id: i64
    ) -> Result<Option<String>, DatabaseError> {
        let db_client = Arc::new(self.clone());

        // Check Redis first
        let refresh_token = db_client.get_user_refresh_token_from_redis_by_user_id(user_id).await?;

        // If Redis has the token, return it
        if !refresh_token.is_none() {
            return Ok(refresh_token);
        }

        // If Redis doesn't have the token, check Postgres
        let refresh_token = db_client.get_user_refresh_token_from_postgres_by_user_id(user_id).await?;

        // If Postgres doesn't have the token, return None
        if refresh_token.is_none() {
            return Ok(None);
        }

        // If Postgres has the token, set it in Redis
        let refresh_token = refresh_token.unwrap();
        db_client.set_user_refresh_token_in_redis(user_id, &refresh_token).await?;

        return Ok(Some(refresh_token));

    }
}
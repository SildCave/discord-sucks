use std::sync::Arc;

use crate::database::{
    methods::DatabaseError,
    DatabaseClientWithCaching
};



impl DatabaseClientWithCaching {
    pub async fn cached_update_user_refresh_token(
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
                db_client.postgres_set_user_refresh_token(user_id, &refresh_token).await
            }
        });

        // Update the token in Redis job
        let redis_job = tokio::spawn({
            let user_id = user_id;
            let refresh_token = refresh_token.to_string();
            let db_client = db_client.clone();
            async move {
                db_client.redis_set_user_refresh_token(user_id, &refresh_token).await
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
                self.redis_delete_user_refresh_token(user_id).await?;
                return Err(DatabaseError::TokioError(e));
            }
        }
        match redis_job_res {
            Ok(_) => {},
            Err(e) => {
                // If there was an error Joining Redis job, delete the token from Postgres
                self.postgres_delete_user_refresh_token(user_id).await?;
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
                self.redis_delete_user_refresh_token(user_id).await?;
                return Err(e);
            }
        }

        match redis_job_res {
            Ok(_) => {},
            Err(e) => {
                // If there was an error Joining Redis job, delete the token from Postgres
                self.postgres_delete_user_refresh_token(user_id).await?;
                return Err(e);
            }
        }

        redis_job_res?;
        postgres_job_res?;

        Ok(())
    }

    pub async fn cached_delete_user_refresh_token(
        &self,
        user_id: i64
    ) -> Result<(), DatabaseError> {
        let db_client = Arc::new(self.clone());

        // Delete the token from Postgres job
        let postgres_job = tokio::spawn({
            let user_id = user_id;
            let db_client = db_client.clone();
            async move {
                db_client.postgres_delete_user_refresh_token(user_id).await
            }
        });

        // Delete the token from Redis job
        let redis_job = tokio::spawn({
            let user_id = user_id;
            let db_client = db_client.clone();
            async move {
                db_client.redis_delete_user_refresh_token(user_id).await
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

    pub async fn cached_get_user_refresh_token(
        &self,
        user_id: i64
    ) -> Result<Option<String>, DatabaseError> {
        let db_client = Arc::new(self.clone());

        // Check Redis first
        let refresh_token = db_client.redis_get_user_refresh_token_by_user_id(user_id).await?;

        // If Redis has the token, return it
        if !refresh_token.is_none() {
            return Ok(refresh_token);
        }

        // If Redis doesn't have the token, check Postgres
        let refresh_token = db_client.postgres_get_user_refresh_token_by_user_id(user_id).await?;

        // If Postgres doesn't have the token, return None
        if refresh_token.is_none() {
            return Ok(None);
        }

        // If Postgres has the token, set it in Redis
        let refresh_token = refresh_token.unwrap();
        db_client.redis_set_user_refresh_token(user_id, &refresh_token).await?;

        return Ok(Some(refresh_token));

    }

}
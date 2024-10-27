use std::sync::Arc;

use redis::FromRedisValue;

use super::{super::client::DatabaseClientWithCaching, DatabaseError};



#[derive(Debug, PartialEq)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password: String,
    pub email: Option<String>,
    pub created_at: i64,
    pub valid_refresh_token: Option<String>,
    pub verified: bool
}


impl DatabaseClientWithCaching {
    pub async fn get_user_from_postgres_by_id(
        &self,
        user_id: i64
    ) -> Result<Option<User>, DatabaseError> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT * FROM users
            WHERE id = $1
            "#,
            user_id
        )
        .fetch_one(&self.postgres_con)
        .await;
        if user.is_err() {
            match user.err().unwrap() {
                sqlx::Error::RowNotFound => {
                    return Ok(None);
                },
                e => {
                    return Err(DatabaseError::SQLXError(e));
                }
            }
        }
        let user = user.unwrap();
        Ok(Some(user))
    }

    pub async fn insert_user_to_postgres(
        &self,
        user: &User
    ) -> Result<(), DatabaseError> {
        let _ = sqlx::query!(
            r#"
            INSERT INTO users (id, username, password, email, created_at, valid_refresh_token, verified)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
            user.id,
            user.username,
            user.password,
            user.email,
            user.created_at,
            user.valid_refresh_token,
            user.verified
        )
        .execute(&self.postgres_con)
        .await?;
        Ok(())
    }

    async fn set_user_refresh_token_in_postgres(
        &self,
        user_id: i64,
        refresh_token: &str
    ) -> Result<(), DatabaseError> {
        let _ = sqlx::query!(
            r#"
            UPDATE users
            SET valid_refresh_token = $1
            WHERE id = $2
            "#,
            refresh_token,
            user_id
        )
        .execute(&self.postgres_con)
        .await?;
        Ok(())
    }

    async fn delete_user_refresh_token_in_postgres(
        &self,
        user_id: i64
    ) -> Result<(), DatabaseError> {
        let _ = sqlx::query!(
            r#"
            UPDATE users
            SET valid_refresh_token = NULL
            WHERE id = $1
            "#,
            user_id
        )
        .execute(&self.postgres_con)
        .await?;
        Ok(())
    }


    async fn get_user_refresh_token_from_postgres_by_user_id(
        &self,
        user_id: i64
    ) -> Result<Option<String>, DatabaseError> {
        let refresh_token = sqlx::query!(
            r#"
            SELECT valid_refresh_token FROM users
            WHERE id = $1
            "#,
            user_id
        )
            .fetch_one(&self.postgres_con)
            .await;
        if refresh_token.is_err() {
            match refresh_token.err().unwrap() {
                sqlx::Error::RowNotFound => {
                    return Err(DatabaseError::UserNotFound(user_id));
                },
                e => {
                    return Err(DatabaseError::SQLXError(e));
                }
            }
        }
        let refresh_token = refresh_token.unwrap();

        let valid_refresh_token = refresh_token.valid_refresh_token;
        Ok(valid_refresh_token)
    }

    async fn get_user_refresh_token_from_redis_by_user_id(
        &self,
        user_id: i64
    ) -> Result<Option<String>, DatabaseError> {
        let mut con = self.redis_con.clone();
        let refresh_token: Result<String, redis::RedisError> = redis::cmd("GET")
            .arg(
                format!("user:{}:refresh_token", user_id)
            )
            .query_async(&mut con)
            .await;
        match refresh_token {
            Ok(token) => {
                return Ok(Some(token));
            },
            Err(e) => {
                if e.kind() != redis::ErrorKind::TypeError {
                    return Err(DatabaseError::RedisError(e));
                } else {
                    return Ok(None);
                }
            }
        }
    }

    async fn set_user_refresh_token_in_redis(
        &self,
        user_id: i64,
        refresh_token: &str
    ) -> Result<(), DatabaseError> {
        let mut con = self.redis_con.clone();
        let _: () = redis::cmd("SET")
            .arg(
                format!("user:{}:refresh_token", user_id)
            )
            .arg(refresh_token)
            .query_async(&mut con)
            .await?;
        Ok(())
    }

    async fn delete_user_refresh_token_in_redis(
        &self,
        user_id: i64
    ) -> Result<(), DatabaseError> {
        let mut con = self.redis_con.clone();
        let _: () = redis::cmd("DEL")
            .arg(
                format!("user:{}:refresh_token", user_id)
            )
            .query_async(&mut con)
            .await?;
        Ok(())
    }

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


#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::configuration::Config;
    use crate::database::DatabaseClientWithCaching;
    #[tokio::test]
    async fn test_get_refresh_token_by_user_id() -> Result<(), DatabaseError> {
        let mut cfg_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        cfg_path.push("../configuration/server/config.toml");
        let config = Config::from_file(cfg_path).unwrap();
        let db_client = DatabaseClientWithCaching::new(
            &config.redis_database,
            &config.postgres_database
        ).await.unwrap();
        db_client.delete_user_refresh_token_in_redis(420).await.unwrap();
        db_client.set_user_refresh_token_in_redis(420, "test_token").await.unwrap();
        let token = db_client.get_user_refresh_token_from_redis_by_user_id(420).await;
        db_client.delete_user_refresh_token_in_redis(420).await.unwrap();
        assert_eq!(token.unwrap(), Some("test_token".to_string()));
        Ok(())
    }

    #[tokio::test]
    async fn test_delete_token_by_user_id() -> Result<(), DatabaseError> {
        let mut cfg_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        cfg_path.push("../configuration/server/config.toml");
        let config = Config::from_file(cfg_path).unwrap();
        let db_client = DatabaseClientWithCaching::new(
            &config.redis_database,
            &config.postgres_database
        ).await.unwrap();
        db_client.set_user_refresh_token_in_redis(420, "test_token").await.unwrap();
        db_client.delete_user_refresh_token_in_redis(420).await.unwrap();
        let token = db_client.get_user_refresh_token_from_redis_by_user_id(420).await;
        assert_eq!(token.unwrap(), None);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_user_from_postgres_by_id() -> Result<(), DatabaseError> {
        let mut cfg_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        cfg_path.push("../configuration/server/config.toml");
        let config = Config::from_file(cfg_path).unwrap();
        let db_client = DatabaseClientWithCaching::new(
            &config.redis_database,
            &config.postgres_database
        ).await.unwrap();
        let user = db_client.get_user_from_postgres_by_id(420).await.unwrap();
        assert_eq!(user, None);

        Ok(())
    }

}
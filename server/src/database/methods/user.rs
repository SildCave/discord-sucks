use std::sync::Arc;

use redis::FromRedisValue;

use crate::app_objects::User;

use super::{super::client::DatabaseClientWithCaching, DatabaseError};




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

    pub async fn get_password_hash_and_salt_from_postgres_by_user_id(
        &self,
        user_id: i64
    ) -> Result<(String, String), DatabaseError> {
        let user = sqlx::query!(
            r#"
            SELECT password_hash, salt FROM users
            WHERE id = $1
            "#,
            user_id
        )
        .fetch_one(&self.postgres_con)
        .await;
        if user.is_err() {
            match user.err().unwrap() {
                sqlx::Error::RowNotFound => {
                    return Err(DatabaseError::UserNotFound(user_id));
                },
                e => {
                    return Err(DatabaseError::SQLXError(e));
                }
            }
        }
        let user = user.unwrap();
        let password_hash = user.password_hash;
        let salt = user.salt;
        Ok((password_hash, salt))
    }

    pub async fn get_user_id_by_email_from_postgres(
        &self,
        email: &str
    ) -> Result<Option<i64>, DatabaseError> {
        let user_id = sqlx::query!(
            r#"
            SELECT id FROM users
            WHERE email = $1
            "#,
            email
        )
        .fetch_one(&self.postgres_con)
        .await;
        if user_id.is_err() {
            match user_id.err().unwrap() {
                sqlx::Error::RowNotFound => {
                    return Ok(None);
                },
                e => {
                    return Err(DatabaseError::SQLXError(e));
                }
            }
        }
        let user_id = user_id.unwrap();
        let user_id = user_id.id;
        Ok(Some(user_id))
    }

    pub async fn insert_user_to_postgres(
        &self,
        user: &User
    ) -> Result<(), DatabaseError> {
        let res = sqlx::query!(
            r#"
            INSERT INTO users (id, username, password_hash, salt, email, created_at, valid_refresh_token, verified)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            user.id,
            user.username,
            user.password_hash,
            user.salt,
            user.email,
            user.created_at,
            user.valid_refresh_token,
            user.verified
        )
        .execute(&self.postgres_con)
        .await?;
        if res.rows_affected() == 0 {
            return Err(DatabaseError::UserAlreadyExists(user.id));
        }
        Ok(())
    }

    async fn delete_user_from_postgres_by_id(
        &self,
        user_id: i64
    ) -> Result<(), DatabaseError> {
        let res = sqlx::query!(
            r#"
            DELETE FROM users
            WHERE id = $1
            "#,
            user_id
        )
        .execute(&self.postgres_con)
        .await?;
        if res.rows_affected() == 0 {
            return Err(DatabaseError::UserNotFound(user_id));
        }
        Ok(())
    }

    async fn set_user_refresh_token_in_postgres(
        &self,
        user_id: i64,
        refresh_token: &str
    ) -> Result<(), DatabaseError> {
        let res = sqlx::query!(
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
        if res.rows_affected() == 0 {
            return Err(DatabaseError::UserNotFound(user_id));
        }
        Ok(())
    }

    async fn delete_user_refresh_token_in_postgres(
        &self,
        user_id: i64
    ) -> Result<(), DatabaseError> {
        let res = sqlx::query!(
            r#"
            UPDATE users
            SET valid_refresh_token = NULL
            WHERE id = $1
            "#,
            user_id
        )
        .execute(&self.postgres_con)
        .await?;
        if res.rows_affected() == 0 {
            return Err(DatabaseError::UserNotFound(user_id));
        }
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

    async fn get_password_hash_by_user_id_from_redis(
        &self,
        user_id: i64
    ) -> Result<Option<String>, DatabaseError> {
        let mut con = self.redis_con.clone();
        let password_hash: Result<String, redis::RedisError> = redis::cmd("GET")
            .arg(
                format!("user:{}:password_hash", user_id)
            )
            .query_async(&mut con)
            .await;
        match password_hash {
            Ok(hash) => {
                return Ok(Some(hash));
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

    async fn get_salt_by_user_id_from_redis(
        &self,
        user_id: i64
    ) -> Result<Option<String>, DatabaseError> {
        let mut con = self.redis_con.clone();
        let salt: Result<String, redis::RedisError> = redis::cmd("GET")
            .arg(
                format!("user:{}:salt", user_id)
            )
            .query_async(&mut con)
            .await;
        match salt {
            Ok(salt) => {
                return Ok(Some(salt));
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

    async fn set_password_hash_in_redis(
        &self,
        user_id: i64,
        password_hash: &str
    ) -> Result<(), DatabaseError> {
        let mut con = self.redis_con.clone();
        let _: () = redis::cmd("SET")
            .arg(
                format!("user:{}:password_hash", user_id)
            )
            .arg(password_hash)
            .query_async(&mut con)
            .await?;
        Ok(())
    }

    async fn get_user_id_by_email_from_redis(
        &self,
        email: &str
    ) -> Result<Option<i64>, DatabaseError> {
        let mut con = self.redis_con.clone();
        let user_id: Result<i64, redis::RedisError> = redis::cmd("GET")
            .arg(
                format!("email:{}:id", email)
            )
            .query_async(&mut con)
            .await;
        match user_id {
            Ok(id) => {
                return Ok(Some(id));
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

    async fn set_salt_in_redis(
        &self,
        user_id: i64,
        salt: &str
    ) -> Result<(), DatabaseError> {
        let mut con = self.redis_con.clone();
        let _: () = redis::cmd("SET")
            .arg(
                format!("user:{}:salt", user_id)
            )
            .arg(salt)
            .query_async(&mut con)
            .await?;
        Ok(())
    }

    async fn set_email_id_in_redis(
        &self,
        email: &str,
        user_id: i64
    ) -> Result<(), DatabaseError> {
        let mut con = self.redis_con.clone();
        let _: () = redis::cmd("SET")
            .arg(
                format!("email:{}:id", email)
            )
            .arg(user_id)
            .query_async(&mut con)
            .await?;
        Ok(())
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


#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use serial_test::serial;

    use super::*;
    use crate::configuration::Config;
    use crate::database::DatabaseClientWithCaching;

    async fn get_db_client() -> DatabaseClientWithCaching {
        let mut cfg_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        cfg_path.push("../configuration/server/config.toml");
        let config = Config::from_file(cfg_path).unwrap();
        let db_client = DatabaseClientWithCaching::new(
            &config.redis_database,
            &config.postgres_database
        ).await.unwrap();
        db_client
    }

    #[tokio::test]
    #[serial]
    async fn test_get_refresh_token_by_user_id() -> Result<(), DatabaseError> {
        let db_client: DatabaseClientWithCaching = get_db_client().await;
        db_client.delete_user_refresh_token_in_redis(420).await.unwrap();
        db_client.set_user_refresh_token_in_redis(420, "test_token").await.unwrap();
        let token = db_client.get_user_refresh_token_from_redis_by_user_id(420).await;
        db_client.delete_user_refresh_token_in_redis(420).await.unwrap();
        assert_eq!(token.unwrap(), Some("test_token".to_string()));
        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_delete_token_by_user_id() -> Result<(), DatabaseError> {
        let db_client: DatabaseClientWithCaching = get_db_client().await;
        db_client.set_user_refresh_token_in_redis(420, "test_token").await.unwrap();
        db_client.delete_user_refresh_token_in_redis(420).await.unwrap();
        let token = db_client.get_user_refresh_token_from_redis_by_user_id(420).await;
        assert_eq!(token.unwrap(), None);
        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_get_user_from_postgres_by_id() -> Result<(), DatabaseError> {
        let db_client: DatabaseClientWithCaching = get_db_client().await;
        let user = User {
            id: 420,
            username: "test_user".to_string(),
            password_hash: "test_password".to_string(),
            email: None,
            created_at: 0,
            valid_refresh_token: None,
            verified: false,
            salt: "".to_string()
        };
        db_client.delete_user_refresh_token_in_postgres(420).await.unwrap();
        let res = db_client.delete_user_from_postgres_by_id(420).await;
        if res.is_err() {
            match res.err().unwrap() {
                DatabaseError::UserNotFound(_) => {},
                e => {
                    return Err(e);
                }
            }
        }

        db_client.insert_user_to_postgres(&user).await.unwrap();
        let user_db = db_client.get_user_from_postgres_by_id(420).await.unwrap();
        assert_eq!(user_db, Some(user));

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_update_refresh_token_with_caching() -> Result<(), DatabaseError> {
        let db_client: DatabaseClientWithCaching = get_db_client().await;
        // prepare test
        let user = User {
            id: 420,
            username: "test_user".to_string(),
            password_hash: "test_password".to_string(),
            email: None,
            created_at: 0,
            valid_refresh_token: None,
            verified: false,
            salt: "".to_string()
        };
        db_client.delete_user_refresh_token_in_redis(420).await.unwrap();

        let res = db_client.delete_user_from_postgres_by_id(420).await;
        if res.is_err() {
            match res.err().unwrap() {
                DatabaseError::UserNotFound(_) => {},
                e => {
                    return Err(e);
                }
            }
        }

        db_client.insert_user_to_postgres(&user).await.unwrap();


        db_client.update_user_refresh_token_with_caching(420, "test_token").await.unwrap();

        // check if the token is in Redis and Postgres
        let token_from_redis = db_client.get_user_refresh_token_from_redis_by_user_id(420).await.unwrap();
        let token_from_postgres = db_client.get_user_refresh_token_from_postgres_by_user_id(420).await.unwrap();
        assert_eq!(token_from_redis.unwrap(), token_from_postgres.unwrap());
        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_delete_refresh_token_with_caching() -> Result<(), DatabaseError> {
        let db_client: DatabaseClientWithCaching = get_db_client().await;
        // prepare test
        let user = User {
            id: 420,
            username: "test_user".to_string(),
            password_hash: "test_password".to_string(),
            email: None,
            created_at: 0,
            valid_refresh_token: None,
            verified: false,
            salt: "".to_string()
        };
        db_client.delete_user_refresh_token_in_redis(420).await.unwrap();

        let res = db_client.delete_user_from_postgres_by_id(420).await;
        if res.is_err() {
            match res.err().unwrap() {
                DatabaseError::UserNotFound(_) => {},
                e => {
                    return Err(e);
                }
            }
        }

        db_client.insert_user_to_postgres(&user).await.unwrap();

        db_client.delete_user_refresh_token_with_caching(420).await.unwrap();

        // check if the token is in Redis and Postgres
        let token_from_redis = db_client.get_user_refresh_token_from_redis_by_user_id(420).await.unwrap();
        let token_from_postgres = db_client.get_user_refresh_token_from_postgres_by_user_id(420).await.unwrap();
        assert_eq!(token_from_redis, None);
        assert_eq!(token_from_postgres, None);
        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_get_user_refresh_token_with_caching() -> Result<(), DatabaseError> {
        let db_client: DatabaseClientWithCaching = get_db_client().await;
        // prepare test
        let user = User {
            id: 420,
            username: "test_user".to_string(),
            password_hash: "test_password".to_string(),
            email: None,
            created_at: 0,
            valid_refresh_token: None,
            verified: false,
            salt: "".to_string()
        };
        db_client.delete_user_refresh_token_in_redis(420).await.unwrap();

        let res = db_client.delete_user_from_postgres_by_id(420).await;
        if res.is_err() {
            match res.err().unwrap() {
                DatabaseError::UserNotFound(_) => {},
                e => {
                    return Err(e);
                }
            }
        }


        db_client.insert_user_to_postgres(&user).await.unwrap();

        db_client.set_user_refresh_token_in_postgres(420, "test_token").await.unwrap();

        let token = db_client.get_user_refresh_token_with_caching(420).await.unwrap();
        assert_eq!(token, Some("test_token".to_string()));


        let token = db_client.get_user_refresh_token_from_redis_by_user_id(420).await.unwrap();

        // clean up
        db_client.delete_user_refresh_token_in_redis(420).await.unwrap();
        db_client.delete_user_refresh_token_in_postgres(420).await.unwrap();

        assert_eq!(token, Some("test_token".to_string()));

        Ok(())
    }

    

}
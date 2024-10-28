use crate::database::{
    methods::DatabaseError,
    DatabaseClientWithCaching
};


impl DatabaseClientWithCaching {
    pub(super) async fn get_user_refresh_token_from_redis_by_user_id(
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

    pub(super) async fn get_password_hash_by_user_id_from_redis(
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

    pub(super) async fn get_salt_by_user_id_from_redis(
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

    pub(super) async fn delete_salt_by_user_id_in_redis(
        &self,
        user_id: i64
    ) -> Result<(), DatabaseError> {
        let mut con = self.redis_con.clone();
        let _: () = redis::cmd("DEL")
            .arg(
                format!("user:{}:salt", user_id)
            )
            .query_async(&mut con)
            .await?;
        Ok(())
    }

    pub async fn delete_password_hash_by_user_id_in_redis(
        &self,
        user_id: i64
    ) -> Result<(), DatabaseError> {
        let mut con = self.redis_con.clone();
        let _: () = redis::cmd("DEL")
            .arg(
                format!("user:{}:password_hash", user_id)
            )
            .query_async(&mut con)
            .await?;
        Ok(())
    }

    pub(super) async fn set_password_hash_in_redis(
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



    pub(super) async fn set_salt_in_redis(
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

    pub(super) async fn set_user_refresh_token_in_redis(
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

    pub(super) async fn delete_user_refresh_token_in_redis(
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

    pub(super) async fn set_email_id_in_redis(
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
    pub(super) async fn delete_user_email_in_redis(
        &self,
        email: &str
    ) -> Result<(), DatabaseError> {
        let mut con = self.redis_con.clone();
        let _: () = redis::cmd("DEL")
            .arg(
                format!("email:{}:id", email)
            )
            .query_async(&mut con)
            .await?;
        Ok(())
    }



    pub(super) async fn get_user_id_by_email_from_redis(
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

}
use crate::database::{
    methods::DatabaseError,
    DatabaseClientWithCaching
};


impl DatabaseClientWithCaching {

    pub async fn redis_get_password_hash_by_user_id(
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

    pub async fn redis_get_salt_by_user_id(
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

    pub async fn redis_delete_salt_by_user_id(
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

    pub async fn redis_delete_password_hash_by_user_id(
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

    pub async fn redis_set_user_password_hash(
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



    pub async fn redis_set_user_salt(
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





}
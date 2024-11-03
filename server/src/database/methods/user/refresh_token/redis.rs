use crate::database::{
    methods::DatabaseError,
    DatabaseClientWithCaching
};


impl DatabaseClientWithCaching {
    pub async fn redis_get_user_refresh_token_by_user_id(
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

    pub async fn redis_set_user_refresh_token(
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

    pub async fn redis_delete_user_refresh_token(
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

}

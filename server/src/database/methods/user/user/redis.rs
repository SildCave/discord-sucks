use crate::database::{
    methods::DatabaseError,
    DatabaseClientWithCaching
};


impl DatabaseClientWithCaching {
    pub async fn redis_set_email_by_user_id(
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
    pub async fn redis_delete_email(
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

    pub async fn redis_get_user_id_by_email(
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

use crate::{app_objects::User, database::{
    methods::DatabaseError,
    DatabaseClientWithCaching
}};


impl DatabaseClientWithCaching {
    pub async fn postgres_set_user_refresh_token(
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
    
    pub async fn postgres_delete_user_refresh_token(
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
    
    
    pub async fn postgres_get_user_refresh_token_by_user_id(
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
}

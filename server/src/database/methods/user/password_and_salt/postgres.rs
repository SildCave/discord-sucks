
use crate::{app_objects::User, database::{
    methods::DatabaseError,
    DatabaseClientWithCaching
}};


impl DatabaseClientWithCaching {


    pub async fn postgres_get_password_hash_and_salt_by_user_id(
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




}
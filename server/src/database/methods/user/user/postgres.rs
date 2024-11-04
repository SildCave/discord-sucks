
use crate::{app_objects::User, database::{
    methods::DatabaseError,
    DatabaseClientWithCaching
}};


impl DatabaseClientWithCaching {
    pub async fn postgres_get_user_by_id(
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

    pub async fn postgres_get_user_id_by_email(
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

    pub async fn postgres_insert_user(
        &self,
        user: &User
    ) -> Result<(), DatabaseError> {
        let res = sqlx::query!(
            r#"
            INSERT INTO users (id, username, password_hash, salt, email, created_at, valid_refresh_token, verified, banned, date_of_birth)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            "#,
            user.id,
            user.username,
            user.password_hash,
            user.salt,
            user.email,
            user.created_at,
            user.valid_refresh_token,
            user.verified,
            user.banned,
            user.date_of_birth
        )
        .execute(&self.postgres_con)
        .await?;
        if res.rows_affected() == 0 {
            return Err(DatabaseError::UserAlreadyExists(user.id));
        }
        Ok(())
    }

    pub async fn postgres_delete_user_by_id(
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
}
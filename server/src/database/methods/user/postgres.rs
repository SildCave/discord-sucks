
use crate::{app_objects::User, database::{
    methods::DatabaseError,
    DatabaseClientWithCaching
}};


impl DatabaseClientWithCaching {
    pub(super) async fn get_user_from_postgres_by_id(
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

    pub(super) async fn get_password_hash_and_salt_from_postgres_by_user_id(
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

    pub(super) async fn get_user_id_by_email_from_postgres(
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
            INSERT INTO users (id, username, password_hash, salt, email, created_at, valid_refresh_token, verified, banned)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#,
            user.id,
            user.username,
            user.password_hash,
            user.salt,
            user.email,
            user.created_at,
            user.valid_refresh_token,
            user.verified,
            user.banned
        )
        .execute(&self.postgres_con)
        .await?;
        if res.rows_affected() == 0 {
            return Err(DatabaseError::UserAlreadyExists(user.id));
        }
        Ok(())
    }

    pub async fn delete_user_from_postgres_by_id(
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

    pub(super) async fn set_user_refresh_token_in_postgres(
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

    pub(super) async fn delete_user_refresh_token_in_postgres(
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


    pub(super) async fn get_user_refresh_token_from_postgres_by_user_id(
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
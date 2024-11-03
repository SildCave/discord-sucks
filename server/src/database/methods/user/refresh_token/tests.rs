#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use pretty_assertions::assert_eq;
    use serial_test::serial;
    use crate::app_objects::User;
    use crate::configuration::Config;
    use crate::database::methods::DatabaseError;
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
        db_client.redis_delete_user_refresh_token(420).await.unwrap();
        db_client.redis_set_user_refresh_token(420, "test_token").await.unwrap();
        let token = db_client.redis_get_user_refresh_token_by_user_id(420).await;
        db_client.redis_delete_user_refresh_token(420).await.unwrap();
        assert_eq!(token.unwrap(), Some("test_token".to_string()));
        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_delete_token_by_user_id() -> Result<(), DatabaseError> {
        let db_client: DatabaseClientWithCaching = get_db_client().await;
        db_client.redis_set_user_refresh_token(420, "test_token").await.unwrap();
        db_client.redis_delete_user_refresh_token(420).await.unwrap();
        let token = db_client.redis_get_user_refresh_token_by_user_id(420).await;
        assert_eq!(token.unwrap(), None);
        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_update_refresh_token_with_caching() -> Result<(), DatabaseError> {
        let db_client: DatabaseClientWithCaching = get_db_client().await;
        // prepare test
        let user = User {
            id: 420,
            ..User::default()
        };
        db_client.redis_delete_user_refresh_token(420).await.unwrap();

        let res = db_client.postgres_delete_user_by_id(420).await;
        if res.is_err() {
            match res.err().unwrap() {
                DatabaseError::UserNotFound(_) => {},
                e => {
                    return Err(e);
                }
            }
        }

        db_client.postgres_insert_user(&user).await.unwrap();


        db_client.cached_update_user_refresh_token(420, "test_token").await.unwrap();

        // check if the token is in Redis and Postgres
        let token_from_redis = db_client.redis_get_user_refresh_token_by_user_id(420).await.unwrap();
        let token_from_postgres = db_client.postgres_get_user_refresh_token_by_user_id(420).await.unwrap();
        assert_eq!(token_from_redis.unwrap(), token_from_postgres.unwrap());
        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_delete_refresh_token_with_caching() -> Result<(), DatabaseError> {
        let db_client: DatabaseClientWithCaching = get_db_client().await;
        // prepare test
        let user = User {
            valid_refresh_token: Some("test_token".to_string()),
            id: 420,
            ..User::default()
        };
        db_client.redis_delete_user_refresh_token(420).await.unwrap();

        let res = db_client.postgres_delete_user_by_id(420).await;
        if res.is_err() {
            match res.err().unwrap() {
                DatabaseError::UserNotFound(_) => {},
                e => {
                    return Err(e);
                }
            }
        }

        db_client.postgres_insert_user(&user).await.unwrap();

        let token_from_redis = db_client.redis_get_user_refresh_token_by_user_id(420).await.unwrap();
        let token_from_postgres = db_client.postgres_get_user_refresh_token_by_user_id(420).await.unwrap();
        assert_eq!(token_from_redis, None);
        assert_eq!(token_from_postgres, Some("test_token".to_string()));

        let _ = db_client.cached_delete_user_refresh_token(420).await;

        // check if the token is in Redis and Postgres
        let token_from_redis = db_client.redis_get_user_refresh_token_by_user_id(420).await.unwrap();
        let token_from_postgres = db_client.postgres_get_user_refresh_token_by_user_id(420).await.unwrap();
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
            ..User::default()
        };
        db_client.redis_delete_user_refresh_token(420).await.unwrap();

        let res = db_client.postgres_delete_user_by_id(420).await;
        if res.is_err() {
            match res.err().unwrap() {
                DatabaseError::UserNotFound(_) => {},
                e => {
                    return Err(e);
                }
            }
        }


        db_client.postgres_insert_user(&user).await.unwrap();

        db_client.postgres_set_user_refresh_token(420, "test_token").await.unwrap();

        let token = db_client.cached_get_user_refresh_token(420).await.unwrap();
        assert_eq!(token, Some("test_token".to_string()));


        let token = db_client.redis_get_user_refresh_token_by_user_id(420).await.unwrap();

        // clean up
        db_client.redis_delete_user_refresh_token(420).await.unwrap();
        db_client.postgres_delete_user_refresh_token(420).await.unwrap();

        assert_eq!(token, Some("test_token".to_string()));

        Ok(())
    }
}
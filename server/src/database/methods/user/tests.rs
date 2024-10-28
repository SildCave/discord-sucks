


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
            ..User::default()
        };
        let _ = db_client.delete_user_refresh_token_in_postgres(420).await.unwrap();
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
            ..User::default()
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
            valid_refresh_token: Some("test_token".to_string()),
            id: 420,
            ..User::default()
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

        let token_from_redis = db_client.get_user_refresh_token_from_redis_by_user_id(420).await.unwrap();
        let token_from_postgres = db_client.get_user_refresh_token_from_postgres_by_user_id(420).await.unwrap();
        assert_eq!(token_from_redis, None);
        assert_eq!(token_from_postgres, Some("test_token".to_string()));

        let _ = db_client.delete_user_refresh_token_with_caching(420).await;

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
            ..User::default()
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

    #[tokio::test]
    #[serial]
    async fn test_get_user_email_by_id() -> Result<(), DatabaseError> {
        let db_client: DatabaseClientWithCaching = get_db_client().await;
        // prepare test
        let user = User {
            email: Some("test_email".to_string()),
            id: 420,
            ..User::default()
        };
        db_client.delete_user_email_in_redis("test_email").await.unwrap();
        assert!(db_client.get_user_id_by_email_from_redis("test_email").await.unwrap().is_none());

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

        let user_id = db_client.get_user_id_by_email_with_cache("test_email").await.unwrap();
        assert_eq!(user_id, Some(420));

        assert_eq!(db_client.get_user_id_by_email_from_redis("test_email").await.unwrap(), Some(420));

        Ok(())
    }

    #[tokio::test]
    #[serial]
    pub async fn test_get_password_hash_and_salt_by_user_id() -> Result<(), DatabaseError> {
        let db_client: DatabaseClientWithCaching = get_db_client().await;
        // prepare test
        let user = User {
            password_hash: "test_password".to_string(),
            salt: "test_salt".to_string(),
            id: 420,
            ..User::default()
        };
        db_client.delete_salt_by_user_id_in_redis(420).await.unwrap();
        db_client.delete_password_hash_by_user_id_in_redis(420).await.unwrap();

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
        assert!(db_client.get_password_hash_by_user_id_from_redis(420).await.unwrap().is_none());
        assert!(db_client.get_salt_by_user_id_from_redis(420).await.unwrap().is_none());

        let (password_hash, salt) = db_client.get_password_hash_and_salt_by_user_id_with_caching(420).await.unwrap();
        assert_eq!(password_hash, "test_password".to_string());
        assert_eq!(salt, "test_salt".to_string());

        assert_eq!(db_client.get_password_hash_by_user_id_from_redis(420).await.unwrap(), Some("test_password".to_string()));
        assert_eq!(db_client.get_salt_by_user_id_from_redis(420).await.unwrap(), Some("test_salt".to_string()));

        Ok(())
    }

}
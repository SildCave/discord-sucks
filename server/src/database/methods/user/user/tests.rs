

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
    async fn test_postgres_get_user_by_id() -> Result<(), DatabaseError> {
        let db_client: DatabaseClientWithCaching = get_db_client().await;
        let user = User {
            id: 420,
            ..User::default()
        };
        let _ = db_client.postgres_delete_user_refresh_token(420).await;
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
        let user_db = db_client.postgres_get_user_by_id(420).await.unwrap();
        assert_eq!(user_db, Some(user));

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
        db_client.redis_delete_email("test_email").await.unwrap();
        assert!(db_client.redis_get_user_id_by_email("test_email").await.unwrap().is_none());

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

        let user_id = db_client.cached_get_user_id_by_email("test_email").await.unwrap();
        assert_eq!(user_id, Some(420));

        assert_eq!(db_client.redis_get_user_id_by_email("test_email").await.unwrap(), Some(420));

        Ok(())
    }


}
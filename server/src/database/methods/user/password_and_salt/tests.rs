
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
    pub async fn test_get_password_hash_and_salt_by_user_id() -> Result<(), DatabaseError> {
        let db_client: DatabaseClientWithCaching = get_db_client().await;
        // prepare test
        let user = User {
            password_hash: "test_password".to_string(),
            salt: "test_salt".to_string(),
            id: 420,
            ..User::default()
        };
        db_client.redis_delete_salt_by_user_id(420).await.unwrap();
        db_client.redis_delete_password_hash_by_user_id(420).await.unwrap();

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
        assert!(db_client.redis_get_password_hash_by_user_id(420).await.unwrap().is_none());
        assert!(db_client.redis_get_salt_by_user_id(420).await.unwrap().is_none());

        let (password_hash, salt) = db_client.cached_get_password_hash_and_salt_by_user_id(420).await.unwrap();
        assert_eq!(password_hash, "test_password".to_string());
        assert_eq!(salt, "test_salt".to_string());

        assert_eq!(db_client.redis_get_password_hash_by_user_id(420).await.unwrap(), Some("test_password".to_string()));
        assert_eq!(db_client.redis_get_salt_by_user_id(420).await.unwrap(), Some("test_salt".to_string()));

        Ok(())
    }
}
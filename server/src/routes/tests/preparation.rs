#[cfg(test)]
pub use preparation::*;

#[cfg(test)]
mod preparation {
    use std::path::PathBuf;
    use crate::configuration::Config;
    use crate::database::DatabaseClientWithCaching;
    use crate::routes::configure_routes;

    pub fn get_config() -> Config {
        let mut cfg_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        cfg_path.push("../configuration/server/config.toml");
        Config::from_file(cfg_path).unwrap()
    }

    pub async fn get_db_client() -> DatabaseClientWithCaching {
        let config = get_config();
        let db_client = DatabaseClientWithCaching::new(
            &config.redis_database,
            &config.postgres_database
        ).await.unwrap();
        db_client
    }
    pub async fn get_axum_app() -> axum::Router {
        let mut config = get_config();
        // jwt_secret_path = "configuration/server/jwt_secret.txt"
        let mut jwt_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        jwt_path.push("../configuration/server/jwt_secret.txt");
        config.jwt_config.jwt_secret_path = jwt_path.to_str().unwrap().to_string();
        let jwt_keys = crate::auth::JWTKeys::new(&config).unwrap();
        let db_client = get_db_client().await;
        let password_requirements = config.password_requirements.clone();
        let app = configure_routes(
            &jwt_keys,
            db_client.clone(),
            password_requirements,
            &config
        ).await;

        app
    }
}


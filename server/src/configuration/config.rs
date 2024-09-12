use anyhow::Result;

use serde::{Deserialize, Serialize};

use crate::auth::JWTKeys;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub server: ServerConfig,
    pub metrics_server: MetricsServerConfig,
    pub database: DatabaseConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub pem_cert_path: String,
    pub pem_key_path: String,
    pub jwt_secret_path: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MetricsServerConfig {
    pub host: String,
    pub port: u16,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct DatabaseConfig {
    pub username: String,
    pub source_db: String,
    pub pem_cert_path: String,
    pub password_file_path: String,
    pub port: u16,
    pub host: String,
    pub password: Option<String>,
}

impl Config {
    pub fn from_file(
        path: impl Into<std::path::PathBuf>,
    ) -> Result<Self> {
        let settings = config::Config::builder()
            .add_source(config::File::with_name(path.into().to_str().unwrap()))
            .build()?;
        let mut config: Config = settings.try_deserialize()?;
        let password = std::fs::read_to_string(&config.database.password_file_path)?;
        config.database.password = Some(password);
        Ok(config)
    }
}
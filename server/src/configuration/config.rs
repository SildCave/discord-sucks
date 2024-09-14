use anyhow::Result;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub server: ServerConfig,
    pub metrics_server: MetricsServerConfig,
    pub mongo_database: MongoDatabaseConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub enable_https: bool,
    pub allow_non_cloudflare_ips: bool,
    pub cloudflare_ips_refresh_interval_s: Option<u32>,
    pub pem_cert_path: Option<String>,
    pub pem_key_path: Option<String>,
    pub jwt_secret_path: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MetricsServerConfig {
    pub host: String,
    pub port: u16,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct MongoDatabaseConfig {
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
        let password = std::fs::read_to_string(&config.mongo_database.password_file_path)?;
        config.mongo_database.password = Some(password);
        Ok(config)
    }
}
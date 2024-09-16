use anyhow::Result;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub server: ServerConfig,
    pub metrics_server: MetricsServerConfig,
    pub postgres_database: PostgresDatabaseConfig,
    pub redis_database: RedisDatabaseConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub enable_https: bool,
    pub allow_non_cloudflare_ips: bool,
    pub cloudflare_ips_refresh_interval_s: Option<u64>,
    pub cloudflare_ips_refresh_interval_jitter_s: Option<u64>,
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
pub struct PostgresDatabaseConfig {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
    pub max_connections: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RedisDatabaseConfig {
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn from_file(
        path: impl Into<std::path::PathBuf>,
    ) -> Result<Self> {
        let settings = config::Config::builder()
            .add_source(config::File::with_name(path.into().to_str().unwrap()))
            .build()?;
        let config: Config = settings.try_deserialize()?;

        Ok(config)
    }
}
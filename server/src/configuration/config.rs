use anyhow::Result;

use serde::{
    Deserialize,
    Serialize
};

use crate::credentials::PasswordRequirements;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub server: ServerConfig,
    pub postgres_database: PostgresDatabaseConfig,
    pub redis_database: RedisDatabaseConfig,
    #[serde(rename = "jwt")]
    pub jwt_config: JWTConfig,
    pub password_requirements: PasswordRequirements,
    pub cloudflare: Cloudflare,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub enable_https: bool,
    pub pem_cert_path: Option<String>,
    pub pem_key_path: Option<String>,
    pub domain: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Cloudflare {
    pub turnstile_secret_key_path: String,
    pub allow_non_cloudflare_ips: bool,
    pub cloudflare_ips_refresh_interval_s: Option<u64>,
    pub cloudflare_ips_refresh_interval_jitter_s: Option<u64>,
    pub allow_invalid_turnstile: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct JWTConfig {
    pub jwt_secret_path: String,
    pub refresh_key_lifetime_s: i64,
    pub access_key_lifetime_s: i64,
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
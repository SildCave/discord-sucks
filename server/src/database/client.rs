use crate::configuration::{PostgresDatabaseConfig, RedisDatabaseConfig};

use super::{
    prepare_postgres_con, prepare_redis_con
};

use anyhow::Result;
use tracing::error;

#[derive(Clone, Debug)]
pub struct DatabaseClientWithCaching {
    pub redis_con: redis::aio::MultiplexedConnection,
    pub postgres_con: sqlx::postgres::PgPool,
}


impl DatabaseClientWithCaching {
    pub async fn new(
        redis_config: &RedisDatabaseConfig,
        postgres_config: &PostgresDatabaseConfig,
    ) -> Result<Self> {

        let redis_con = prepare_redis_con(
            &redis_config,
        ).await;
        let postgres_con = prepare_postgres_con(
            postgres_config,
        ).await;

        match redis_con {
            Ok(_) => {},
            Err(e) => {
                error!("Failed to prepare Redis connection: {}", e);
                return Err(e.into());
            }
        }
        match postgres_con {
            Ok(_) => {},
            Err(e) => {
                error!("Failed to prepare Postgres connection: {}", e);
                return Err(e.into());
            }
        }

        Ok(Self {
            redis_con: redis_con.unwrap(),
            postgres_con: postgres_con.unwrap(),
        })
    }
}
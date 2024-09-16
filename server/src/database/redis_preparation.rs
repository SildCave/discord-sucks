
use redis::aio::MultiplexedConnection;

use crate::configuration::RedisDatabaseConfig;




pub async fn prepare_redis_con(
    redis_config: &RedisDatabaseConfig
) -> Result<MultiplexedConnection, redis::RedisError> {
    let client = redis::Client::open(
        format!("redis://{}:{}", redis_config.host, redis_config.port)
    )?;
    let con = client.get_multiplexed_async_connection().await?;
    Ok(con)
}

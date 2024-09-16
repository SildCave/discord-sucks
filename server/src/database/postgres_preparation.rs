use crate::configuration::PostgresDatabaseConfig;

use sqlx::postgres::{
    PgConnectOptions, PgPool
};
use sqlx::ConnectOptions;

pub async fn prepare_postgres_con(
    postgres_config: &PostgresDatabaseConfig,
) -> Result<PgPool, sqlx::Error> {

    let mut db_connect_options = PgConnectOptions::new()
        .host(&postgres_config.host)
        .port(postgres_config.port)
        .username(&postgres_config.username)
        .password(&postgres_config.password)
        .database(&postgres_config.database_name);

    db_connect_options = db_connect_options.log_statements(log::LevelFilter::Debug);

    let pool = PgPool::connect_with(db_connect_options).await?;
    sqlx::query_file!("sql/init_messages_db.sql")
        .execute(&pool)
        .await?;

    Ok(pool)
}
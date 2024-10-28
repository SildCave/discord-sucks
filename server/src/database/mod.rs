mod redis_preparation;
mod postgres_preparation;

mod client;

mod methods;

pub use methods::DatabaseError;

pub (super) use redis_preparation::prepare_redis_con;
pub (super) use postgres_preparation::prepare_postgres_con;
pub use client::DatabaseClientWithCaching;

use std::sync::Arc;

use anyhow::Result;
use sqlx::{Pool, Postgres};
use tracing::{error, info};
use redis::aio::MultiplexedConnection;
use super::super::client::DatabaseClientWithCaching;

impl DatabaseClientWithCaching {

}


// pub async fn insert_message(
//     &self,
//     message: &Message,
//     timestamp: Option<i64>
// ) -> Result<()> {
//     let message = Arc::new(message.clone());
//     let created_at = match timestamp {
//         Some(ts) => ts,
//         None => chrono::Utc::now().timestamp()
//     };
//     let sql_insert_job = tokio::spawn({
//         let postgres_con = self.postgres_con.clone();
//         let message = message.clone();
//         async move {
//             insert_message_to_postgres_db(
//                 &postgres_con,
//                 message,
//                 &created_at
//             ).await
//         }
//     });
//     let redis_insert_job = tokio::spawn({
//         let redis_con = self.redis_con.clone();
//         let message = message.clone();
//         async move {
//             insert_message_to_redis_db(
//                 &redis_con,
//                 message,
//                 &created_at
//             ).await
//         }
//     });

//     let (sql_insert_job_res, redis_insert_job_res) = tokio::join!(
//         sql_insert_job,
//         redis_insert_job
//     );

//     match sql_insert_job_res {
//         Ok(_) => {},
//         Err(e) => {
//             error!("Failed to insert message into Postgres: {}", e);
//             redis_insert_job_res??;
//             delete_message_from_redis_db(&self.redis_con, &message.get_id()).await?;
//             info!("Deleted message from Redis");
//             return Err(e.into());
//         }
//     }
//     redis_insert_job_res??;
//     drop(message);
//     Ok(())
// }

// async fn insert_message_to_postgres_db (
//     postgres_con: &Pool<Postgres>,
//     message: Arc<Message>,
//     created_at: &i64
// ) -> Result<()> {
//     return Ok(());
//     sqlx::query!(
//         r#"
//         INSERT INTO messages (id, content, author_id, created_at, channel_id)
//         VALUES ($1, $2, $3, $4, $5)
//         "#,
//         message.get_id(),
//         message.get_content(),
//         message.get_author_id(),
//         created_at,
//         message.get_channel_id()
//     )
//     .execute(postgres_con)
//     .await?;

//     Ok(())
// }

// #[allow(dependency_on_unit_never_type_fallback)]
// async fn delete_message_from_redis_db (
//     redis_con: &MultiplexedConnection,
//     message_id: &i64
// ) -> Result<()> {
//     let mut con = redis_con.clone();
//     redis::pipe()
//         .atomic()
//         .cmd("DEL")
//         .arg(message_id)
//         .cmd("ZREM")
//         .arg("messages")
//         .arg(message_id)
//         .query_async(&mut con)
//         .await?;

//     Ok(())
// }

// #[allow(dependency_on_unit_never_type_fallback)]
// async fn insert_message_to_redis_db (
//     redis_con: &MultiplexedConnection,
//     message: Arc<Message>,
//     created_at: &i64
// ) -> Result<()> {
//     let mut con = redis_con.clone();
//     let message_json = message.to_json()?;
//     redis::pipe()
//         .atomic()
//         .cmd("SET")
//         .arg(message.get_id())
//         .arg(message_json)
//         .cmd("ZADD")
//         .arg("messages")
//         .arg(created_at)
//         .arg(message.get_id())
//         .query_async(&mut con)
//         .await?;

//     Ok(())
// }
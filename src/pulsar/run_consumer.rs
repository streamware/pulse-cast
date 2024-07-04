use std::io;

use bb8::Pool;
use diesel_async::{pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection};
use futures::TryStreamExt;
use oauth_fcm::SharedTokenManager;
use pulsar::{authentication::token, Consumer, DeserializeMessage, TokioExecutor};

use super::messages::MessageHandler;

pub async fn run_consumer<T>(
    mut consumer: Consumer<T, TokioExecutor>,
    pool: Pool<AsyncDieselConnectionManager<AsyncPgConnection>>,
    token_manager: SharedTokenManager,
) -> Result<(), io::Error>
where
    T: DeserializeMessage<Output = Result<T, serde_json::Error>>
        + MessageHandler
        + std::fmt::Debug
        + 'static,
    <T as DeserializeMessage>::Output: std::fmt::Debug,
{
    while let Some(msg) = consumer
        .try_next()
        .await
        .map_err(|e| std::io::Error::new(io::ErrorKind::Other, format!("Pulsar error: {:?}", e)))?
    {
        let data = match msg.deserialize() {
            Ok(data) => data,
            Err(e) => {
                println!("could not deserialize message: {:?}", e);
                break;
            }
        };
        data.handle_message(&pool, &token_manager).await?;

        // Acknowledge the message
        consumer.ack(&msg).await.map_err(|e| {
            std::io::Error::new(io::ErrorKind::Other, format!("Pulsar error: {:?}", e))
        })?;
    }

    Ok(())
}

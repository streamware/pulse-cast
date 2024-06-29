use std::io;

use futures::TryStreamExt;
use pulsar::{Consumer, DeserializeMessage, TokioExecutor};

use super::messages::MessageHandler;

pub async fn run_consumer<T>(mut consumer: Consumer<T, TokioExecutor>) -> Result<(), io::Error>
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
        data.handle_message()?;
        // let payload = msg.deserialize()?;
        println!("Received message: {:?}", data);

        // Acknowledge the message
        consumer.ack(&msg).await.map_err(|e| {
            std::io::Error::new(io::ErrorKind::Other, format!("Pulsar error: {:?}", e))
        })?;
    }

    Ok(())
}

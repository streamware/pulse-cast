use pulsar::{Consumer, DeserializeMessage, Pulsar, SubType, TokioExecutor};

use pulsar::Error as PulsarError;
use serde_json::Error as SerdeJsonError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MyError {
    #[error("Pulsar error: {0}")]
    PulsarError(#[from] PulsarError),

    #[error("Deserialization error: {0}")]
    DeserializationError(#[from] SerdeJsonError),

    #[error("Other error: {0}")]
    Other(String),
}

pub async fn create_consumer<T: DeserializeMessage>(
    pulsar: Pulsar<TokioExecutor>,
    topic: &str,
    subscription: &str,
) -> Result<Consumer<T, TokioExecutor>, MyError> {
    let consumer: Consumer<T, _> = pulsar
        .consumer()
        .with_topic(topic)
        .with_subscription_type(SubType::Shared)
        .with_subscription(subscription)
        .build()
        .await?;
    Ok(consumer)
}

extern crate pulse_cast;
use std::{env, io};

use axum::{extract::Extension, routing::post, routing::get, Router};
use diesel::{insert_into, Connection, SqliteConnection};
use dotenvy::dotenv;
use oauth_fcm::{create_shared_token_manager, send_fcm_message, FcmNotification, SharedTokenManager};
use pulsar::{Consumer, DeserializeMessage, Payload, Pulsar, SubType, TokioExecutor};use pulse_cast::models::{device::Device, user::User};
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use futures::TryStreamExt;


#[derive(Serialize)]
struct MyData {
    message: String,
}

async fn send_notification(
    Extension(token_manager): Extension<SharedTokenManager>,
) -> Result<String, String> {
    

    let device_token = "fRUVBXSLRm6dHydW2IND8h:APA91bFIMEK1RPvuU4X1ltQbcTMFuanoqgk6_vq-I64TVEbGZ9SDkXAejx7AAmuZ6XWGHx3NR04rRDpmBt2RVSYmXbGAQeKBAwjpaupqZRu5pouVkX49sFfDBTdyOXrRNyKkHoPUJwks";
    let project_id = "pheme-1c7fd";
    let data = MyData {
        message: "Hello from Axum!".to_string()
    };
    let ok: FcmNotification = FcmNotification {
        body: "qqq".to_string(),
        title: "qqq".to_string(),
    };

    send_fcm_message(device_token, Some(ok), Some(data), &token_manager, &project_id)
        .await
        .map_err(|e| e.to_string())?;

    Ok("FCM message sent successfully".to_string())
}

async fn root() -> &'static str {
    "streamware greets you!"
}

#[derive(Debug, Deserialize)]
struct UserCreated {
    id: String,
    username: String,
    created_at: String
}

impl DeserializeMessage for UserCreated {
    type Output = Result<UserCreated, serde_json::Error>;

    fn deserialize_message(payload: &Payload) -> Self::Output {
        serde_json::from_slice(&payload.data)
    }
}

#[derive(Debug, Deserialize)]
struct SendNotification {
    value: i32,
}

use thiserror::Error;
use pulsar::Error as PulsarError;
use serde_json::Error as SerdeJsonError;
use tokio::task;

#[derive(Debug, Error)]
pub enum MyError {
    #[error("Pulsar error: {0}")]
    PulsarError(#[from] PulsarError),

    #[error("Deserialization error: {0}")]
    DeserializationError(#[from] SerdeJsonError),

    #[error("Other error: {0}")]
    Other(String),
}

impl DeserializeMessage for SendNotification {
    type Output = Result<SendNotification, serde_json::Error>;

    fn deserialize_message(payload: &Payload) -> Self::Output {
        serde_json::from_slice(&payload.data)
    }
}

async fn create_consumer<T: DeserializeMessage>(
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



async fn run_consumer<T>(mut consumer: Consumer<T, TokioExecutor>) -> Result<(), io::Error>
where
    T: DeserializeMessage<Output = Result<T, serde_json::Error>> + std::fmt::Debug + 'static,
    <T as DeserializeMessage>::Output: std::fmt::Debug,
{
    
    while let Some(msg) = consumer.try_next().await.map_err(|e| std::io::Error::new(io::ErrorKind::Other, format!("Pulsar error: {:?}", e)))? {

        let data = match msg.deserialize() {
            Ok(data) => data,
            Err(e) => {
                println!("could not deserialize message: {:?}", e);
                break;
            }
        };
        // let payload = msg.deserialize()?;
        println!("Received message: {:?}", data);

        // Acknowledge the message
        consumer.ack(&msg).await.map_err(|e| std::io::Error::new(io::ErrorKind::Other, format!("Pulsar error: {:?}", e)))?;


    }
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();

    use self::pulse_cast::schema::users::dsl::*;

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let mut conncetion = SqliteConnection::establish(&database_url).expect("Failed to connect to database");

    let results = insert_into(users)
    .values(vec![
        User {
            id: Some("1".to_string()),
            username: "test".to_string(),
            created_at: "2021-08-01".to_string(),
            updated_at: "2021-08-01".to_string(),
        },
    ])
        .execute(&mut conncetion);


        let addr = env::var("PULSAR_ADDRESS")
        .ok()
        .unwrap_or_else(|| "pulsar://127.0.0.1:6650".to_string());

        let builder = Pulsar::builder(addr, TokioExecutor);

        let pulsar: Pulsar<_> = builder.build().await.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
 
        let user_created_consumer = create_consumer::<UserCreated>(pulsar.clone(), "USER_CREATED", "user-created-subscription").await.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        let send_notification_consumer = create_consumer::<SendNotification>(pulsar.clone(), "SEND_NOTIFCATION", "send-notification-subscription").await.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;


        let user_created_runner = task::spawn(run_consumer(user_created_consumer));
        let send_notification_runner = task::spawn(run_consumer(send_notification_consumer));

        let _ = user_created_runner.await?;
        let _ = send_notification_runner.await?;

        let mut consumer: Consumer<UserCreated, _> = pulsar
        .consumer()
        .with_topic("USER_CREATED")
        .with_consumer_name("test_consumer")
        .with_subscription_type(SubType::Exclusive)
        .with_subscription("test_subscription")
        .build()
        .await.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    let mut counter: usize = 0usize;
    while let Some(msg) = consumer.try_next().await.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))? {
        let data = match msg.deserialize() {
            Ok(data) => data,
            Err(e) => {
                break;
            }
        };

        println!("got message from pulsar {:?}", data);

        counter += 1;

        if counter > 10 {
            consumer.close().await.expect("Unable to close consumer");
            break;
        }
    }

    let shared_token_manager = create_shared_token_manager("./firebase.json")
        .expect("Could not find credentials.json");

    let app = Router::new()
    .route("/", get(root))
        .route("/send", post(send_notification))
        .layer(Extension(shared_token_manager));

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", "127.0.0.1", "8080"))
        .await
        .expect("Failed to bind to address");

    axum::serve(listener, app)
        .await
        .expect("Failed to start axum server");

    Ok(())
}
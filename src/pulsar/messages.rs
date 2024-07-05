use bb8::Pool;
use chrono::{DateTime, Utc};
use diesel::{insert_into, query_dsl::methods::FilterDsl, ExpressionMethods};
use diesel_async::{
    pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection, RunQueryDsl,
};
use futures::stream::{self, StreamExt};
use oauth_fcm::{send_fcm_message, SharedTokenManager};
use pulsar::{DeserializeMessage, Payload};
use serde::{Deserialize, Serialize};

use crate::{
    models::{device::Device, user::User},
    schema::{self},
};

pub trait MessageHandler {
    fn handle_message(
        &self,
        pool: &Pool<AsyncDieselConnectionManager<AsyncPgConnection>>,
        token_manager: &SharedTokenManager,
    ) -> impl std::future::Future<Output = Result<(), std::io::Error>> + Send;
}

#[derive(Debug, Deserialize)]
pub struct UserCreated {
    id: String,
    username: String,
    created_at: String,
}

impl DeserializeMessage for UserCreated {
    type Output = Result<UserCreated, serde_json::Error>;

    fn deserialize_message(payload: &Payload) -> Self::Output {
        serde_json::from_slice(&payload.data)
    }
}

impl MessageHandler for UserCreated {
    async fn handle_message(
        &self,
        pool: &Pool<AsyncDieselConnectionManager<AsyncPgConnection>>,
        _: &SharedTokenManager,
    ) -> Result<(), std::io::Error> {
        use schema::users::dsl::*;

        println!("Creating user: {:?}", self.created_at);

        let result = insert_into(users)
            .values(User {
                id: self.id.clone(),
                username: self.username.clone(),
                created_at: DateTime::parse_from_rfc3339(self.created_at.as_str())
                    .expect("Failed to parse datetime")
                    .with_timezone(&Utc)
                    .naive_utc(), // Convert to NaiveDateTime
                updated_at: Utc::now().naive_utc(), // Provide a default value for updated_at
            })
            .execute(&mut pool.get().await.unwrap())
            .await;

        println!("User created: {:?} {:?}", self, result);

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserNotification {
    recipient_id: String,
    sender_id: String,
    sender_username: String,
    content: String,
    created_at: String,
}

impl DeserializeMessage for UserNotification {
    type Output = Result<UserNotification, serde_json::Error>;

    fn deserialize_message(payload: &Payload) -> Self::Output {
        serde_json::from_slice(&payload.data)
    }
}

impl MessageHandler for UserNotification {
    async fn handle_message(
        &self,
        pool: &Pool<AsyncDieselConnectionManager<AsyncPgConnection>>,
        token_manager: &SharedTokenManager,
    ) -> Result<(), std::io::Error> {
        use schema::devices::dsl::*;

        let user_tokens: Vec<Device> = devices
            .filter(owner.eq(self.recipient_id.clone()))
            .filter(enabled.eq(true))
            .load(&mut pool.get().await.unwrap())
            .await
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        let devices_stream = stream::iter(user_tokens);

        devices_stream
            .for_each(|device| async move {
                send_fcm_message(
                    &device.device_token,
                    None,
                    Some(self),
                    token_manager,
                    "pheme-1c7fd",
                )
                .await
                .map_err(|e| {
                    println!("Error sending FCM message: {:?}", e);
                })
                .ok();
            })
            .await;

        Ok(())
    }
}

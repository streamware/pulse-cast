use bb8::Pool;
use chrono::{DateTime, Utc};
use diesel::insert_into;
use diesel_async::{
    pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection, RunQueryDsl,
};
use pulsar::{DeserializeMessage, Payload};
use serde::Deserialize;

use crate::{
    models::user::User,
    schema::{self},
};

pub trait MessageHandler {
    fn handle_message(
        &self,
        pool: &Pool<AsyncDieselConnectionManager<AsyncPgConnection>>,
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

#[derive(Debug, Deserialize)]
pub struct SendNotification {
    value: i32,
}

impl DeserializeMessage for SendNotification {
    type Output = Result<SendNotification, serde_json::Error>;

    fn deserialize_message(payload: &Payload) -> Self::Output {
        serde_json::from_slice(&payload.data)
    }
}

impl MessageHandler for SendNotification {
    async fn handle_message(
        &self,
        _: &Pool<AsyncDieselConnectionManager<AsyncPgConnection>>,
    ) -> Result<(), std::io::Error> {
        println!("notification received: {:?}", self);
        Ok(())
    }
}

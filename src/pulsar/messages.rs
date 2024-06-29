use pulsar::{DeserializeMessage, Payload};
use serde::Deserialize;

pub trait MessageHandler {
    fn handle_message(&self) -> Result<(), std::io::Error>;
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
    fn handle_message(&self) -> Result<(), std::io::Error> {
        println!("User created: {:?}", self);
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
    fn handle_message(&self) -> Result<(), std::io::Error> {
        println!("User created: {:?}", self);
        Ok(())
    }
}

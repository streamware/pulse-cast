extern crate pulse_cast;
use std::env;

use axum::{extract::Extension, routing::post, routing::get, Router};
use diesel::{connection, insert_into, Connection, SqliteConnection};
use dotenvy::dotenv;
use oauth_fcm::{create_shared_token_manager, send_fcm_message, FcmNotification, SharedTokenManager};
use pulse_cast::models::{device::Device, user::User};
use serde::Serialize;
use diesel::prelude::*;


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

#[tokio::main]
async fn main() {
    dotenv().ok();

    use self::pulse_cast::schema::users::dsl::*;

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let mut conncetion = SqliteConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

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
}
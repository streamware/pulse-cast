extern crate pulse_cast;
use axum::{extract::Extension, routing::get, routing::post, Router};
use diesel::prelude::*;
use diesel::{insert_into, Connection, SqliteConnection};
use dotenvy::dotenv;
use oauth_fcm::{
    create_shared_token_manager, send_fcm_message, FcmNotification, SharedTokenManager,
};
use pulsar::{Pulsar, TokioExecutor};
use pulse_cast::{
    models::user::User,
    pulsar::{
        create_consumer::create_consumer,
        messages::{SendNotification, UserCreated},
        run_consumer::run_consumer,
    },
};
use serde::Serialize;
use std::env;
use tokio::task;

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
        message: "Hello from Axum!".to_string(),
    };
    let ok: FcmNotification = FcmNotification {
        body: "qqq".to_string(),
        title: "qqq".to_string(),
    };

    send_fcm_message(
        device_token,
        Some(ok),
        Some(data),
        &token_manager,
        &project_id,
    )
    .await
    .map_err(|e| e.to_string())?;

    Ok("FCM message sent successfully".to_string())
}

async fn root() -> &'static str {
    "streamware greets you!"
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();

    use self::pulse_cast::schema::users::dsl::*;

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let mut conncetion =
        SqliteConnection::establish(&database_url).expect("Failed to connect to database");

    let results = insert_into(users)
        .values(vec![User {
            id: "1".to_string(),
            username: "test".to_string(),
            created_at: "2021-08-01".to_string(),
            updated_at: "2021-08-01".to_string(),
        }])
        .execute(&mut conncetion);

    let addr = env::var("PULSAR_ADDRESS")
        .ok()
        .unwrap_or_else(|| "pulsar://127.0.0.1:6650".to_string());

    let builder = Pulsar::builder(addr, TokioExecutor);

    let pulsar: Pulsar<_> = builder
        .build()
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    let user_created_consumer =
        create_consumer::<UserCreated>(pulsar.clone(), "USER_CREATED", "user-created-subscription")
            .await
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    let send_notification_consumer = create_consumer::<SendNotification>(
        pulsar.clone(),
        "SEND_NOTIFCATION",
        "send-notification-subscription",
    )
    .await
    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    let user_created_runner = task::spawn(run_consumer(user_created_consumer));
    let send_notification_runner = task::spawn(run_consumer(send_notification_consumer));

    let _ = user_created_runner.await?;
    let _ = send_notification_runner.await?;

    let shared_token_manager =
        create_shared_token_manager("./firebase.json").expect("Could not find credentials.json");

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

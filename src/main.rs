extern crate pulse_cast;

use axum::{
    routing::{get, post},
    Router,
};
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use dotenvy::dotenv;
use oauth_fcm::create_shared_token_manager;
use pulsar::{Pulsar, TokioExecutor};
use pulse_cast::pulsar::{
    create_consumer::create_consumer,
    messages::{UserCreated, UserNotification},
    run_consumer::run_consumer,
};
use pulse_cast::routes::register_device::register_device;
use std::env;
use tokio::task;

async fn root() -> &'static str {
    "streamware greets you!"
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();

    let shared_token_manager =
        create_shared_token_manager().expect("Could not find credentials.json");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(database_url);
    let pool = bb8::Pool::builder().build(config).await.unwrap();

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

    let user_notification_consumer = create_consumer::<UserNotification>(
        pulsar.clone(),
        "USER_NOTIFICATION",
        "user-notification-subscription",
    )
    .await
    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    let user_created_runner = task::spawn(run_consumer(
        user_created_consumer,
        pool.clone(),
        shared_token_manager.clone(),
    ));
    let user_notification_runner = task::spawn(run_consumer(
        user_notification_consumer,
        pool.clone(),
        shared_token_manager.clone(),
    ));

    let _ = user_created_runner.await?;
    let _ = user_notification_runner.await?;

    let app = Router::new()
        .route("/", get(root))
        .route("/register-device", post(register_device))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", "127.0.0.1", "9090"))
        .await
        .expect("Failed to bind to address");

    axum::serve(listener, app)
        .await
        .expect("Failed to start axum server");

    Ok(())
}

use axum::{extract::Extension, routing::post, routing::get, Router};
use oauth_fcm::{create_shared_token_manager, send_fcm_message, FcmNotification, SharedTokenManager};
use serde::Serialize;

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
use expo_notification::{ExpoClient, ExpoConfig, Notification};

#[tokio::main]
async fn main() {
    let client = ExpoClient::new(ExpoConfig {
        host: "https://exp.host".to_string(),
        push_path: "/--/api/v2/push/send".to_string(),
        access_token: None,
    });

    client.send_push_notification(Notification {
        to: "".to_string(),
        title: Some("Hello world".to_string()),
        body: Some("This is a notification from Rust".to_string()),
    }).await;
}

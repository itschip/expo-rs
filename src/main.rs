use expo_notification::{ExpoClient, ExpoConfig, Notification};

#[tokio::main]
async fn main() {
    let client = ExpoClient::new(ExpoConfig {
        host: "https://exp.host".to_string(),
        push_path: "/--/api/v2/push/send".to_string(),
        access_token: None,
    });

    let mut notification = Notification::new(vec!["".to_string()]);
    let expo_notification = notification
        .with_title("Hello chip".to_string())
        .with_body("What's up?".to_string())
        .with_sound("default".to_string());

    let res = client.send_push_notification(expo_notification).await;

     match res {
         Ok(res) => {
            println!("Last response: {:?}", res[0]);
         },
         Err(err) => {
             println!("{err}")
         }
     }
}

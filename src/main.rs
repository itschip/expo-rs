use expo_notification::{ExpoClient, ExpoConfig, Notification};

#[tokio::main]
async fn main() {
    let client = ExpoClient::new(ExpoConfig {
        host: "https://exp.host".to_string(),
        push_path: "/--/api/v2/push/send".to_string(),
        access_token: None,
    });

    let notification = Notification::new(vec!["ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]".to_string()])
        .with_title("Hello World".to_string())
        .with_body("This is a test notification".to_string());

     let res = client.send_push_notification(notification).await;

     match res {
         Ok(res) => {
            println!("Response: {:?}", res.data[0]);
         },
         Err(err) => {
             println!("{err}")
         }
     }
}

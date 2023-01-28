use serde::{Deserialize, Serialize};

pub struct ExpoClient {
    host: String,
    push_path: String,
    access_token: Option<String>,
    http_client: reqwest::Client,
}

pub struct ExpoConfig {
    pub host: String,
    pub push_path: String,
    pub access_token: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Notification {
    pub to: Vec<String>,
    pub title: Option<String>,
    pub body: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PushTicket {
    status: String,
    id: String,
    details: Option<TicketDetails>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TicketDetails {
    error: String,
    expo_push_token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PushTicketResponse {
    data: Vec<PushTicket>,
}

impl ExpoClient {
    pub fn new(config: ExpoConfig) -> ExpoClient {
        let http_client = reqwest::Client::new();
        ExpoClient {
            host: config.host,
            push_path: config.push_path,
            access_token: config.access_token,
            http_client,
        }
    }

    pub async fn send_push_notification(&self, notification: Notification) -> Result<PushTicketResponse, reqwest::Error> {
        let url = format!("{}{}", self.host, self.push_path);

        let res = self.http_client.post(&url).json(&notification).send().await.unwrap();
        let data = res.json::<PushTicketResponse>().await.unwrap();

        Ok(data)
    }
}

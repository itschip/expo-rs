use reqwest::header::{HeaderMap, HOST, ACCEPT, ACCEPT_ENCODING, CONTENT_TYPE};
use serde::{Deserialize, Serialize, de::Error};

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

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sound: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub badge: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "channelId")]
    pub channel_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "categoryId")]
    pub category_id: Option<String>
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
    pub data: Vec<PushTicket>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Priority {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "high")]
    High
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
        
        let mut headers = HeaderMap::new();

        headers.insert(HOST, "exp.host".parse().unwrap());
        headers.insert(ACCEPT, "application/json".parse().unwrap());
        headers.insert(ACCEPT_ENCODING, "gzip, deflate".parse().unwrap());
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

        let res = self.http_client.post(&url).headers(headers).json(&notification).send().await; 

        match res {
            Ok(res) => {
                let res = res.json::<PushTicketResponse>().await;
                match res {
                    Ok(res) => Ok(res),
                    Err(err) => Err(err),
                }
            },
            Err(err) => Err(err),
        }
    }
}

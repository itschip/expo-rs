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

impl Notification {
    pub fn new(to: Vec<String>) -> Self {
        Notification {
            to,
            data: None,
            title: None,
            body: None,
            ttl: None,
            expiration: None,
            priority: None,
            subtitle: None,
            sound: None,
            badge: None,
            channel_id: None,
            category_id: None,
        }
    }

    pub fn with_data(mut self, data: String) -> Self {
        self.data = Some(data);
        self
    }

    pub fn with_title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    pub fn with_body(mut self, body: String) -> Self {
        self.body = Some(body);
        self
    }

    pub fn with_ttl(mut self, ttl: u32) -> Self {
        self.ttl = Some(ttl);
        self
    }

    pub fn with_expiration(mut self, expiration: u32) -> Self {
        self.expiration = Some(expiration);
        self
    }

    pub fn with_priority(mut self, priority: String) -> Self {
        self.priority = Some(priority);
        self
    }

    pub fn with_subtitle(mut self, subtitle: String) -> Self {
        self.subtitle = Some(subtitle);
        self
    }

    pub fn with_sound(mut self, sound: String) -> Self {
        self.sound = Some(sound);
        self
    }

    pub fn with_badge(mut self, badge: u32) -> Self {
        self.badge = Some(badge);
        self
    }

    pub fn with_channel_id(mut self, channel_id: String) -> Self {
        self.channel_id = Some(channel_id);
        self
    }

    pub fn with_category_id(mut self, category_id: String) -> Self {
        self.category_id = Some(category_id);
        self
    }

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

use anyhow::Result;
use obws::{client::ConnectConfig, requests::EventSubscription, responses::scenes::Scenes};
use serde::{Deserialize, Serialize};
use tokio::sync::oneshot::Sender;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ConnectInfo {
    host: String,
    port: u16,
    password: Option<String>,
}
impl Default for ConnectInfo {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 4455,
            password: None,
        }
    }
}
impl Into<ConnectConfig<String, String>> for ConnectInfo {
    fn into(self) -> ConnectConfig<String, String> {
        ConnectConfig {
            host: self.host,
            port: self.port,
            password: self.password,
            event_subscriptions: Some(EventSubscription::ALL),
            broadcast_capacity: Some(100),
        }
    }
}

pub enum OBSRequestMessage {
    SetConnectInfo(ConnectInfo, Sender<Result<ConnectInfo>>),
    GetConnectInfo(Sender<Result<ConnectInfo>>),
    Connect(Sender<Result<()>>),
    GetScenes(Sender<obws::Result<Scenes>>),
}

use chrono::{serde::ts_milliseconds, DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio::sync::oneshot::Sender;
use url::Url;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FieldUpdateType {
    MatchLoad,
    MatchStart,
    MatchAbort,
    MatchCommit,
    MatchPost,
    ShowPreview,
    ShowRandom,
    ShowMatch,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldUpdatePayload {
    number: u32,
    field: u32,
    short_name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldUpdate {
    #[serde(deserialize_with = "ts_milliseconds::deserialize")]
    update_time: DateTime<Utc>,
    payload: FieldUpdatePayload,
    update_type: FieldUpdateType,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum FTCLiveBroadcastMessage {
    FieldUpdate(FieldUpdate),
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EventDetails {
    event_code: String,
    division: i8,
    finals: bool,
    name: String,
    #[serde(deserialize_with = "ts_milliseconds::deserialize")]
    start: DateTime<Utc>,
    #[serde(deserialize_with = "ts_milliseconds::deserialize")]
    end: DateTime<Utc>,
    #[serde(rename = "type")]
    event_type: String,
    status: String,
}

#[derive(Debug)]
pub enum FTCLiveRequest {
    GetEvents(Sender<anyhow::Result<Vec<String>>>),
    SetUrl(Url, Sender<anyhow::Result<Vec<String>>>),
    SetEventCode(String, Sender<anyhow::Result<EventDetails>>),
    ConnectWebsocket(Sender<anyhow::Result<()>>),
    CheckWebsocket(Sender<anyhow::Result<bool>>),
}

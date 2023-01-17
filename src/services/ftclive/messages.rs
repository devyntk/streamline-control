use serde::{Deserialize, Serialize};
use tokio::sync::oneshot::Sender;
use url::Url;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FieldUpdate {
    time: u64,
    number: u32,
    field: u32,
    short_name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "updateType")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FTCLiveBroadcastMessage {
    MatchLoad(FieldUpdate),
    MatchStart(FieldUpdate),
    MatchAbort(FieldUpdate),
    MatchCommit(FieldUpdate),
    MatchPost(FieldUpdate),
    ShowPreview(FieldUpdate),
    ShowRandom(FieldUpdate),
    ShowMatch(FieldUpdate),
}

#[derive(Debug)]
pub enum FTCLiveRequest {
    GetEvents(Sender<anyhow::Result<Vec<String>>>),
    SetUrl(Url, Sender<anyhow::Result<Vec<String>>>),
    SetEventCode(String, Sender<anyhow::Result<()>>),
    ConnectWebsocket(Sender<anyhow::Result<()>>),
}

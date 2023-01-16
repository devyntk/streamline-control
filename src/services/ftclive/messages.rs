use tokio::sync::oneshot::Sender;
use url::Url;

#[derive(Clone, Debug)]
pub enum FTCLiveBroadcastMessage {
    Init,
}

#[derive(Debug)]
pub enum FTCLiveRequest {
    GetEvents(Sender<anyhow::Result<Vec<String>>>),
    SetUrl(Url, Sender<anyhow::Result<Vec<String>>>),
}

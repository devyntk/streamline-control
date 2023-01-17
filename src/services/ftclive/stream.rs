use crate::services::ftclive::messages::{FTCLiveBroadcastMessage, FieldUpdate};
use flume::Sender;
use futures::prelude::*;
use tokio::net::TcpStream;
use tokio::task::JoinHandle;
use tokio_tungstenite::tungstenite::Message::Text;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use url::Url;

pub(crate) async fn connect_ws(
    base_url: Url,
    event_code: String,
    task: &mut Option<JoinHandle<()>>,
    private_tx: Sender<FTCLiveBroadcastMessage>,
) -> anyhow::Result<()> {
    let mut url = base_url.clone();
    url.set_path("/api/v2/stream/");
    url.set_scheme("ws");
    url.query_pairs_mut().append_pair("code", &event_code);

    let (ws_stream, _) = connect_async(url).await?;
    log::info!("Starting FTC Live WS Stream connection");

    if task.is_some() {
        log::warn!("FTC Live WS task already exists, aborting previous task and starting new");
        task.take().unwrap().abort();
    }

    tokio::spawn(handle_stream(ws_stream, private_tx));
    Ok(())
}

async fn handle_stream(
    mut ws_stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
    private_tx: Sender<FTCLiveBroadcastMessage>,
) {
    loop {
        let msg = ws_stream.next().await;
        log::debug!("FTC Live WS Message: {:?}", msg);
        if let Some(Ok(Text(text))) = msg {
            let dec_msg: FieldUpdate = serde_json::from_str(&*text).unwrap();
            private_tx
                .send_async(FTCLiveBroadcastMessage::FieldUpdate(dec_msg))
                .await
                .unwrap();
        }
    }
}

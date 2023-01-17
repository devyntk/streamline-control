use reqwest::Client;
use std::fmt::Debug;
use std::future::Future;
use tokio::spawn;
use tokio::sync::oneshot::Sender;
use url::Url;

use self::messages::{FTCLiveBroadcastMessage, FTCLiveRequest};

mod connection;
pub mod messages;
mod stream;

pub async fn init() -> (
    flume::Receiver<FTCLiveBroadcastMessage>,
    flume::Sender<FTCLiveRequest>,
) {
    let (private_tx, public_rx) = flume::unbounded();
    let (public_tx, private_rx) = flume::unbounded();

    spawn(async {
        listener(private_rx, private_tx).await;
    });

    return (public_rx, public_tx);
}

async fn wrap_response<R: Debug>(func: impl Future<Output = R>, sender: Sender<R>) {
    sender
        .send(func.await)
        .expect("Failed to return FTC live message");
}

async fn listener(
    private_rx: flume::Receiver<FTCLiveRequest>,
    _private_tx: flume::Sender<FTCLiveBroadcastMessage>,
) {
    let client = Client::new();
    let mut url = Url::parse("http://localhost").unwrap();
    let mut event_code = "".to_string();
    loop {
        match private_rx.recv_async().await {
            Ok(FTCLiveRequest::GetEvents(sender)) => {
                wrap_response(
                    connection::get_event_codes(url.clone(), client.clone()),
                    sender,
                )
                .await
            }
            Ok(FTCLiveRequest::SetUrl(new_url, sender)) => {
                wrap_response(
                    async {
                        let res =
                            connection::get_event_codes(new_url.clone(), client.clone()).await;
                        url = new_url;
                        res
                    },
                    sender,
                )
                .await
            }
            Ok(FTCLiveRequest::SetEventCode(new_event_code, sender)) => {
                wrap_response(
                    async {
                        event_code = new_event_code;
                        Ok(())
                    },
                    sender,
                )
                .await
            }

            Ok(FTCLiveRequest::ConnectWebsocket(sender)) => {
                wrap_response(stream::connect_ws(url.clone(), event_code.clone()), sender).await
            }
            Err(_) => {
                log::info!("All FTC Live request senders were dropped, killing listener");
                return;
            }
        }
    }
}

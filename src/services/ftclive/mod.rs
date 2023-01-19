use std::{fmt::Debug, future::Future};

use reqwest::Client;
use sqlx::{Pool, Sqlite};
use tokio::{spawn, sync::oneshot::Sender};
use url::Url;

use self::messages::{FTCLiveBroadcastMessage, FTCLiveRequest};
use crate::config::Config;

mod connection;
pub mod messages;
mod stream;

pub async fn init(
    db: Pool<Sqlite>,
) -> (
    flume::Receiver<FTCLiveBroadcastMessage>,
    flume::Sender<FTCLiveRequest>,
) {
    let (private_tx, public_rx) = flume::unbounded();
    let (public_tx, private_rx) = flume::unbounded();

    spawn(async {
        listener(private_rx, private_tx, db).await;
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
    private_tx: flume::Sender<FTCLiveBroadcastMessage>,
    db: Pool<Sqlite>,
) {
    let client = Client::new();
    let mut ws_handle = None;
    let mut url = Config::new(
        "sc_url",
        Url::parse("http://localhost").unwrap(),
        db.clone(),
    )
    .await
    .unwrap();
    let mut event_code = Config::new("sc_event_code", "".to_string(), db.clone())
        .await
        .unwrap();
    loop {
        match private_rx.recv_async().await {
            Ok(FTCLiveRequest::GetEvents(sender)) => {
                wrap_response(
                    connection::get_event_codes(url.get(), client.clone()),
                    sender,
                )
                .await
            }
            Ok(FTCLiveRequest::SetUrl(new_url, sender)) => {
                wrap_response(
                    async {
                        let res =
                            connection::get_event_codes(new_url.clone(), client.clone()).await;
                        url.set(new_url).await.unwrap();
                        res
                    },
                    sender,
                )
                .await
            }
            Ok(FTCLiveRequest::SetEventCode(new_event_code, sender)) => {
                wrap_response(
                    async {
                        let res = connection::get_event_details(
                            url.get(),
                            client.clone(),
                            new_event_code.clone(),
                        )
                        .await;
                        event_code.set(new_event_code).await.unwrap();
                        res
                    },
                    sender,
                )
                .await
            }
            Ok(FTCLiveRequest::ConnectWebsocket(sender)) => {
                wrap_response(
                    stream::connect_ws(
                        url.get(),
                        event_code.get(),
                        &mut ws_handle,
                        private_tx.clone(),
                    ),
                    sender,
                )
                .await
            }
            Ok(FTCLiveRequest::CheckWebsocket(sender)) => {
                wrap_response(
                    async {
                        return Ok(ws_handle.is_some());
                    },
                    sender,
                )
                .await
            }
            Err(_) => {
                log::info!("All FTC Live request senders were dropped, killing listener");
                return;
            }
        }
    }
}

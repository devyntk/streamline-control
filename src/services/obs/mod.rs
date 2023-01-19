use obws::Client;
use sqlx::{Pool, Sqlite};
use tokio::spawn;

use crate::{
    config::Config,
    services::obs::messages::{ConnectInfo, OBSRequestMessage, OBSRequestMessage::*},
};

pub mod messages;

pub async fn init(db: Pool<Sqlite>) -> flume::Sender<OBSRequestMessage> {
    let (public_tx, private_rx) = flume::unbounded();

    spawn(async {
        listener(private_rx, db).await.unwrap();
    });

    return public_tx;
}

async fn listener(rx: flume::Receiver<OBSRequestMessage>, db: Pool<Sqlite>) -> anyhow::Result<()> {
    let mut connect_config = Config::new("obs_info", ConnectInfo::default(), db.clone()).await?;
    let mut client = None;
    loop {
        match rx.recv_async().await {
            Ok(Connect(sender)) => {
                client = match Client::connect_with_config(connect_config.get().into()).await {
                    Ok(client) => {
                        sender.send(Ok(())).unwrap();
                        Some(client)
                    }
                    Err(err) => {
                        log::error!("Can't connect to OBS: {:?}", err);
                        sender.send(Err(err.into())).unwrap();
                        None
                    }
                };
            }
            Ok(SetConnectInfo(info, sender)) => {
                sender.send(connect_config.set(info).await).unwrap();
            }
            Ok(GetConnectInfo(sender)) => {
                sender.send(Ok(connect_config.get())).unwrap();
            }
            Ok(GetScenes(sender)) => {
                if let Some(client) = &client {
                    sender.send(client.scenes().list().await).unwrap();
                } else {
                    sender.send(Err(obws::Error::Disconnected)).unwrap();
                }
            }
            Err(_) => {
                log::info!("All OBS request senders were dropped, killing listener");
                return Ok(());
            }
        }
    }
}

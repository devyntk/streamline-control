use flume::Receiver;
use sqlx::{Pool, Sqlite};
use tokio::spawn;

use crate::controllers::obs_scenes::messages::{OBSSceneRequest, OBSSceneRequest::Close};

pub mod messages;

pub async fn init(db: Pool<Sqlite>) -> flume::Sender<OBSSceneRequest> {
    let (public_tx, private_rx) = flume::unbounded();

    spawn(async {
        listener(private_rx, db).await.unwrap();
    });

    return public_tx;
}

async fn listener(private_rx: Receiver<OBSSceneRequest>, _db: Pool<Sqlite>) -> anyhow::Result<()> {
    loop {
        match private_rx.recv_async().await.unwrap_or(Close) {
            Close => return Ok(()),
        }
    }
}

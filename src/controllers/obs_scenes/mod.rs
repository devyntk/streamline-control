use std::{fmt::Debug, future::Future, sync::Arc};

use flume::{Receiver, Sender};
use sqlx::{Pool, Sqlite};
use tokio::{spawn, sync::Mutex};

use crate::{
    config::Config,
    controllers::obs_scenes::messages::{
        OBSSceneFieldMapping, OBSSceneRequest, OBSSceneRequest::*,
    },
    services::{
        ftclive::messages::{FTCLiveBroadcastMessage, FieldUpdateType},
        obs::messages::OBSRequestMessage,
    },
};

pub mod messages;
mod stream;

pub async fn init(
    db: Pool<Sqlite>,
    sk_rx: Receiver<FTCLiveBroadcastMessage>,
    obs_tx: Sender<OBSRequestMessage>,
) -> Sender<OBSSceneRequest> {
    let (public_tx, private_rx) = flume::unbounded();

    spawn(async {
        listener(private_rx, db, sk_rx, obs_tx).await.unwrap();
    });

    return public_tx;
}

async fn wrap_response<R: Debug>(
    func: impl Future<Output = R>,
    sender: tokio::sync::oneshot::Sender<R>,
) {
    sender
        .send(func.await)
        .expect("Failed to return OBS Scene Controller message");
}

async fn listener(
    private_rx: Receiver<OBSSceneRequest>,
    db: Pool<Sqlite>,
    sk_rx: Receiver<FTCLiveBroadcastMessage>,
    obs_tx: Sender<OBSRequestMessage>,
) -> anyhow::Result<()> {
    // TODO: make the Config class send/sync with internal mutex
    let mapping = Arc::new(Mutex::new(
        Config::new(
            "obs_scene_mappings",
            OBSSceneFieldMapping::new(),
            db.clone(),
        )
        .await?,
    ));
    let mut listener_handle = None;
    loop {
        match private_rx.recv_async().await.unwrap_or(Close) {
            Close => return Ok(()),
            SetSceneMapping {
                new_mapping,
                sender,
            } => {
                wrap_response(
                    update_mapping(
                        mapping.clone(),
                        new_mapping.field,
                        new_mapping.update_type,
                        new_mapping.scene,
                    ),
                    sender,
                )
                .await
            }
            GetSceneMapping(sender) => {
                let data = mapping.lock().await;
                sender
                    .send(Ok(data.get()))
                    .expect("Can't send OBS Scene mapping");
            }
            StartListener(sender) => {
                wrap_response(
                    async {
                        listener_handle = Some(spawn(stream::handle_stream(
                            mapping.clone(),
                            sk_rx.clone(),
                            obs_tx.clone(),
                        )));
                        Ok(())
                    },
                    sender,
                )
                .await
            }
            CheckListener(sender) => {
                wrap_response(async { Ok(listener_handle.is_some()) }, sender).await
            }
        }
    }
}

async fn update_mapping(
    mapping: Arc<Mutex<Config<OBSSceneFieldMapping>>>,
    field: u32,
    update_type: FieldUpdateType,
    scene: String,
) -> anyhow::Result<()> {
    let mut lock = mapping.lock().await;
    let mut local_mapping = lock.get();
    let db_field = local_mapping.entry(field).or_default();
    let db_scene = db_field.entry(update_type).or_default();
    *db_scene = scene;
    lock.set(local_mapping).await?;
    Ok(())
}

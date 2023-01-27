use std::sync::Arc;

use anyhow::Result;
use flume::{Receiver, Sender};
use tokio::sync::{oneshot::channel, Mutex};

use crate::{
    config::Config,
    controllers::obs_scenes::messages::OBSSceneFieldMapping,
    services::{
        ftclive::messages::{FTCLiveBroadcastMessage, FTCLiveBroadcastMessage::FieldUpdate},
        obs::messages::OBSRequestMessage,
    },
};

pub(crate) async fn handle_stream(
    mapping: Arc<Mutex<Config<OBSSceneFieldMapping>>>,
    sk_rx: Receiver<FTCLiveBroadcastMessage>,
    obs_tx: Sender<OBSRequestMessage>,
) -> Result<()> {
    loop {
        let msg = sk_rx.recv_async().await?;
        if let FieldUpdate(update) = msg {
            let scene = {
                let lock = mapping.lock().await;
                let config = lock.get();
                if let Some(field_map) = config.get(&update.payload.field) {
                    let Some(scene) = field_map.get(&update.update_type) else {continue};
                    scene.clone()
                } else {
                    continue;
                }
            };

            let (tx, rx) = channel();
            obs_tx
                .send_async(OBSRequestMessage::SetScene(scene, tx))
                .await?;
            rx.await??;
        }
    }
}

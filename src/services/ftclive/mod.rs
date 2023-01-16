use tokio::spawn;

use self::messages::{FTCLiveBroadcastMessage, FTCLiveRequest};

pub mod messages;

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

async fn listener(
    mut private_rx: flume::Receiver<FTCLiveRequest>,
    private_tx: flume::Sender<FTCLiveBroadcastMessage>,
) {
    while let Ok(msg) = private_rx.recv_async().await {
        log::debug!("{:?}", msg);
    }
}

use std::{fs, sync::Arc};

use app_dirs2::{app_root, AppDataType};
use axum::extract::FromRef;
use biscuit_auth::KeyPair;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    Pool, Sqlite,
};

use crate::{
    controllers::obs_scenes::messages::OBSSceneRequest,
    services::{
        ftclive::messages::{FTCLiveBroadcastMessage, FTCLiveRequest},
        obs::messages::OBSRequestMessage,
    },
    APP_INFO,
};

#[derive(Clone, Debug, FromRef)]
pub struct SharedState {
    pub pool: Pool<Sqlite>,
    pub key: Arc<KeyPair>,
    pub sk_rx: flume::Receiver<FTCLiveBroadcastMessage>,
    pub sk_tx: flume::Sender<FTCLiveRequest>,
    pub obs_tx: flume::Sender<OBSRequestMessage>,
    pub obs_scene_tx: flume::Sender<OBSSceneRequest>,
}

pub async fn get_pool() -> anyhow::Result<Pool<Sqlite>> {
    let mut db_url = app_root(AppDataType::UserConfig, &APP_INFO)?;
    db_url.push("streamline.db");

    let db_options = SqliteConnectOptions::new()
        .filename(db_url)
        .create_if_missing(true);
    let pool = SqlitePoolOptions::new().connect_with(db_options).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(pool)
}

pub async fn get_state() -> anyhow::Result<SharedState> {
    let pool = get_pool().await?;

    let mut key_path = app_root(AppDataType::UserConfig, &APP_INFO)?;
    key_path.push("key");

    let kp: ed25519_dalek::Keypair = if key_path.is_file() {
        let file = fs::read(key_path)?;
        ed25519_dalek::Keypair::from_bytes(&file)?
    } else {
        let kp = ed25519_dalek::Keypair::generate(&mut rand_old::OsRng);
        fs::write(key_path, kp.to_bytes())?;
        kp
    };
    let key = Arc::new(KeyPair { kp });

    let (sk_rx, sk_tx) = crate::services::ftclive::init(pool.clone()).await;
    let obs_tx = crate::services::obs::init(pool.clone()).await;
    let obs_scene_tx = crate::controllers::obs_scenes::init(pool.clone()).await;

    Ok(SharedState {
        pool,
        key,
        sk_rx,
        sk_tx,
        obs_tx,
        obs_scene_tx,
    })
}

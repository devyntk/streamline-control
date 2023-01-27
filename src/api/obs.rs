use axum::{
    extract::State,
    middleware::from_fn_with_state,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde_json::json;
use tokio::sync::oneshot::channel;

use crate::{
    api::{auth::auth, state::SharedState, AppError},
    controllers::obs_scenes::messages::{OBSSceneRequest, SetMapping},
    services::obs::messages::{ConnectInfo, OBSRequestMessage},
};

pub fn obs_routes(state: SharedState) -> Router {
    Router::new()
        .route("/config", get(get_config).post(set_config))
        .route("/connect", get(connect))
        .route("/scenes", get(get_scenes))
        .route("/scene", post(set_scene))
        .route("/mapping", get(get_mapping).post(set_mapping))
        .layer(from_fn_with_state(state.clone(), auth))
        .with_state(state)
}

async fn get_config(
    State(obs_tx): State<flume::Sender<OBSRequestMessage>>,
) -> Result<impl IntoResponse, AppError> {
    let (tx, rx) = channel();
    obs_tx.send(OBSRequestMessage::GetConnectInfo(tx))?;
    Ok(Json(json!(rx.await??)))
}

async fn set_config(
    State(obs_tx): State<flume::Sender<OBSRequestMessage>>,
    Json(info): Json<ConnectInfo>,
) -> Result<impl IntoResponse, AppError> {
    let (tx, rx) = channel();
    obs_tx.send(OBSRequestMessage::SetConnectInfo(info, tx))?;
    Ok(Json(json!(rx.await??)))
}

async fn connect(
    State(obs_tx): State<flume::Sender<OBSRequestMessage>>,
) -> Result<impl IntoResponse, AppError> {
    let (tx, rx) = channel();
    obs_tx.send(OBSRequestMessage::Connect(tx))?;
    Ok(Json(json!(rx.await??)))
}

async fn get_scenes(
    State(obs_tx): State<flume::Sender<OBSRequestMessage>>,
) -> Result<impl IntoResponse, AppError> {
    let (tx, rx) = channel();
    obs_tx.send(OBSRequestMessage::GetScenes(tx))?;
    Ok(Json(json!(rx.await??)))
}

async fn set_scene(
    State(obs_tx): State<flume::Sender<OBSRequestMessage>>,
    body: String,
) -> Result<impl IntoResponse, AppError> {
    let (tx, rx) = channel();
    obs_tx.send(OBSRequestMessage::SetScene(body, tx))?;
    Ok(Json(json!(rx.await??)))
}

async fn get_mapping(
    State(obs_scene_tx): State<flume::Sender<OBSSceneRequest>>,
) -> Result<impl IntoResponse, AppError> {
    let (tx, rx) = channel();
    obs_scene_tx.send(OBSSceneRequest::GetSceneMapping(tx))?;
    Ok(Json(json!(rx.await??)))
}

async fn set_mapping(
    State(obs_scene_tx): State<flume::Sender<OBSSceneRequest>>,
    Json(body): Json<SetMapping>,
) -> Result<impl IntoResponse, AppError> {
    let (tx, rx) = channel();
    obs_scene_tx.send(OBSSceneRequest::SetSceneMapping {
        new_mapping: body,
        sender: tx,
    })?;
    Ok(Json(json!(rx.await??)))
}

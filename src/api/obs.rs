use crate::api::auth::auth;
use crate::api::state::SharedState;
use crate::api::AppError;
use crate::services::obs::messages::{ConnectInfo, OBSRequestMessage};
use axum::extract::State;
use axum::middleware::from_fn_with_state;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use serde_json::json;
use tokio::sync::oneshot::channel;

pub fn obs_routes(state: SharedState) -> Router {
    Router::new()
        .route("/config", get(get_config).post(set_config))
        .route("/connect", get(connect))
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
    body: String,
) -> Result<impl IntoResponse, AppError> {
    let info: ConnectInfo = serde_json::from_str(&body)?;
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

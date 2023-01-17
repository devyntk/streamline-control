use crate::api::auth::auth;
use crate::api::state::SharedState;
use crate::api::AppError;
use crate::services::ftclive::messages::{FTCLiveBroadcastMessage, FTCLiveRequest};
use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    middleware::from_fn_with_state,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde_json::json;
use tokio::sync::oneshot::channel;
use url::Url;

pub fn scoring_routes(state: SharedState) -> Router {
    Router::new()
        .route("/events", get(get_events))
        .route("/url", post(set_scoring_url))
        .route("/event", post(set_event))
        .route("/subscribe", get(connect_scoring_ws))
        .layer(from_fn_with_state(state.clone(), auth))
        .with_state(state)
}

async fn get_events(
    State(sk_rx): State<flume::Sender<FTCLiveRequest>>,
) -> Result<impl IntoResponse, AppError> {
    let (tx, rx) = channel();
    sk_rx.send(FTCLiveRequest::GetEvents(tx))?;
    Ok(Json(json!(rx.await??)))
}

async fn set_scoring_url(
    State(sk_rx): State<flume::Sender<FTCLiveRequest>>,
    body: String,
) -> Result<impl IntoResponse, AppError> {
    let base_url = Url::parse("http://localhost")?;
    let mut url = Url::options().base_url(Some(&base_url)).parse(&*body)?;
    url.set_path("/");

    let (tx, rx) = channel();
    sk_rx.send(FTCLiveRequest::SetUrl(url, tx))?;
    Ok(Json(json!(rx.await??)))
}

async fn set_event(
    State(sk_rx): State<flume::Sender<FTCLiveRequest>>,
    body: String,
) -> Result<impl IntoResponse, AppError> {
    let (tx, rx) = channel();
    sk_rx.send(FTCLiveRequest::SetEventCode(body, tx))?;
    Ok(Json(json!(rx.await??)))
}

async fn connect_scoring_ws(
    State(sk_rx): State<flume::Sender<FTCLiveRequest>>,
    State(sk_tx): State<flume::Receiver<FTCLiveBroadcastMessage>>,
    ws: WebSocketUpgrade,
) -> Result<impl IntoResponse, AppError> {
    let (tx, rx) = channel();
    sk_rx.send(FTCLiveRequest::CheckWebsocket(tx))?;
    if !rx.await?? {
        log::info!("FTCLive WS not connected, sending message to start it");
        let (ws_tx, ws_rx) = channel();
        sk_rx.send(FTCLiveRequest::ConnectWebsocket(ws_tx))?;
        ws_rx.await??;
        log::info!("FTCLive WS started");
    };
    Ok(ws.on_upgrade(|socket| handle_scoring_ws(socket, sk_tx)))
}

async fn handle_scoring_ws(mut socket: WebSocket, sk_tx: flume::Receiver<FTCLiveBroadcastMessage>) {
    loop {
        let sk_msg = sk_tx.recv_async().await.unwrap();
        socket
            .send(Message::Text(serde_json::to_string(&sk_msg).unwrap()))
            .await;
    }
}

use crate::api::auth::auth;
use crate::api::state::SharedState;
use crate::api::AppError;
use crate::services::ftclive::messages::FTCLiveRequest;
use axum::extract::State;
use axum::middleware::from_fn_with_state;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde_json::json;
use tokio::sync::oneshot::channel;
use url::Url;

pub fn scoring_routes(state: SharedState) -> Router {
    Router::new()
        .route("/events", get(get_events))
        .route("/url", post(set_scoring_url))
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

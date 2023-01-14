#[cfg(feature = "with-gui")]
use crate::gui::{SERVER_START, UPDATE_STATUS};
#[cfg(not(feature = "with-gui"))]
use crate::ExtEventSink;
use crate::{
    api,
    interfaces::ftclive::messages::{FTCLiveBroadcastMessage, FTCLiveRequest},
    APP_INFO,
};
use app_dirs2::{app_root, AppDataType};
use axum::{
    body::{boxed, Full},
    http::{header, StatusCode, Uri},
    response::Response,
};
use biscuit_auth::KeyPair;
#[cfg(feature = "with-gui")]
use druid::{ExtEventSink, Target};
use log::{error, info};
use port_scanner::local_ports_available;
use rust_embed::RustEmbed;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::{Pool, Sqlite};
use std::fs;
use std::sync::Arc;
use tokio::sync::{broadcast, mpsc, oneshot};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

#[derive(Clone, Debug)]
pub struct SharedState {
    pub pool: Pool<Sqlite>,
    pub key: Arc<KeyPair>,
    pub sk_rx: flume::Receiver<FTCLiveBroadcastMessage>,
    pub sk_tx: flume::Sender<FTCLiveRequest>,
}

#[derive(Clone, Debug, Copy)]
pub enum SharedMessage {
    Exit,
}

#[allow(unused_variables)]
fn publish_error(error: String, sink: Option<ExtEventSink>) {
    error!("{}", error);
    #[cfg(feature = "with-gui")]
    if sink.is_some() {
        sink.unwrap()
            .submit_command(UPDATE_STATUS, error, Target::Auto)
            .expect("Error sending GUI update");
    }
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

#[tokio::main]
pub async fn start_server(sink: Option<ExtEventSink>, rx: oneshot::Receiver<()>) {
    let pool = match get_pool().await {
        Ok(pool) => pool,
        Err(err) => {
            publish_error(err.to_string(), sink);
            return;
        }
    };

    let (tx, _rx) = broadcast::channel(10);

    let mut key_path = app_root(AppDataType::UserConfig, &APP_INFO).unwrap();
    key_path.push("key");

    let kp: ed25519_dalek::Keypair = if key_path.is_file() {
        let file = fs::read(key_path).unwrap();
        ed25519_dalek::Keypair::from_bytes(&file).unwrap()
    } else {
        let kp = ed25519_dalek::Keypair::generate(&mut rand_old::OsRng);
        fs::write(key_path, kp.to_bytes()).unwrap();
        kp
    };
    let key = Arc::new(KeyPair { kp });

    let (sk_rx, sk_tx) = crate::interfaces::ftclive::init().await;

    let state = SharedState {
        pool,
        key,
        sk_rx,
        sk_tx,
    };

    let app = axum::Router::new()
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
        .nest("/api", api::api_routes(state.clone()))
        .fallback(static_handler);

    let mut ports: Vec<u16> = local_ports_available(vec![3030, 8888, 8080, 80]);

    let port = match ports.pop() {
        Some(num) => num,
        None => {
            publish_error("No open ports to bind to!".to_string(), sink);
            return;
        }
    };
    let addr = &([127, 0, 0, 1], port).into();
    let tx_clone = tx.clone();
    let server = axum::Server::bind(addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(async move {
            rx.await.ok();
            tx_clone
                .clone()
                .send(SharedMessage::Exit)
                .expect("Error sending exit message");
        });

    info!("Server started at http://localhost:{}", port);
    #[cfg(feature = "with-gui")]
    if sink.is_some() {
        sink.unwrap()
            .submit_command(SERVER_START, *addr, Target::Auto)
            .expect("Error sending GUI update");
    }
    server.await.expect("Cannot await server");

    state.clone().pool.close().await;
}

#[derive(RustEmbed)]
#[folder = "frontend/dist"]
struct Assets;

async fn static_handler(uri: Uri) -> Response {
    let path = uri.path().trim_start_matches('/');

    if path.is_empty() || path == "index.html" {
        return index_html().await;
    }

    match Assets::get(path) {
        Some(content) => {
            let body = boxed(Full::from(content.data));
            let mime = mime_guess::from_path(path).first_or_octet_stream();

            Response::builder()
                .header(header::CONTENT_TYPE, mime.as_ref())
                .body(body)
                .unwrap()
        }
        None => {
            if path.contains('.') {
                return not_found().await;
            }

            index_html().await
        }
    }
}

async fn index_html() -> Response {
    match Assets::get("index.html") {
        Some(content) => {
            let body = boxed(Full::from(content.data));

            Response::builder()
                .header(header::CONTENT_TYPE, "text/html")
                .body(body)
                .unwrap()
        }
        None => not_found().await,
    }
}

async fn not_found() -> Response {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(boxed(Full::from("404")))
        .unwrap()
}

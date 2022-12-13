#[cfg(feature = "with-gui")]
use crate::gui::{SERVER_START, UPDATE_STATUS};
use crate::APP_INFO;
use app_dirs2::{app_root, AppDataType};
#[cfg(feature = "with-gui")]
use druid::{ExtEventSink, Target};
#[cfg(not(feature = "with-gui"))]
use crate::ExtEventSink;
use log::{debug, error, info};
use axum_extra::routing::SpaRouter;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use port_scanner::local_ports_available;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use tokio::sync::{oneshot, broadcast};
use sqlx::{Pool, Sqlite};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct SharedState {
    pub pool: Pool<Sqlite>,
    pub tx: broadcast::Sender<SharedMessage>,
}

#[derive(Clone, Debug, Copy)]
pub enum SharedMessage {
    Exit
}

fn publish_error(error: String, sink: Option<ExtEventSink>) {
    error!("{}", error);
    #[cfg(feature = "with-gui")]
    if sink.is_some() {
        sink.unwrap()
            .submit_command(UPDATE_STATUS, error, Target::Auto)
            .expect("Error sending GUI update");
    }
}

#[tokio::main]
pub async fn start_server(sink: Option<ExtEventSink>, rx: oneshot::Receiver<()>) {
    let mut db_url = match app_root(AppDataType::UserConfig, &APP_INFO) {
        Ok(db) => {
            debug!("Location of user data: {:#?}", db);
            db
        }
        Err(error) => {
            publish_error(error.to_string(), sink);
            return;
        }
    };
    db_url.push("streamline.db");

    let db_options = SqliteConnectOptions::new().filename(db_url).create_if_missing(true);

    let pool = match SqlitePoolOptions::new().connect_with(db_options).await {
        Ok(pool) => pool,
        Err(error) => {
            publish_error(error.to_string(), sink);
            return;
        }
    };

    let (tx, _rx) = broadcast::channel(10);

    let state = Arc::new(SharedState { pool, tx });

    let app = axum::Router::new()
        .merge(SpaRouter::new("/assets", "web_dist"))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

    let mut ports: Vec<u16> = local_ports_available(vec![3030, 8888, 8080, 80]);

    let port = match ports.pop() {
        Some(num) => num,
        None => {
            publish_error("No open ports to bind to!".to_string(), sink);
            return;
        }
    };
    let addr = &([127, 0, 0, 1], port).into();
    let tx_clone = state.tx.clone();
    let server =
        axum::Server::bind(addr).serve(app.into_make_service())
            .with_graceful_shutdown(
                async move {
                    rx.await.ok();
                    tx_clone.clone().send(SharedMessage::Exit)
                        .expect("Error sending exit message");
                },
            );

    info!("Server started at http://localhost:{}", port);
    #[cfg(feature = "with-gui")]
    if sink.is_some() {
        sink.unwrap()
            .submit_command(SERVER_START, *addr, Target::Auto)
            .expect("Error sending GUI update");
    }
    server.await.expect("Cannot await server");

    state.pool.close().await;
}

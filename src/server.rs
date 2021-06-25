#[cfg(feature = "with-gui")]
use crate::gui::{SERVER_START, UPDATE_STATUS};
use crate::APP_INFO;
use app_dirs2::{app_root, AppDataType};
#[cfg(feature = "with-gui")]
use druid::{ExtEventSink, Target};
#[cfg(not(feature="with-gui"))]
use crate::ExtEventSink;
use log::{debug, error, info};
use port_scanner::local_ports_available;
use rusqlite::Connection;
use rust_embed::RustEmbed;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use tokio::sync::{oneshot, broadcast};
use warp::http::{header::HeaderValue, Response};
use warp::{path, path::Tail, path::peek, Filter, Rejection, Reply, get};
use tokio::task::spawn_blocking;
use crate::api::{api_filter, handle_api_rejection};
use warp::filters::path::Peek;
use sqlx::{Pool, Sqlite};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct SharedState {
    pub pool: Pool<Sqlite>,
    pub tx: broadcast::Sender<SharedMessage>
}

#[derive(Clone, Debug, Copy)]
pub enum SharedMessage {
    Exit
}

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!();
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

    let mut db_config = match Connection::open(db_url.clone()) {
        Ok(db) => db,
        Err(error) => {
            publish_error(error.to_string(), sink);
            return;
        }
    };

    let migrations_run = spawn_blocking(move || {embedded::migrations::runner().run(&mut db_config)});
    match migrations_run.await {
        Ok(migrations) => {
            match migrations {
                Ok(_) => {},
                Err(error) => {
                    publish_error(error.to_string(), sink);
                    return;
                }
            }
        }
        Err(error) => {
            publish_error(error.to_string(), sink);
            return;
        }
    }

    let db_options = SqliteConnectOptions::new().filename(db_url);

    let pool = match SqlitePoolOptions::new().connect_with(db_options).await {
        Ok(pool) => pool,
        Err(error) => {
            publish_error(error.to_string(), sink);
            return;
        }
    };

    let (tx , _rx) = broadcast::channel(10);

    let state = Arc::new(SharedState{ pool, tx });

    let static_route = path("static").and(path::tail()).and_then(static_serve);
    let dist_route = path("dist").and(path::tail()).and_then(dist_serve);

    let app = get().and(peek()).and_then(serve_index);

    let routes = static_route
        .or(dist_route)
        .or(api_filter(state.clone()))
        .or(app);
        // .recover(handle_api_rejection);

    let mut ports: Vec<u16> = local_ports_available(vec![3030, 8888, 8080, 80]);

    let port = match ports.pop() {
        Some(num) => num,
        None => {
            publish_error("No open ports to bind to!".to_string(), sink);
            return;
        }
    };

    let tx_clone = state.tx.clone();
    let server_result =
        warp::serve(routes)
            .try_bind_with_graceful_shutdown(
                ([127, 0, 0, 1], port),
                async move {
                    rx.await.ok();
                    tx_clone.clone().send(SharedMessage::Exit)
                        .expect("Error sending exit message");
                }
        );

    let server_handle = match server_result {
        Ok((addr, future)) => {
            info!("Server started at http://{}", addr);
            #[cfg(feature = "with-gui")]
            if sink.is_some() {
                sink.unwrap()
                    .submit_command(SERVER_START, addr, Target::Auto)
                    .expect("Error sending GUI update");
            }
            tokio::task::spawn(future)
        }
        Err(error) => {
            publish_error(error.to_string(), sink);
            return;
        }
    };

    server_handle.await.expect("Error starting server thread");

    state.pool.close().await;
}

#[derive(RustEmbed)]
#[folder = "static/"]
struct Asset;

#[derive(RustEmbed)]
#[folder = "frontend/dist/"]
struct Dist;

async fn dist_serve(path: Tail) -> Result<impl Reply, Rejection> {
    let asset = Dist::get(path.as_str()).ok_or_else(warp::reject::not_found)?;
    let mime = mime_guess::from_path(path.as_str()).first_or_octet_stream();

    let mut res: Response<std::borrow::Cow<'_, [u8]>> = Response::new(asset.into());
    res.headers_mut().insert(
        "content-type",
        HeaderValue::from_str(mime.as_ref()).unwrap(),
    );
    Ok(res)
}

async fn serve_index(path: Peek) -> Result<impl Reply, Rejection> {
    // if path.segments().next() == Some("api") {
    //     return Err(warp::reject::not_found())
    // }
    serve_impl("index.html")
}

async fn static_serve(path: Tail) -> Result<impl Reply, Rejection> {
    serve_impl(path.as_str())
}

fn serve_impl(path: &str) -> Result<impl Reply, Rejection> {
    let asset = Asset::get(path).ok_or_else(warp::reject::not_found)?;
    let mime = mime_guess::from_path(path).first_or_octet_stream();

    let mut res: Response<std::borrow::Cow<'_, [u8]>> = Response::new(asset.into());
    res.headers_mut().insert(
        "content-type",
        HeaderValue::from_str(mime.as_ref()).unwrap(),
    );
    Ok(res)
}

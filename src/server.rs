use warp::{Filter, path, Reply, Rejection, http::Uri, path::Tail};
use druid::ExtEventSink;
use tokio::sync::oneshot::Receiver;
use crate::gui::{SERVER_START, UPDATE_STATUS};
use port_scanner::local_ports_available;
use warp::http::{Response, header::HeaderValue};
use rust_embed::RustEmbed;
use refinery::{Config, ConfigDbType, embed_migrations, migrate_from_config};
use app_dirs2::{app_root, AppDataType};
use crate::APP_INFO;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!();
}
#[tokio::main]
pub async fn start_server(sink: ExtEventSink, rx: Receiver<()>) {

    let mut db_url = app_root(AppDataType::UserConfig, &APP_INFO)
        .expect("Unable to get DB location");
    db_url.push("streamline.db");

    let mut db_config = Config::new(ConfigDbType::Sqlite)
        .set_db_path(db_url.as_os_str().to_str().expect("Unable to parse DB string"));
    // migrate_from_config(&db_config, false, true, true, migrations::runner);

    // GET /hello/warp => 200 OK with body "Hello,include!(concat!(env!("OUT_DIR"), "/templates.rs")); warp!"
    let static_route = path("static").and(path::tail()).and_then(static_serve);
    let dist_route = path("dist").and(path::tail()).and_then(dist_serve);

    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    let index = warp::path::end()
        .map(|| {
            warp::redirect(Uri::from_static("/app"))
        });

    let app = warp::path("app").and_then(serve_index);

    let routes = static_route.or(dist_route).or(hello).or(index).or(app);

    let mut ports: Vec<u16> = local_ports_available(vec![3030,8888,8080,80]);

    let server_result = warp::serve(routes)
        .try_bind_with_graceful_shutdown(
            ([127, 0, 0, 1], ports.pop().expect("No socket address to bind to")), async {
            rx.await.ok();
        });

    let server_handle;
    match server_result {
        Ok((addr, future)) => {
            server_handle = Some(tokio::task::spawn(future));
            sink.submit_command(SERVER_START, addr, None)
                .expect("Error sending GUI update");
        }
        Err(error) => {
            sink.submit_command(UPDATE_STATUS, error.to_string(), None)
                .expect("Error sending GUI update");
            return;
        }
    }

    server_handle.expect("No server future found").await
        .expect("Error starting server thread");

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
    res.headers_mut().insert("content-type", HeaderValue::from_str(mime.as_ref()).unwrap());
    Ok(res)
}

async fn serve_index() -> Result<impl Reply, Rejection> {
    serve_impl("index.html")
}

async fn static_serve(path: Tail) -> Result<impl Reply, Rejection> {
    serve_impl(path.as_str())
}

fn serve_impl(path: &str) -> Result<impl Reply, Rejection> {
    let asset = Asset::get(path).ok_or_else(warp::reject::not_found)?;
    let mime = mime_guess::from_path(path).first_or_octet_stream();

    let mut res: Response<std::borrow::Cow<'_, [u8]>> = Response::new(asset.into());
    res.headers_mut().insert("content-type", HeaderValue::from_str(mime.as_ref()).unwrap());
    Ok(res)
}
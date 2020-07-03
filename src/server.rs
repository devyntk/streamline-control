use warp::{Filter, path, Reply, Rejection};
use druid::ExtEventSink;
use tokio::sync::oneshot::Receiver;
use crate::gui::{SERVER_START, UPDATE_STATUS};
use port_scanner::local_ports_available;
use templates::{statics::StaticFile, RenderRucte};
use warp::http::{Response, StatusCode};

#[tokio::main]
pub async fn start_server(sink: ExtEventSink, rx: Receiver<()>) {

    // GET /hello/warp => 200 OK with body "Hello,include!(concat!(env!("OUT_DIR"), "/templates.rs")); warp!"
    let static_route = path("static").and(path::param()).and_then(static_file);

    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    let index = path::end().and_then(home_page);

    let routes = static_route.or(hello).or(index);

    let mut ports: Vec<u16> = local_ports_available(vec![3030,8080,80]);

    let server_result = warp::serve(routes)
        .try_bind_with_graceful_shutdown(
            ([127, 0, 0, 1], ports.pop().expect("No socket address to bind to")), async {
            rx.await.ok();
        });

    let mut server_handle = None;
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

/// Handler for static files.
/// Create a response from the file data with a correct content type
/// and a far expires header (or a 404 if the file does not exist).
async fn static_file(name: String) -> Result<impl Reply, Rejection> {
    if let Some(data) = StaticFile::get(&name) {
        Ok(Response::builder()
            .status(StatusCode::OK)
            .header("content-type", data.mime.as_ref())
            .body(data.content))
    } else {
        println!("Static file {} not found", name);
        Err(warp::reject::not_found())
    }
}

/// Home page handler; just render a template with some arguments.
async fn home_page() -> Result<impl Reply, Rejection> {
    Response::builder().html(|o| {
        templates::index(o, &[("first", 3), ("second", 7), ("third", 2)])
    })
}

include!(concat!(env!("OUT_DIR"), "/templates.rs"));
use warp::Filter;
use druid::ExtEventSink;
use tokio::sync::oneshot::{channel, Receiver};

use crate::gui::RECV_QUIT_SENDER;

#[tokio::main]
pub async fn start_server(sink: ExtEventSink, rx: Receiver<()>) {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    let port = 8080;

    let (addr, server) = warp::serve(hello)
        .bind_with_graceful_shutdown(([127, 0, 0, 1], port), async {
            rx.await.ok();
        });

    // Spawn the server into a runtime
    tokio::task::spawn(server);
}
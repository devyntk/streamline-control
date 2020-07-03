use warp::Filter;
use druid::ExtEventSink;
use tokio::sync::oneshot::Receiver;
use crate::gui::{SERVER_START, UPDATE_STATUS};
use port_scanner::local_ports_available;

#[tokio::main]
pub async fn start_server(sink: ExtEventSink, rx: Receiver<()>) {

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let routes = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

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
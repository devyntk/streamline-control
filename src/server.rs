use warp::{Filter, Error};
use druid::ExtEventSink;
use tokio::sync::oneshot::Receiver;
use std::net::SocketAddr;
use tokio::macros::support::Future;
use crate::gui::SERVER_START;
use std::thread::JoinHandle;
use port_scanner::local_ports_available;

#[tokio::main]
pub async fn start_server(sink: ExtEventSink, rx: Receiver<()>) {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    let ports: Vec<u16> = vec![3030,8080,80];
    let mut ports: Vec<u16> = local_ports_available(ports);
    println!("{:#?}", ports);

    let server_result = warp::serve(hello)
        .try_bind_with_graceful_shutdown(
            ([127, 0, 0, 1], ports.pop().expect("No socket address to bind to")), async {
            rx.await.ok();
        });

    let mut server_handle = None;
    match server_result {
        Ok((addr, future)) => {
            server_handle = Some(tokio::task::spawn(future));
            sink.submit_command(SERVER_START, addr, None);
        }
        Err(error) => {

        }
    }

    server_handle.expect("No server future found").await;

}
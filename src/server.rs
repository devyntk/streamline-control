#[cfg(feature = "with-gui")]
use druid::{ExtEventSink, Target};
use log::{error, info};
use port_scanner::local_ports_available;
use tokio::sync::{broadcast, oneshot};

use crate::api::{app_router, state::get_state};
#[cfg(feature = "with-gui")]
use crate::gui::{SERVER_START, UPDATE_STATUS};
#[cfg(not(feature = "with-gui"))]
use crate::ExtEventSink;

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

#[tokio::main]
pub async fn start_server(sink: Option<ExtEventSink>, rx: oneshot::Receiver<()>) {
    let state = match get_state().await {
        Ok(res) => res,
        Err(err) => {
            publish_error(err.to_string(), sink);
            return;
        }
    };

    let (tx, _rx) = broadcast::channel(10);

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
        .serve(app_router(state.clone()).into_make_service())
        .with_graceful_shutdown(async move {
            rx.await.ok();
            tx_clone
                .clone()
                .send(SharedMessage::Exit)
                .expect("Error sending exit message");
        });

    info!("Server started at http://localhost:{}", port);
    println!("Server started!");
    #[cfg(feature = "with-gui")]
    if sink.is_some() {
        sink.unwrap()
            .submit_command(SERVER_START, *addr, Target::Auto)
            .expect("Error sending GUI update");
    }
    server.await.expect("Cannot await server");

    state.clone().pool.close().await;
}

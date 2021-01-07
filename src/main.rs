// For the `cargo_crate_version!` macro
#[macro_use]
extern crate self_update;

use crate::server::start_server;
use app_dirs2::*;
use clap::{crate_authors, crate_version, App, Arg};
use flexi_logger::{Duplicate, Logger};
use std::thread;
use tokio::sync::oneshot::channel;

mod api;
mod dns;
mod gui;
mod server;
mod update;

pub const APP_INFO: AppInfo = AppInfo {
    name: "Streamline Control",
    author: "devyntk",
};

fn main() {
    let matches = App::new("Streamline Control")
        .version(crate_version!())
        .author(crate_authors!())
        .arg(
            Arg::new("headless")
                .about("Skip running the GUI and run headless")
                .short('c')
                .long("console"),
        )
        .get_matches();

    let log_dir =
        app_dir(AppDataType::UserConfig, &APP_INFO, "log/").expect("Error getting log directory");

    Logger::with_env_or_str("debug")
        .log_to_file()
        .directory(log_dir)
        .duplicate_to_stdout(Duplicate::Debug)
        .start()
        .unwrap();

    let dnsargs = dns::Opt {
        multicast_group: "239.255.70.77".parse().unwrap(),
        host: "0.0.0.0".parse().unwrap(),
        port: 50765,
        command: dns::Command::Broadcast {
            name: Some("streamline-control".parse().unwrap()),
        },
    };
    thread::spawn(move || dns::run(dnsargs));

    if matches.is_present("headless") {
        let (_, rx) = channel();
        start_server(None, rx);
    } else {
        gui::run_ui();
    }
}

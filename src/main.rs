// For the `cargo_crate_version!` macro
#[macro_use]
extern crate self_update;

use crate::server::start_server;
use app_dirs2::*;
use clap::{crate_authors, crate_version, App, Arg};
use flexi_logger::{Duplicate, Logger};
use std::thread;
use tokio::sync::oneshot::channel;
use log::info;

#[cfg(debug_assertions)]
use std::process::{Command, Stdio};
#[cfg(debug_assertions)]
use std::env::var_os;
#[cfg(debug_assertions)]
use std::path::Path;
use std::process::Child;

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
    let app = App::new("Streamline Control")
        .version(crate_version!())
        .author(crate_authors!())
        .arg(
            Arg::new("headless")
                .about("Skip running the GUI and run headless")
                .short('c')
                .long("console")
        );

    #[cfg(debug_assertions)]
    let app = app.arg(
        Arg::new("watch-frontend")
            .about("Launch a frontend watch process")
            .requires("headless")
            .long("watch-frontend")
        )
        .arg(
        Arg::new("silent-watch")
            .about("Silence output from the frontend watch process")
            .requires("watch-frontend")
            .long("silent-watch")
        );

    let matches = app.get_matches();

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
        let (tx, rx) = channel();
        let mut shutdown_signal = Some(tx);

        let mut frontend: Option<Child> = None;
        #[cfg(debug_assertions)] {
            let dir_string = var_os("CARGO_MANIFEST_DIR").expect("Can't find cargo dir. Make sure to run this using 'cargo run'.");

            if matches.is_present("watch-frontend") {
                info!("Launching frontend watcher.");
                let mut process = Command::new("yarn");

                let mut process = process.current_dir(Path::new(&dir_string).join("frontend"))
                    .args(&["run", "watch"]);

                if matches.is_present("silent-watch"){
                    info!("Frontend watcher will be silent. Remove the '--silent-watch' flag to see output");
                    process = process.stdout(Stdio::null())
                        .stderr(Stdio::null());
                }

                frontend = Some(process.spawn().expect("Could not launch frontend watcher"))

            }
        }

        ctrlc::set_handler(move || {
            #[cfg(debug_assertions)] {
                let frontend = frontend.take();
                if let Some(mut ps) = frontend {
                    ps.kill().expect("Unable to kill frontned");
                }
            }

            let signal = shutdown_signal.take();
            signal.unwrap().send(()).expect("Error sending shutdown signal");
        }).expect("Error setting Ctrl-C handler");

        start_server(None, rx);
    } else {
        gui::run_ui();
    }
}

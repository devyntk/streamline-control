// For the `cargo_crate_version!` macro
#[macro_use]
extern crate self_update;

use app_dirs2::*;
use flexi_logger::{Logger, Duplicate};
use std::thread;

mod gui;
mod update;
mod server;
mod dns;

pub const APP_INFO: AppInfo = AppInfo{name: "Streamline Control", author: "devyntk"};

fn main() {

    let log_dir = app_dir(AppDataType::UserConfig, &APP_INFO, "log/");

    Logger::with_env_or_str("debug")
        .log_to_file()
        .directory(log_dir.expect("Error getting log directory"))
        .duplicate_to_stdout(Duplicate::Debug)
        .start()
        .unwrap();
    let dnsargs = dns::Opt {
        multicast_group: "239.255.70.77".parse().unwrap(),
        host: "0.0.0.0".parse().unwrap(),
        port: 50765,
        command: dns::Command::Broadcast { name: Some("streamline-control".parse().unwrap()) }
    };
    thread::spawn(move || {
        dns::run(dnsargs)
    });
    gui::run_ui();
}

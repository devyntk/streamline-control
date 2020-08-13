// For the `cargo_crate_version!` macro
#[macro_use]
extern crate self_update;
use astro_dnssd;

use app_dirs2::*;
use flexi_logger::{Logger, Duplicate};

mod gui;
mod update;
mod server;

pub const APP_INFO: AppInfo = AppInfo{name: "Streamline Control", author: "bkeeneykid"};

fn main() {
    let mut dns = astro_dnssd::register::DNSServiceBuilder::new("streamline");
    dns = dns.with_port(80);
    dns = dns.with_name("control");
    dns.build().expect("Error creating DNS entry!");

    let log_dir = app_dir(AppDataType::UserConfig, &APP_INFO, "log/");

    Logger::with_env_or_str("debug")
        .log_to_file()
        .directory(log_dir.expect("Error getting log directory"))
        .duplicate_to_stdout(Duplicate::Debug)
        .start()
        .unwrap();

    gui::run_ui();
}

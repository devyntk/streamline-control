// For the `cargo_crate_version!` macro
#[macro_use]
extern crate self_update;

use app_dirs2::*;
use flexi_logger::{Logger, Duplicate};

mod gui;
mod update;
mod server;

pub const APP_INFO: AppInfo = AppInfo{name: "Streamline Control", author: "bkeeneykid"};

fn main() {

    let log_dir = app_dir(AppDataType::UserConfig, &APP_INFO, "log/");

    Logger::with_env_or_str("debug")
        .log_to_file()
        .directory(log_dir.expect("Error getting log directory"))
        .duplicate_to_stdout(Duplicate::Debug)
        .start()
        .unwrap();

    gui::run_ui();
}

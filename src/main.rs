use std::thread;

use app_dirs2::*;
use clap::Parser;
#[cfg(feature = "with-gui")]
use druid::ExtEventSink;
use flexi_logger::{Duplicate, FileSpec, Logger};
use tokio::sync::oneshot::channel;

use crate::{
    commands::{handle_command, Commands},
    server::start_server,
};

mod api;
mod commands;
mod config;
mod controllers;
mod dns;
#[cfg(feature = "with-gui")]
mod gui;
mod server;
mod services;
#[cfg(feature = "with-gui")]
mod update;

pub const APP_INFO: AppInfo = AppInfo {
    name: "Streamline Control",
    author: "devyntk",
};

#[derive(Parser, Clone)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[cfg(feature = "with-gui")]
    /// Disables desktop GUI
    #[arg(long)]
    headless: bool,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

fn main() {
    let cli = Cli::parse();

    start_logging_dns(cli.clone());

    if let Some(command) = cli.command {
        handle_command(command).unwrap();
        return;
    }

    #[cfg(not(feature = "with-gui"))]
    return start_headless();

    #[cfg(feature = "with-gui")]
    if cli.headless {
        start_headless();
    } else {
        gui::run_ui();
    }
}

fn start_headless() {
    let (tx, rx) = channel();
    let mut shutdown_signal = Some(tx);

    ctrlc::set_handler(move || {
        let signal = shutdown_signal.take();
        signal
            .unwrap()
            .send(())
            .expect("Error sending shutdown signal");
    })
    .expect("Error setting Ctrl-C handler");

    let sink: Option<ExtEventSink> = None;
    start_server(sink, rx);
}

fn start_logging_dns(cli: Cli) {
    let log_dir =
        app_dir(AppDataType::UserConfig, &APP_INFO, "log/").expect("Error getting log directory");

    let log_level = match cli.debug {
        0 => "warning",
        1 => "info",
        2 => "debug",
        3 => "trace",
        _ => {
            panic!("Unknown debug level")
        }
    };

    Logger::try_with_env_or_str(log_level)
        .expect("Cannot setup logger!")
        .log_to_file(FileSpec::default().directory(log_dir))
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
}

#[cfg(not(feature = "with-gui"))]
pub struct ExtEventSink {}

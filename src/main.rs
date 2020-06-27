// For the `cargo_crate_version!` macro
#[macro_use]
extern crate self_update;

mod gui;
mod update;
mod server;

fn main() {
    gui::run_ui();
}

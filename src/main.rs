// For the `cargo_crate_version!` macro
#[macro_use]
extern crate self_update;

mod gui;
mod update;

fn main() {
    gui::run_ui();
}
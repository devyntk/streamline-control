// For the `cargo_crate_version!` macro
#[macro_use]
extern crate self_update;

use iui::prelude::*;

mod gui;
mod update;

fn main() {
    // Initialize the UI library
    let ui = UI::init().expect("Couldn't initialize UI library");
    // Create a window into which controls can be placed

    gui::build_ui(&ui);

    // Run the application
    ui.main();
}
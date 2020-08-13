// For the `cargo_crate_version!` macro
#[macro_use]
extern crate self_update;
use astro_dnssd;
mod gui;
mod update;

fn main() {
    let mut dns = astro_dnssd::register::DNSServiceBuilder::new("streamline");
    dns = dns.with_port(80);
    dns = dns.with_name("control");
    dns.build().expect("Error creating DNS entry!");
    gui::run_ui();
}

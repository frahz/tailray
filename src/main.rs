mod clipboard;
mod pkexec;
mod svg;
mod tailscale;
mod tray;

use crate::tray::utils::start_tray_service;
use std::process::exit;
use std::thread::park;

fn main() {
    // initialize logger
    env_logger::init();

    // start tray service
    match start_tray_service() {
        Ok(()) => println!("Tray service started successfully."),
        Err(e) => {
            eprintln!("Failed to start the tray service: {e}");
            exit(1);
        }
    }

    // keep the main thread alive
    park();
}

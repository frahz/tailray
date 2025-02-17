mod clipboard;
mod pkexec;
mod svg;
mod tailscale;
mod tray;

use log::{error, info, trace};

use crate::tailscale::status::Status;
use crate::tray::utils::start_tray_service;
use std::process::exit;

fn main() {
    // initialize logger
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    // start tray service
    let handle = match start_tray_service() {
        Ok(handle) => handle,
        Err(e) => {
            error!("Failed to start the tray service: {e}");
            exit(1);
        }
    };
    info!("Tray service started successfully.");

    // keep the main thread alive
    let mut state = false;
    loop {
        let ctx = Status::get_current().expect("success");
        let update_state = ctx.status.is_up();
        trace!("Tailscale Status = [{}]", update_state);
        if update_state != state {
            handle.update(|tray| {
                tray.ctx = ctx;
            });
        }
        state = update_state;
        std::thread::sleep(std::time::Duration::from_secs(5));
    }
}

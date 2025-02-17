use crate::tailscale::status::Status;
use crate::tray::menu::SysTray;
use ksni::blocking::{Handle, TrayMethods};
use std::error::Error;

type TrayServiceError = Box<dyn Error>;

pub fn start_tray_service() -> Result<Handle<SysTray>, TrayServiceError> {
    let status =
        Status::get_current().map_err(|e| format!("Failed to update Tailscale status: {e}"))?;

    let tray = SysTray { ctx: status };
    let handle = tray
        .spawn()
        .map_err(|e| format!("Failed to spawn Tray implementation: {e}"))?;

    Ok(handle)
}

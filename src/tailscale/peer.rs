use crate::clipboard::{copy, get};
use log::{error, info};
use notify_rust::Notification;
use thiserror::Error;

type Result<T> = std::result::Result<T, CopyPeerIpError>;

#[derive(Error, Debug)]
pub enum CopyPeerIpError {
    #[error("clipboard operation failed")]
    Clipboard(#[from] arboard::Error),

    #[error("notification failed")]
    Notification(#[from] notify_rust::error::Error),
}

pub fn check_peer_ip(peer_ip: &str) {
    if peer_ip.is_empty() {
        error!("No peer IP.");
    } else {
        info!("Peer IP: {peer_ip}");
    }
}

pub fn copy_peer_ip(peer_ip: &str, notif_body: &str, host: bool) -> Result<()> {
    check_peer_ip(peer_ip);

    copy(peer_ip)?;

    // Get IP from clipboard to verify
    let clip_ip = get()?;

    // Create summary for host/peer
    let summary = format!("Copied {} IP address", if host { "host" } else { "peer" });

    // log success
    info!("{summary} {clip_ip} to the clipboard");

    // send a notification through dbus
    Notification::new()
        .summary(&summary)
        .body(notif_body)
        .icon("tailscale")
        .show()?;

    Ok(())
}

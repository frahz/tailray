use crate::tailscale::types::{BackendState, ExitNodeStatus, Machine, TailnetStatus, User};
use crate::tray::menu::Context;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, process::Command};
use thiserror::Error;

type Result<T> = std::result::Result<T, StatusError>;

#[derive(Error, Debug)]
pub enum StatusError {
    #[error("tailscale command failed")]
    Command(#[from] std::io::Error),

    #[error("failed to decode tailscale command response")]
    CommandDecode(#[from] std::string::FromUtf8Error),

    #[error("failed to fetch tailscale status")]
    FetchFailed,

    #[error(transparent)]
    Deserialize(#[from] serde_json::Error),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Status {
    #[serde(rename(deserialize = "Version"))]
    version: String,
    #[serde(rename(deserialize = "TUN"))]
    tun: bool,
    #[serde(rename(deserialize = "BackendState"))]
    backend_state: BackendState,
    #[serde(rename(deserialize = "Self"))]
    pub this_machine: Machine,
    #[serde(rename(deserialize = "ExitNodeStatus"))]
    pub exit_node_status: Option<ExitNodeStatus>,
    #[serde(rename(deserialize = "MagicDNSSuffix"))]
    magic_dnssuffix: String,
    #[serde(rename(deserialize = "CurrentTailnet"))]
    current_tailnet: TailnetStatus,
    #[serde(rename(deserialize = "Peer"))]
    pub peers: HashMap<String, Machine>,
    #[serde(rename(deserialize = "User"))]
    user: HashMap<String, User>,
}

impl Status {
    pub fn get_current() -> Result<Context> {
        let status = Self::get()?;

        Ok(Context {
            ip: status.this_machine.ips[0].to_string(),
            status,
        })
    }

    fn get() -> Result<Status> {
        let status_json = Self::get_json()?;
        let mut status: Status = serde_json::from_str(&status_json)?;
        let dnssuffix = &status.current_tailnet.magic_dnssuffix;

        status.this_machine.set_display_name(dnssuffix);
        status
            .peers
            .values_mut()
            .for_each(|m| m.set_display_name(dnssuffix));

        Ok(status)
    }

    fn get_json() -> Result<String> {
        let output = Command::new("tailscale")
            .arg("status")
            .arg("--json")
            .output()?;

        if output.status.success() {
            let stdout = String::from_utf8(output.stdout)?;
            Ok(stdout)
        } else {
            Err(StatusError::FetchFailed)
        }
    }

    // TODO: mutex
    pub fn is_up(&self) -> bool {
        self.backend_state == BackendState::Running
    }
}

use crate::tailscale::utils::{sanitize_hostname, trim_suffix};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum PeerKind {
    DNSName(String),
    HostName(String),
}

impl Default for PeerKind {
    fn default() -> Self {
        Self::HostName("default".to_owned())
    }
}

impl Display for PeerKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::DNSName(d) => write!(f, "{d}"),
            Self::HostName(h) => write!(f, "{h}"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum BackendState {
    NoState,
    NeedsLogin,
    NeedsMachineAuth,
    Stopped,
    Starting,
    Running,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TailnetStatus {
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
    #[serde(rename(deserialize = "MagicDNSSuffix"))]
    pub magic_dnssuffix: String,
    #[serde(rename(deserialize = "MagicDNSEnabled"))]
    pub magic_dnsenabled: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Machine {
    #[serde(skip)]
    pub display_name: PeerKind,
    #[serde(rename(deserialize = "DNSName"))]
    pub dns_name: String,
    #[serde(rename(deserialize = "HostName"))]
    pub hostname: String,
    #[serde(rename(deserialize = "TailscaleIPs"))]
    pub ips: Vec<String>,
}

impl Machine {
    pub fn set_display_name(&mut self, dns_suffix: &str) {
        let dns_name = trim_suffix(&self.dns_name, dns_suffix);

        if dns_name.is_empty() {
            self.display_name = PeerKind::DNSName(sanitize_hostname(&self.hostname));
        } else {
            self.display_name = PeerKind::HostName(dns_name);
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(rename(deserialize = "ID"))]
    id: u64,
    #[serde(rename(deserialize = "LoginName"))]
    login_name: String,
    #[serde(rename(deserialize = "DisplayName"))]
    display_name: String,
    #[serde(rename(deserialize = "ProfilePicURL"))]
    profile_pic_url: String,
    #[serde(rename(deserialize = "Roles"))]
    roles: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExitNodeStatus {
    #[serde(rename(deserialize = "ID"))]
    pub id: String,
    #[serde(rename(deserialize = "Online"))]
    pub online: bool,
    #[serde(rename(deserialize = "TailscaleIPs"))]
    pub ips: Vec<String>,
}

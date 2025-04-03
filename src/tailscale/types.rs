use crate::tailscale::utils::{sanitize_hostname, trim_suffix};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::net::IpAddr;

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
        match self {
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
pub struct StableNodeId(String);

#[derive(Serialize, Deserialize, Debug)]
pub struct UserId(u64);

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
    #[serde(rename(deserialize = "ID"))]
    pub id: StableNodeId,
    #[serde(rename(deserialize = "DNSName"))]
    pub dns_name: String,
    #[serde(rename(deserialize = "HostName"))]
    pub host_name: String,
    #[serde(rename(deserialize = "TailscaleIPs"))]
    pub ips: Vec<IpAddr>,
    #[serde(rename(deserialize = "Online"))]
    pub online: bool,
    #[serde(rename(deserialize = "ExitNode"))]
    pub exit_node: bool,
    #[serde(rename(deserialize = "ExitNodeOption"))]
    pub exit_node_option: bool,
}

impl Machine {
    pub fn set_display_name(&mut self, dns_suffix: &str) {
        let dns_name = trim_suffix(&self.dns_name, dns_suffix);

        // TODO: look into why host_name is PeerKind::DNSName
        self.display_name = if dns_name.is_empty() {
            PeerKind::DNSName(sanitize_hostname(&self.host_name))
        } else {
            PeerKind::HostName(dns_name)
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(rename(deserialize = "ID"))]
    id: UserId,
    #[serde(rename(deserialize = "LoginName"))]
    login_name: String,
    #[serde(rename(deserialize = "DisplayName"))]
    display_name: String,
    #[serde(rename(deserialize = "ProfilePicURL"))]
    profile_pic_url: String,
    #[serde(rename(deserialize = "Roles"))]
    roles: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExitNodeStatus {
    #[serde(rename(deserialize = "ID"))]
    pub id: StableNodeId,
    #[serde(rename(deserialize = "Online"))]
    pub online: bool,
    #[serde(rename(deserialize = "TailscaleIPs"))]
    pub ips: Vec<String>,
}

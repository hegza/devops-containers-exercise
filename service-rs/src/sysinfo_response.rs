mod disk;
mod net;
mod ps_ax;
mod uptime;

use std::{net::IpAddr, path::PathBuf};

use serde::{Deserialize, Serialize};
use sysinfo::{Disks, Networks};

#[derive(Serialize)]
pub(crate) struct Response {
    service1: SysInfo,
    service2: SysInfo,
}

impl Response {
    pub fn from_infos(service1: SysInfo, service2: SysInfo) -> Self {
        Self { service1, service2 }
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct SysInfo {
    /// IP address information, each can be IPv4 or IPv6
    ips: Vec<std::net::IpAddr>,
    /// Output of `ps -ax`
    ps_ax: String,
    /// Available disk space in bytes
    available_bytes: u64,
    /// Time since last boot in seconds
    uptime_secs: u64,
}

impl SysInfo {
    pub fn from_local_info() -> Self {
        let ips = net::get_local_ips();
        let pss = ps_ax::ps_ax();
        let available_bytes = disk::available_bytes();
        let uptime_secs = uptime::uptime_secs();

        Self {
            ips,
            ps_ax: pss,
            available_bytes,
            uptime_secs,
        }
    }

    /// Generate mocked up data, e.g., to emulate server-go
    #[cfg(test)]
    pub fn from_mock() -> Self {
        Self {
            ips: get_local_ips(),
            ps_ax: String::new(),
            available_bytes: 1024,
            uptime_secs: 42,
        }
    }
}

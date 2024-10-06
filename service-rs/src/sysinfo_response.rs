use std::{
    path::PathBuf,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use serde::Serialize;
use sysinfo::{Disks, Networks, Pid, System};

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

#[derive(Serialize)]
pub(crate) struct SysInfo {
    /// IP address information, each can be IPv4 or IPv6
    ip_addrs: Vec<std::net::IpAddr>,
    /// List of running processes
    pss: Vec<(sysinfo::Pid, String)>,
    /// Available disk space in bytes
    available_bytes: u64,
    /// Time since last boot in seconds
    uptime_secs: u64,
}

impl SysInfo {
    pub fn from_local_info() -> Self {
        let ip_addrs = get_local_ips();
        let pss = processes();
        let available_bytes = available_bytes();
        let uptime_secs = uptime_secs();

        Self {
            ip_addrs,
            pss,
            available_bytes,
            uptime_secs,
        }
    }

    pub fn from_mock() -> Self {
        Self {
            ip_addrs: get_local_ips(),
            pss: vec![(Pid::from(1), String::from("dummy process"))],
            available_bytes: 1024,
            uptime_secs: 42,
        }
    }
}

fn uptime_secs() -> u64 {
    let boot = UNIX_EPOCH + Duration::from_secs(System::boot_time());
    let now = SystemTime::now();
    let since = now.duration_since(boot).unwrap();
    since.as_secs()
}

fn processes() -> Vec<(sysinfo::Pid, String)> {
    let mut sys = System::new();
    sys.refresh_processes(sysinfo::ProcessesToUpdate::All);
    sys.processes()
        .into_iter()
        .map(|(pid, proc)| (*pid, proc.name().to_string_lossy().into_owned()))
        .collect()
}

fn available_bytes() -> u64 {
    let disks = Disks::new_with_refreshed_list();

    let root_disk = disks
        .into_iter()
        .find(|d| d.mount_point() == PathBuf::from("/"))
        .expect("root is not mounted");
    // Returns available space in bytes
    root_disk.available_space()
}

fn get_local_ips() -> Vec<std::net::IpAddr> {
    let networks = Networks::new_with_refreshed_list();

    let ips = networks
        .into_iter()
        // Ignore docker networks
        .filter(|(name, _)| !name.starts_with("docker"))
        .flat_map(|(_, net)| {
            // Safety: there's only one network remaining
            let ip_nets = net.ip_networks();
            ip_nets.iter().map(|ip_net| ip_net.addr)
        })
        // Ignore loopback networks
        .filter(|addr| !addr.is_loopback())
        .collect::<Vec<_>>();

    if ips.len() == 0 {
        panic!("No IP available");
    }

    // Safety: we have made sure there is exactly one IP
    ips
}

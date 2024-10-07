use std::{
    net::IpAddr,
    path::PathBuf,
    process,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use serde::Serialize;
use sysinfo::{Disks, Networks, System};

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
        let ips = get_local_ips();
        let pss = ps_ax();
        let available_bytes = available_bytes();
        let uptime_secs = uptime_secs();

        Self {
            ips,
            ps_ax: pss,
            available_bytes,
            uptime_secs,
        }
    }

    pub fn from_mock() -> Self {
        Self {
            ips: get_local_ips(),
            ps_ax: String::new(),
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

/// Retrieve output for the `ps -ax` command
fn ps_ax() -> String {
    let out = process::Command::new("ps")
        .arg("-ax")
        .output()
        .expect("Failed to execute command");

    if !out.status.success() {
        eprintln!("Command failed to execute");
    }

    // Convert command output into UTF-8
    let stdout = String::from_utf8(out.stdout).expect("`ps -ax` output is not valid UTF-8");

    stdout.to_owned()
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

fn is_internal(ip: &IpAddr) -> bool {
    match ip {
        // Check if the IPv4 address falls in the private ranges (10.0.0.0/8,
        // 172.16.0.0/12, and 192.168.0.0/16) as specified by RFC 1918 Address
        // Allocation for Private Internets
        IpAddr::V4(ipv4) => ipv4.is_private(),
        // Check if the IPv6 address is within the fc00::/7 range (Unique Local
        // Addresses) as specified by RFC 4193
        IpAddr::V6(ipv6) => ipv6.segments()[0] & 0xfe00 == 0xfc00,
    }
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
        // Ignore loopback networks & internal IPs
        .filter(|addr| !addr.is_loopback() && !is_internal(addr))
        .collect::<Vec<_>>();

    if ips.len() == 0 {
        panic!("No IP available");
    }

    ips
}

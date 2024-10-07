use std::net::IpAddr;
use sysinfo::Networks;

/// Returns all local, public IPs
pub(crate) fn get_local_ips() -> Vec<IpAddr> {
    let networks = Networks::new_with_refreshed_list();
    let ips = networks.into_iter().flat_map(|(_, net)| {
        let ip_nets = net.ip_networks();
        ip_nets.into_iter().map(|ip_net| ip_net.addr)
    });

    // Return public IPs
    ips
        // Ignore loopback networks & internal IPs
        .filter(|addr| !addr.is_loopback() && !is_internal(addr))
        .collect::<Vec<_>>()
}

/// Returns whether `ip` is a private/internal IP address or not, based on RFC 1918 and 4193
fn is_internal(ip: &IpAddr) -> bool {
    match ip {
        // Check if the IPv4 address falls in the private ranges (10.0.0.0/8, 172.16.0.0/12, and
        // 192.168.0.0/16) as specified by RFC 1918 Address Allocation for Private Internets
        IpAddr::V4(ipv4) => ipv4.is_private(),
        // Check if the IPv6 address is within the fc00::/7 range (Unique Local Addresses) as
        // specified by RFC 4193
        IpAddr::V6(ipv6) => ipv6.segments()[0] & 0xfe00 == 0xfc00,
    }
}

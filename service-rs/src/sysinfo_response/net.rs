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
        .filter(|addr| !addr.is_loopback())
        .collect::<Vec<_>>()
}

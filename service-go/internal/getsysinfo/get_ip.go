package getsysinfo

import (
	"fmt"
	"net"
)

// Determine whether the IPv4 address is private as specified by RFC 1918
func isPrivateRfc1918(ip net.IP) bool {
	privateIPv4Ranges := []net.IPNet{
		{IP: net.IPv4(10, 0, 0, 0), Mask: net.CIDRMask(8, 32)},
		{IP: net.IPv4(172, 16, 0, 0), Mask: net.CIDRMask(12, 32)},
		{IP: net.IPv4(192, 168, 0, 0), Mask: net.CIDRMask(16, 32)},
	}

	for _, r := range privateIPv4Ranges {
		if r.Contains(ip) {
			return true
		}
	}

	return false
}

// Determine whether the IPv6 address is private / Unique Local Address as
// specified by RFC 4193
func isPrivateRfc4193(ip net.IP) bool {
	privateIPv6Ranges := []net.IPNet{
		{IP: net.ParseIP("fc00::"), Mask: net.CIDRMask(7, 128)},
	}

	for _, r := range privateIPv6Ranges {
		if r.Contains(ip) {
			return true
		}
	}

	return false
}

// Check if an IP is an internal (private) IP address
func isInternalIP(ip net.IP) bool {
	// Check for IPv4 private ranges
	if isPrivateRfc1918(ip) {
		return true
	}

	// Check for IPv6 ULA (Unique Local Address) ranges
	return isPrivateRfc4193(ip)
}

// Get public IP addresses for all network interfaces
func GetIpAddress() ([]string, error) {
	addrs, err := net.InterfaceAddrs()
	if err != nil {
		return nil, err
	}

	var ips []string
	for _, addr := range addrs {
		// Check for IP address and ensure it's not a loopback nor an internal IP address
		if ipNet, ok := addr.(*net.IPNet); ok && !ipNet.IP.IsLoopback() && !isInternalIP(ipNet.IP) {
			ips = append(ips, ipNet.IP.String())
		}
	}

	if len(ips) == 0 {
		return nil, fmt.Errorf("no IP addresses found")
	}

	return ips, nil
}

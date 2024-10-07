package getsysinfo

import (
	"fmt"
	"net"
)

// Get public IP addresses for all network interfaces
func GetIpAddress() ([]string, error) {
	addrs, err := net.InterfaceAddrs()
	if err != nil {
		return nil, err
	}

	var ips []string
	for _, addr := range addrs {
		// Check for IP address and ensure it's not a loopback nor an internal IP address
		if ipNet, ok := addr.(*net.IPNet); ok && !ipNet.IP.IsLoopback() {
			ips = append(ips, ipNet.IP.String())
		}
	}

	if len(ips) == 0 {
		return nil, fmt.Errorf("no IP addresses found")
	}

	return ips, nil
}

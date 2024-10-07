package getsysinfo

import (
	"time"

	"github.com/shirou/gopsutil/host"
)

// Get system uptime in seconds
func GetUptime() (uint64, error) {
	uptime, err := host.Uptime()
	if err != nil {
		return 0, err
	}

	return uint64(time.Duration(uptime)), nil
}

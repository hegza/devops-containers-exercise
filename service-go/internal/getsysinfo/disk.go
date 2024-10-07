package getsysinfo

import (
	"fmt"

	"github.com/shirou/gopsutil/disk"
)

// Gets the number of bytes available on rootfs
func GetAvailableBytesOnRoot() (uint64, error) {
	// Get all available partitions
	partitions, err := disk.Partitions(false)
	if err != nil {
		return 0, fmt.Errorf("error retrieving partitions: %s", err)
	}

	// Find rootfs and return free bytes
	for _, partition := range partitions {
		if partition.Mountpoint == "/" {
			usage, err := disk.Usage(partition.Mountpoint)
			if err != nil {
				return 0, fmt.Errorf("error retrieving disk usage for root partition: %s", err)
			}

			return usage.Free, nil
		}
	}

	return 0, fmt.Errorf("root partition not found")
}

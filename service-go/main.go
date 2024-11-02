package main

import (
	"encoding/json"
	"fmt"
	"internal/getsysinfo"
	"net/http"
	"os"
	"strconv"
)

const ListenAddr = "0.0.0.0"
const DefaultListenPort = 3000

// Get an environment variable with a default value if it's missing
func GetEnvUint64(key string, defaultValue uint64) uint64 {
	value, exists := os.LookupEnv(key)
	if !exists {
		return defaultValue
	}
	u64, err := strconv.ParseUint(value, 10, 64)
	if err != nil {
		panic(fmt.Sprintf("%s must be a number, %s", key, err))
	}
	return u64
}

func main() {
	http.HandleFunc("/", handler)

	listenAddr := fmt.Sprintf("%s:%d", ListenAddr, GetEnvUint64("PORT", DefaultListenPort))

	// Listen for TCP
	fmt.Println("Starting server at", listenAddr)
	go func() {
		err := http.ListenAndServe(listenAddr, nil)
		if err != nil {
			fmt.Println("Error starting server:", err)
		}
	}()
	fmt.Println("Server listening for TCP at", listenAddr)

	select {}
}

// Struct to hold the IP address in JSON format
type SysInfo struct {
	IPs            []string `json:"ips"`
	Ps_ax          string   `json:"ps_ax"`
	AvailableBytes uint64   `json:"available_bytes"`
	UptimeSecs     uint64   `json:"uptime_secs"`
}

func handler(w http.ResponseWriter, r *http.Request) {
	ips, err := getsysinfo.GetIpAddress()
	if err != nil {
		http.Error(w, fmt.Sprintf("Unable to get IP address: %s", err), http.StatusInternalServerError)
		return
	}

	ps_ax, err := getsysinfo.GetPsAx()
	if err != nil {
		http.Error(w, fmt.Sprintf("Unable to get output of `ps -ax`: %s", err), http.StatusInternalServerError)
		return
	}

	available_bytes, err := getsysinfo.GetAvailableBytesOnRoot()
	if err != nil {
		http.Error(w, fmt.Sprintf("Unable to get available bytes on root partition: %s", err), http.StatusInternalServerError)
		return
	}

	uptime_secs, err := getsysinfo.GetUptime()
	if err != nil {
		http.Error(w, fmt.Sprintf("Unable to get uptime: %s", err), http.StatusInternalServerError)
		return
	}

	// Create a response object
	response := SysInfo{IPs: ips, Ps_ax: ps_ax, AvailableBytes: available_bytes, UptimeSecs: uptime_secs}

	// Convert the response to JSON
	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(response)

}

package getsysinfo

import "os/exec"

// Executes `ps -ax` and returns the output
func GetPsAx() (string, error) {
	// Execute `ps -ax`
	cmd := exec.Command("ps", "-ax")

	// Capture output
	output, err := cmd.Output()
	if err != nil {
		return "", err
	}

	return string(output), nil
}

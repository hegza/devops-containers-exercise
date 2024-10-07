use std::process::Command;

/// Retrieve output for the `ps -ax` command
pub(crate) fn ps_ax() -> String {
    let out = Command::new("ps")
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

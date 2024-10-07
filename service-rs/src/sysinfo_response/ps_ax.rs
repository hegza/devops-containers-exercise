use std::process::Command;

/// Retrieve output for the `ps -ax` command (be it successful or not)
pub(crate) fn ps_ax() -> String {
    let out = Command::new("ps")
        .arg("-ax")
        .output()
        .expect("failed to execute `ps -ax` on host");

    if !out.status.success() {
        eprintln!("command failed to execute");
    }

    // Convert command output into UTF-8 before sending over the network. This could reasonably fail
    // on some systems but we'll worry about that only if it ever actually happens. Proper handling
    // of this would complicate the entire software stack since the entire internet is not ready for
    // communicating about non UTF-8 streams.
    String::from_utf8(out.stdout).expect("`ps -ax` output is not valid UTF-8")
}

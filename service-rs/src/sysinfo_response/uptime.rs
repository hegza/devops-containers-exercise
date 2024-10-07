use std::time::{Duration, SystemTime, UNIX_EPOCH};

use sysinfo::System;

/// Returns system uptime
pub(crate) fn uptime_secs() -> u64 {
    let boot = UNIX_EPOCH + Duration::from_secs(System::boot_time());
    let now = SystemTime::now();
    let since = now.duration_since(boot).unwrap();
    since.as_secs()
}

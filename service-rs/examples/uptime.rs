use std::time::{Duration, SystemTime, UNIX_EPOCH};

use sysinfo::System;

fn main() {
    let boot = UNIX_EPOCH + Duration::from_secs(System::boot_time());
    let now = SystemTime::now();

    let since = now.duration_since(boot).unwrap();
    println!("Time since boot: {} s", since.as_secs());
}

use std::path::PathBuf;

use sysinfo::Disks;

fn main() {
    // We display all disks' information:
    println!("=> disks:");
    let disks = Disks::new_with_refreshed_list();
    for disk in &disks {
        println!(
            "Disk: {}, available space: {} B",
            disk.name().to_str().unwrap(),
            disk.available_space()
        );
    }

    println!("Bytes on rootfs: {}", available_bytes());
}

fn available_bytes() -> u64 {
    let disks = Disks::new_with_refreshed_list();

    let root_disk = disks
        .into_iter()
        .find(|d| d.mount_point() == PathBuf::from("/"))
        // This will never happen as long as this application is run on a Linux machine (container
        // or not)
        .expect("rootfs is not mounted");

    // Returns available space in bytes
    root_disk.available_space()
}

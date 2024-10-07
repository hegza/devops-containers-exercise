use std::path::PathBuf;

use sysinfo::Disks;

pub(crate) fn available_bytes() -> u64 {
    let disks = Disks::new_with_refreshed_list();

    let root_disk = disks
        .into_iter()
        .find(|d| d.mount_point() == PathBuf::from("/"))
        .expect("rootfs is not mounted");

    // Returns available space in bytes
    root_disk.available_space()
}

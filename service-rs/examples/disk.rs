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
}

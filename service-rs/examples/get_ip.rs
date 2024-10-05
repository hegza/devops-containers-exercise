use sysinfo::Networks;

fn main() {
    let networks = Networks::new_with_refreshed_list();
    for (interface_name, network) in &networks {
        println!("If: {}, IP Networks: {{", interface_name);
        for ipnet in network.ip_networks() {
            println!("\t{:?}", ipnet);
        }
        println!("}}");
    }
}

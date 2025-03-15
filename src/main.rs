
#![allow(dead_code, unused, non_snake_case, unused_mut, unused)]  // Apply the allow directly in the test file
use subnet2db2::{core::setup_targets::*, interfaces::ip::IpInterface};

pub fn main() {
    let mut interface_ip = IpInterface::new();
    let ip_inserted: i64 = set_targets().unwrap().try_into().unwrap();
    let ip_in_db = interface_ip.len().unwrap();

    if ip_inserted == ip_in_db {
        println!("Number of ip generated match exploded hostfile.");
    }
}

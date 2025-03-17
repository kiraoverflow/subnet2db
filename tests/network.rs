use std::net::{Ipv4Addr, IpAddr};
use subnet2db2::network::ping::packet::{ICMP};


#[test]
fn test_send_icmp() {

    let mut packet = ICMP::new();
    let target = "8.8.8.8".parse::<Ipv4Addr>().expect("Invalid IP address");
    let dest = IpAddr::V4(target);
    println!("Target : {:?}", dest); 

    let ttl = packet.send(dest).expect("");
    println!("Received ICMP response with TTL: {}", ttl); // TTL might be 64

    let resp = packet.listen_for_response().expect("");
    //println!("Response : {:?}", resp);
    //packet.listen_for_response(receiver, destination);
}

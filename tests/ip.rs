use subnet2db2::interfaces::ip::IpInterface;
use subnet2db2::interfaces::subnet::SubnetInterface;
use subnet2db2::interfaces::target::TargetInterface;

#[test]
pub fn test_get_by_subnet_target_id() {
    let mut target = TargetInterface::new();

    let target = target.get_by_country(
        String::from("Syria").clone()
    ).unwrap();

    let mut subnet = SubnetInterface::new();

    let subnet = subnet.get_by_target(
        &target
    ).unwrap();

    let subnet = subnet.first().unwrap();

    let mut ip = IpInterface::new();

    let ips = ip.get_by_subnet_target_id(
        &subnet
    ).unwrap();

    for ip in ips {
        let s = format!("Country : {}\nSubnet : {}\nIp : {:?}",  
            target.country,
            subnet.cidr,
            ip.v4.unwrap(),
        );
        println!("{}", s);
    };

}

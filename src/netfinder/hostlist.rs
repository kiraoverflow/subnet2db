use anyhow::Error;
use std::collections::HashMap;
use ipnetwork::IpNetwork;
use std::net::IpAddr;
use std::fs;


pub struct Hostlist {
    pub cidr_file : String
}
impl Hostlist {
    pub fn load_subnets_from_str(
        &self
    ) ->  Result<HashMap<String, Vec<IpAddr>>, Error>{
        let contents = fs::read_to_string(&self.cidr_file)?;
        let lines: Vec<String> = contents.lines().map(String::from).collect();

        let mut subnet_map: HashMap<String, Vec<IpAddr>> = HashMap::new();
        let subnet_map = self.parse_network(lines.clone());

        return Ok(subnet_map);
    }

    pub fn parse_network(
        &self,
        subnets : Vec<String>
    ) -> HashMap<String, Vec<IpAddr>>{
        let mut subnet_map: HashMap<String, Vec<IpAddr>> = HashMap::new();
        for subnet in &subnets {
            if let Ok(network) = subnet.parse::<IpNetwork>() {
                let ips: Vec<IpAddr> = network.iter().collect();
                subnet_map.insert(subnet.to_string(), ips);
            }
        };
        return subnet_map;
    }
}

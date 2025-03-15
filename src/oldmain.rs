use ipnetwork::IpNetwork;
use std::net::IpAddr;
use std::collections::HashMap;
use std::fs;
use anyhow::Error;
use serde::{Serialize, Deserialize};
use diesel::prelude::*;

use subnet2db::schema::syria_subnets;
use subnet2db::schema::syria_subnets::dsl::subnet as dsl_subnet;
use subnet2db::database::*;

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = syria_subnets)]// Ensure this matches your Diesel schema
#[diesel(check_for_backend(dieesel::pg::Pg))]
pub struct SubnetEntry {
    pub subnet: String,
    pub ip: Vec<String>,
}


pub struct HostList{
    cidr_file : String
}

impl HostList {
    pub fn new(cidr_file: String) -> Self {
        HostList {
            cidr_file
        }
    }
    pub fn load_subnets_from_str(
        &self,
        cidr_file : String
    ) -> Result<Vec<String>, Error>{
        let contents = fs::read_to_string(cidr_file)?;
        let lines: Vec<String> = contents.lines().map(String::from).collect();
        return Ok(lines);
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

    pub fn entries(&self) -> Result<Vec<SubnetEntry>, Error> {
        let mut entries = Vec::new();
        let subnets = self.load_subnets_from_str(self.cidr_file.clone()).unwrap();
        let subnet_map = self.parse_network(subnets);
        for (subnet, ips) in subnet_map.iter() {
            entries.push(SubnetEntry {
                subnet: subnet.clone(),
                ip: ips.iter().map(|ip| ip.to_string()).collect(), // Convert IPs to a comma-separated string
            }); 
        };
        Ok(entries)
    }

    pub fn insert(&self, subnet: SubnetEntry) -> Result<uSize, diesel::result::Error> {
        let mut conn = db.pool.get().map_err(|_| "Failed to get DB connection").unwrap();

        diesel::insert_into(syria_subnets::table)
            .values(&subnet)
            .execute(conn)
    }
    pub fn print(&self, subnet : SubnetEntry) {
        for subnets in entries {
            for ip in subnets.ip {
                let s = format!("subnet : {}, ip : {}\n", subnets.subnet, ip);
                println!("{}",s);
            }
        }
    }
} 
pub fn main() {
    let hosts = HostList {
        cidr_file : "cidr.txt".to_string()
    };
    let entries = hosts.entries().unwrap();

        
}

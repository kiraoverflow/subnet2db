use crate::{
    interfaces::{
        port::PortInterface, 
        subnet::SubnetInterface, 
        target::TargetInterface,
        ip::IpInterface,
    }, 
    models::{
        ip::{Ip, IpInsert}, 
        subnet::{Subnet, SubnetInsert}, 
        target::{Target, TargetInsert},
    }, 
    schema::{
        ip::dsl::{self as dsl_ip, ip as ip_table}, 
        subnet::dsl::{self as dsl_subnet, subnet as subnet_table}, 
        target::dsl::{self as dsl_target, target as target_table},
    },
};
use crate::database::Db;
use crate::netfinder::hostlist::Hostlist;
use crate::helpers::progress::*;

use ipnetwork::IpNetwork;
use std::net::IpAddr;
use std::collections::HashMap;
use std::fs;
use anyhow::{anyhow, Error};
use serde::{Serialize, Deserialize};
use diesel::prelude::*;

use serde_json::json;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

pub fn set_targets() -> Result<usize, Error>{
    env_logger::init();


    let mut target_interface = TargetInterface::new();
    let mut subnet_interface = SubnetInterface::new();
    let mut ip_interface = IpInterface::new();


    let target_insert = target_interface.insert(
        String::from("Syria")
    );

    let  hostlist = Hostlist {
        cidr_file : "cidr.txt".to_string(),
    };
    
    if let Ok(subnets ) = hostlist.load_subnets_from_str() {
        let total_ips: usize = subnets.iter().map(|(_, ips)| ips.len()).sum(); // Calculate total IPs to insert
        let mut cnt = 0;  // Counter for the progress
        for (sub, ipvec)  in subnets {

            let subnet = subnet_interface.insert(
                target_insert.id,
                sub
            );

            for ip in ipvec {

                if let Some(ips) = ip_interface.insert(subnet.id, Some(ip.to_string()), None) {

                    cnt += 1;       
                    display_progress_bar(cnt, total_ips);
                } else {
                    break;
                }
                //pintln!("{}", json!(&ips));
            //println!("{}", json!(&target_insert));

            //println!("{}", json!(&subnet));
            }
        }
        println!("Inserted {} IP", cnt);
        return Ok(cnt);
    } else {
        return Err(anyhow!("Unknown error"));
    };
}

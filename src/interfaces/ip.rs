use crate::{
    logging::Logger, models::{
        ip::{Ip, IpInsert}, subnet::{Subnet, SubnetInsert}
    }, schema::{
        ip::dsl::{self as dsl_ip, ip as ip_table}, subnet::dsl::{self as dsl_subnet, subnet as subnet_table}
    }
};

use crate::database::Db;


use diesel::{prelude::*, r2d2::{ConnectionManager, PooledConnection}};
use diesel::BelongingToDsl;

pub struct IpInterface(PooledConnection<ConnectionManager<PgConnection>>, Logger);

pub type Error = diesel::result::Error;

impl IpInterface { 
     pub fn new() -> IpInterface {
        let db = Db::connect().expect("Cant connect");

        let mut conn = db.pool.get().map_err(|_| "Failed to get DB connection").unwrap();
        
        let logger = Logger::new();
        IpInterface(
            conn,
            logger,
        )
    } 
    
    pub fn len(&mut self) -> Result<i64, Error> {
        ip_table 
            .count()  // Count the number of rows
            .get_result::<i64>(&mut self.0) // Execute query and get result as i64
    }

    pub fn get_all(
        &mut self
    ) -> Result<Vec<Ip>, Error> {
        ip_table
            .load::<Ip>(&mut self.0) // Load all rows into a Vec<Subnet>
    }
    

    pub fn get_by_id(
        &mut self,
        id : i32
    ) -> Result<Ip, Error> {
        ip_table 
            .filter(dsl_ip::id.eq(id))
            .first::<Ip>(&mut self.0)
    }

    pub fn get_by_subnet(
        &mut self,
        subnet: &Subnet
    ) -> Result<Vec<Ip>, Error> {
        
        Ip::belonging_to(subnet)  // 👈 This uses the `belongs_to` relationship
            .load::<Ip>(&mut self.0)
    }

    pub fn get_by_subnet_target_id(
        &mut self,
        subnet : &Subnet
    ) -> Result<Vec<Ip>, Error>{

        Ip::belonging_to(subnet)  // Uses the relationship defined in Diesel
            .inner_join(subnet_table.on(dsl_ip::subnet_id.eq(dsl_subnet::id))) // Join with subnet
            .filter(dsl_subnet::target_id.eq(subnet.target_id)) // Ensure subnet's target_id matches
            .select(Ip::as_select())  // Select only Ip fields
            .load::<Ip>(&mut self.0)
    }

    pub fn insert(
        &mut self,
        subnet_id : i32, 
        v4 : Option<String>,
        v6 : Option<String>
    ) -> Option<Ip> {
        let ip_object : IpInsert = IpInsert {
            subnet_id,
            v4,
            v6,
        };
        diesel::insert_into(ip_table)
            .values(&ip_object)
            .on_conflict(dsl_ip::v4)
            .do_nothing()
            .get_result::<Ip>(&mut self.0)
            .ok()
            .or_else(|| {
                self.1.info(String::from("Insertion skipped: IP already exists or another error occurred."));
                None // Return None instead of panicking
            })
               /*
        diesel::insert_into(ip_table)
            .values(&ip_object)
            .on_conflict(dsl_ip::v4)
            //.do_update()
            //.set(dsl_ip::v4.eq(&ip_object.v4))
            .do_nothing()
            .get_result::<Ip>(&mut self.0)
            .unwrap_or_else(|e| {
                self.1.info(String::from("Inserting ip failed."));
                panic!("{}", e);
            })
        */
    }

}


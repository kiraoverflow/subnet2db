use crate::{
    logging::Logger, models::{
        subnet::{Subnet, SubnetInsert},
        target::Target,
    }, schema::subnet::dsl::{self as dsl_subnet, subnet as subnet_table}
};

use crate::database::Db;
use diesel::{prelude::*, r2d2::{ConnectionManager, PooledConnection}};
use diesel::BelongingToDsl;

pub struct SubnetInterface(
    PooledConnection<ConnectionManager<PgConnection>>,
    Logger,
);

pub type Error = diesel::result::Error;

impl SubnetInterface { 
     pub fn new() -> SubnetInterface {
        let db = Db::connect().expect("Cant connect");

        let mut conn = db.pool.get().map_err(|_| "Failed to get DB connection").unwrap();
        let logger = Logger::new();
        SubnetInterface(
            conn,
            logger,
        )
    }

    pub fn get_all(
        &mut self
    ) -> Result<Vec<Subnet>, Error> {
        subnet_table
            .load::<Subnet>(&mut self.0) // Load all rows into a Vec<Subnet>
    }
    
    pub fn len(
        &mut self
    ) -> Result<i64, Error> {
        subnet_table
            .count()  // Count the number of rows
            .get_result::<i64>(&mut self.0) // Execute query and get result as i64
    }

    pub fn get_by_id(
        &mut self,
        id : i32
    ) -> Result<Subnet, Error> {
        let db = Db::connect().expect("Cant connect");

        let mut conn = db.pool.get().map_err(|_| "Failed to get DB connection").unwrap();
        subnet_table 
            .filter(dsl_subnet::id.eq(id))
            .first::<Subnet>(&mut self.0)
    }

    pub fn get_by_cidr(
        &mut self,
        cidr : String
    ) -> Result<Subnet, Error> {
        let db = Db::connect().expect("Cant connect");

        let mut conn = db.pool.get().map_err(|_| "Failed to get DB connection").unwrap();
        
        subnet_table 
            .filter(dsl_subnet::cidr.eq(&cidr))
            .first::<Subnet>(&mut self.0) 
    }

    pub fn get_by_target(
        &mut self,
        target: &Target
    ) -> Result<Vec<Subnet>, Error> {
        let db = Db::connect().expect("Cant connect");
        let mut conn = db.pool.get().map_err(|_| "Failed to get DB connection").unwrap();
        
        Subnet::belonging_to(target)  // 👈 This uses the `belongs_to` relationship
            .load::<Subnet>(&mut self.0)
    }
    
    pub fn insert(
        &mut self,
        target_id : i32,
        cidr : String,
    ) -> Subnet {
            let subnet_object = SubnetInsert {
                target_id,
                cidr,
            };
            diesel::insert_into(subnet_table)
                .values(&subnet_object)
                .on_conflict(dsl_subnet::cidr)
                .do_update()
                .set(dsl_subnet::cidr.eq(&subnet_object.cidr))
                .get_result::<Subnet>(&mut self.0)
                .unwrap_or_else(|e| {
                    self.1.info(String::from("Inserting subnet failed"));
                    panic!("{}", e);
                }) 

    }
    
}


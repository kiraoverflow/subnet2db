use crate::{
    logging::Logger, models::{
        ip::{Ip, IpInsert}, networkmetrics::NetworkMetrics, port::{Port,PortInsert}
    }, schema::{
        ip::dsl::{self as dsl_ip, ip as ip_table}, port::dsl::{self as dsl_port, port as port_table}
    }
};

use crate::database::Db;
use diesel::{prelude::*, r2d2::{ConnectionManager, PooledConnection}};
use diesel::BelongingToDsl;

pub struct PortInterface(
    PooledConnection<ConnectionManager<PgConnection>>,
    Logger,

);

pub type Error = diesel::result::Error;

impl PortInterface {
    pub fn new() -> PortInterface {
        let db = Db::connect().expect("Cant connect");

        let mut conn = db.pool.get().map_err(|_| "Failed to get DB connection").unwrap();
        let logger = Logger::new();
        PortInterface(
            conn,
            logger,
        )
    } 
     
    pub fn get_by_id(
        &mut self,
        id : i32
    ) -> Result<Port, Error> {
        let db = Db::connect().expect("Cant connect");

        let mut conn = db.pool.get().map_err(|_| "Failed to get DB connection").unwrap();
        port_table 
            .filter(dsl_port::id.eq(id))
            .first::<Port>(&mut self.0)
    }

    pub fn get_by_ip(
        &mut self,
        ip: &Ip
    ) -> Result<Vec<Port>, Error> {
        let db = Db::connect().expect("Cant connect");
        let mut conn = db.pool.get().map_err(|_| "Failed to get DB connection").unwrap();
        
        Port::belonging_to(ip)  // 👈 This uses the `belongs_to` relationship
            .load::<Port>(&mut self.0)
    }
    
}


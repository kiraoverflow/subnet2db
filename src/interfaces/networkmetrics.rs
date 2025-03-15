use crate::{
    logging::Logger, models::{
        ip::{Ip, IpInsert}, networkmetrics::NetworkMetrics
    }, schema::{
        ip::dsl::{self as dsl_ip, ip as ip_table}, network_metrics::dsl::{self as dsl_nm, network_metrics as nm_table}
    }
};

use crate::database::Db;
use diesel::{prelude::*, r2d2::{ConnectionManager, PooledConnection}};
use diesel::BelongingToDsl;

pub struct NetworkMetricsInterface(
    PooledConnection<ConnectionManager<PgConnection>>,
    Logger,
);

pub type Error = diesel::result::Error;

impl NetworkMetricsInterface { 
     pub fn new() -> NetworkMetricsInterface {
        let db = Db::connect().expect("Cant connect");

        let mut conn = db.pool.get().map_err(|_| "Failed to get DB connection").unwrap();
        let logger = Logger::new();
        NetworkMetricsInterface(
            conn,
            logger,
        )
    } 

    pub fn get_by_id(
        &mut self,
        id : i32
    ) -> Result<NetworkMetrics, Error> { 
        nm_table 
            .filter(dsl_nm::id.eq(id))
            .first::<NetworkMetrics>(&mut self.0)
    }

    pub fn get_by_ip(
        &mut self,
        ip: &Ip
    ) -> Result<Vec<NetworkMetrics>, Error> {
        NetworkMetrics::belonging_to(ip)  // 👈 This uses the `belongs_to` relationship
            .load::<NetworkMetrics>(&mut self.0)
    }
    
}


use crate::{
    logging::Logger, models::target::{Target, TargetInsert}, schema::target::dsl::{self as dsl_target, target as target_table}
};

use crate::database::Db;
use diesel::{prelude::*, r2d2::{ConnectionManager, PooledConnection}};

pub struct TargetInterface(
    PooledConnection<ConnectionManager<PgConnection>>,
    Logger,
);

pub type Error = diesel::result::Error;

impl TargetInterface { 
    pub fn new() -> TargetInterface {
        let db = Db::connect().expect("Cant connect");

        let mut conn = db.pool.get().map_err(|_| "Failed to get DB connection").unwrap();
        let logger = Logger::new();
        TargetInterface(
            conn,
            logger,
        )
    } 

    pub fn get_by_id(
        &mut self, 
        id : i32
    ) -> Result<Target, Error> {
        let db = Db::connect().expect("Cant connect");

        let mut conn = db.pool.get().map_err(|_| "Failed to get DB connection").unwrap();
        target_table 
            .filter(dsl_target::id.eq(id))
            .first::<Target>(&mut self.0)
    }

    pub fn get_by_country(
        &mut self, 
        country : String
    ) -> Result<Target, Error> {
        let db = Db::connect().expect("Cant connect");

        let mut conn = db.pool.get().map_err(|_| "Failed to get DB connection").unwrap();
        
        target_table 
            .filter(dsl_target::country.eq(&country))
            .first::<Target>(&mut self.0) 
    }
    
    pub fn insert(
        &mut self, 
        country : String
    ) -> Target{
        let target_object = TargetInsert{
            country
        };

        diesel::insert_into(target_table)
            .values(&target_object)
            .on_conflict(dsl_target::country) // Conflict on the unique `country`
            .do_update()
            .set(dsl_target::country.eq(&target_object.country))
            .get_result::<Target>(&mut self.0)
            .unwrap_or_else(|e| {
                self.1.info(String::from("Inserting target failed"));
                panic!("{}", e);
            })

    }

}


use crate::schema::ip;

use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use crate::models::subnet::Subnet;
//use crate::models::target::Target;
#[derive(Debug, Queryable, Selectable, Identifiable, Associations, Serialize, Deserialize)]
#[diesel(table_name = ip)]// Ensure this matches your Diesel schema
#[diesel(belongs_to(Subnet, foreign_key = subnet_id))]  // 👈 Define the relationship
//#[diesel(belongs_to(Target, foreign_key = target_id))]  // 👈 Define the relationship
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Ip {
    pub id: i32,
    pub subnet_id : i32,
    //pub target_id : i32,
    pub v4 : Option<String>,
    pub v6 : Option<String>,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = ip)]// Ensure this matches your Diesel schema
#[diesel(check_for_backend(dieesel::pg::Pg))]
pub struct IpInsert {
    pub subnet_id : i32,
    //pub target_id : i32,
    pub v4 : Option<String>,
    pub v6 : Option<String>,
}



use crate::schema::subnet;
use crate::models::target::Target;

use serde::{Serialize, Deserialize};
use diesel::prelude::*;


#[derive(Debug, Queryable,Associations, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = subnet)]// Ensure this matches your Diesel schema
#[diesel(belongs_to(Target, foreign_key = target_id))]  // 👈 Define the relationship
#[diesel(check_for_backend(dieesel::pg::Pg))]
pub struct Subnet {
    pub id: i32,
    pub target_id : i32,
    pub cidr : String,
}


#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = subnet)]// Ensure this matches your Diesel schema
#[diesel(check_for_backend(dieesel::pg::Pg))]
pub struct SubnetInsert {
    pub target_id : i32,
    pub cidr : String,
}

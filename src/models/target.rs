use crate::schema::target;

use serde::{Serialize, Deserialize};
use diesel::prelude::*;


#[derive(Debug, Queryable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = target)]// Ensure this matches your Diesel schema
#[diesel(check_for_backend(dieesel::pg::Pg))]
pub struct Target {
    pub id: i32,
    pub country: String,
}

#[derive(Debug, Insertable,Serialize, Deserialize)]
#[diesel(table_name = target)]// Ensure this matches your Diesel schema
#[diesel(check_for_backend(dieesel::pg::Pg))]
pub struct TargetInsert {
    pub country: String,
}

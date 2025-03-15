use crate::schema::port;

use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use crate::models::ip::Ip;

#[derive(Debug, Queryable,Identifiable, Associations, Serialize, Deserialize)]
#[diesel(table_name = port)]// Ensure this matches your Diesel schema
#[diesel(belongs_to(Ip, foreign_key = ip_id))]  // 👈 Define the relationship
#[diesel(check_for_backend(dieesel::pg::Pg))]
pub struct Port {
    pub id: i32,
    pub ip_id: i32,
    pub number: i32,
    pub service: Option<String>,
    pub timeout: Option<bool>,
    pub answer: Option<bool>,
    pub deny: Option<bool>,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = port)]// Ensure this matches your Diesel schema
#[diesel(check_for_backend(dieesel::pg::Pg))]
pub struct PortInsert {
    pub ip_id: i32,
    pub number: i32,
    pub service: Option<String>,
    pub timeout: Option<bool>,
    pub answer: Option<bool>,
    pub deny: Option<bool>,
}

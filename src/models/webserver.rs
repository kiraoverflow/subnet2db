use crate::schema::webserver;

use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use crate::models::ip::Ip;

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = webserver)]// Ensure this matches your Diesel schema
#[diesel(belongs_to(Ip, foreign_key = ip_id))]  // 👈 Define the relationship
#[diesel(check_for_backend(dieesel::pg::Pg))]
pub struct Webserver {
   pub id: i32,
   pub ip_id: i32,
   pub port_id: i32,
   pub crawer_id: i32,
   pub reachable: Option<bool>,
   pub framework: Option<String>,
}

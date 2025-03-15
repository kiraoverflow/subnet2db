use crate::schema::crawler;

use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use crate::models::ip::Ip;

#[derive(Debug, Insertable, Associations, Serialize, Deserialize)]
#[diesel(table_name = crawler)]// Ensure this matches your Diesel schema
#[diesel(belongs_to(Ip, foreign_key = ip_id))]  // 👈 Define the relationship
#[diesel(check_for_backend(dieesel::pg::Pg))]
pub struct Crawler {
    pub id: i32,
    pub ip_id: i32,
    pub endpoint_id: i32,
}

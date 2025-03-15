use crate::schema::geo_metrics;

use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use crate::models::ip::Ip;

#[derive(Debug, Insertable, Associations, Serialize, Deserialize)]
#[diesel(table_name = geo_metrics)]// Ensure this matches your Diesel schema
#[diesel(belongs_to(Ip, foreign_key = ip_id))]  // 👈 Define the relationship
#[diesel(check_for_backend(dieesel::pg::Pg))]
pub struct GeoMetrics {
    pub id: i32,
    pub ip_id: i32,
    pub city: String,
    pub hops_to_turkey: Option<String>,
    pub hops_to_iraq: Option<String>,
    pub hops_to_jordan: Option<String>,
    pub hops_to_lebanon: Option<String>,
    pub hops_to_israel: Option<String>,
}

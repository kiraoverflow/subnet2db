use crate::schema::network_metrics;

use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use crate::models::ip::Ip;

#[derive(Debug, Queryable, Identifiable, Associations, Serialize, Deserialize)]
#[diesel(table_name = network_metrics)]// Ensure this matches your Diesel schema
#[diesel(belongs_to(Ip, foreign_key = ip_id))]  // 👈 Define the relationship
#[diesel(check_for_backend(dieesel::pg::Pg))]
pub struct NetworkMetrics {
    pub id: i32,
    pub ip_id: i32,
    pub pingable: Option<bool>,
    pub reachable: Option<bool>,
    pub unreachable: Option<bool>,
    pub ttr: Option<f64>,
    pub ttl: Option<f64>,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = network_metrics)]// Ensure this matches your Diesel schema
#[diesel(check_for_backend(dieesel::pg::Pg))]
pub struct NetworkMetricsInsert {
    pub ip_id: i32,
    pub pingable: Option<bool>,
    pub reachable: Option<bool>,
    pub unreachable: Option<bool>,
    pub ttr: Option<f64>,
    pub ttl: Option<f64>,
}

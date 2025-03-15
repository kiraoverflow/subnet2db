use crate::schema::endpoint;

use serde::{Serialize, Deserialize};
use diesel::prelude::*;


#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = endpoint)]// Ensure this matches your Diesel schema
#[diesel(check_for_backend(dieesel::pg::Pg))]
pub struct Endpoint {
    pub id: i32,
    pub route: Option<String>,
    pub status_code: Option<i32>,
    pub credentials: Option<bool>,
    pub authentification: Option<bool>,

}

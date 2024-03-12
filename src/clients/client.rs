use serde::{Deserialize, Serialize};
use chrono::naive::NaiveDate;
// @todo:
//use chrono::naive::NaiveDate;

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Client {
    pub id: u64,
    pub client_name: String,
    // @todo:
    //pub birth_date: NaiveDate,
    pub document_number: u64,
    pub country: String,
    pub balance: f64
}
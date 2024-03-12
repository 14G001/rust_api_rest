use actix_web::{http::StatusCode, post, HttpResponse, Responder};
use serde_json::from_str;
use serde::{Deserialize, Serialize};
// @todo:
//use chrono::naive::NaiveDate;
use crate::{clients::{client::Client, db::add::add_client},
    errors::parse::parsing_error};

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct RecvFields {
    pub client_name: String,
    // @todo:
    //pub birth_date: NaiveDate,
    pub document_number: u64,
    pub country: String
}

#[post("/new_client")]
async fn service(body: String) -> impl Responder {
    let client_parse: Result<RecvFields, serde_json::Error>
        = from_str::<RecvFields>(&body);
    if !client_parse.is_ok() {
        return parsing_error(client_parse.err().unwrap().to_string());
    } 
    let recv_fields = client_parse.ok().unwrap();
    println!("Recv fields: {:#?}", recv_fields);
    let result: Result<String, rusqlite::Error> = add_client(Client {
        id: 0,
        client_name:     recv_fields.client_name,
        // @todo: add birth_date
        document_number: recv_fields.document_number,
        country:         recv_fields.country,
        balance: 0.0
    }).await;
    HttpResponse::Ok().body(result.unwrap())
}
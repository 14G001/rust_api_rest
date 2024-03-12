use actix_web::{http::StatusCode, post, HttpResponse, Responder};
use serde_json::from_str;
use serde::{Deserialize, Serialize};
use crate::{transaction::process::credit::credit_transaction,
    errors::parse::parsing_error};

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct RecvFields {
    pub client_id: u64,
    pub credit_amount: f64
}

// @todo: unrepeat
#[post("/new_credit_transaction")]
async fn service(body: String) -> impl Responder {
    let tran_data: Result<RecvFields, serde_json::Error>
        = from_str::<RecvFields>(&body);
    if !tran_data.is_ok() {
        return parsing_error(tran_data.err().unwrap().to_string());
    } 
    let recv_fields: RecvFields = tran_data.ok().unwrap();
    let result: String = credit_transaction(
        recv_fields.client_id, recv_fields.credit_amount
    ).await;
    HttpResponse::Ok().body(result)
}
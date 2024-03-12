use actix_web::{get, HttpResponse, Responder, web::Query};
use serde_json::from_str;
use serde::{Deserialize, Serialize};
use crate::{clients::{client::Client, db::get::get_client},
    errors::{bad_req::bad_request, parse::parsing_error}};

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct RecvFields {
    pub user_id: u64,
}

#[get("/client_balance")]
async fn service(info: Query<RecvFields>) -> impl Responder {
    /// @todo: validate
    let result: Result<String, rusqlite::Error> = get_client(info.user_id).await;
    let result_ref = result.as_ref();
    if result_ref.err() != None {
        let res_msg = format!("{:#?}", result_ref.err().unwrap());
        return bad_request("Error getting client: ", &res_msg);
    }
    let client = from_str::<Client>(result_ref.unwrap());
    HttpResponse::Ok().body(format!("Client´s current balance is: {:#?}$", client.unwrap().balance))
}
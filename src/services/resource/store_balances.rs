use actix_web::{post, HttpResponse, Responder};

#[post("/store_balances")]
async fn service(body: String) -> impl Responder {
    // @todo
    let msg: String = "@todo ".to_owned() + &body;
    HttpResponse::Ok().body(msg)
}
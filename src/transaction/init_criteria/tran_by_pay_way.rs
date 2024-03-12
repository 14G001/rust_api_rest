use {actix_web::HttpResponse, crate::{db::{class::ram_db::singleton::get_db, error::ok},
    errors::bad_req::bad_request, transaction::{pay_ways::pay_way::PayWay, transaction::Transaction}}};

/// Procesa una transacción en base al método
/// pago de la misma.
///
/// # Argumentos
/// * `client_id`: ID del cliente de la transacción.
/// * `amount`: monto de la transacción.
/// * `pay_way`: método de pago de la transacción.
///
/// # Devuelve
/// Tipo de error en caso de que haya ocurrido
/// alguno durante el procesamiento de la
/// transacción.
pub fn tran_by_pay_way(client_id: u8, amount: f64, pay_way: PayWay) -> HttpResponse {
    let mut transaction = Transaction::new();
    let result = Transaction::process(
        &mut transaction, client_id, amount, pay_way
    );
    let res_ref = &result;
    if !ok(res_ref) {
        return bad_request("Transaction error: ", res_ref);
    }
    HttpResponse::Ok().body(format!(
        "Transaction success - New client amount: {}$",
        get_db().clients[&client_id].balance))
}
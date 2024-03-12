use {actix_web::{post, Responder}, serde_json::from_str, serde::{Deserialize, Serialize},
    crate::{errors::parse::parsing_error, transaction::{init_criteria::tran_by_pay_way::tran_by_pay_way,
    pay_ways::pay_way::PayWay}}};

/// Estructura con los datos que admite
/// la API en su servicio de transacción
/// de débito de dinero.
///
/// # Campos
///
/// * `client_id`: ID del cliente que realizará la transacción.
/// * `debit_amount`: Monto de la transacción a debitar.
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct RecvFields {
    pub client_id: u8,
    pub debit_amount: f64
}

/// Servicio de la API que procesa una
/// transacción de débito de dinero
/// en base a un ID y monto recibidos.
///
/// # Argumentos
/// * `body`: Datos de la transacción a procesar.
///
/// # Devuelve
/// Respuesta HTTP con el resultado de
/// la operación.
#[post("/new_debit_transaction")]
async fn service(body: String) -> impl Responder {
    let tran_data = from_str::<RecvFields>(&body);
    if !tran_data.is_ok() {
        return parsing_error();
    } 
    let recv_fields: RecvFields = tran_data.ok().unwrap();
    tran_by_pay_way(recv_fields.client_id, recv_fields.debit_amount, PayWay::Debit)
}
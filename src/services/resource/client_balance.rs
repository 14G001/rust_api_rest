use {actix_web::{get, HttpResponse, Responder, web::Query},
    serde::Deserialize, crate::{client::client::Client, errors::bad_req::bad_request,
    db::{class::ram_db::ram_db::RamDbConn, error::ok}}};

/// Estructura con los datos que admite
/// la API en su servicio de obtención
/// de saldo de cliente.
///
/// # Campos
///
/// * `user_id`: ID del cliente a consultar el saldo.
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct RecvFields {
    pub user_id: u8,
}

/// Servicio de la API que recibe un ID
/// de cliente y devuelve su saldo.
///
/// # Argumentos
/// * `recv_data`: Datos del mensaje recibido por el servicio.
///
/// # Devuelve
/// Respuesta HTTP con el resultado de
/// la operación
#[get("/client_balance")]
async fn service(recv_data: Query<RecvFields>) -> impl Responder {
    // setting param "recv_data: Query<RecvFields>" with that format deserializes params and throws errors automatically.
    let mut client: Client = Client::new();
    let result = RamDbConn::get_client(&mut client, recv_data.user_id);
    if !ok(&result) {
        return bad_request("Error getting client balance: ", &result);
    }
    HttpResponse::Ok().body(format!("Client´s current balance is: {}$", client.balance))
}

use {actix_web::{post, HttpResponse, Responder}, serde_json::from_str, serde, serde::Deserialize,
    crate::{client::client::Client, db::{class::ram_db::ram_db::RamDbConn, error::ok},
    errors::{bad_req::bad_request, parse::parsing_error}, utils::parse::naive_date::CustomNaiveDate}};

/// Estructura con los datos que admite
/// la API en su servicio de agregado
/// de cliente nuevo.
///
/// # Campos
///
/// * `client_name`: Nombre del cliente a agregar.
/// * `birth_date`:  Fecha de nacimiento del cliente a agregar.
/// * `document_number`: Número de documento del cliente a agregar.
/// * `country`: País del cliente a agregar.
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct RecvFields {
    pub client_name: String,
    pub birth_date: CustomNaiveDate,
    pub document_number: usize,
    pub country: String
}

/// Servicio de la API que recibe los
/// datos de un cliente y agrega el
/// cliente recibido a la base de
/// datos; devolviendo el ID del
/// cliente asignado.
///
/// # Argumentos
/// * `body`: Datos del cliente en formato de Json a procesar.
///
/// # Devuelve
/// Respuesta HTTP con el resultado de
/// la operación.
#[post("/new_client")]
async fn service(body: String) -> impl Responder {
    let client_parse = from_str::<RecvFields>(&body);
    if !client_parse.is_ok() {
        return parsing_error();
    } 
    let recv_fields = client_parse.ok().unwrap();
    let client = Client {
        id: 0,
        client_name:     recv_fields.client_name,
        birth_date:      recv_fields.birth_date,
        document_number: recv_fields.document_number,
        country:         recv_fields.country,
        balance: 0.0
    };
    let mut assigned_cli_id: u8 = 0;
    let result = RamDbConn::add_client(&mut assigned_cli_id, client);
    let aux_res = &result;
    if !ok(aux_res) {
        return bad_request("Error adding client: ", aux_res)
    }
    HttpResponse::Ok().body(format!("Client added successfully - Assigned client ID: {}", assigned_cli_id))
}
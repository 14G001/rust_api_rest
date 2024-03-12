use {actix_web::{post, HttpResponse, Responder},
    crate::{db::error::ok, errors::bad_req::bad_request, statistics::balances::generate_result_balances}};

/// Servicio de la API que persiste
/// los datos de todos los clientes y
/// sus balances en un archivo con
/// formato:
/// "DDMMAAAA_<Número de archivo>.DAT"
/// Y luego deja los saldos de todos
/// los clientes con valor 0.
///
/// # Devuelve
/// Respuesta HTTP con el resultado de
/// la operación
#[post("/store_balances")]
async fn service() -> impl Responder {
    let result = generate_result_balances();
    if ok(&result) {
        return HttpResponse::Ok().body("Balances file generated successfully")
    }
    bad_request("Error: ", &result)
}
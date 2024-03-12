use {actix_web::{HttpResponse, body::BoxBody},crate::{
    errors::bad_req::bad_request, db::error::DbError}};

/// Función que genera un mensaje de
/// respuesta de error HTTP para
/// casos de error de formato
/// incorrecto de mensaje enviado a
/// la API.
///
/// # Devuelve
/// Mensaje de respuesta de error HTTP generado.
pub fn parsing_error() -> HttpResponse<BoxBody> {
    bad_request("Error parsing json: ", &DbError::InvalidFields)
}
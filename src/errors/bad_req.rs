use {actix_web::{HttpResponse, body::BoxBody, http::StatusCode},
    crate::{db::error::DbError, errors::cases::get_err_msg}};

/// Función que genera un mensaje de
/// respuesta de error HTTP en base a
/// ciertos parámetros y lo devuelve.
///
/// # Argumentos
/// * `main_err_msg`: Mensaje de error que aparecerá al inicio del cuerpo de la respuesta.
/// * `err`: Código de error para obtener el mensaje de error.
///
/// # Devuelve
/// Mensaje de respuesta de error HTTP generado.
pub fn bad_request(main_err_msg: &str, err: &DbError) -> HttpResponse<BoxBody> {
    let result_msg = format!("{}{}", main_err_msg, get_err_msg(err));
    HttpResponse::build(StatusCode::BAD_REQUEST).body(result_msg)
}
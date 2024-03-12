/// Contiene una función que genera un
/// mensaje de respuesta de error HTTP
/// en base a ciertos parámetros.
pub mod bad_req;

/// Contiene una función que devuelve un
/// mensaje de error a partir de un código
/// de error.
pub mod cases;

/// Contiene una función que genera un
/// mensaje de respuesta de error HTTP
/// para casos de error de formato
/// incorrecto de mensaje enviado a la
/// API.
pub mod parse;
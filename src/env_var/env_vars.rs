/// Estructura con las variables de
/// entorno de la aplicación que se
/// pueden configurar en el archivo
/// ".env".
///
/// # Campos
///
/// * `port`: Puerto en el que se proveerá el servicio.
/// * `host`: Dirección IP desde la que se proveerá el servicio.
pub struct EnvVars {
    pub port: u16,
    pub host: String
}
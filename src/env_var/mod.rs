/// Estructura con las variables de
/// entorno de la aplicación que se
/// pueden configurar en el archivo
/// ".env"; como la IP y puerto.
pub mod env_vars;

/// Función que lee las variables de
/// entorno del archivo ".env", las
/// almacena en memoria en una estructura
/// y procesa los errores en caso de
/// haberlos.
pub mod load;
use {actix_web::{App, HttpServer},
    api::{services::init::config, env_var::load::load_env_vars}};

/// Función principal en la que se realizan
/// todas las configuraciones de la aplicación
/// y se inician los servicios de la API.
///
/// # Devuelve
///
/// Resultado de la ejecución de la aplicación;
/// en caso de haber sido exitosa no devolverá
/// nada; en el caso contrario; devolverá un
/// error.
#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let env_vars = load_env_vars();
    HttpServer::new(|| {
        App::new().configure(config)
    })
    .bind((env_vars.host, env_vars.port))?
    .run()
    .await
}
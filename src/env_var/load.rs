use {std::env, dotenv::dotenv, crate::env_var::env_vars::EnvVars};

/// Esta función lee las variables de
/// entorno del archivo ".env" y las
/// almacena en memoria en una estructura.
/// En caso de haber alguna variable
/// faltante; arrojará una excepción y
/// finalizará la ejecución del programa.
///
/// # Devuelve
///
/// La estructura con las variables de
/// entorno.
pub fn load_env_vars() -> EnvVars {
    dotenv().ok();
    EnvVars {
        port: env::var("PORT")
            .expect("Port not specified on environment variables")
            .parse().expect("Error: wrong environment variables port value type"),
        host: env::var("HOST")
            .expect("Host not specified on environment variables")
    }
}
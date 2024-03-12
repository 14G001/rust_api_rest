use actix_web::{App, HttpServer};
use api::services::init::config;
use api::env_var::load::load_env_vars;
use api::init::init_databases::init_databases;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let env_vars = load_env_vars();
    init_databases().await;
    HttpServer::new(|| {
        App::new().configure(config)
    })
    .bind((env_vars.host, env_vars.port))?
    .run()
    .await
}
use std::env;
use dotenv::dotenv;
use crate::env_var::env_vars::{EnvVars};

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
use chrono::prelude::*;

/// # Devuelve
/// Fecha actual en formato DDMMYYYY.
pub fn get_curr_date_ddmmyyyy() -> String {
    Utc::now().format("%d%m%Y").to_string()
}
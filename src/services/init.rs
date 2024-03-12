use {actix_web::web::ServiceConfig, crate::services::resource::*};

/// Esta función inicia todos los servicios
/// del servidor y su respectivos manejos
/// de los posibles mensajes que podrán
/// recibir.
///
/// # Argumentos
///
/// * `cfg`: Configuración del servidor a la que se le agregarán los servicios.
/// 
/// # Por hacer
/// 
/// Implementar polimorfismo para que no
/// se repita la parte de "cfg.service"
/// innecesariamente.
pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(new_client::service);
    cfg.service(new_credit_transaction::service);
    cfg.service(new_debit_transaction::service);
    cfg.service(store_balances::service);
    cfg.service(client_balance::service);
}
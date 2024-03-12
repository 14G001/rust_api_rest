use actix_web::web::ServiceConfig;
use crate::services::resource::*;

pub fn config(cfg: &mut ServiceConfig) {
    ///< @todo: polimorphism:
    cfg.service(new_client::service);
    cfg.service(new_credit_transaction::service);
    cfg.service(new_debit_transaction::service);
    cfg.service(store_balances::service);
    cfg.service(client_balance::service);
}
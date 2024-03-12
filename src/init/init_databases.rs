use crate::clients::db::init::init_clients;

pub async fn init_databases() {
    init_clients().await;
}
use rusqlite::{Connection, Result, Error, Statement, Row};
use crate::{database::connect::{connect_to_db, ERR_DB_CONN_MSG}};

pub async fn update_client_amount(client_id: u64, new_amount: f64) -> Result<String, Error> {
    let connection: Connection = connect_to_db().expect(ERR_DB_CONN_MSG);
    connection.execute("
        update clients set balance = ?1 where id = ?2
    ", [
        new_amount.to_string(), client_id.to_string()
    ]).expect("Error updating balance");
    Ok("Balance updated successfully".to_string())
}
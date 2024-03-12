use rusqlite::Error;
use serde_json::to_string_pretty;

use rusqlite::{Connection, Statement, Row};
use crate::{clients::client::Client,
    database::connect::{connect_to_db, ERR_DB_CONN_MSG},
    errors::db::process_query_result};

pub async fn get_client(client_id: u64) -> Result<String, Error> {
    println!("CLIID: {}", client_id);
    let err_msg: String = String::from("Error getting client: ");
    let mut result_msg: String = String::from("");
    let connection: Connection = connect_to_db().expect(ERR_DB_CONN_MSG);
    let mut stmt: Statement<'_> = connection.prepare(
        "select * from clients where id = ?"
    ).expect("Error");
    let result: Result<Client, Error> = stmt.query_row([client_id], 
        |row: &Row<'_>| {
        Ok(Client {
            id:              row.get(0)?,
            client_name:     row.get(1)?,
            // @todo:
            //birth_date: NaiveDate,
            document_number: row.get(3)?,
            country:         row.get(4)?,
            balance:         row.get(5)?,
        })
    });
    let result_ref = result.as_ref();
    println!("RES {:#?}", result);
    result_msg = process_query_result(err_msg, result_ref.err());
    if result_msg != "" {
        println!("A");
        return Err(result.err().unwrap());
    }
    Ok(to_string_pretty(result_ref.unwrap()).unwrap())
}
use rusqlite::{Connection, Result, Error, Statement, Row};
use crate::{clients::client::Client,
    database::utils::AutoincrementId,
    errors::db::process_query_result,
    database::connect::{connect_to_db, ERR_DB_CONN_MSG}};

pub async fn add_client(client: Client) -> Result<String, Error> {
    let err_msg: String = String::from("Error adding client: ");
    let mut result_msg: String = String::from("");
    let connection: Connection = connect_to_db().expect(ERR_DB_CONN_MSG);
    let result: Result<usize, Error> = connection
        .execute("
            insert into clients (
                client_name,
                birth_date,
                document_number,
                country,
                balance
            ) values (?1, ?2, ?3, ?4, ?5)
        ", [
            client.client_name,
            // @todo: format!("{}", client.birth_date),
            format!("{}", "2000-00-00"),
            format!("{}", client.document_number),
            client.country,
            format!("{}", 0.0)
        ]);
    let result_ref = result.as_ref();
    result_msg = process_query_result(err_msg, result_ref.err());
    if result_msg != "" {
        return Ok(result_msg);
    }
    let mut stmt: Statement<'_> = connection.prepare(
        "select max(id) FROM clients"
    ).expect("Error");
    let row: AutoincrementId = stmt.query_row([], 
        |row: &Row<'_>| {
        Ok(AutoincrementId {
            id: row.get(0)?
        })
      }).expect("Error");
    Ok(format!("Client added successfully - ID assigned: {}", row.id))
}
use rusqlite::{Connection, Result};
use crate::database::connect::{connect_to_db, ERR_DB_CONN_MSG};

pub async fn init_clients() -> Result<()> {
    let connection: Connection = connect_to_db().expect(ERR_DB_CONN_MSG);
    connection.execute("
        create table if not exists clients (
            id integer primary key autoincrement,
            client_name text not null,
            birth_date date not null,
            document_number bigint unique,
            country text not null,
            balance double not null
        )
    ", []).expect("Error creating table");
    // @todo: transaction
    /*connection.execute("
        create index fast_search_by_doc
            on clients (document_number)
    ", []).expect("Error creating index");*/
    Ok(())
}
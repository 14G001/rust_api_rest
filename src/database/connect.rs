use rusqlite::{Connection, Error};

pub const ERR_DB_CONN_MSG: &str = "Error connecting to database";

pub fn connect_to_db() -> Result<Connection, Error> {
    Connection::open("system.db")
}
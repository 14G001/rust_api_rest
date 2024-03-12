use crate::{client::client::Client, db::{class::ram_db::singleton::{
    get_db, is_document_already_used}, error::{DbError, ok}}};

const MAX_CLIENT_ID: u8 = 99;

/// Contiene las operaciones para poder
/// persistir los datos de los clientes
/// y sus saldos en una base de datos en
/// memoria RAM.
pub struct RamDbConn;
impl RamDbConn {
    /// La documentación de la función se encuentra
    /// en la interfaz de esta clase/estructura en el
    /// archivo "src/db/db.rs" para evitar posibles
    /// problemas por redundancia de descripciones.
    pub fn add_client(assigned_id: &mut u8, mut client: Client) -> DbError {
        let ram_db = get_db();
        if ram_db.client_id_to_assign > MAX_CLIENT_ID {
            return DbError::CantCreateMoreClients;
        }
        if is_document_already_used(client.document_number) {
            return DbError::DocNumAlreadyInUse;
        }
        client.id = ram_db.client_id_to_assign;
        ram_db.clients.insert(client.id, client);
        *assigned_id = ram_db.client_id_to_assign;
        ram_db.client_id_to_assign += 1;
        DbError::None
    }
    /// La documentación de la función se encuentra
    /// en la interfaz de esta clase/estructura en el
    /// archivo "src/db/db.rs" para evitar posibles
    /// problemas por redundancia de descripciones.
    pub fn get_client(client: &mut Client, client_id: u8) -> DbError {
        let record = get_db().clients.get(&client_id);
        if let Some(value) = record {
            *client = value.clone();
        } else {
            return DbError::ClientNotExist;
        }
        DbError::None
    }
    /// La documentación de la función se encuentra
    /// en la interfaz de esta clase/estructura en el
    /// archivo "src/db/db.rs" para evitar posibles
    /// problemas por redundancia de descripciones.
    pub fn update_client_balance(client_id: u8, new_balance: f64) -> DbError {
        let mut client: Client = Client::new();
        let result = Self::get_client(&mut client, client_id);
        if !ok(&result) {
            return result;
        }
        client.balance = new_balance;
        get_db().clients.insert(client_id, client);
        DbError::None
    }
}
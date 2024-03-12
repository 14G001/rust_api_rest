use crate::{client::client::Client, db::error::DbError};

/// Contiene las operaciones que
/// deberán tener los distintos tipos
/// de bases de datos del aplicativo;
/// logrando así el patrón de diseño
/// Abstract Factory; forzando a que
/// los distintos tipos de bases de
/// datos se implementen
/// correctamente.
/// Las bases de datos contienen
/// los datos de los clientes y sus
/// saldos.
pub trait Database {
    /// Agrega un cliente a la base de datos
    /// y devuelve el ID que se le asignó al
    /// mismo.
    ///
    /// # Argumentos
    /// * `assigned_id`: ID asignado a devolver.
    /// * `client`: Datos del cliente a agregar.
    ///
    /// # Devuelve
    /// Tipo de error en caso de que haya ocurrido
    /// alguno al intentar agregar al cliente.
    fn add_client(assigned_id: &mut u8, client: Client) -> DbError;

    /// Devuelve los datos de un cliente
    /// encontrado en la base de datos a
    /// partir de su ID.
    ///
    /// # Argumentos
    /// * `client`: Datos del cliente a devolver.
    /// * `client_id`: ID de cliente a bsucar.
    ///
    /// # Devuelve
    /// Tipo de error en caso de que haya ocurrido
    /// alguno al intentar obtener los datos del
    /// cliente.
    fn get_client(client: &mut Client, client_id: u8) -> DbError;

    /// Actualiza el saldo de un cliente en la
    /// base de datos.
    ///
    /// # Argumentos
    /// * `client_id`: ID de cliente a bsucar.
    /// * `new_balance`: Saldo nuevo del cliente.
    ///
    /// # Devuelve
    /// Tipo de error en caso de que haya ocurrido
    /// alguno al intentar actualizar el saldo del
    /// cliente.
    fn update_client_balance(client_id: u8, new_balance: f64) -> DbError;
}
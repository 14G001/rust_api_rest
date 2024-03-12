use {std::{sync::Once, collections::BTreeMap},
    crate::client::client::Client};

static mut SINGLETON: Option<RamDb> = None;
static ONCE: Once = Once::new();

/// Devuelve la base de datos que se
/// encuentra en la memoria RAM y si
/// aún no tiene memoria la asigna.
///
/// # Devuelve
/// Base de datos que se encuentra en
/// la memoria RAM.
pub fn get_db() -> &'static mut RamDb {
    unsafe {
        ONCE.call_once(|| {
            SINGLETON = Some(RamDb::new());
        });
        SINGLETON.as_mut().unwrap()
    }
}

/// Estructura con los datos globales de
/// la base de datos en memoria RAM.
///
/// # Campos
///
/// * `clients`: Los datos de todos los clientes.
/// * `client_name`: Próximo ID de cliente a asignar.
/// * `file_count`: Contador de archivos creados desde que se inició el programa.
pub struct RamDb {
  pub clients: BTreeMap<u8, Client>,
  pub client_id_to_assign: u8,
  pub file_count: usize
}
impl RamDb {
    /// Inicia y devuelve los datos de la
    /// base de datos en memoria RAM.
    ///
    /// # Devuelve
    /// Datos para base de datos en memoria RAM.
    fn new() -> Self {
        Self {
            clients: BTreeMap::new(),
            client_id_to_assign: 1,
            file_count: 1
        }
    }
}

/// Verifica si el número de documento ya
/// fué utilizado por algún cliente.
///
/// # Argumentos
/// * `document_number`: Número de documento a verificar si se encuentra usado por algún cliente.
///
/// # Devuelve
/// Verdadero si el número de documento
/// ya es utilizado por un cliente; falso
/// en el caso contrario.
pub fn is_document_already_used(document_number: usize) -> bool {
    for (_cli_id, cli) in get_db().clients.iter() {
        if document_number == cli.document_number {
            return true;
        }
    }
    false
}
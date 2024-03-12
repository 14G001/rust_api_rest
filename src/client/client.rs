use {std::num::NonZeroI32, crate::utils::parse::naive_date::CustomNaiveDate};

/// Estructura con los datos de cliente de la aplicación.
///
/// # Campos
///
/// * `id`: ID del cliente generado por la aplicación.
/// * `client_name`: Nombre del cliente.
/// * `birth_date`: Fecha de nacimiento del cliente.
/// * `document_number`: Número de documento del cliente.
/// * `country`: País del cliente.
/// * `balance`: El saldo del cliente.
#[derive(Debug, Clone)]
pub struct Client {
    pub id: u8,
    pub client_name: String,
    pub birth_date: CustomNaiveDate,
    pub document_number: usize,
    pub country: String,
    pub balance: f64
}
impl Client {
    /// Crea un cliente con todos sus datos
    /// sin ninguno de sus datos definido y
    /// lo devuelve.
    ///
    /// # Devuelve
    /// El cliente creado.
    pub fn new() -> Self {
        Client {
            id: 0,
            client_name: "-".to_string(),
            birth_date: CustomNaiveDate {
                yof: NonZeroI32::new(1).unwrap()
            },
            document_number: 0,
            country: "-".to_string(),
            balance: 0.0
        }
    }
}
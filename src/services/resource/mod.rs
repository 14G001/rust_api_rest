/// Servicio de la API que recibe los
/// datos de un cliente y agrega el
/// cliente recibido a la base de
/// datos; devolviendo el ID del
/// cliente asignado.
pub mod new_client;

/// Servicio de la API que procesa una
/// transacción de acreditación de
/// dinero en base a un ID y monto
/// recibidos.
pub mod new_credit_transaction;

/// Servicio de la API que procesa una
/// transacción de débito de dinero
/// en base a un ID y monto recibidos.
pub mod new_debit_transaction;

/// Servicio de la API que persiste
/// los datos de todos los clientes y
/// sus balances en un archivo con
/// formato:
/// "DDMMAAAA_<Número de archivo>.DAT"
/// Y luego deja los saldos de todos
/// los clientes con valor 0.
pub mod store_balances;

/// Servicio de la API que recibe un ID
/// de cliente y devuelve su saldo.
pub mod client_balance;
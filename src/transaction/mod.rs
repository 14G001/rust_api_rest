/// Contiene los diferentes métodos de
/// pago de las transacciones y una
/// función para actualizar el saldo
/// del cliente en base al mismo.
pub mod pay_ways;

/// Contiene estructura y funciones para
/// procesar las distintas transacciones.
pub mod transaction;

/// Contiene funciones para procesar
/// transacciones de maneras distintas
/// en base a los tipos de datos que
/// se conzcan de la misma antes de
/// iniciar la transacción.
pub mod init_criteria;
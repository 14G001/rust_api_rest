/// Contiene los posibles métodos de
/// pago de las transacciones.
///
/// Las variantes del enum son:
///
/// * `Credit`: La transacción es de crédito.
/// * `Debit`: La transacción es de débito.
#[derive(Debug, Clone)]
pub enum PayWay {
    Credit,
    Debit
}
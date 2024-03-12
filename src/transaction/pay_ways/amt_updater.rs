use crate::transaction::pay_ways::pay_way::PayWay;

/// Devuelve el saldo nuevo del cliente
/// en base al monto y método de pago
/// de la transacción.
/// 
/// # Argumentos
/// * `curr_cli_balance`: saldo del cliente antes de actualizar su monto por la transacción.
/// * `tran_amount`: monto de la transacción.
/// * `pay_way`: método de pago de la transacción.
///
/// # Devuelve
/// Saldo nuevo del cliente.
pub fn get_new_client_amt(curr_cli_balance: f64, tran_amount: f64, pay_way: PayWay) -> f64 {
    curr_cli_balance + match pay_way {
        PayWay::Credit => tran_amount,
        PayWay::Debit  => tran_amount * (-1.0),
        _              => 0.0
    }
}
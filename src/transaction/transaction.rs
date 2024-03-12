use {chrono::{Utc, DateTime}, crate::{client::client::Client,
    db::{class::ram_db::ram_db::RamDbConn, error::{DbError, ok}},
    transaction::pay_ways::{pay_way::PayWay, amt_updater::get_new_client_amt}
}};

/// Estructura con los datos de una tranacción.
///
/// # Campos
///
/// * `client`: datos del cliente de la transacción.
/// * `date_time`: fecha y hora de la transacción.
/// * `amount`: monto de la transacción.
/// * `pay_way`: método de pago de la transacción.
pub struct Transaction {
    client: Client,
    date_time: DateTime<Utc>,
    amount: f64,
    pay_way: PayWay
}
impl Transaction {
    /// Crea una transacción con todos sus
    /// datos sin ninguno de sus datos
    /// definido y lo devuelve.
    ///
    /// # Devuelve
    /// La transacción creada.
    pub fn new() -> Self {
        Transaction {
            client    : Client::new(),
            date_time  : Utc::now(),
            amount    : 0.0,
            pay_way   : PayWay::Credit
        }
    }
    /// Inicia los datos de una transacción
    /// a partir de el ID del cliente,
    /// monto y método de pago de la misma.
    /// 
    /// # Argumentos
    /// * `transaction`: datos de la transacción a iniciar.
    /// * `client_id`: ID del cliente de la transacción.
    /// * `amount`: monto de la transacción.
    /// * `pay_way`: método de pago de la transacción.
    ///
    /// # Devuelve
    /// Tipo de error en caso de que haya ocurrido
    /// alguno al intentar obtener los datos del
    /// cliente de la transacción.
    fn init_data(transaction: &mut Transaction, client_id: u8, amount: f64, pay_way: PayWay) -> DbError {
        let result = RamDbConn::get_client(&mut transaction.client, client_id);
        if !ok(&result) {
            return result;
        }
        transaction.amount = amount;
        transaction.pay_way = pay_way;
        DbError::None
    }
    /// Procesa una transacción en base a
    /// los parámetros recibidos.
    /// 
    /// # Argumentos
    /// * `this`: Datos de la transacción.
    /// * `client_id`: ID del cliente.
    /// * `amount`: monto de la transacción.
    /// * `pay_way`: método de pago de la transacción.
    ///
    /// # Devuelve
    /// Tipo de error en caso de que haya ocurrido
    /// al procesar la transacción.
    pub fn process(this: &mut Transaction, client_id: u8, amount: f64, pay_way: PayWay) -> DbError {
        let mut result = Self::init_data(this, client_id, amount, pay_way);
        if !ok(&result) {
            return result;
        }
        let new_balance: f64 = get_new_client_amt(
            this.client.balance, this.amount, this.pay_way.to_owned());
        result = RamDbConn::update_client_balance(this.client.id, new_balance);
        if !ok(&result) {
            return result;
        }
        DbError::None
    } 
}
use crate::transaction::{transaction::Transaction, pay_ways::pay_way::PayWay};

pub async fn credit_transaction(client_id: u64, amount: f64) -> String {
    Transaction::process(
        client_id, amount, PayWay::Credit
    ).await
}
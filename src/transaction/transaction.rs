use std::any::Any;

use crate::{clients::{db::{get::get_client, update_amount::update_client_amount}, client::Client},
    transaction::pay_ways::pay_way::PayWay,
    errors::db::process_query_result};
use chrono::{Utc, DateTime};
use serde_json::from_str;

pub struct Transaction {
    pub client: Option<Client>,
    pub date: DateTime<Utc>,
    pub amount: f64,
    pub pay_way: PayWay,
    pub error: String
}
impl Transaction {
    async fn new(cli_id: u64, __amount: f64, __pay_way: PayWay) -> Transaction {
        let mut transaction = Transaction {
            client:     None,
            date:       Utc::now(),
            amount:     __amount,
            pay_way:    __pay_way,
            error:      "".to_string()
        };
        let __error: String = "".to_string();
        let result: Result<String, rusqlite::Error> = get_client(cli_id).await;
        let result_ref = result.as_ref();
        if result_ref.err() != None {
            transaction.error = process_query_result("Error getting client: ".to_string(), result_ref.err());
            return transaction;
        }
        transaction.client = Some(from_str::<Client>(result_ref.unwrap()).unwrap());
        transaction
    }
    pub async fn process(cli_id: u64, amount: f64, pay_way: PayWay) -> String {
        let transaction: Transaction = Self::new(cli_id, amount, pay_way).await;
        let this: &Transaction = &transaction;
        if transaction.error != "" {
            return transaction.error;
        }
        let client: &Client = this.client.as_ref().unwrap();
        println!("ID: {} - BALANCE: {}", client.id, client.balance);
        let mut new_amt: f64 = client.balance;
        match this.pay_way {
            PayWay::Credit => new_amt += this.amount,
            PayWay::Debit  => new_amt -= this.amount
        }
        if new_amt < 0.0 {
            return "Error; client´s money isn´t enough to perform that transaction".to_string();
        }
        let result = update_client_amount(client.id, new_amt).await;
        
        format!("New client´s amount is: {}$", new_amt)
    } 
}
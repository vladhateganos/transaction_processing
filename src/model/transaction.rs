use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
    Default,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub transaction_type: TransactionType,
    pub client_id: i16,
    pub transaction_id: i32,
    pub amount: Option<f32>,
}

pub const TRANSACTION_FIELDS: usize = 4;

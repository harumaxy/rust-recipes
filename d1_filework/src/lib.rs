pub mod error;
use error::*;

use serde_derive::*;

#[derive(Deserialize, Serialize, Debug)]
pub struct Transaction {
    from: String,
    to: String,
    amount: u64,
}

pub fn get_transactions(fname: &str) -> Result<Vec<Transaction>, TransactionError> {
    let s = match std::fs::read_to_string(fname) {
        Ok(v) => v,
        Err(err) => return Err(err.into()),
    };
    let t: Vec<Transaction> = match serde_json::from_str(&s) {
        Ok(v) => v,
        Err(err) => return Err(err.into()),
    };
    Ok(t)
}

pub fn get_transactions_b(fname: &str) -> Result<Vec<Transaction>, TransactionError> {
    let json_str = std::fs::read_to_string(fname)?;
    let json = serde_json::from_str(&json_str)?;
    Ok(json)
}

pub fn get_first_transaction_for(fname: &str, uname: &str) -> Result<Transaction, failure::Error> {
    let trans = get_transactions(fname)?;
    let first_tran = trans.into_iter().find(|t| t.from == uname);
    first_tran.ok_or(TransactionError::Mess("Could not find transaction with that name").into())
}

extern crate d1_filework;

use d1_filework::error::TransactionError;
use d1_filework::*;
use failure::Error;

fn main() -> Result<(), Error> {
    println!("Hello, world!");
    let trans = get_transactions_b("test_data/transactions.json")?;
    for t in trans {
        println!("{:?}", t)
    }

    let t = get_first_transaction_for("test_data/transactions.json", "Terry");
    match t {
        Ok(v) => println!("{:?}", v),
        Err(e) => println!("{}, {}", e, e.backtrace()),
    };
    Ok(())
}

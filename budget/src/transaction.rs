extern crate rustc_serialize;

/// This struct represents a transaction, either recurring or one-time, which can be applied to an
/// account, either adding to or subtracting from the balance of that account
#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct Transaction {
    /// The amount of the transaction, positive for a deposit, negative for a withdrawl
    pub amount: i64,
    /// The day of the month on which this transaction occurs
    pub recur_day: Option<u8>,
}

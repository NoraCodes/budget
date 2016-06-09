extern crate time;
pub use transaction::Transaction;

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct Account {
    /// The root recurring payment for this account.
    /// This is the date on which the account resets.
    pub payday: Option<Transaction>,
    /// These are the transactions for the account, historical
    /// and scheduled. Because scheduling is based on day of the month,
    /// you cannot schedule one-time transactions.
    pub transactions: Vec<Transaction>,
    /// The account's balance, in the currency's minimal unit (i.e., cents). 
    /// If negative, the account is in debt.
    pub balance: i64,
}

impl Account {
    fn apply_transaction(&mut self, trans: &mut Transaction) {
        // Apply the transaction to us
        self.balance += trans.amount;
        // And let the transaction know it's been applied
        trans.complete = true;
    }

}

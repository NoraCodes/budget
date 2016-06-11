extern crate time;
pub use transaction::Transaction;

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct Account {
    /// The root recurring payment for this account.
    /// This is the date on which the account resets.
    pub payday: Transaction,
    /// These are the transactions for the account, historical
    /// and scheduled. Because scheduling is based on day of the month,
    /// you cannot schedule one-time transactions.
    pub transactions: Vec<Transaction>,
    /// The account's balance, in the currency's minimal unit (i.e., cents). 
    /// If negative, the account is in debt.
    pub balance: i64,
    /// The last month in which this was reset
    pub last_reset: i32,
}

impl Account {
    pub fn new(payday: Transaction, 
               balance: i64) -> Account 
    {
        let mut new_account = Account {
            payday: payday,
            transactions: Vec::<Transaction>::new(),
            balance: balance,
            last_reset: 0,
        };

        // Now, we need to update the dates 
        // We're pretending to have executed the payday transaction
        //  last month.
        //  This means that the user can start using budget at any time during the month, not just
        //  on payday
        let this_month = time::now().tm_mon;
        if this_month == 0 {
            new_account.last_reset = 11;
        } else {
            new_account.last_reset = this_month - 1;
        }

        return new_account;

    }

    pub fn apply_transaction(&mut self, trans: &Transaction) {
       // Apply the transaction to us
        self.balance += trans.amount;
    }

    /// Totally reset the state of the Account
    pub fn reset (&mut self) {
        // Reset the state of everything
        self.balance = 0;
        // Now apply the payday transaction
        self.balance += self.payday.amount;
        self.last_reset = time::now().tm_mon;

        // Finally, apply each transaction in turn
        for transaction in &self.transactions {
            self.balance += transaction.amount;
        }

    }

    pub fn update(&mut self) -> Result<(), String>{
        // Get the date and time
        let t = time::now();

        // If the payday account has no recur date, the account
        // is in a suspended state and should not be updated 
        let payday_recur_day  = match self.payday.recur_day {
            Some(i) => i,
            None => return Err(String::from("Account is suspended."))
        };

        // Check if we need to reset the account
        // If it's payday and we haven't done it yet this month
        if (payday_recur_day as i32) == t.tm_mday && self.last_reset != t.tm_mon {
            self.reset();
        }
        return Ok(());

    }


}

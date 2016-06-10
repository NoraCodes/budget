use account::*;
use commands::*;
use data::*;

/// Sets up the initial config file with an initial transaction
pub fn do_setup(config_file_path: &str, payday_amount: &i64, payday_day: &u8) 
    -> Result<(), String>
{
    // We're setting up a new config file
    // First, create the payday transaction
    let payday = Transaction {
        amount: payday_amount.clone(),
        recur_day: Some(payday_day.clone()),
        complete: false,
    };
    // Create the new account
    let empty_acc = Account {
        payday: Some(payday),
        transactions: Vec::<Transaction>::new(),
        balance: 0,
    };
    match write_account(&config_file_path, &empty_acc){
        Ok(_) => return Ok(()),
        Err(s) => return Err(s),
    };
}

/// Match and dispatch on commands except for setup
/// This function provides a common environment that allows each function
/// to not have to reuse file opening code
pub fn do_other(config_file_path: &str, command: &Command) 
    -> Result<(), String> {
    println!("{:?}", command);
    // Open read the config file
    match read_account(&config_file_path) {
        Ok(account) => println!("{:?}", account),
        Err(s) => return Err(s),
    };
    // dummy return
    return Ok(());
}


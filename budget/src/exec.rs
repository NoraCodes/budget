use account::*;
use commands::*;
use data::*;

/// Sets up the initial config file with an initial transaction
pub fn do_setup(config_file_path: &str, payday_amount: i64, payday_day: u8) 
    -> Result<(), String>
{
    // We're setting up a new config file
    // First, create the payday transaction
    let payday = Transaction {
        amount: payday_amount,
        recur_day: Some(payday_day),
    };
    // Create the new account
    let empty_acc = Account::new(payday, 0);
    match write_account(&config_file_path, &empty_acc){
        Ok(_) => return Ok(()),
        Err(s) => return Err(s),
    };
}

/// Match and dispatch on commands except for setup
/// This function provides a common environment that allows each function
/// to not have to reuse file opening code
pub fn do_other(config_file_path: &str, command: Command) 
    -> Result<(), String> {
        //TODO: Remove unnecessary printing
    println!("{:?}", command);
    // Open read the config file
    let mut account = match read_account(&config_file_path) {
        Ok(a) => a,
        // All these commands require opening the config file, so if we can't 
        // get to it, we have to abort
        Err(s) => return Err(s),
    };

    // Once we have the guarantee of the command (which this function requires to be called) and of
    // the data from the config file, we can simply dispatch to the other do_ functions, since
    // their return type matches that of this function
    let command_output = match command {
        Command::Status => do_status(&account),
        Command::OneTime(amount) => do_one_time(&mut account, 
                                                       amount),
        _ => return Err(String::from("Not implemented: ") 
                        + &format!("{:?}", command)),
    };

    // If the command returned an error, we have a problem
    match command_output {
        Ok(()) => (),
        Err(s) => return Err(s)
    };

    // Update the account before saving
    match account.update() {
        Ok(_) => (),
        Err(s) => return Err(s),
    };

    // Write out the account to the config file.
    match write_account(&config_file_path, &account) {
        Ok(_) => (),
        Err(s) => return Err(s),
    };
    return Ok(());
}

fn do_status(account: &Account) -> Result<(), String> 
{
    println!("{:?}", account);
    return Ok(());
}

fn do_one_time(account: &mut Account, amount: i64)  -> Result<(), String>
{
    let mut transaction = Transaction {
        amount: amount,
        recur_day: None,
    };

    account.apply_transaction(&mut transaction);
    return Ok(());
}



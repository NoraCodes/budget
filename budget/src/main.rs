extern crate time;
extern crate rustc_serialize;
extern crate app_dirs;

use std::env::args;

mod transaction;
mod account;
mod system;
mod commands;

use account::*;
use commands::*;
use system::*;


fn main() {
    // Allow changing this with a command line option
    let default_config_name = "budget.cfg";

    // Collect arguments into a vector
    // We assume no more than 3 args + program name
    // (The consequences are not dire if we are wrong)
    let mut arguments = Vec::with_capacity(4);
    for argument in args() {
        arguments.push(argument);
    }
    
    // Count how many args we have
    let n_args = arguments.len();

    // Parse our arguments into the Command enum symbolic representation
    // FIXME: This is inelegant.
    let parsed = match n_args {
        1 => args_to_command((None, 
                              None, 
                              None)),
        2 => args_to_command((Some(arguments[1].as_ref()),
                              None,
                              None)),
        3 => args_to_command((Some(arguments[1].as_ref()),
                              Some(arguments[2].as_ref()),
                              None)),
        // By making this the default case we handle too many
        //  arguments by simply ignoring the extras
        _ => args_to_command((Some(arguments[1].as_ref()),
                              Some(arguments[2].as_ref()),
                              Some(arguments[3].as_ref()))),
    };

    // TODO: Replace with a match for the actual operations
    match parsed {
        Ok(command) => println!("{:?}", command),
        Err(r) => println!("{}", r),
    };
    
    // Get the config file path, or say why not
    let config_file_path = match get_config_file_path(&default_config_name) {
        Ok(s) => s,
        Err(r) => panic!("{}", r)
    };
    println!("{}", config_file_path);

    // Open read the config file
    match read_account(&config_file_path) {
        Ok(account) => println!("{:?}", account),
        Err(r) => panic!("{}", r)
    };
}


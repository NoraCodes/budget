extern crate time;
extern crate rustc_serialize;
extern crate app_dirs;

use rustc_serialize::json;

mod transaction;
mod account;
mod system;
mod commands;

use account::*;
use commands::*;

fn main() {
    use std::env;
    // Collect arguments into a vector
    // We assume no more than 3 args + program name
    // (The consequences are not dire if we are wrong)
    let mut arguments = Vec::with_capacity(4);
    for argument in env::args() {
        arguments.push(argument);
    }
    
    // Count how many args we have
    let n_args = arguments.len();
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

    match parsed {
        Ok(command) => println!("{:?}", command),
        Err(r) => println!("{}", r),
    };

    
}


extern crate time;
extern crate rustc_serialize;

use rustc_serialize::json;

mod transaction;
mod account;

use account::*;

fn main() {
    use std::env;
    // Collect arguments into a vector
    let mut arguments = Vec::new();
    for argument in env::args() {
        arguments.push(argument);
    }
    let t = Transaction {
        amount: 100,
        recur_day: None,
        complete: true,
    };

    println!("{:?}", &t);
    let enc = json::encode(&t).unwrap();
    println!("{}", &enc);
    let dec: Transaction = json::decode(&enc).unwrap();
    println!("{:?}", dec);
}


extern crate time;
extern crate rustc_serialize;

use rustc_serialize::json;

mod transaction;
mod account;
mod system;

use account::*;

fn main() {
    use std::env;
    // Collect arguments into a vector
    let mut arguments = Vec::new();
    for argument in env::args() {
        arguments.push(argument);
    }
}


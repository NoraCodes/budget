use system::{read_file, write_file};
use rustc_serialize::json;
use account::Account;

/// Read in a json file to an Account
pub fn read_account(file_path: &str) -> Result<Account, String>{

    // Try to read the file
    let s = match read_file(&file_path) {
        Ok(s_val) => s_val,
        Err(r) => return Err(r)
    };
    
    // Try to parse the file into an Account
    match json::decode(&s) {
        Ok(decoded) => return Ok(decoded),
        Err(r) => return Err(String::from("Failed to parse config file ") 
                             + file_path
                             + ": " + r.to_string().as_ref())
    };
}


/// Write an account to a json file
pub fn write_account(file_path: &str, account: &Account) -> Result<(), String>{
    // Try to encode the Account into json
    let s = match json::encode(account) {
        Ok(e_val) => e_val,
        Err(r) => return Err(String::from("Failed to encode Account: ")
                             + r.to_string().as_ref()),
    };

    // Try to write the json to the file
    match write_file(&file_path, &s) {
        Ok(_) => return Ok(()),
        Err(r) => return Err(r)
    };
    
}

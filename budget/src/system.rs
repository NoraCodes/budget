//! System functions for `budget`
use account::Account;
use app_dirs::*;
use std::path::PathBuf;

use std::io::prelude::*;
use std::fs::File;

use rustc_serialize::json;

/// Acquire the path to the config file
pub fn get_config_file_path(file_name: &str) -> Result<String, String> {
    // Try getting the home directory string, and concatenate it with the filename
    let mut path: PathBuf;
    match get_app_dir(AppDirType::UserConfig) {
        Ok(config_dir_path) => path = config_dir_path,
        Err(_) => return Err(String::from("Failed to get user config directory!")),
    }
    
    path.push(file_name);
    return Ok(path.to_string_lossy().into_owned());
}

/// Read in a json file to an Account
pub fn read_account(file_path: &str) -> Result<Account, String>{

    // Try to open the file
    let mut f = match File::open(file_path) {
        Ok(f_val) => f_val,
        Err(_) => return Err(String::from("Failed to open config file!"))
    };

    // Try to read the file into a buffer
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => (),
        Err(_) => return Err(String::from("Failed to read config file!"))
    };
    
    // Try to parse the file into an Account
    match json::decode(&s) {
        Ok(decoded) => return Ok(decoded),
        Err(_) => return Err(String::from("Failed to parse config file!"))
    };

    
}

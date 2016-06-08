//! System functions for `budget`
use app_dirs::*;
use std::path::PathBuf;

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


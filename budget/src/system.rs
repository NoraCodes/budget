//! System functions for `budget`

/// Acquire the home directory's path
fn get_homedir_string() -> Option<String> {
    use std::env;
    // Attempt to retrieve the home directory, if we fail, return None
    match env::home_dir() {
        // Take the homedir path, convert it to a string, and append the filename
        Some(path) => Some(String::from(
                path.to_string_lossy().into_owned())),
        None => None
    }
}

/// Acquire the path to the config file
pub fn get_config_file_name(file_name: &str) -> Option<String> {
    // TODO: Reimplement this to not suck

    // Try getting the home directory string, and concatenate it with the filename
    match get_homedir_string() {
        Some(path) => Some(path + "/" + &file_name),
        None => None
    }
}


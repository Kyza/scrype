use std::{ffi::OsString, fs, path::Path};

use home::home_dir;

pub fn get_config_directory() -> OsString {
    match home_dir() {
        Some(home) => {
            let dir = Path::new(&home).join(".scrype").into();
            fs::create_dir_all(&dir).unwrap_or(());
            return dir;
        }
        None => {
            panic!("Failed to get home directory. Something has gone horribly wrong.");
        }
    }
}

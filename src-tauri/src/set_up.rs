use std::env;
use std::fs;
use std::path::PathBuf;

pub fn set_up() {
    let appdata = env::var("APPDATA")
    .or_else(|_| env::var("HOME"))
    .unwrap();

    let folder_path: PathBuf = [appdata, "Ancrypt".to_string(), "Vaults".to_string()].iter().collect();

    if !folder_path.exists() {
        fs::create_dir_all(&folder_path).expect("Failed to create folder")
    }
}
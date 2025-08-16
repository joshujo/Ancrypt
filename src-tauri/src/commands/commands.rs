use std::{env, fs::{self, create_dir_all}, path::PathBuf};

use serde::Serialize;
use tauri::{async_runtime::Mutex, State};

use crate::vault::vault::{attempt_unlock, init, Unlocked, Vault};

#[derive(Serialize, Clone)]
pub struct VaultSurfaceData {
    name: String,
    id: u32
}

type Error = String;

#[derive(Default)]
pub struct VaultCollection {
    pub vaults: Vec<VaultSurfaceData>,
    pub open_vault: Option<Vault<Unlocked>>
}


#[tauri::command]
pub async fn request_vaults(state: State<'_, Mutex<VaultCollection>>) -> Result<Vec<VaultSurfaceData>, Error> {
    let appdata = env::var("APPDATA")
    .or_else(|_| env::var("HOME"))
    .unwrap();

    let path: PathBuf = [appdata, "Ancrypt".to_string(), "Vaults".to_string()].iter().collect();

    create_dir_all(&path).map_err(|e| e.to_string())?;

    let mut state = state.lock().await;

    let mut vaults = Vec::new();
    let mut id = 0;
    if let Ok(entries) = fs::read_dir(&path) {
        for entry in entries.flatten() {
            let dir = entry.path();
            if dir.is_file() {
                if let Some(extension) = dir.extension() {
                    if extension == "ANCRYPT" {
                        if let Some(file_name) = dir.file_stem().and_then(|n| n.to_str()) {
                            vaults.push(VaultSurfaceData {
                                name: file_name.to_string(),
                                id,
                            });
                            id += 1;
                        }
                    }
                }
            }
        }
    }

    state.vaults = vaults.clone();

    Ok(vaults)
}

#[derive(Serialize, Clone)]
pub struct VaultResult {
    success: bool,
    message: Option<String>
}

#[tauri::command(rename_all = "snake_case")]
pub async fn create_vault(state: tauri::State<'_, Mutex<VaultCollection>>, vault_name: String, vault_password: String) -> Result<VaultResult, ()> {
    if vault_name.len() < 1 || vault_password.len() < 1 {
        return Ok(VaultResult {
            success: false,
            message: Some(String::from("Invalid vault name and/or password. Try again!"))
        })
    }

    let (name, password) = {
        (vault_name.trim(),
        vault_password.trim())
    };

    let new = Vault::new();
    let vault = new.create_new(password, name);
    state.lock().await.open_vault = Some(vault);

    Ok(VaultResult { success: true, message: None })
}

#[tauri::command(rename_all = "snake_case")]
pub async fn open_vault(state: tauri::State<'_, Mutex<VaultCollection>>, id: u32, password: String) -> Result<VaultResult, ()>{
    let mut lock = state.lock().await;

    let vault = match lock.vaults.iter().find(|&x| x.id == id) {
        Some(ok) => ok,
        None => return Ok(VaultResult { success: false, message: Some(String::from("Something went wrong")) })
    };

    let pending = match init(&vault.name) {
        Some(ok) => ok,
        None => return Ok(VaultResult { success: false, message: Some(String::from("Something went wrong")) })
    };

    let unlocked = match attempt_unlock(pending, &password) {
        Ok(ok) => ok,
        Err(_) => return Ok(VaultResult { success: false, message: Some(String::from("Incorrect Password")) })
    };

    lock.open_vault = Some(unlocked);
    

    Ok(VaultResult { success: true, message: None })
}
use std::{env, fs::{self, create_dir_all}, path::PathBuf};

use serde::Serialize;
use tauri::{async_runtime::Mutex, State};

use crate::vault::vault::{Unlocked, Vault};

#[derive(Serialize, Clone)]
pub struct VaultSurfaceData {
    name: String,
    id: u32
}

type Error = String;

#[derive(Default)]
pub struct VaultCollection {
    pub vaults: Vec<VaultSurfaceData>
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
pub struct CreateVaultResult {
    success: bool,
    message: Option<String>
}

#[derive(Default)]
pub struct OpenVault(pub Option<Vault<Unlocked>>);

#[tauri::command(rename_all = "snake_case")]
pub async fn create_vault(state: tauri::State<'_, Mutex<OpenVault>>, vault_name: String, vault_password: String) -> Result<CreateVaultResult, ()> {
    if vault_name.len() < 1 || vault_password.len() < 1 {
        return Ok(CreateVaultResult {
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
    state.lock().await.0 = Some(vault);

    Ok(CreateVaultResult { success: true, message: None })
}
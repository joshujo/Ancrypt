use std::{ env, fs::{ self, create_dir_all }, path::PathBuf };

use serde::Serialize;
use tauri::{ async_runtime::Mutex, Manager, State };
use tauri_plugin_clipboard_manager::ClipboardExt;
use tokio::time;
use zeroize::Zeroize;

use crate::vault::vault::{ attempt_unlock, delete_vault, init, Unlocked, Vault };

static CHAR_SET: &[char] = &[
    'a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z',
    'A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z',
    '1','2','3','4','5','6','7','8','9','0',
    '!','@','#','$','%','^','&','*','(',')','-','=','_','+','[',']','\\','{','}','|',';',':','\'','"','<','>','?',',','.','/','`','~'
];

#[derive(Serialize, Clone)]
pub struct VaultSurfaceData {
    name: String,
    id: u32,
}

#[derive(Clone)]
struct OpenVault {
    vault: Vault<Unlocked>,
    name: String,
}

impl Drop for OpenVault {
    fn drop(&mut self) {
        self.name.zeroize();
        self.vault.zeroize();
    }
}

type Error = String;

#[derive(Default)]
pub struct VaultCollection {
    pub vaults: Vec<VaultSurfaceData>,
    open_vault: Option<OpenVault>,
}

impl VaultCollection {
    pub fn clean(&mut self) {
        let lock = &mut self.open_vault;
        if let Some(lock) = lock.take() {
            drop(lock);
        }
    }
}

impl Drop for VaultCollection {
    fn drop(&mut self) {
        self.clean();
    }
}

#[tauri::command]
pub async fn request_vaults(
    state: State<'_, Mutex<VaultCollection>>
) -> Result<Vec<VaultSurfaceData>, Error> {
    let appdata = env
        ::var("APPDATA")
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
    message: Option<String>,
}

#[tauri::command(rename_all = "snake_case")]
pub async fn create_vault(
    state: tauri::State<'_, Mutex<VaultCollection>>,
    vault_name: String,
    vault_password: String
) -> Result<VaultResult, ()> {
    if vault_name.len() < 1 || vault_password.len() < 1 {
        return Ok(VaultResult {
            success: false,
            message: Some(String::from("Invalid vault name and/or password. Try again!")),
        });
    }

    let (vault_name, mut password) = { (vault_name.trim().to_string(), vault_password.trim().to_string()) };

    let new = Vault::new();
    let vault = new.create_new(&password, &vault_name);

    let open_vault = OpenVault {
        vault,
        name: vault_name,
    };

    state.lock().await.open_vault = Some(open_vault);
    password.zeroize();

    Ok(VaultResult { success: true, message: None })
}

#[tauri::command(rename_all = "snake_case")]
pub async fn open_vault(
    state: tauri::State<'_, Mutex<VaultCollection>>,
    id: u32,
    password: String
) -> Result<VaultResult, ()> {
    let mut lock = state.lock().await;

    let vault = match lock.vaults.iter().find(|&x| x.id == id) {
        Some(ok) => ok,
        None => {
            return Ok(VaultResult {
                success: false,
                message: Some(String::from("Something went wrong")),
            });
        }
    };

    let pending = match init(&vault.name) {
        Some(ok) => ok,
        None => {
            return Ok(VaultResult {
                success: false,
                message: Some(String::from("Something went wrong")),
            });
        }
    };

    let unlocked = match attempt_unlock(pending, &password) {
        Ok(ok) => ok,
        Err(_) => {
            return Ok(VaultResult {
                success: false,
                message: Some(String::from("Incorrect Password")),
            });
        }
    };

    let open_vault = OpenVault {
        vault: unlocked,
        name: vault.name.clone(),
    };

    lock.open_vault = Some(open_vault);

    Ok(VaultResult { success: true, message: None })
}

#[tauri::command(rename_all = "snake_case")]
pub async fn retrieve_password_list(
    state: State<'_, Mutex<VaultCollection>>
) -> Result<Vec<String>, ()> {
    let lock = state.lock().await;
    let vault = &lock.open_vault.as_ref().unwrap().vault;

    let open_vault = vault;

    Ok(open_vault.list_password())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn copy_to_clipboard(
    app: tauri::AppHandle,
    state: State<'_, Mutex<VaultCollection>>,
    password: String
) -> Result<(), ()> {
    let lock = state.lock().await;
    let vault = &lock.open_vault.as_ref().unwrap().vault;

    let content = vault.retrieve_password(&password).unwrap();

    app.clipboard().write_text(content).unwrap();

    let app_handle = app.app_handle().clone();

    tauri::async_runtime::spawn(async move {
        tokio::time::sleep(time::Duration::from_secs(30)).await;
        app_handle.clipboard().clear().unwrap();
    });

    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn add_password(
    state: State<'_, Mutex<VaultCollection>>,
    name: String,
    password: String
) -> Result<(), String> {
    if name.len() < 1 || password.len() < 1 {
        return Err(String::from("You need a password and a password name"));
    } else if name.len() < 1 {
        return Err(String::from("You need a password name"));
    } else if password.len() < 1 {
        return Err(String::from("You need a password"));
    }

    let mut lock = state.lock().await;
    let vault_name = lock.open_vault.as_ref().unwrap().name.clone();
    let vault = &mut lock.open_vault.as_mut().unwrap().vault;

    let password = password.trim().to_string();

    vault
        .insert_password(name, password, &vault_name)
        .map_err(|_| String::from("Something went wrong inserting your password"))?;

    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn delete_password(
    state: State<'_, Mutex<VaultCollection>>,
    name: String
) -> Result<(), String> {
    let mut lock = state.lock().await;
    let vault_name = lock.open_vault.as_ref().unwrap().name.clone();
    let vault = &mut lock.open_vault.as_mut().unwrap().vault;

    vault.delete_password(name, &vault_name).map_err(|e| String::from(e))?;

    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn lock_vault(
    app: tauri::AppHandle,
    state: tauri::State<'_, Mutex<VaultCollection>>
) -> Result<(), ()> {
    let mut lock = state.lock().await;
    lock.clean();
    drop(lock);
    app.app_handle().clipboard().clear().unwrap();
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn request_delete_vault(
    state: State<'_, Mutex<VaultCollection>>,
    vault: u32
) -> Result<(), ()> {
    let lock = state.lock().await;

    let vault = lock.vaults.iter().find(|&x| x.id == vault);

    match vault {
        Some(x) => {
            let name = &x.name;
            delete_vault(name).unwrap();
            return Ok(())
        },
        None => {
            return Err(());
        }
    }
}

#[tauri::command(rename_all = "snake_case")]
pub fn five_number_rng() -> u32 {
rand::random_range(111111..1000000)
}

#[tauri::command(rename_all = "snake_case")]
pub fn clear_clipboard(app: tauri::AppHandle) {
    app.app_handle().clipboard().clear().unwrap();
}

#[tauri::command(rename_all = "snake_case")]
pub async fn generate_password(state: State<'_, Mutex<VaultCollection>>, name: String) -> Result<(), String> {
    let mut lock = state.lock().await;
    let vault = lock.open_vault.as_mut().unwrap();

    let name_unique = vault.vault.list_password().iter().find(|&x| x == &name).is_none();

    if name.len() > 1 && name_unique {
        let password: Vec<char> = rand::random_iter().take(18).collect();
        let string: String = password.iter().collect();
        vault.vault.insert_password(name, string, &vault.name).unwrap();
        Ok(())
    } else if name.len() < 1 {
        Err(String::from("You need to insert a name for your password."))
    } else {
        Err(String::from("You need to use a unique password name."))
    }
}
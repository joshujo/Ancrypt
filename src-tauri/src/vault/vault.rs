//Hello anyone looking at this code, some of it might not make too much sense due to it being adapted from a CLI only version of this application
use bincode::{
    self,
    config::{self, Configuration},
    decode_from_slice, encode_to_vec, Decode, Encode,
};
use std::env::var_os;
use std::fs;
use std::{collections::HashMap, marker::PhantomData, path::PathBuf};
use zeroize::Zeroize;

use crate::vault::{master_password::{empty_master_password, init_master_password, Pbkdf2Component}};

pub enum RetrieveResult {
    Success,
    Failure,
}

#[derive(Encode, Decode, Debug)]
pub struct Locked;
#[derive(Encode, Decode, Debug)]
pub struct Unlocked;
#[derive(Encode, Decode, Debug)]
pub struct Pending;

pub trait LockState {
    fn is_locked() -> bool;
}

impl LockState for Locked {
    fn is_locked() -> bool {
        true
    }
}

impl LockState for Unlocked {
    fn is_locked() -> bool {
        false
    }
}

#[derive(Clone, Encode, Decode, Debug)]
pub struct Vault<State = Pending> {
    passwords: HashMap<String, String>,
    pbkdf2_component: Pbkdf2Component,
    state: PhantomData<State>,
}

impl<Unlocked> Zeroize for Vault<Unlocked> {
    fn zeroize(&mut self) {
        self.passwords.iter_mut().for_each(|(_, y)| {
            y.zeroize();
        });   

        self.pbkdf2_component.derived_key.zeroize();
        self.pbkdf2_component.encrypted_passwords.data.zeroize();
    }
}


impl Vault {
    pub fn new() -> Self {
        Vault {
            passwords: HashMap::new(),
            state: PhantomData::<Pending>,
            pbkdf2_component: empty_master_password(),
        }
    }
}

impl Vault<Locked> {
    fn unlock(self, password: &str) -> Vault<Unlocked> {
        let pbkdf2_component = init_master_password(password);

        Vault {
            passwords: self.passwords,
            pbkdf2_component: pbkdf2_component,
            state: PhantomData::<Unlocked>,
        }
    }

    pub fn retrieve_from_file(&mut self) -> RetrieveResult {
        // let path = get_data_path();
        // let file = fs::read(path).unwrap();
        // let config = config::standard();

        // let decoded: Option<(Vault<Locked>, usize)> = match decode_from_slice::<Vault<Locked>, Configuration>(&file, config) {
        //     Ok(result) => Some(result),
        //     Err(_) => {
        //         return RetrieveResult::Failure
        //     }
        // };

        //let passwords = decoded.unwrap().0.pbkdf2_component.encrypted_passwords.decrypt(self.pbkdf2_component.derived_key);
        // let (decoded, _) = decode_from_slice::<HashMap<String, String>, Configuration>(&passwords, config).unwrap();
        // self.passwords = decoded;

        return RetrieveResult::Success;
    }
}

impl Clone for Vault<Unlocked> {
    fn clone(&self) -> Self {
        Self {
            passwords: self.passwords.clone(),
            pbkdf2_component: self.pbkdf2_component.clone(),
            state: self.state.clone(),
        }
    }
}

impl Clone for Vault<Locked> {
    fn clone(&self) -> Self {
        Self {
            passwords: self.passwords.clone(),
            pbkdf2_component: self.pbkdf2_component.clone(),
            state: self.state.clone(),
        }
    }
}

impl Vault<Pending> {
    pub fn create_new(self, password: &str, vault_name: &str) -> Vault<Unlocked> {
        let pbkdf2_component = init_master_password(password);

        let mut passwords = Vault {
            passwords: HashMap::new(),
            pbkdf2_component,
            state: PhantomData::<Unlocked>,
        };

        passwords.save_to_file(vault_name);

        passwords
    }

    fn retrieve_from_file(&mut self, vault_name: &str) {
        let path = get_data_path(vault_name);
        let file = fs::read(path).unwrap();
        let config = config::standard();

        let (decoded, _) = match decode_from_slice(&file, config) {
            Ok(r) => r,
            Err(_) => {
                println!("Current save file is probabily incompatible with this version");
                panic!()
            }
        };

        *self = decoded;
    }

    fn retrieved(self) -> Vault<Locked> {
        Vault {
            passwords: self.passwords,
            pbkdf2_component: self.pbkdf2_component,
            state: PhantomData::<Locked>,
        }
    }
}

impl Vault<Unlocked> {
    pub fn retrieve_password(&self, name: &str) -> Result<String, &str> {
        match self.passwords.contains_key(name) {
            true => {
                return Ok(self.passwords.get(name).unwrap().to_string());
            }
            false => {
                return Err("No entry of that name found");
            }
        }
    }

    pub fn insert_password(
        &mut self,
        name: String,
        password: String,
        vault_name: &str,
    ) -> Result<(), &str> {
        match self.passwords.contains_key(&name) {
            true => {
                return Err("Name already in use");
            }
            false => {
                self.passwords.insert(name, password);
                self.save_to_file(vault_name);
                return Ok(());
            }
        }
    }

    pub fn delete_password(
        &mut self,
        name: String,
        vault_name: &str
    ) -> Result<(), &str> {
        match self.passwords.contains_key(&name) {
            true => {
                self.passwords.remove(&name).unwrap();
                self.save_to_file(vault_name);
                return Ok(());
            },
            false => {
                return Err("That's not an existing password")
            }
        }
    }

    pub fn list_password(&self) -> Vec<String> {
        let mut vector = vec![];

        for (key, _) in &self.passwords {
            vector.push(key.to_string());
        }

        vector
    }

    pub fn retrieve_from_file(&mut self, vault_name: &str) {
        let path = get_data_path(vault_name);
        let file = fs::read(path).unwrap();
        let config = config::standard();

        let decoded = match decode_from_slice::<Vault<Unlocked>, Configuration>(&file, config) {
            Ok((mut result, _)) => {
                let decrypted_data = result
                    .pbkdf2_component
                    .encrypted_passwords
                    .decrypt(result.pbkdf2_component.derived_key);
                let decoded = match decode_from_slice::<HashMap<String, String>, Configuration>(
                    &decrypted_data,
                    config,
                ) {
                    Ok(r) => r.0,
                    Err(_) => HashMap::new(),
                };
                let data = Vault::<Unlocked> {
                    passwords: decoded,
                    pbkdf2_component: result.pbkdf2_component,
                    state: PhantomData::<Unlocked>,
                };
                data
            }
            Err(_) => self.clone(),
        };

        *self = decoded;
    }

    pub fn lock(mut self) -> Vault<Locked> {
        self.zeroize();
        Vault {
            passwords: HashMap::new(),
            pbkdf2_component: self.pbkdf2_component,
            state: PhantomData::<Locked>,
        }
    }
}

impl<State: LockState> Vault<State>
where
    Self: bincode::Encode,
{
    pub fn check_lock(&self) -> bool {
        State::is_locked()
    }

    fn save_to_file(&mut self, vault_name: &str) {
        let path = get_data_path(vault_name);
        let config = config::standard();
        let mut to_encrypt = Vault::<Locked> {
            passwords: self.passwords.clone(),
            pbkdf2_component: self.pbkdf2_component.clone(),
            state: PhantomData::<Locked>,
        };
        let encoded = encode_to_vec(&self.passwords, config).unwrap();
        to_encrypt.pbkdf2_component.encrypted_passwords = to_encrypt
            .pbkdf2_component
            .encrypted_passwords
            .encrypt_data(self.pbkdf2_component.derived_key, encoded);
        let encoded = encode_to_vec(&to_encrypt, config).unwrap();
        fs::write(&path, encoded).unwrap();
    }
}

pub fn check_file(vault_name: &str) -> RetrieveResult {
    let exists = fs::exists(get_data_path(vault_name)).expect("Should work");

    match exists {
        true => {
            return RetrieveResult::Success;
        }
        false => {
            return RetrieveResult::Failure;
        }
    }
}

pub fn init(vault_name: &str) -> Option<Vault<Locked>> {
    let exists = fs::exists(get_data_path(vault_name)).expect("Should work");
    let mut passwords = Vault::new();
    if exists == false {
        return None;
    } else {
        passwords.retrieve_from_file(vault_name);
        return Some(passwords.retrieved());
    }
}

fn get_data_path(vault_name: &str) -> PathBuf {
    if let Some(roaming) = var_os("APPDATA") {
        let mut dir = PathBuf::from(roaming);
        dir.push("Ancrypt");
        dir.push("Vaults");
        fs::create_dir_all(&dir).unwrap();

        let mut file_path = dir;
        file_path.push(format!("{}.ANCRYPT", vault_name));
        return file_path;
    }
    PathBuf::from("./Vault.pass")
}

pub fn attempt_unlock(
    pass: Vault<Locked>,
    password: &str,
) -> Result<Vault<Unlocked>, Vault<Locked>> {
    match pass.pbkdf2_component.verify_password(password) {
        Ok(()) => Ok(pass.unlock(password)),
        Err(_) => Err(pass),
    }
}

pub fn delete_vault(
    vault_name: &str
) -> Result<(), String> {
    let path = get_data_path(vault_name);
    fs::remove_file(path).map_err(|e| e.to_string())
}

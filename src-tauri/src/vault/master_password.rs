use std::num::NonZeroU32;

use bincode::{ Decode, Encode };
use ring::{ self, digest, pbkdf2::{ self, derive }, rand::{ self, generate } };

use crate::vault::encrypted_password::EncryptedPasswords;

static PBKDF2_ALG: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA512;
const CREDENTIAL_LEN: usize = digest::SHA512_256_OUTPUT_LEN;
pub type Credential = [u8; CREDENTIAL_LEN];

pub enum Error {
    WrongUsernameOrPassword,
}

#[derive(Clone, Encode, Decode, Debug)]
pub struct Pbkdf2Component {
    pbkdf2_iterations: NonZeroU32,
    db_salt_component: [u8; 128],

    master_password: ([u8; 128], Credential),
    pub derived_key: [u8; CREDENTIAL_LEN],

    pub encrypted_passwords: EncryptedPasswords,
}

impl Pbkdf2Component {
    fn create_password(&mut self, password: &str) {
        let rng = rand::SystemRandom::new();

        let username = generate::<[u8; 128]>(&rng).unwrap().expose();

        let mut to_store: Credential = [0u8; CREDENTIAL_LEN];
        let salt = self.salt(&username);
        pbkdf2::derive(
            PBKDF2_ALG,
            self.pbkdf2_iterations,
            &salt,
            password.as_bytes(),
            &mut to_store
        );
        self.derived_key = to_store;
        self.master_password = (username, to_store);
    }

    fn salt(&self, username: &[u8]) -> Vec<u8> {
        let mut salt = Vec::with_capacity(self.db_salt_component.len() + username.len());
        salt.extend(self.db_salt_component.as_ref());
        salt.extend(username);
        salt
    }

    pub fn verify_password(&self, attempted_password: &str) -> Result<(), Error> {
        let salt = self.salt(&self.master_password.0);
        let actual_password = &self.master_password.1;
        pbkdf2
            ::verify(
                PBKDF2_ALG,
                self.pbkdf2_iterations,
                &salt,
                attempted_password.as_bytes(),
                actual_password
            )
            .map_err(|_| Error::WrongUsernameOrPassword)
    }

    pub fn sanitise(self) -> Pbkdf2Component {
        Pbkdf2Component {
            derived_key: [0u8; CREDENTIAL_LEN],
            ..self
        }
    }
}

pub fn init_master_password(password: &str) -> Pbkdf2Component {
    let rng = rand::SystemRandom::new();
    let db_salt_component = generate::<[u8; 128]>(&rng).unwrap().expose();

    let mut pass = Pbkdf2Component {
        pbkdf2_iterations: NonZeroU32::new(600_000).unwrap(),
        db_salt_component,
        master_password: ([0; 128], [0; 32]),
        derived_key: [0; 32],
        encrypted_passwords: EncryptedPasswords::new(),
    };

    let mut out: [u8; CREDENTIAL_LEN] = [0; 32];
    derive(
        PBKDF2_ALG,
        pass.pbkdf2_iterations,
        &pass.db_salt_component,
        password.as_bytes(),
        &mut out
    );

    pass.create_password(password);
    pass.derived_key = out;
    pass.encrypted_passwords = EncryptedPasswords::initialise_data(
        pass.encrypted_passwords,
        pass.derived_key,
        vec![]
    );
    pass
}

pub fn empty_master_password() -> Pbkdf2Component {
    let rng = rand::SystemRandom::new();
    let db_salt_component = generate::<[u8; 128]>(&rng).unwrap().expose();
    Pbkdf2Component {
        pbkdf2_iterations: NonZeroU32::new(100).unwrap(),
        db_salt_component: db_salt_component,
        master_password: ([0; 128], [0; 32]),
        derived_key: [0; 32],
        encrypted_passwords: EncryptedPasswords::new(),
    }
}

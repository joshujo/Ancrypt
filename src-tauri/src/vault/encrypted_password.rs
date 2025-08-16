use bincode::{self, Decode, Encode};
use ring::{
    self,
    aead::{
        Aad, BoundKey, Nonce, NonceSequence, OpeningKey, SealingKey, UnboundKey, CHACHA20_POLY1305,
    },
    digest,
    error::Unspecified,
    rand::{generate, SystemRandom},
};

const CREDENTIAL_LEN: usize = digest::SHA512_256_OUTPUT_LEN;

#[derive(Clone, Encode, Decode, Debug)]
pub struct EncryptedPasswords {
    aad: [u8; 128],
    nonce_sequence: CounterNonce,
    pub data: Vec<u8>,
    index: u64,
}

#[derive(Clone, Copy, Encode, Decode, Debug)]
struct CounterNonce {
    nonce_bytes: [u8; 4],
    index: u64,
}

impl CounterNonce {
    fn new() -> CounterNonce {
        let rng = SystemRandom::new();
        let prefix = generate::<[u8; 4]>(&rng).unwrap().expose();

        let mut nonce_bytes = [0u8; 4];

        nonce_bytes[0..4].copy_from_slice(&prefix);

        CounterNonce {
            nonce_bytes,
            index: 0,
        }
    }
}

impl NonceSequence for CounterNonce {
    fn advance(&mut self) -> Result<ring::aead::Nonce, ring::error::Unspecified> {
        if self.index == u64::MAX {
            return Err(Unspecified);
        }
        let mut nonce: [u8; 12] = [0u8; 12];
        nonce[0..4].copy_from_slice(&self.nonce_bytes);
        nonce[4..].copy_from_slice(&self.index.to_be_bytes());

        Nonce::try_assume_unique_for_key(&nonce)
    }
}

impl EncryptedPasswords {
    pub fn new() -> EncryptedPasswords {
        let nonce_sequence = CounterNonce::new();
        let rng = SystemRandom::new();

        EncryptedPasswords {
            aad: generate::<[u8; 128]>(&rng).unwrap().expose(),
            nonce_sequence,
            data: vec![],
            index: 0,
        }
    }
}

impl EncryptedPasswords {
    pub fn initialise_data(self, key: [u8; CREDENTIAL_LEN], data: Vec<u8>) -> EncryptedPasswords {
        let unbound_key = UnboundKey::new(&CHACHA20_POLY1305, &key).unwrap();

        let mut sealing_key = SealingKey::new(unbound_key, self.nonce_sequence);

        let mut data = data;

        sealing_key
            .seal_in_place_append_tag(Aad::from(self.aad), &mut data)
            .unwrap();

        EncryptedPasswords {
            aad: self.aad,
            nonce_sequence: self.nonce_sequence,
            data: data,
            index: self.nonce_sequence.index,
        }
    }
}

impl EncryptedPasswords {
    pub fn decrypt(&mut self, key: [u8; CREDENTIAL_LEN]) -> Vec<u8> {
        let unbound_key = UnboundKey::new(&CHACHA20_POLY1305, &key).unwrap();
        let index: u64 = self.index;
        let nonce_sequence = CounterNonce {
            nonce_bytes: self.nonce_sequence.nonce_bytes,
            index: index,
        };
        let mut opening_key = OpeningKey::new(unbound_key, nonce_sequence);

        let encrypted_data = self.data.as_mut_slice();

        let data = match opening_key.open_in_place(Aad::from(self.aad), encrypted_data) {
            Ok(d) => d,
            Err(_) => {
                println!("Failed lol");
                &mut [0; 0]
            }
        };

        let data = data.to_vec();

        data
    }

    pub fn encrypt_data(&mut self, key: [u8; CREDENTIAL_LEN], data: Vec<u8>) -> EncryptedPasswords {
        let unbound_key = UnboundKey::new(&CHACHA20_POLY1305, &key).unwrap();
        self.nonce_sequence.index += 1;
        let mut sealing_key = SealingKey::new(unbound_key, self.nonce_sequence);

        let mut data = data;

        sealing_key
            .seal_in_place_append_tag(Aad::from(self.aad), &mut data)
            .unwrap();

        EncryptedPasswords {
            aad: self.aad,
            nonce_sequence: self.nonce_sequence,
            data: data,
            index: self.nonce_sequence.index,
        }
    }
}

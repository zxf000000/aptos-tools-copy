use ed25519_dalek::{ExpandedSecretKey, PublicKey, SecretKey};
use hex::ToHex;
use rand::{rngs::OsRng, Rng, RngCore, SeedableRng};
use reqwest;
use std::{
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tiny_keccak::{Hasher, Sha3};

pub struct Account {
    signing_key: SecretKey,
}

impl Account {
    pub fn new(priv_key_bytes: Option<Vec<u8>>) -> Self {
        let signing_key = match priv_key_bytes {
            Some(key) => SecretKey::from_bytes(&key).unwrap(),
            None => {
                let mut rng = rand::rngs::StdRng::from_seed(OsRng.gen());
                let mut bytes = [0; 32];
                rng.fill_bytes(&mut bytes);
                SecretKey::from_bytes(&bytes).unwrap()
            }
        };

        Account { signing_key }
    }

    pub fn address(&self) -> String {
        self.auth_key()
    }

    pub fn auth_key(&self) -> String {
        let mut sha3 = Sha3::v256();
        sha3.update(PublicKey::from(&self.signing_key).as)
    }
}

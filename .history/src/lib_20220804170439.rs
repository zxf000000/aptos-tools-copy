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
    pub fn 
}

// Copyright (c) 2023-2024 thesimplekid (BSD 3-Clause License)
// Copyright (c) 2024 Yuki Kishimoto
// Distributed under the MIT software license

//! Util

#[cfg(not(target_arch = "wasm32"))]
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use bitcoin::hashes::sha256::Hash as Sha256;
use bitcoin::hashes::Hash;
use bitcoin::secp256k1::rand::{self, Rng, RngCore};
use bitcoin::secp256k1::{All, Secp256k1};
#[cfg(target_arch = "wasm32")]
use instant::{Instant, SystemTime};
use once_cell::sync::Lazy;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub mod dhke;
pub mod hex;

#[cfg(target_arch = "wasm32")]
const UNIX_EPOCH: SystemTime = SystemTime::UNIX_EPOCH;

/// Secp256k1 global context
pub static SECP256K1: Lazy<Secp256k1<All>> = Lazy::new(|| {
    let mut ctx = Secp256k1::new();
    let mut rng = rand::thread_rng();
    ctx.randomize(&mut rng);
    ctx
});

pub fn random_hash() -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut random_bytes: [u8; 32] = [0u8; Sha256::LEN];
    rng.fill_bytes(&mut random_bytes);

    let hash = Sha256::hash(&random_bytes);
    hash.to_byte_array().to_vec()
}

pub fn unix_time() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|x| x.as_secs())
        .unwrap_or(0)
}

/// JSON util
pub trait JsonUtil: Sized + Serialize + DeserializeOwned
where
    <Self as JsonUtil>::Err: From<serde_json::Error>,
{
    /// Error
    type Err;

    /// Deserialize JSON
    fn from_json<T>(json: T) -> Result<Self, Self::Err>
    where
        T: AsRef<[u8]>,
    {
        Ok(serde_json::from_slice(json.as_ref())?)
    }

    /// Serialize to JSON string
    fn as_json(&self) -> String {
        // TODO: remove unwrap
        serde_json::to_string(self).unwrap()
    }
}

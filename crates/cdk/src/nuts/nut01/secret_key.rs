// Copyright (c) 2023-2024 thesimplekid (BSD 3-Clause License)
// Copyright (c) 2024 Yuki Kishimoto
// Distributed under the MIT software license

use core::fmt;
use core::str::FromStr;

use bitcoin::secp256k1;
use bitcoin::secp256k1::rand::rngs::OsRng;
use serde::{Deserialize, Deserializer};

use super::Error;
use crate::SECP256K1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecretKey {
    inner: secp256k1::SecretKey,
}

impl fmt::Display for SecretKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_secret_hex())
    }
}

impl SecretKey {
    /// Parse from `bytes`
    pub fn from_slice(slice: &[u8]) -> Result<Self, Error> {
        Ok(Self {
            inner: secp256k1::SecretKey::from_slice(slice)?,
        })
    }

    /// Parse from `hex` string
    pub fn from_hex<S>(hex: S) -> Result<Self, Error>
    where
        S: AsRef<str>,
    {
        Ok(Self {
            inner: secp256k1::SecretKey::from_str(hex.as_ref())?,
        })
    }

    /// Generate random secret key
    pub fn generate() -> Self {
        let (secret_key, _) = SECP256K1.generate_keypair(&mut OsRng);
        Self { inner: secret_key }
    }

    /// Get secret key as `hex` string
    pub fn to_secret_hex(&self) -> String {
        self.inner.display_secret().to_string()
    }

    /// Get secret key as `bytes`
    pub fn as_secret_bytes(&self) -> &[u8] {
        self.inner.as_ref()
    }

    /// Get secret key as `bytes`
    pub fn to_secret_bytes(&self) -> [u8; 32] {
        self.inner.secret_bytes()
    }
}

impl FromStr for SecretKey {
    type Err = Error;

    /// Try to parse [SecretKey] from `hex` or `bech32`
    fn from_str(secret_key: &str) -> Result<Self, Self::Err> {
        Self::parse(secret_key)
    }
}

impl<'de> Deserialize<'de> for SecretKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let secret_key: String = String::deserialize(deserializer)?;
        Self::parse(secret_key).map_err(serde::de::Error::custom)
    }
}

impl Drop for SecretKey {
    fn drop(&mut self) {
        self.inner.non_secure_erase();
        tracing::trace!("Secret Key dropped.");
    }
}

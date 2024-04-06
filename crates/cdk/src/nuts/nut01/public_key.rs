// Copyright (c) 2023-2024 thesimplekid (BSD 3-Clause License)
// Copyright (c) 2024 Yuki Kishimoto
// Distributed under the MIT software license

use core::fmt;
use core::str::FromStr;

use bitcoin::secp256k1;
use serde::{Deserialize, Deserializer, Serialize};

use super::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PublicKey {
    inner: secp256k1::PublicKey,
}

impl PublicKey {
    /// Parse from `bytes`
    #[inline]
    pub fn from_slice(slice: &[u8]) -> Result<Self, Error> {
        Ok(Self {
            inner: secp256k1::PublicKey::from_slice(slice)?,
        })
    }

    /// Parse from `hex` string
    #[inline]
    pub fn from_hex<S>(hex: S) -> Result<Self, Error>
    where
        S: AsRef<str>,
    {
        Ok(Self {
            inner: secp256k1::PublicKey::from_str(hex.as_ref())?,
        })
    }

    #[inline]
    pub fn to_bytes(&self) -> [u8; 33] {
        self.inner.serialize()
    }

    /// Get public key as `hex` string
    #[inline]
    pub fn to_hex(&self) -> String {
        self.inner.to_string()
    }
}

impl FromStr for PublicKey {
    type Err = Error;

    fn from_str(hex: &str) -> Result<Self, Self::Err> {
        Self::from_hex(hex)
    }
}

impl fmt::Display for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

impl Serialize for PublicKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_hex())
    }
}

impl<'de> Deserialize<'de> for PublicKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let public_key: String = String::deserialize(deserializer)?;
        Self::from_hex(public_key).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_public_key_from_hex() {
        PublicKey::from_hex("02194603ffa36356f4a56b7df9371fc3192472351453ec7398b8da8117e7c3e104")
            .unwrap();
    }
}

#[cfg(bench)]
mod benches {
    use test::{black_box, Bencher};

    use super::*;

    const HEX: &str = "02194603ffa36356f4a56b7df9371fc3192472351453ec7398b8da8117e7c3e104";

    #[bench]
    pub fn public_key_from_hex(bh: &mut Bencher) {
        bh.iter(|| {
            black_box(PublicKey::from_hex(HEX)).unwrap();
        });
    }
}

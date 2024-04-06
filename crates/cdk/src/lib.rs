// Copyright (c) 2023-2024 thesimplekid (BSD 3-Clause License)
// Copyright (c) 2024 Yuki Kishimoto
// Distributed under the MIT software license

//! Cashu Development Kit

#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![warn(rustdoc::bare_urls)]
#![cfg_attr(
    feature = "default",
    doc = include_str!("../README.md")
)]

extern crate core;
extern crate core;
extern crate core;
#[cfg(bench)]
extern crate test;

pub use bitcoin::secp256k1;
pub use {bip39, bitcoin};

#[cfg(feature = "wallet")]
pub mod client;
pub mod error;
#[cfg(feature = "mint")]
pub mod mint;
pub mod nuts;
pub mod prelude;
pub mod secret;
pub mod types;
pub mod url;
pub mod util;
#[cfg(feature = "wallet")]
pub mod wallet;

pub use self::util::SECP256K1;

pub type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

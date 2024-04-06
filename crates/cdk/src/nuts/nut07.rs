// Copyright (c) 2023-2024 thesimplekid (BSD 3-Clause License)
// Copyright (c) 2024 Yuki Kishimoto
// Distributed under the MIT software license

//! Spendable Check
//!
//! <https://github.com/cashubtc/nuts/blob/main/07.md>

use serde::{Deserialize, Serialize};

use super::PublicKey;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum State {
    Spent,
    Unspent,
    Pending,
}

/// Check spendabale request [NUT-07]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckStateRequest {
    /// Y's of the proofs to check
    #[serde(rename = "Ys")]
    pub ys: Vec<PublicKey>,
}

/// Proof state [NUT-07]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProofState {
    /// Y of proof
    #[serde(rename = "Y")]
    pub y: PublicKey,
    /// State of proof
    pub state: State,
    /// Witness data if it is supplied
    pub witness: Option<String>,
}

/// Check Spendable Response [NUT-07]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckStateResponse {
    pub states: Vec<ProofState>,
}

/// Spendable Settings
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Settings {
    supported: bool,
}

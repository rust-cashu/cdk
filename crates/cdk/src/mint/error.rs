// Copyright (c) 2023-2024 thesimplekid (BSD 3-Clause License)
// Copyright (c) 2024 Yuki Kishimoto
// Distributed under the MIT software license

use bitcoin::secp256k1;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("No key for amount")]
    AmountKey,
    #[error("Amount miss match")]
    Amount,
    #[error("Token Already Spent")]
    TokenSpent,
    #[error(transparent)]
    Secp256k1(#[from] secp256k1::Error),
    #[error("`Token not verified`")]
    TokenNotVerifed,
    #[error("Invoice amount undefined")]
    InvoiceAmountUndefined,
    /// Duplicate Proofs sent in request
    #[error("Duplicate proofs")]
    DuplicateProofs,
    /// Keyset id not active
    #[error("Keyset id is not active")]
    InactiveKeyset,
    /// Keyset is not known
    #[error("Unknown Keyset")]
    UnknownKeySet,
    #[error(transparent)]
    Secret(#[from] crate::secret::Error),
    #[error(transparent)]
    CustomError(String),
}

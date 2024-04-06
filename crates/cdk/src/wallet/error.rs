// Copyright (c) 2023-2024 thesimplekid (BSD 3-Clause License)
// Copyright (c) 2024 Yuki Kishimoto
// Distributed under the MIT software license

use std::string::FromUtf8Error;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    /// Serde Json error
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
    /// Insufficient Funds
    #[error("Insufficient funds")]
    InsufficientFunds,
    /// Utf8 parse error
    #[error(transparent)]
    Utf8ParseError(#[from] FromUtf8Error),
    /// Base64 error
    #[error(transparent)]
    Base64Error(#[from] base64::DecodeError),
    /// Unsupported Token
    #[error("Token unsupported")]
    UnsupportedToken,
    /// Token Requires proofs
    #[error("Proofs Required")]
    ProofsRequired,
    /// Url Parse error
    #[error("Url Parse")]
    UrlParse,
    #[error(transparent)]
    Secret(#[from] crate::secret::Error),
    /// Custom Error message
    #[error(transparent)]
    CustomError(String),
}

impl From<crate::url::Error> for Error {
    fn from(_err: crate::url::Error) -> Error {
        Error::UrlParse
    }
}

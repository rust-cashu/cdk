// Copyright (c) 2023-2024 thesimplekid (BSD 3-Clause License)
// Copyright (c) 2024 Yuki Kishimoto
// Distributed under the MIT software license

pub mod memory;
#[cfg(all(not(target_arch = "wasm32"), feature = "redb"))]
pub mod redb_store;

use std::collections::HashMap;

use async_trait::async_trait;
use cashu::nuts::nut02::mint::KeySet;
use cashu::nuts::{BlindSignature, CurrencyUnit, Id, MintInfo, Proof, PublicKey};
use cashu::secret::Secret;
use cashu::types::{MeltQuote, MintQuote};
pub use memory::MemoryLocalStore;
#[cfg(all(not(target_arch = "wasm32"), feature = "redb"))]
pub use redb_store::RedbLocalStore;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[cfg(all(not(target_arch = "wasm32"), feature = "redb"))]
    #[error("`{0}`")]
    Redb(#[from] redb::Error),
    #[cfg(all(not(target_arch = "wasm32"), feature = "redb"))]
    #[error("`{0}`")]
    Database(#[from] redb::DatabaseError),
    #[cfg(all(not(target_arch = "wasm32"), feature = "redb"))]
    #[error("`{0}`")]
    Transaction(#[from] redb::TransactionError),
    #[cfg(all(not(target_arch = "wasm32"), feature = "redb"))]
    #[error("`{0}`")]
    Commit(#[from] redb::CommitError),
    #[cfg(all(not(target_arch = "wasm32"), feature = "redb"))]
    #[error("`{0}`")]
    Table(#[from] redb::TableError),
    #[cfg(all(not(target_arch = "wasm32"), feature = "redb"))]
    #[error("`{0}`")]
    Storage(#[from] redb::StorageError),
    #[cfg(all(not(target_arch = "wasm32"), feature = "redb"))]
    #[error("`{0}`")]
    Serde(#[from] serde_json::Error),
    #[error("Unknown Mint Info")]
    UnknownMintInfo,
    #[error("`{0}`")]
    Cashu(#[from] cashu::error::Error),
    #[error("`{0}`")]
    CashuNut02(#[from] cashu::nuts::nut02::Error),
    #[error("`{0}`")]
    Secret(#[from] cashu::secret::Error),
}

#[async_trait]
pub trait LocalStore {
    async fn set_mint_info(&self, mint_info: &MintInfo) -> Result<(), Error>;
    async fn get_mint_info(&self) -> Result<MintInfo, Error>;

    async fn add_active_keyset(&self, unit: CurrencyUnit, id: Id) -> Result<(), Error>;
    async fn get_active_keyset_id(&self, unit: &CurrencyUnit) -> Result<Option<Id>, Error>;
    async fn get_active_keysets(&self) -> Result<HashMap<CurrencyUnit, Id>, Error>;

    async fn add_mint_quote(&self, quote: MintQuote) -> Result<(), Error>;
    async fn get_mint_quote(&self, quote_id: &str) -> Result<Option<MintQuote>, Error>;
    async fn get_mint_quotes(&self) -> Result<Vec<MintQuote>, Error>;
    async fn remove_mint_quote(&self, quote_id: &str) -> Result<(), Error>;

    async fn add_melt_quote(&self, quote: MeltQuote) -> Result<(), Error>;
    async fn get_melt_quote(&self, quote_id: &str) -> Result<Option<MeltQuote>, Error>;
    async fn get_melt_quotes(&self) -> Result<Vec<MeltQuote>, Error>;
    async fn remove_melt_quote(&self, quote_id: &str) -> Result<(), Error>;

    async fn add_keyset(&self, keyset: KeySet) -> Result<(), Error>;
    async fn get_keyset(&self, id: &Id) -> Result<Option<KeySet>, Error>;
    async fn get_keysets(&self) -> Result<Vec<KeySet>, Error>;

    async fn add_spent_proof(&self, proof: Proof) -> Result<(), Error>;
    async fn get_spent_proof_by_secret(&self, secret: &Secret) -> Result<Option<Proof>, Error>;
    async fn get_spent_proof_by_y(&self, y: &PublicKey) -> Result<Option<Proof>, Error>;

    async fn add_pending_proof(&self, proof: Proof) -> Result<(), Error>;
    async fn get_pending_proof_by_secret(&self, secret: &Secret) -> Result<Option<Proof>, Error>;
    async fn get_pending_proof_by_y(&self, y: &PublicKey) -> Result<Option<Proof>, Error>;
    async fn remove_pending_proof(&self, secret: &Secret) -> Result<(), Error>;

    async fn add_blinded_signature(
        &self,
        blinded_message: PublicKey,
        blinded_signature: BlindSignature,
    ) -> Result<(), Error>;
    async fn get_blinded_signature(
        &self,
        blinded_message: &PublicKey,
    ) -> Result<Option<BlindSignature>, Error>;
    async fn get_blinded_signatures(
        &self,
        blinded_messages: Vec<PublicKey>,
    ) -> Result<Vec<Option<BlindSignature>>, Error>;
}

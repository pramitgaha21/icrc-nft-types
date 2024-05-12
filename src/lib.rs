pub mod burnable;
pub mod icrc37;
pub mod icrc7;
pub mod mintable;

use candid::CandidType;
// rexport
pub use icrc_ledger_types::icrc1::account::{Account, Subaccount, DEFAULT_SUBACCOUNT};
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;

#[derive(
    CandidType, Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash,
)]
pub struct EncodedBlock(pub ByteBuf);

impl EncodedBlock {
    pub fn from_vec(bytes: Vec<u8>) -> Self {
        Self(ByteBuf::from(bytes))
    }

    pub fn into_vec(self) -> Vec<u8> {
        self.0.to_vec()
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }

    pub fn size_bytes(&self) -> usize {
        self.0.len()
    }
}

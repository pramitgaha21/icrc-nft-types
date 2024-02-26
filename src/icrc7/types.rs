use std::collections::HashMap;

use candid::{CandidType, Deserialize};
use icrc_ledger_types::{
    icrc::generic_metadata_value::MetadataValue,
    icrc1::account::{Account, Subaccount},
};

use super::errors::TransferError;

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct TransferArg {
    pub from_subaccount: Option<Subaccount>,
    pub to: Account,
    pub token_id: u128,
    pub memo: Option<Vec<u8>>,
    pub created_at_time: Option<u64>,
}

pub type TransferResult = Result<u128, TransferError>;

pub type Icrc7TokenMetadata = HashMap<String, MetadataValue>;

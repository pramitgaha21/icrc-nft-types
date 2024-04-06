use candid::{CandidType, Decode, Encode};
use ic_stable_structures::storable::{Bound, Storable};
use icrc_ledger_types::{icrc::generic_metadata_value::MetadataValue, icrc1::account::Account};
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Mint {
    pub tid: u128,
    pub from: Account,
    pub to: Account,
    pub meta: MetadataValue,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Burn {
    pub tid: u128,
    pub from: Account,
    pub to: Account,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Transfer {
    pub tid: u128,
    pub from: Account,
    pub to: Account,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Update {
    pub tid: u128,
    pub from: Account,
    pub meta: MetadataValue,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub ts: u64,
    pub txn_id: u128,
    pub btype: String,
    pub mint: Option<Mint>,
    pub burn: Option<Burn>,
    pub transfer: Option<Transfer>,
    pub update: Option<Update>,
    pub memo: Option<Vec<u8>>,
}

impl Storable for Transaction {
    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        std::borrow::Cow::Owned(Encode!(self).unwrap())
    }

    const BOUND: Bound = Bound::Unbounded;
}

impl Transaction {
    pub fn new(
        txn_id: u128,
        mint: Option<Mint>,
        burn: Option<Burn>,
        transfer: Option<Transfer>,
        update: Option<Update>,
        ts: u64,
        memo: Option<Vec<u8>>,
    ) -> Self {
        let btype: String = if mint.is_some() {
            "7mint".into()
        } else if burn.is_some() {
            "7burn".into()
        } else if transfer.is_some() {
            "7xfer".into()
        } else {
            "7update_token".into()
        };
        Self {
            btype,
            burn,
            mint,
            transfer,
            update,
            txn_id,
            ts,
            memo,
        }
    }
}

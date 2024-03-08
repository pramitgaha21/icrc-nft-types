use candid::{CandidType, Encode, Decode};
use icrc_ledger_types::{icrc::generic_metadata_value::MetadataValue, icrc1::account::Account};
use serde::{Serialize, Deserialize};
use ic_stable_structures::storable::{Bound, Storable};

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum TransactionType {
    Mint {
        tid: u128,
        from: Account,
        to: Account,
        meta: MetadataValue,
    },
    Burn {
        tid: u128,
        from: Account,
        to: Account,
    },
    Transfer {
        tid: u128,
        from: Account,
        to: Account,
    },
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub ts: u64,
    pub txn_id: u128,
    pub op: String,
    pub txn_type: TransactionType,
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
    pub fn new(txn_id: u128, txn_type: TransactionType, ts: u64, memo: Option<Vec<u8>>) -> Self {
        let op = match &txn_type {
            TransactionType::Transfer {
                tid: _,
                from: _,
                to: _,
            } => "7xfer".into(),
            TransactionType::Mint {
                tid: _,
                from: _,
                to: _,
                meta: _,
            } => "7mint".into(),
            TransactionType::Burn {
                tid: _,
                from: _,
                to: _,
            } => "7burn".into(),
        };
        Self {
            op,
            txn_id,
            ts,
            txn_type,
            memo,
        }
    }
}

/*
#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct  Burn {
    pub tid: u128,
    pub from: Account,
    pub to: Account,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct  Transfer {
    pub tid: u128,
    pub from: Account,
    pub to: Account,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct  Mint {
    pub tid: u128,
    pub from: Account,
    pub to: Account,
    pub meta: MetadataValue,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub ts: u64,
    pub txn_id: u128,
    pub op: String,
    pub mint: Option<Mint>,
   pub tranfer: Option<Transfer>,
    pub burn: Option<Burn>,
    pub memo: Option<Vec<u8>>,
}

impl Transaction {
    pub fn new(txn_id: u128, txn_type: TransactionType, ts: u64, memo: Option<Vec<u8>>) -> Self {
        match txn_type {
            TransactionType::Transfer {
                tid,
                from,
                to,
            } => {
                Self{
                    tranfer: Some(Transfer { tid, from, to }),
                    op: "7xfer".into(),
                    ts,
                    txn_id,
                    mint: None,
                    burn: None,
                    memo,
                }
            },
            TransactionType::Mint {
                tid,
                from,
                to,
                meta,
            } => {
                Self{
                    tranfer: None,
                    op: "7mint".into(),
                    ts,
                    txn_id,
                    mint: Some(Mint { tid, from, to, meta }),
                    burn: None,
                    memo,
                }
            },
            TransactionType::Burn {
                tid,
                from,
                to,
            } => {
                Self{
                    tranfer: None,
                    op: "7burn".into(),
                    ts,
                    txn_id,
                    mint: None,
                    burn: Some(Burn { tid, from, to }),
                    memo,
                }
            },
        }
    }
}
*/
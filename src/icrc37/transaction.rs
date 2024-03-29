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
    ApproveToken{
        tid: u128,
        from: Account,
        spender: Account,
        exp: Option<u64>
    },
    ApproveCollection{
        from: Account,
        spender: Account,
        exp: Option<u64>
    },
    RevokeTokenApproval{
        tid: u128,
        from: Account,
        spender: Account,
    },
    RevokeCollectionApproval{
        from: Account,
        spender: Account,
    },
    TransferFrom{
        tid: u128,
        spender: Account,
        from: Account,
        to: Account
    }
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
            TransactionType::ApproveToken { tid: _, from: _, spender: _, exp: _ } => {
                "37appr".into()
            },
            TransactionType::ApproveCollection { from: _, spender: _, exp: _ } => {
                "37appr_coll".into()
            },
            TransactionType::RevokeTokenApproval { tid: _, from: _, spender: _ } => "37revoke".into(),
            TransactionType::RevokeCollectionApproval { from: _, spender: _ } => "37revoke_coll".into(),
            TransactionType::TransferFrom { tid: _, spender: _, from: _, to: _ } => "37xfer".into(),
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
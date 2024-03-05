use candid::CandidType;
use icrc_ledger_types::icrc1::account::{Account, Subaccount};
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct ApprovalInfo{
    pub spender: Account,
    pub from_subaccount: Option<Subaccount>,
    pub expires_at: Option<u64>,
    pub memo: Option<Vec<u8>>,
    pub created_at_time: u64,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct ApproveTokenArg{
    pub token_id: u128,
    pub approval_info: ApprovalInfo,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum ApproveTokenError{
    InvalidSpender,
    Unauthorized,
    NonExistingTokenId,
    TooOld,
    CreatedInFuture{ ledger_time: u64 },
    GenericError{ error_code: u128, message: String },
    GenericBatchError{ error_code: u128, message: String }
}

pub type ApproveTokenResult = Result<u128, ApproveTokenError>;

pub struct ApproveCollectionArg{
    pub approval_info: ApprovalInfo
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum ApproveCollectionError{
    InvalidSpender,
    TooOld,
    CreatedInFuture{ ledger_time: u64 },
    GenericError{ error_code: u128, message: String },
    GenericBatchError{ error_code: u128, message: String },
}

pub type ApproveCollectionResult = Result<u128, ApproveCollectionError>;
use candid::CandidType;
use icrc_ledger_types::icrc1::account::{Account, Subaccount};
use serde::Deserialize;

#[derive(CandidType, Deserialize)]
pub struct ApprovalInfo {
    pub spender: Account,
    pub from_subaccount: Option<Subaccount>,
    pub expires_at: Option<u64>,
    pub memo: Option<Vec<u8>>,
    pub created_at_time: u64,
}

#[derive(CandidType, Deserialize)]
pub struct ApproveTokenArg{
    pub token_id: u128,
    pub approval_info: ApprovalInfo
}
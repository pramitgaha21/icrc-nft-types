use candid::CandidType;
use icrc_ledger_types::icrc1::account::Account;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct MintArg{
    pub token_id: u128,
    pub to: Account,
}

pub trait Mintable{
    type MintingAuthority;

    fn safe_mint(&mut self, mint_arg: &MintArg) -> bool;

    fn minting_authority(&self) -> Self::MintingAuthority;
}
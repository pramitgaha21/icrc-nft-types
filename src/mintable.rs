use std::fmt::Debug;
use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Deserialize, Serialize, Debug)]
pub enum MintError{
    SupplyCapReached,
    TokenIdAlreadyExist,
    Unauthorized,
    GenericError{ error_code: u128, message: String }
}

pub trait Mintable{
    type MintingAuthority;
    type MintArg;

    fn is_mintable(&self) -> bool;

    fn safe_mint(&mut self, mint_arg: &Self::MintArg) -> bool;

    fn minting_authority(&self) -> Self::MintingAuthority;
}
use std::fmt::Debug;
use candid::CandidType;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(CandidType, Deserialize, Serialize, Debug)]
pub enum MintError{
    SupplyCapReached,
    TokenIdAlreadyExist,
    Unauthorized,
    GenericError{ error_code: u128, message: String }
}

pub trait Mintable{
    type MintingAuthority;

    fn is_mintable(&self) -> bool;

    fn safe_mint<A: CandidType + Serialize + DeserializeOwned + Debug>(&mut self, mint_arg: &A) -> Result<u128, MintError>;

    fn minting_authority(&self) -> Self::MintingAuthority;
}
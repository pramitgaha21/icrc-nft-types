pub trait Mintable{
    type MintingAuthority;

    fn safe_mint(&mut self, mint_arg: ()) -> bool;

    fn minting_authority(&self) -> Self::MintingAuthority;
}
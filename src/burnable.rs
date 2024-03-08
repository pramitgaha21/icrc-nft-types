use icrc_ledger_types::icrc1::account::Account;

pub enum BurnError{
    Unauthorized,
    GenericError{ error_code: u128, message: String },
}
pub trait Burnable{
    type BurnArg;

    fn is_burnable(&self) -> bool;

    fn burn_account() -> Account;

    fn safe_burn(&mut self, burn_args: &Self::BurnArg) -> bool;
}
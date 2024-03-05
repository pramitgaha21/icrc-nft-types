use icrc_ledger_types::icrc1::account::Account;

pub trait Burnable{
    fn is_burnable(&self) -> bool;

    fn burn_account() -> Account;

    fn safe_burn(&mut self, burn_args: &()) -> bool;
}
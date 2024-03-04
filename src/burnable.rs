use icrc_ledger_types::icrc1::account::Account;

pub trait Burnable{
    fn burn_account() -> Account;

    fn safe_burn(&mut self, burn_args: &()) -> bool;
}
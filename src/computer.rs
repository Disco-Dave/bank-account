use crate::account::{CreditAccount, RatedAccount};
use crate::amount::Amount;
use crate::customer::Customer;

pub trait Computer {
    fn get_account(&self, customer: Customer) -> RatedAccount;
    fn deposit(&self, amount: Amount, account: RatedAccount) -> RatedAccount;
    fn withdraw(&self, amount: Amount, account: CreditAccount) -> RatedAccount;
}

pub struct LiveComputer();

impl Computer for LiveComputer {
    fn get_account(&self, customer: Customer) -> RatedAccount {
        RatedAccount::new(customer)
    }

    fn deposit(&self, amount: Amount, account: RatedAccount) -> RatedAccount {
        account.deposit(amount)
    }

    fn withdraw(&self, amount: Amount, account: CreditAccount) -> RatedAccount {
        account.withdraw(amount)
    }
}

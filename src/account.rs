use super::amount::Amount;
use super::customer::Customer;
use std::convert::From;
use std::ops::{Add, Sub};

#[derive(Debug, PartialEq)]
pub struct Account {
    customer: Customer,
    balance: f64,
}

#[derive(Debug, PartialEq)]
pub struct CreditAccount(Account);

#[derive(Debug, PartialEq)]
pub enum RatedAccount {
    Overdrawn(Account),
    InCredit(CreditAccount),
}

impl From<RatedAccount> for Account {
    fn from(rated_account: RatedAccount) -> Self {
        use RatedAccount::*;

        match rated_account {
            Overdrawn(account) | InCredit(CreditAccount(account)) => account,
        }
    }
}

impl Account {
    fn update_balance(self, amount: Amount, op: impl Fn(f64, f64) -> f64) -> RatedAccount {
        let account = Account {
            balance: op(self.balance, amount.into()),
            ..self
        };
        RatedAccount::rate_account(account)
    }
}

impl RatedAccount {
    pub fn rate_account(account: Account) -> Self {
        if account.balance >= 0.0 {
            RatedAccount::InCredit(CreditAccount(account))
        } else {
            RatedAccount::Overdrawn(account)
        }
    }

    pub fn new(customer: Customer) -> Self {
        let account = Account {
            customer,
            balance: 0.0,
        };
        RatedAccount::rate_account(account)
    }

    pub fn deposit(self, amount: Amount) -> Self {
        Account::from(self).update_balance(amount, f64::add)
    }
}

impl CreditAccount {
    pub fn withdraw(self, amount: Amount) -> RatedAccount {
        self.0.update_balance(amount, f64::sub)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn account_opens_with_balance_of_zero_and_in_credit() {
        let customer = Customer::new("sagwa".into()).unwrap();
        let account = RatedAccount::new(customer);
        let expected_account = RatedAccount::InCredit(CreditAccount(Account {
            customer: Customer::new("sagwa".into()).unwrap(),
            balance: 0.0,
        }));
        assert_eq!(expected_account, account);
    }

    fn is_in_credit(account: &RatedAccount) -> bool {
        match account {
            RatedAccount::InCredit(_) => true,
            _ => false,
        }
    }

    #[test]
    fn going_negative_causes_account_be_overdrawn() {
        let customer = Customer::new("sagwa".into()).unwrap();
        let account = RatedAccount::new(customer);
        match account {
            RatedAccount::Overdrawn(_) => panic!(),
            RatedAccount::InCredit(credit_account) => {
                let new_account = credit_account.withdraw(Amount::new(100.0).unwrap());
                assert!(!is_in_credit(&new_account));
            }
        }
    }

    #[test]
    fn depositting_into_an_overdrawn_account_to_make_the_new_balance_zero_causes_it_go_in_credit() {
        let account = RatedAccount::rate_account(Account {
            customer: Customer::new("sagwa".into()).unwrap(),
            balance: -100.00,
        });

        assert!(!is_in_credit(&account));
        assert!(is_in_credit(&account.deposit(Amount::new(100.00).unwrap())))
    }

    #[test]
    fn depositting_into_an_overdrawn_account_to_make_the_new_balance_positive_causes_it_go_in_credit(
    ) {
        let account = RatedAccount::rate_account(Account {
            customer: Customer::new("sagwa".into()).unwrap(),
            balance: -100.00,
        });

        assert!(!is_in_credit(&account));
        assert!(is_in_credit(&account.deposit(Amount::new(200.00).unwrap())))
    }

    #[test]
    fn depositting_into_an_overdrawn_account_to_make_the_new_balance_still_negative_causes_it_to_stay_overdrawn(
    ) {
        let account = RatedAccount::rate_account(Account {
            customer: Customer::new("sagwa".into()).unwrap(),
            balance: -100.00,
        });

        assert!(!is_in_credit(&account));
        assert!(!is_in_credit(&account.deposit(Amount::new(25.50).unwrap())))
    }

    #[test]
    fn withdrawing_less_than_balance_keeps_account_in_credit() {
        let account = RatedAccount::rate_account(Account {
            customer: Customer::new("sagwa".into()).unwrap(),
            balance: 100.00,
        });

        match account {
            RatedAccount::Overdrawn(_) => panic!(),
            RatedAccount::InCredit(credit_account) => {
                let new_account = credit_account.withdraw(Amount::new(25.0).unwrap());
                assert!(is_in_credit(&new_account));
            }
        }
    }

    #[test]
    fn withdrawing_the_same_as_the_balance_keeps_account_in_credit() {
        let account = RatedAccount::rate_account(Account {
            customer: Customer::new("sagwa".into()).unwrap(),
            balance: 100.00,
        });

        match account {
            RatedAccount::Overdrawn(_) => panic!(),
            RatedAccount::InCredit(credit_account) => {
                let new_account = credit_account.withdraw(Amount::new(100.0).unwrap());
                assert!(is_in_credit(&new_account));
            }
        }
    }

    #[test]
    fn withdrawing_more_than_the_balance_causes_account_to_be_overdrawn() {
        let account = RatedAccount::rate_account(Account {
            customer: Customer::new("sagwa".into()).unwrap(),
            balance: 100.00,
        });

        match account {
            RatedAccount::Overdrawn(_) => panic!(),
            RatedAccount::InCredit(credit_account) => {
                let new_account = credit_account.withdraw(Amount::new(110.0).unwrap());
                assert!(!is_in_credit(&new_account));
            }
        }
    }
}

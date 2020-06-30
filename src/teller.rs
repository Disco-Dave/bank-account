use super::account::{Account, RatedAccount};
use super::amount::Amount;
use super::communicate::Communicate;
use super::computer::Computer;
use super::customer::Customer;
use std::str::FromStr;

pub struct Teller<'a> {
    pub communicate: &'a dyn Communicate,
    pub computer: &'a dyn Computer,
}

impl<'a> Teller<'a> {
    fn get_customer(&self) -> Customer {
        self.communicate.say("Enter name: ");

        match Customer::new(self.communicate.get_line()) {
            Ok(c) => c,
            Err(_) => {
                self.communicate.say_line("Customer name may not be empty.");
                self.get_customer()
            }
        }
    }

    fn get_amount(&self) -> Amount {
        self.communicate.say("Enter amount: ");

        let res = f64::from_str(&self.communicate.get_line().trim())
            .map_err(|_| "Invalid number.")
            .and_then(|a| Amount::new(a).map_err(|_| "Number cannot be negative."));

        match res {
            Ok(amount) => amount,
            Err(msg) => {
                self.communicate.say_line(msg);
                self.get_amount()
            }
        }
    }

    fn summarize_account(&self, account: &RatedAccount) {
        let account: &Account = account.into();
        let message = format!("Current balance is: {}", account.balance);
        self.communicate.say_line(&message);
    }

    fn prompt(&self) {
        let message = format!("(d)eposit, (w)ithdraw, or (q)uit: ");
        self.communicate.say(&message);
    }

    pub fn interact(&self) {
        let customer = self.get_customer();
        let mut account = self.computer.get_account(customer);

        let mut did_quit = false;

        while !did_quit {
            self.summarize_account(&account);
            self.prompt();

            match self.communicate.get_char() {
                Some('q') => did_quit = true,
                Some('d') => {
                    let amount = self.get_amount();
                    account = self.computer.deposit(amount, account);
                }
                Some('w') => match account {
                    RatedAccount::Overdrawn(_) => {
                        self.communicate.say_line("Account is already overdrawn.")
                    }
                    RatedAccount::InCredit(credit_account) => {
                        let amount = self.get_amount();
                        account = self.computer.withdraw(amount, credit_account);
                    }
                },
                _ => self.communicate.say_line("Invalid command."),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::account::{CreditAccount, RatedAccount};
    use crate::amount::Amount;
    use crate::communicate::Communicate;
    use crate::customer::Customer;
    use std::cell::RefCell;

    struct MockCommunicate {
        input: RefCell<Vec<String>>,
        output: RefCell<Vec<String>>,
    }

    impl MockCommunicate {
        fn new(input: Vec<String>) -> MockCommunicate {
            MockCommunicate {
                input: RefCell::new(input),
                output: RefCell::new(Vec::new()),
            }
        }

        fn pop_input(&self) -> Option<String> {
            self.input.borrow_mut().pop()
        }

        fn get_output(&self) -> Vec<String> {
            self.output.borrow().to_vec()
        }
    }

    impl Communicate for MockCommunicate {
        fn get_line(&self) -> String {
            self.pop_input().unwrap()
        }

        fn get_char(&self) -> Option<char> {
            self.pop_input().and_then(|s| s.chars().next())
        }

        fn say(&self, message: &str) {
            self.output.borrow_mut().push(message.to_owned());
        }

        fn say_line(&self, message: &str) {
            self.output.borrow_mut().push(message.to_owned() + "\n");
        }
    }

    struct MockComputer();

    impl Computer for MockComputer {
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

    #[test]
    fn can_get_customer_name() {
        let mock_communicate = MockCommunicate::new(vec!["Mochi".to_owned()]);
        let mock_computer = MockComputer();
        let teller = Teller {
            communicate: &mock_communicate,
            computer: &mock_computer,
        };

        let expected_customer = Customer::new("Mochi".to_owned()).unwrap();
        let actual_customer = teller.get_customer();
        assert_eq!(expected_customer, actual_customer);
        assert_eq!(
            vec!["Enter name: ".to_owned()],
            mock_communicate.get_output()
        );
    }

    #[test]
    fn retries_customer_name_until_a_valid_one_is_given() {
        let mock_communicate = MockCommunicate::new(vec![
            "not mochi".to_owned(),
            "Mochi".to_owned(),
            "".to_owned(),
            "  ".to_owned(),
        ]);
        let mock_computer = MockComputer();
        let teller = Teller {
            communicate: &mock_communicate,
            computer: &mock_computer,
        };

        let expected_customer = Customer::new("Mochi".to_owned()).unwrap();
        let actual_customer = teller.get_customer();
        assert_eq!(expected_customer, actual_customer);
        assert_eq!(
            vec![
                "Enter name: ".to_owned(),
                "Customer name may not be empty.\n".to_owned(),
                "Enter name: ".to_owned(),
                "Customer name may not be empty.\n".to_owned(),
                "Enter name: ".to_owned()
            ],
            mock_communicate.get_output()
        );
    }

    #[test]
    fn can_get_amount() {
        let mock_communicate = MockCommunicate::new(vec!["123.12".to_owned()]);
        let mock_computer = MockComputer();
        let teller = Teller {
            communicate: &mock_communicate,
            computer: &mock_computer,
        };
        let expected_amount = Amount::new(123.12).unwrap();
        let actual_amount = teller.get_amount();
        assert_eq!(expected_amount, actual_amount);
    }

    #[test]
    fn retries_amount_until_a_vali_one_is_given() {
        let mock_communicate =
            MockCommunicate::new(vec!["22.33".to_owned(), "-10".to_owned(), "xyz".to_owned()]);
        let mock_computer = MockComputer();
        let teller = Teller {
            communicate: &mock_communicate,
            computer: &mock_computer,
        };
        let expected_amount = Amount::new(22.33).unwrap();
        let actual_amount = teller.get_amount();
        assert_eq!(expected_amount, actual_amount);
        assert_eq!(
            vec![
                "Enter amount: ".to_owned(),
                "Invalid number.\n".to_owned(),
                "Enter amount: ".to_owned(),
                "Number cannot be negative.\n".to_owned(),
                "Enter amount: ".to_owned()
            ],
            mock_communicate.get_output()
        );
    }
}

use super::amount::Amount;
use super::communicate::Communicate;
use super::customer::Customer;
use std::str::FromStr;

pub struct Teller<'a> {
    pub communicate: &'a dyn Communicate,
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

        let res = f64::from_str(&self.communicate.get_line())
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::communicate::Communicate;
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

    #[test]
    fn can_get_customer_name() {
        let mock_communicate = MockCommunicate::new(vec!["Mochi".to_owned()]);
        let teller = Teller {
            communicate: &mock_communicate,
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
        let teller = Teller {
            communicate: &mock_communicate,
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
}

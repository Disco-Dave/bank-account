use std::fmt::{self, Display, Formatter};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Customer(String);

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum Error {
    IsEmpty,
}

impl Display for Customer {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Customer {
    pub fn new(customer: String) -> Result<Customer, Error> {
        let customer = customer.trim();

        if customer.is_empty() {
            Err(Error::IsEmpty)
        } else {
            Ok(Customer(customer.into()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn does_not_allow_empty_strings() {
        let customer_result = Customer::new("".into());
        assert_eq!(Err(Error::IsEmpty), customer_result);
    }

    #[test]
    fn does_not_allow_all_whitespace() {
        let whitespace_examples = vec![" ".into(), "    ".into(), "\n".into()];
        for example in whitespace_examples {
            let customer_result = Customer::new(example);
            assert_eq!(Err(Error::IsEmpty), customer_result);
        }
    }

    #[test]
    fn trims_leading_and_trailing_whitespace() {
        let whitespace_examples: Vec<String> =
            vec![" abc ".into(), "  \tjohn doe\n  ".into(), "crab\n".into()];

        for example in whitespace_examples {
            let customer_result = Customer::new(example.clone());
            assert_eq!(
                Ok(example.trim().into()),
                customer_result.map(|c| c.to_string())
            );
        }
    }
}

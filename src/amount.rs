use std::convert::From;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Amount(f64);

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum Error {
    IsNegative,
}

impl From<Amount> for f64 {
    fn from(amount: Amount) -> Self {
        amount.0
    }
}

impl Amount {
    pub fn new(amount: f64) -> Result<Amount, Error> {
        if amount < 0.0 {
            Err(Error::IsNegative)
        } else {
            Ok(Amount(amount))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn does_not_allow_negative_numbers() {
        let numbers: Vec<f64> = vec![-100.0, -23.23, -44.21, -0.001];
        for number in numbers {
            let amount_result = Amount::new(number);
            assert_eq!(Err(Error::IsNegative), amount_result);
        }
    }

    #[test]
    pub fn does_allow_positive_numbers() {
        let numbers: Vec<f64> = vec![0.0, 100.0, 23.23, 44.21, 0.001];
        for number in numbers {
            let amount_result = Amount::new(number);
            assert_eq!(Ok(number), amount_result.map(|a| a.into()));
        }
    }
}

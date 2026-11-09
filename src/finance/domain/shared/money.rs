use crate::finance::domain::shared::currency::Currency;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MoneyError {
    CurrencyMismatch,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Money {
    amount_in_cents: i64,
    currency: Currency,
}

impl Money {
    pub fn new(amount_in_cents: i64, currency: Currency) -> Self {
        Self {
            amount_in_cents,
            currency,
        }
    }

    pub fn add(&self, other: &Money) -> Result<Money, MoneyError> {
        if self.currency != other.currency {
            return Err(MoneyError::CurrencyMismatch);
        }
        Ok(Money {
            amount_in_cents: &self.amount_in_cents + other.amount_in_cents,
            currency: self.currency.clone(),
        })
    }
    pub fn subtract(&self, other: &Money) -> Result<Money, MoneyError> {
        if self.currency != other.currency {
            return Err(MoneyError::CurrencyMismatch);
        }

        Ok(Money {
            amount_in_cents: &self.amount_in_cents - other.amount_in_cents,
            currency: self.currency.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::finance::domain::shared::{
        currency::Currency,
        money::{Money, MoneyError},
    };

    #[test]
    fn add_should_return_self_plus_other() {
        let current = Money::new(100_000, Currency::brl());
        let other = Money::new(100_000, Currency::brl());

        let result = current.add(&other).unwrap();
        assert_eq!(result.amount_in_cents, 200_000);
        assert_eq!(result.currency, current.currency);
    }

    #[test]
    fn add_should_return_error_when_currency_is_different() {
        let current = Money::new(100_000, Currency::brl());
        let other = Money::new(100_000, Currency::usa());

        let result = current.add(&other);
        assert!(result.is_err());
        assert_eq!(result, Err(MoneyError::CurrencyMismatch));
    }

    #[test]
    fn subtract_should_return_self_plus_other() {
        let current = Money::new(100_000, Currency::brl());
        let other = Money::new(50_000, Currency::brl());

        let result = current.subtract(&other).unwrap();
        assert_eq!(result.amount_in_cents, 50_000);
        assert_eq!(result.currency, current.currency);
    }

    #[test]
    fn subtract_should_return_error_when_currency_is_different() {
        let current = Money::new(100_000, Currency::brl());
        let other = Money::new(50_000, Currency::usa());

        let result = current.subtract(&other);
        assert!(result.is_err());
        assert_eq!(result, Err(MoneyError::CurrencyMismatch));
    }
}

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
            amount_in_cents: self.amount_in_cents + other.amount_in_cents,
            currency: self.currency.clone(),
        })
    }
    pub fn subtract(&self, other: &Money) -> Result<Money, MoneyError> {
        if self.currency != other.currency {
            return Err(MoneyError::CurrencyMismatch);
        }

        Ok(Money {
            amount_in_cents: self.amount_in_cents - other.amount_in_cents,
            currency: self.currency.clone(),
        })
    }
    pub fn zero(currency: Currency) -> Self {
        Self {
            amount_in_cents: 0,
            currency,
        }
    }
    pub fn is_zero(&self) -> bool {
        self.amount_in_cents == 0
    }

    pub fn is_less_or_eq_zero(&self) -> bool {
        self.amount_in_cents <= 0
    }

    pub fn amount_in_cents(&self) -> i64 {
        self.amount_in_cents
    }

    pub fn currency(&self) -> &Currency {
        &self.currency
    }
}

#[cfg(test)]
mod tests {
    use crate::finance::domain::shared::{
        currency::Currency,
        money::{Money, MoneyError},
    };

    #[test]
    fn creates_money() {
        let money = Money::new(100_000, Currency::brl());
        assert_eq!(money.amount_in_cents(), 100_000);
        assert_eq!(money.currency(), &Currency::brl());
    }

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

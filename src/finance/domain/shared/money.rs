use crate::finance::domain::shared::currency::Currency;

#[derive(Clone)]
pub struct Money {
    amount: f64,
    currency: Currency,
}

impl Money {
    pub fn new(amount: f64, currency: Currency) -> Self {
        Self { amount, currency }
    }
}

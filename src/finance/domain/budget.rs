use crate::finance::domain::{
    account::FinanceAccountId,
    movement::{Movement, MovementError, MovementType},
    shared::{currency::Currency, id::Id, money::Money},
};

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct BudgetId(Id);

#[derive(Debug, PartialEq)]
pub enum BudgetError {
    Archived,
    CurrencyMismatch,
    InvalidMovementAmount,
}

impl From<MovementError> for BudgetError {
    fn from(value: MovementError) -> Self {
        match value {
            MovementError::InvalidAmount => Self::InvalidMovementAmount,
        }
    }
}

impl BudgetId {
    pub fn new() -> Self {
        BudgetId(Id::new())
    }
}

pub struct Budget {
    id: BudgetId,
    name: String,
    currency: Currency,
    is_archived: bool,
}

impl Budget {
    pub fn new(name: String, currency: Currency) -> Self {
        Budget {
            id: BudgetId(Id::new()),
            name,
            currency,
            is_archived: false,
        }
    }

    pub fn id(&self) -> &BudgetId {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn currency(&self) -> &Currency {
        &self.currency
    }

    pub fn is_archived(&self) -> bool {
        self.is_archived
    }

    pub fn archive(&mut self) {
        self.is_archived = true;
    }
    pub fn validate_movement_creation(&self, amount: &Money) -> Result<(), BudgetError> {
        if self.currency() != amount.currency() {
            return Err(BudgetError::CurrencyMismatch);
        }
        if self.is_archived() {
            return Err(BudgetError::Archived);
        }
        if amount.amount_in_cents() <= 0 {
            return Err(BudgetError::InvalidMovementAmount);
        }

        Ok(())
    }

    pub fn create_movement(
        &self,
        account_id: FinanceAccountId,
        movement_type: MovementType,
        amount: Money,
        decription: Option<String>,
    ) -> Result<Movement, BudgetError> {
        self.validate_movement_creation(&amount)?;

        let movement = Movement::new(
            self.id().clone(),
            account_id,
            movement_type,
            amount,
            decription,
        )?;

        Ok(movement)
    }
}

#[cfg(test)]
mod tests {
    use uuid::Version;

    use super::*;

    #[test]
    fn creates_budget_with_expected_values() {
        let budget = Budget::new("Monthly expenses".to_string(), Currency::brl());

        assert_eq!(budget.id().0.value().get_version(), Some(Version::Random));
        assert_eq!(budget.name(), "Monthly expenses");
        assert_eq!(budget.currency(), &Currency::brl());
        assert_eq!(budget.is_archived(), false);
    }

    #[test]
    fn creates_budget_id_with_random_uuid() {
        let id = BudgetId::new();

        assert_eq!(id.0.value().get_version(), Some(Version::Random));
    }

    #[test]
    fn archive_returns_ok_and_keeps_budget_archived() {
        let mut budget = Budget::new("Monthly expenses".to_string(), Currency::usa());

        assert_eq!(budget.archive(), ());
        assert_eq!(budget.is_archived(), true);
    }

    #[test]
    fn create_movement_returns_error_for_currency_mismatch() {
        let budget = Budget::new("Monthly expenses".to_string(), Currency::usa());
        let account_id = FinanceAccountId::new();
        let movement_type = MovementType::Credit;
        let amount = Money::new(100, Currency::brl());
        let decription = Some("text".to_string());
        let result = budget.create_movement(account_id, movement_type, amount, decription);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), BudgetError::CurrencyMismatch);
    }

    #[test]
    fn create_movement_returns_error_when_archived() {
        let mut budget = Budget::new("Monthly expenses".to_string(), Currency::usa());
        let account_id = FinanceAccountId::new();
        let movement_type = MovementType::Credit;
        let amount = Money::new(100, Currency::usa());
        let decription = Some("text".to_string());

        budget.archive();

        let result = budget.create_movement(account_id, movement_type, amount, decription);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), BudgetError::Archived);
    }

    #[test]
    fn create_movement_returns_error_when_amount_is_non_positive() {
        let budget = Budget::new("Monthly expenses".to_string(), Currency::usa());

        {
            let result = budget.create_movement(
                FinanceAccountId::new(),
                MovementType::Credit,
                Money::new(0, Currency::usa()),
                Some("text".to_string()),
            );

            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), BudgetError::InvalidMovementAmount);
        }

        {
            let result = budget.create_movement(
                FinanceAccountId::new(),
                MovementType::Credit,
                Money::new(-10, Currency::usa()),
                Some("text".to_string()),
            );

            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), BudgetError::InvalidMovementAmount);
        }
    }
}

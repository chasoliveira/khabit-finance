use crate::finance::domain::{
    account::FinanceAccountId,
    movement::{Movement, MovementError, MovementId, MovementType},
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
    movements: Vec<Movement>,
}

impl Budget {
    pub fn new(name: String, currency: Currency) -> Self {
        Budget {
            id: BudgetId(Id::new()),
            name,
            currency,
            is_archived: false,
            movements: Vec::new(),
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
    pub fn movements(&self) -> &[Movement] {
        &self.movements
    }

    pub fn archive(&mut self) {
        self.is_archived = true;
    }

    fn _validate_movement_creation(&self, amount: &Money) -> Result<(), BudgetError> {
        if self.currency() != amount.currency() {
            return Err(BudgetError::CurrencyMismatch);
        }
        if self.is_archived() {
            return Err(BudgetError::Archived);
        }

        Ok(())
    }

    pub fn create_movement(
        &mut self,
        account_id: FinanceAccountId,
        movement_type: MovementType,
        amount: Money,
        description: Option<String>,
    ) -> Result<MovementId, BudgetError> {
        self._validate_movement_creation(&amount)?;

        let movement = Movement::new(
            self.id().clone(),
            account_id,
            movement_type,
            amount,
            description,
        )?;
        let movement_id = movement.id().clone();
        self.movements.push(movement);
        Ok(movement_id)
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
        assert_eq!(budget.movements().len(), 0)
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
    fn create_movement_success() {
        let mut budget = Budget::new("Monthly expenses".to_string(), Currency::usa());
        let account_id = FinanceAccountId::new();
        let movement_type = MovementType::Credit;
        let amount = Money::new(100, Currency::usa());
        let decription = Some("text".to_string());
        let result = budget.create_movement(account_id, movement_type, amount, decription);

        assert!(result.is_ok());
        assert_eq!(budget.movements().len(), 1)
    }

    #[test]
    fn create_movement_returns_error_for_currency_mismatch() {
        let mut budget = Budget::new("Monthly expenses".to_string(), Currency::usa());
        let account_id = FinanceAccountId::new();
        let movement_type = MovementType::Credit;
        let amount = Money::new(100, Currency::brl());
        let decription = Some("text".to_string());
        let result = budget.create_movement(account_id, movement_type, amount, decription);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), BudgetError::CurrencyMismatch);
        assert_eq!(budget.movements().len(), 0)
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
        assert_eq!(budget.movements().len(), 0)
    }

    #[test]
    fn create_movement_returns_error_when_amount_is_non_positive() {
        let mut budget = Budget::new("Monthly expenses".to_string(), Currency::usa());

        {
            let result = budget.create_movement(
                FinanceAccountId::new(),
                MovementType::Credit,
                Money::new(0, Currency::usa()),
                Some("text".to_string()),
            );

            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), BudgetError::InvalidMovementAmount);
            assert_eq!(budget.movements().len(), 0)
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
            assert_eq!(budget.movements().len(), 0)
        }
    }
}

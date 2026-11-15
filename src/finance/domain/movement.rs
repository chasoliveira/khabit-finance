use crate::finance::domain::{
    account::AccountId,
    budgets::{budget::BudgetId, movement_created::MovementCreatedDomainEvent},
    events::domain_events::{DomainEvent, DomainEvents},
    shared::{id::Id, money::Money},
};

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct MovementId(Id);

impl MovementId {
    pub fn new() -> Self {
        MovementId(Id::new())
    }
}

#[derive(Debug, PartialEq)]
pub enum MovementError {
    InvalidAmount,
}

#[derive(Debug, PartialEq, Clone)]
pub enum MovementType {
    Credit,
    Debit,
}

#[derive(Clone, Debug)]
pub struct Movement {
    id: MovementId,
    budget_id: BudgetId,
    account_id: AccountId,
    movement_type: MovementType,
    amount: Money,
    description: Option<String>,
    domain_events: DomainEvents,
}

impl Movement {
    pub(super) fn new(
        budget_id: BudgetId,
        account_id: AccountId,
        movement_type: MovementType,
        amount: Money,
        description: Option<String>,
    ) -> Result<Movement, MovementError> {
        if amount.is_non_positive() {
            return Err(MovementError::InvalidAmount);
        }
        let movement_id = MovementId::new();
        let mut movement = Movement {
            id: movement_id.clone(),
            budget_id: budget_id.clone(),
            account_id,
            movement_type,
            amount,
            description,
            domain_events: DomainEvents::new(),
        };

        movement.domain_events.raise(
            MovementCreatedDomainEvent::new(budget_id.clone(), movement_id.clone()).in_box(),
        );
        Ok(movement)
    }

    pub fn id(&self) -> &MovementId {
        &self.id
    }
    pub fn budget_id(&self) -> &BudgetId {
        &self.budget_id
    }

    pub fn account_id(&self) -> &AccountId {
        &self.account_id
    }

    pub fn movement_type(&self) -> &MovementType {
        &self.movement_type
    }

    pub fn amount(&self) -> &Money {
        &self.amount
    }
    pub fn description(&self) -> Option<String> {
        self.description.clone()
    }

    pub fn domain_events(&self) -> &[Box<dyn DomainEvent>] {
        self.domain_events.events()
    }
}

#[cfg(test)]
mod tests {
    use uuid::Version;

    use crate::finance::domain::{
        account::AccountId,
        budgets::budget::BudgetId,
        movement::{Movement, MovementError, MovementType},
        shared::{currency::Currency, money::Money},
    };

    #[test]
    fn test_movement_creation() {
        let movement = Movement::new(
            BudgetId::new(),
            AccountId::new(),
            MovementType::Credit,
            Money::new(100, Currency::brl()),
            Some("Adding amount".to_string()),
        )
        .unwrap();

        assert_eq!(movement.id().0.value().get_version(), Some(Version::Random));
        assert_eq!(movement.description(), Some("Adding amount".to_string()));
        assert_eq!(movement.movement_type, MovementType::Credit);
        assert_eq!(movement.amount(), &Money::new(100, Currency::brl()));
        assert_eq!(movement.domain_events().len(), 1);
        assert_eq!(
            movement.domain_events()[0].event_name(),
            "MovementCreatedDomainEvent"
        );
    }

    #[test]
    fn creates_valid_credit() {
        let movement = Movement::new(
            BudgetId::new(),
            AccountId::new(),
            MovementType::Credit,
            Money::new(100, Currency::brl()),
            None,
        )
        .unwrap();

        assert_eq!(movement.movement_type, MovementType::Credit);
        assert_eq!(movement.amount().amount_in_cents(), 100);
    }

    #[test]
    fn creates_valid_debit() {
        let movement = Movement::new(
            BudgetId::new(),
            AccountId::new(),
            MovementType::Debit,
            Money::new(100, Currency::brl()),
            None,
        )
        .unwrap();

        assert_eq!(movement.movement_type, MovementType::Debit);
        assert_eq!(movement.amount().amount_in_cents(), 100);
    }

    #[test]
    fn rejects_zero_amount() {
        let result = Movement::new(
            BudgetId::new(),
            AccountId::new(),
            MovementType::Credit,
            Money::zero(Currency::brl()),
            None,
        );

        assert!(matches!(result, Err(MovementError::InvalidAmount)));
    }

    #[test]
    fn rejects_negative_amount() {
        let result = Movement::new(
            BudgetId::new(),
            AccountId::new(),
            MovementType::Debit,
            Money::new(-1, Currency::brl()),
            None,
        );

        assert!(matches!(result, Err(MovementError::InvalidAmount)));
    }
    #[test]
    fn preserves_budget_id() {
        let budget_id = BudgetId::new();
        let movement = Movement::new(
            budget_id.clone(),
            AccountId::new(),
            MovementType::Credit,
            Money::new(100, Currency::brl()),
            None,
        )
        .unwrap();

        assert_eq!(movement.budget_id, budget_id);
    }

    #[test]
    fn preserves_account_id() {
        let account_id = AccountId::new();
        let movement = Movement::new(
            BudgetId::new(),
            account_id.clone(),
            MovementType::Credit,
            Money::new(100, Currency::brl()),
            None,
        )
        .unwrap();

        assert_eq!(movement.account_id, account_id);
    }

    #[test]
    fn preserves_currency() {
        let currency = Currency::eur();
        let movement = Movement::new(
            BudgetId::new(),
            AccountId::new(),
            MovementType::Credit,
            Money::new(100, currency.clone()),
            None,
        )
        .unwrap();

        assert_eq!(movement.amount().currency(), &currency);
    }
}

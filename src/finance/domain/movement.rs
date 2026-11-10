use crate::{
    FinanceAccount,
    finance::domain::{
        account::FinanceAccountId,
        shared::{
            currency::Currency,
            id::Id,
            money::{Money, MoneyError},
        },
    },
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

#[derive(Clone)]
pub struct Movement {
    id: MovementId,
    account_id: FinanceAccountId,
    movement_type: MovementType,
    amount: Money,
    description: Option<String>,
}

impl Movement {
    pub fn new(
        account_id: FinanceAccountId,
        movement_type: MovementType,
        amount: Money,
        description: Option<String>,
    ) -> Result<Movement, MovementError> {
        if amount.is_less_or_eq_zero() {
            return Err(MovementError::InvalidAmount);
        }
        Ok(Movement {
            id: MovementId(Id::new()),
            account_id,
            movement_type,
            amount,
            description,
        })
    }

    pub fn id(&self) -> &MovementId {
        &self.id
    }

    pub fn amount(&self) -> &Money {
        &self.amount
    }
    pub fn description(&self) -> Option<String> {
        self.description.clone()
    }
}

#[cfg(test)]
mod tests {
    use uuid::Version;

    use crate::finance::domain::{
        account::FinanceAccountId,
        movement::{Movement, MovementError, MovementType},
        shared::{currency::Currency, money::Money},
    };

    #[test]
    fn test_movement_creation() {
        let movement = Movement::new(
            FinanceAccountId::new(),
            MovementType::Credit,
            Money::new(100, Currency::brl()),
            Some("Adding amount".to_string()),
        )
        .unwrap();

        assert_eq!(movement.id().0.value().get_version(), Some(Version::Random));
        assert_eq!(movement.description(), Some("Adding amount".to_string()));
        assert_eq!(movement.movement_type, MovementType::Credit);
        assert_eq!(movement.amount(), &Money::new(100, Currency::brl()));
    }

    #[test]
    fn creates_valid_credit() {
        let movement = Movement::new(
            FinanceAccountId::new(),
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
            FinanceAccountId::new(),
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
            FinanceAccountId::new(),
            MovementType::Credit,
            Money::zero(Currency::brl()),
            None,
        );

        assert!(matches!(result, Err(MovementError::InvalidAmount)));
    }

    #[test]
    fn rejects_negative_amount() {
        let result = Movement::new(
            FinanceAccountId::new(),
            MovementType::Debit,
            Money::new(-1, Currency::brl()),
            None,
        );

        assert!(matches!(result, Err(MovementError::InvalidAmount)));
    }

    #[test]
    fn preserves_account_id() {
        let account_id = FinanceAccountId::new();
        let movement = Movement::new(
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
            FinanceAccountId::new(),
            MovementType::Credit,
            Money::new(100, currency.clone()),
            None,
        )
        .unwrap();

        assert_eq!(movement.amount().currency(), &currency);
    }
}

use crate::{
    AccountError,
    finance::{
        application::{
            account_repository_trait::{AccountRepository, AccountRepositoryError},
            budget_repository_trait::{BudgetRepository, BudgetRepositoryError},
        },
        domain::{
            account::AccountId,
            budget::{BudgetError, BudgetId},
            movement::{MovementId, MovementType},
        },
    },
};

#[derive(Debug, PartialEq)]
pub enum BudgetMovementError {
    AccountNotFound,
    BudgetNotFound,
    InvalidAmount,
    AccountArchived,
    RepositoryFailure,
    InsufficientFunds,
    CurrencyMismatch,
}
impl From<AccountError> for BudgetMovementError {
    fn from(error: AccountError) -> Self {
        match error {
            AccountError::AccountArchived => Self::AccountArchived,
            AccountError::InsufficientFunds => Self::InsufficientFunds,
            AccountError::InvalidAmount => Self::InvalidAmount,
            AccountError::CurrencyMismatch => Self::CurrencyMismatch,
        }
    }
}
impl From<AccountRepositoryError> for BudgetMovementError {
    fn from(error: AccountRepositoryError) -> Self {
        match error {
            AccountRepositoryError::ReadFailed => Self::RepositoryFailure,
            AccountRepositoryError::UpdateFailed => Self::RepositoryFailure,
        }
    }
}

impl From<BudgetError> for BudgetMovementError {
    fn from(error: BudgetError) -> Self {
        match error {
            BudgetError::Archived => Self::AccountArchived,
            BudgetError::CurrencyMismatch => BudgetMovementError::CurrencyMismatch,
            BudgetError::InvalidMovementAmount => BudgetMovementError::InvalidAmount,
        }
    }
}

impl From<BudgetRepositoryError> for BudgetMovementError {
    fn from(error: BudgetRepositoryError) -> Self {
        match error {
            BudgetRepositoryError::ReadFailed => BudgetMovementError::RepositoryFailure,
            BudgetRepositoryError::UpdateFailed => BudgetMovementError::RepositoryFailure,
        }
    }
}

pub fn create_movement_in_budget<R, RA>(
    repository: &mut R,
    account_repo: &mut RA,
    budget_id: BudgetId,
    account_id: AccountId,
    amount_in_cents: i64,
    movement_type: MovementType,
    description: Option<String>,
) -> Result<MovementId, BudgetMovementError>
where
    R: BudgetRepository,
    RA: AccountRepository,
{
    if !account_repo.existing_by_id(&account_id)? {
        return Err(BudgetMovementError::AccountNotFound);
    };

    let Some(mut budget) = repository.find_by_id(&budget_id)? else {
        return Err(BudgetMovementError::BudgetNotFound);
    };

    let amount_money = budget.get_money(amount_in_cents);
    let result = budget.create_movement(account_id, movement_type, amount_money, description)?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::finance::{
        application::{
            budget_repository_trait::{BudgetRepository, BudgetRepositoryError},
            create_movement_in_budget::create_movement_in_budget,
        },
        domain::{
            budget::{Budget, BudgetId},
            shared::currency::Currency,
        },
    };

    struct FakeBudgetRepository {
        budget: Option<Budget>,
        with_error: bool,
    }

    impl FakeBudgetRepository {
        fn new(budget: Option<Budget>, with_error: bool) -> Self {
            FakeBudgetRepository { budget, with_error }
        }
    }

    impl BudgetRepository for FakeBudgetRepository {
        fn find_by_id(&self, id: &BudgetId) -> Result<Option<Budget>, BudgetRepositoryError> {
            if self.with_error {
                return Err(BudgetRepositoryError::ReadFailed);
            }
            match &self.budget {
                Some(bgt) => {
                    if bgt.id() == id {
                        Ok(self.budget.clone())
                    } else {
                        { Ok(None) }
                    }
                }
                None => Ok(None),
            }
        }

        fn update(&mut self, budget: Budget) -> Result<(), BudgetRepositoryError> {
            match &self.budget {
                Some(bgt) => {
                    if bgt.id() == budget.id() {
                        self.budget = Some(budget);
                        Ok(())
                    } else {
                        Err(BudgetRepositoryError::UpdateFailed)
                    }
                }
                None => Err(BudgetRepositoryError::UpdateFailed),
            }
        }
    }

    #[test]
    fn test_create_movement() {
        let mut budget = Budget::new("My Wallet".to_string(), Currency::brl());
        let mut repo = FakeBudgetRepository::new(Some(budget), false);

        // let result = create_movement_in_budget(&mut repo, account_repo, budget_id, account_id, amount_in_cents, movement_type, description)
    }
}

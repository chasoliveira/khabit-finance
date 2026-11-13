use crate::{
    AccountError,
    finance::{
        application::{
            account_repository_trait::{AccountRepository, AccountRepositoryError},
            budget_repository_trait::{BudgetRepository, BudgetRepositoryError},
        },
        domain::{
            account::AccountId,
            budgets::budget::{BudgetError, BudgetId},
            movement::{MovementId, MovementType},
        },
    },
};

#[derive(Debug, PartialEq)]
pub enum BudgetMovementError {
    AccountNotFound,
    AccountArchived,
    BudgetNotFound,
    BudgetArchived,
    InvalidAmount,
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
            BudgetError::Archived => Self::BudgetArchived,
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
    account_repo: &RA,
    budget_id: &BudgetId,
    account_id: &AccountId,
    amount_in_cents: i64,
    movement_type: MovementType,
    description: Option<String>,
) -> Result<MovementId, BudgetMovementError>
where
    R: BudgetRepository,
    RA: AccountRepository,
{
    let Some(account) = account_repo.find_by_id(account_id)? else {
        return Err(BudgetMovementError::AccountNotFound);
    };

    let Some(mut budget) = repository.find_by_id(&budget_id)? else {
        return Err(BudgetMovementError::BudgetNotFound);
    };

    if !account.matches_currency(budget.currency()) {
        return Err(BudgetMovementError::CurrencyMismatch);
    }

    let amount_money = budget.get_money(amount_in_cents);
    let result = budget.create_movement(account_id, movement_type, amount_money, description)?;

    repository.update(budget)?;

    Ok(result)
}

#[cfg(test)]
mod tests {
    use std::assert_matches;

    use crate::finance::{
        application::{
            account_repository_trait::AccountRepositoryError,
            budget_repository_trait::{BudgetRepository, BudgetRepositoryError},
            create_movement_in_budget::{BudgetMovementError, create_movement_in_budget},
            test_support::{
                fake_account_repository::{FakeAccountRepository, new_account, new_account_brl},
                fake_budget_repository::{FakeBudgetRepository, new_budget_brl},
            },
        },
        domain::{
            account::AccountId, budgets::budget::BudgetId, movement::MovementType,
            shared::currency::Currency,
        },
    };

    #[test]
    fn test_create_movement() {
        let account = new_account_brl();
        let account_id = account.id().clone();
        let account_repo = FakeAccountRepository::new(Some(account));
        let budget = new_budget_brl();
        let budget_id = budget.id().clone();
        let mut repo = FakeBudgetRepository::new(Some(budget));

        let result = create_movement_in_budget(
            &mut repo,
            &account_repo,
            &budget_id,
            &account_id,
            12345,
            MovementType::Credit,
            Some("Movement".to_string()),
        );

        assert_matches!(result, Ok(_));
        assert_eq!(
            repo.find_by_id(&budget_id)
                .unwrap()
                .unwrap()
                .movements()
                .len(),
            1
        );
    }

    #[test]
    fn test_create_movement_error_when_account_not_found() {
        let account_repo = FakeAccountRepository::new(None);
        let budget = new_budget_brl();
        let budget_id = budget.id().clone();
        let mut repo = FakeBudgetRepository::new(Some(budget));

        let result = create_movement_in_budget(
            &mut repo,
            &account_repo,
            &budget_id,
            &AccountId::new(),
            12345,
            MovementType::Credit,
            Some("Movement".to_string()),
        );

        assert_eq!(result, Err(BudgetMovementError::AccountNotFound));
    }

    #[test]
    fn test_create_movement_error_when_account_has_error() {
        let account = new_account_brl();
        let account_repo = FakeAccountRepository::with_read_error(
            Some(account),
            Some(AccountRepositoryError::ReadFailed),
        );
        let budget = new_budget_brl();
        let budget_id = budget.id().clone();
        let mut repo = FakeBudgetRepository::new(Some(budget));

        let unmatch_account_id = AccountId::new();
        let result = create_movement_in_budget(
            &mut repo,
            &account_repo,
            &budget_id,
            &unmatch_account_id,
            12345,
            MovementType::Credit,
            Some("Movement".to_string()),
        );

        assert_eq!(result, Err(BudgetMovementError::RepositoryFailure));
    }

    #[test]
    fn test_create_movement_error_when_budget_not_found() {
        let account = new_account_brl();
        let account_id = account.id().clone();
        let account_repo = FakeAccountRepository::new(Some(account));
        let mut repo = FakeBudgetRepository::new(None);

        let result = create_movement_in_budget(
            &mut repo,
            &account_repo,
            &BudgetId::new(),
            &account_id,
            12345,
            MovementType::Credit,
            Some("Movement".to_string()),
        );

        assert_eq!(result, Err(BudgetMovementError::BudgetNotFound));
    }

    #[test]
    fn test_create_movement_error_when_budget_read_fails() {
        let account = new_account_brl();
        let account_id = account.id().clone();
        let account_repo = FakeAccountRepository::new(Some(account));
        let budget = new_budget_brl();
        let budget_id = budget.id().clone();
        let mut repo = FakeBudgetRepository::with_read_error(
            Some(budget),
            Some(BudgetRepositoryError::ReadFailed),
        );

        let result = create_movement_in_budget(
            &mut repo,
            &account_repo,
            &budget_id,
            &account_id,
            12345,
            MovementType::Credit,
            Some("Movement".to_string()),
        );

        assert_eq!(result, Err(BudgetMovementError::RepositoryFailure));
    }

    #[test]
    fn test_create_movement_error_when_budget_update_fails() {
        let account = new_account_brl();
        let account_id = account.id().clone();
        let account_repo = FakeAccountRepository::new(Some(account));
        let budget = new_budget_brl();
        let budget_id = budget.id().clone();
        let mut repo = FakeBudgetRepository::with_update_error(
            Some(budget),
            Some(BudgetRepositoryError::UpdateFailed),
        );

        let result = create_movement_in_budget(
            &mut repo,
            &account_repo,
            &budget_id,
            &account_id,
            12345,
            MovementType::Credit,
            Some("Movement".to_string()),
        );

        assert_eq!(result, Err(BudgetMovementError::RepositoryFailure));
    }

    #[test]
    fn test_create_movement_error_when_budget_is_archived() {
        let account = new_account_brl();
        let account_id = account.id().clone();
        let account_repo = FakeAccountRepository::new(Some(account));
        let mut budget = new_budget_brl();
        budget.archive();
        let budget_id = budget.id().clone();
        let mut repo = FakeBudgetRepository::new(Some(budget));

        let result = create_movement_in_budget(
            &mut repo,
            &account_repo,
            &budget_id,
            &account_id,
            12345,
            MovementType::Credit,
            Some("Movement".to_string()),
        );

        assert_eq!(result, Err(BudgetMovementError::BudgetArchived));
    }

    #[test]
    fn test_create_movement_error_when_account_mismatch_currency() {
        let account = new_account(Currency::can());
        let account_id = account.id().clone();
        let account_repo = FakeAccountRepository::new(Some(account));
        let budget = new_budget_brl();
        let budget_id = budget.id().clone();
        let mut repo = FakeBudgetRepository::new(Some(budget));

        let result = create_movement_in_budget(
            &mut repo,
            &account_repo,
            &budget_id,
            &account_id,
            12345,
            MovementType::Credit,
            Some("Movement".to_string()),
        );

        assert_eq!(result, Err(BudgetMovementError::CurrencyMismatch));
    }
}

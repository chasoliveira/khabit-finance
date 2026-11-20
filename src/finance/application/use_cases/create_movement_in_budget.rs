use crate::{
    AccountError,
    finance::{
        application::unit_of_work_trait::{UnitOfWork, UnitOfWorkError},
        domain::{
            account::AccountId,
            budgets::{
                budget::{BudgetError, BudgetId},
                movements::movement::{MovementId, MovementType},
            },
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
impl From<UnitOfWorkError> for BudgetMovementError {
    fn from(error: UnitOfWorkError) -> Self {
        match error {
            UnitOfWorkError::ReadFailed
            | UnitOfWorkError::UpdateFailed
            | UnitOfWorkError::CommitFailed => Self::RepositoryFailure,
        }
    }
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

impl From<BudgetError> for BudgetMovementError {
    fn from(error: BudgetError) -> Self {
        match error {
            BudgetError::Archived => Self::BudgetArchived,
            BudgetError::CurrencyMismatch => BudgetMovementError::CurrencyMismatch,
            BudgetError::InvalidMovementAmount => BudgetMovementError::InvalidAmount,
        }
    }
}

pub fn create_movement_in_budget<U>(
    transact: &mut U,
    budget_id: &BudgetId,
    account_id: &AccountId,
    amount_in_cents: i64,
    movement_type: MovementType,
    description: Option<String>,
) -> Result<MovementId, BudgetMovementError>
where
    U: UnitOfWork<Error = UnitOfWorkError>,
{
    let tx = transact;

    let account = tx
        .find_account(account_id)?
        .ok_or(BudgetMovementError::AccountNotFound)?;

    let mut budget = tx
        .find_budget(&budget_id)?
        .ok_or(BudgetMovementError::BudgetNotFound)?;

    if !account.matches_currency(budget.currency()) {
        return Err(BudgetMovementError::CurrencyMismatch);
    }

    let movement_id = budget.create_movement(
        account_id,
        movement_type,
        budget.get_money(amount_in_cents),
        description,
    )?;

    tx.update_budget(budget)?;

    Ok(movement_id)
}

#[cfg(test)]
mod tests {
    use std::assert_matches;

    use crate::finance::{
        application::{
            test_support::{
                fake_unit_of_work::FakeUnitOfWork,
                test_helper::{new_account, new_account_brl, new_budget_brl},
            },
            unit_of_work_trait::UnitOfWorkError,
            use_cases::create_movement_in_budget::{
                BudgetMovementError, create_movement_in_budget,
            },
        },
        domain::{
            account::AccountId,
            budgets::{budget::BudgetId, movements::movement::MovementType},
            shared::currency::Currency,
        },
        infrastructure::contexts::{
            in_memory_database::InMemoryDatabase, in_memory_unit_of_work::InMemoryUnitOfWork,
        },
    };

    #[test]
    fn test_create_movement() {
        let mut db = InMemoryDatabase::new();

        let account = new_account_brl();
        let account_id = account.id().clone();

        let budget = new_budget_brl();
        let budget_id = budget.id().clone();

        let mut transaction = InMemoryUnitOfWork::begin(&mut db);

        transaction.database_mut().add_account(account);
        transaction.database_mut().add_budget(budget);

        let result = create_movement_in_budget(
            &mut transaction,
            &budget_id,
            &account_id,
            12345,
            MovementType::Credit,
            Some("Movement".to_string()),
        );

        assert_matches!(result, Ok(_));

        transaction.commit();

        let stored_budget = db
            .budgets()
            .iter()
            .find(|budget| budget.id() == &budget_id)
            .expect("budget should have been persisted");

        assert_eq!(stored_budget.movements().len(), 1);
    }

    #[test]
    fn test_create_movement_error_when_account_not_found() {
        let mut db = InMemoryDatabase::new();
        let budget = new_budget_brl();
        let budget_id = budget.id().clone();

        let result = {
            let mut transaction = InMemoryUnitOfWork::begin(&mut db);
            transaction.database_mut().add_budget(budget);

            create_movement_in_budget(
                &mut transaction,
                &budget_id,
                &AccountId::new(),
                12345,
                MovementType::Credit,
                Some("Movement".to_string()),
            )
        };

        assert_eq!(result, Err(BudgetMovementError::AccountNotFound));
    }

    #[test]
    fn test_create_movement_error_when_account_has_error() {
        let account = new_account_brl();
        let account_id = account.id().clone();

        let budget = new_budget_brl();
        let budget_id = budget.id().clone();

        let mut transaction = FakeUnitOfWork::new(Some(account), Some(budget))
            .with_account_read_error(UnitOfWorkError::ReadFailed);

        let result = create_movement_in_budget(
            &mut transaction,
            &budget_id,
            &account_id,
            12345,
            MovementType::Credit,
            Some("Movement".to_string()),
        );

        assert_eq!(result, Err(BudgetMovementError::RepositoryFailure));

        assert!(!transaction.committed());
    }

    #[test]
    fn test_create_movement_error_when_budget_not_found() {
        let mut db = InMemoryDatabase::new();

        let account = new_account_brl();
        let account_id = account.id().clone();
        let budget_id = BudgetId::new();

        let result = {
            let mut transaction = InMemoryUnitOfWork::begin(&mut db);

            transaction.database_mut().add_account(account);

            create_movement_in_budget(
                &mut transaction,
                &budget_id,
                &account_id,
                12345,
                MovementType::Credit,
                Some("Movement".to_string()),
            )
        };

        assert_eq!(result, Err(BudgetMovementError::BudgetNotFound));
    }

    #[test]
    fn test_create_movement_error_when_budget_read_fails() {
        let account = new_account_brl();
        let account_id = account.id().clone();

        let budget = new_budget_brl();
        let budget_id = budget.id().clone();

        let mut transaction = FakeUnitOfWork::new(Some(account), Some(budget))
            .with_budget_read_error(UnitOfWorkError::ReadFailed);

        let result = create_movement_in_budget(
            &mut transaction,
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

        let budget = new_budget_brl();
        let budget_id = budget.id().clone();

        let mut transaction = FakeUnitOfWork::new(Some(account), Some(budget))
            .with_budget_update_error(UnitOfWorkError::UpdateFailed);

        let result = create_movement_in_budget(
            &mut transaction,
            &budget_id,
            &account_id,
            12345,
            MovementType::Credit,
            Some("Movement".to_string()),
        );

        assert_eq!(result, Err(BudgetMovementError::RepositoryFailure));

        assert!(!transaction.committed());
    }

    #[test]
    fn test_create_movement_error_when_budget_is_archived() {
        let mut db = InMemoryDatabase::new();

        let account = new_account_brl();
        let account_id = account.id().clone();

        let mut budget = new_budget_brl();
        budget.archive();

        let budget_id = budget.id().clone();

        let result = {
            let mut transaction = InMemoryUnitOfWork::begin(&mut db);

            transaction.database_mut().add_account(account);

            transaction.database_mut().add_budget(budget);

            create_movement_in_budget(
                &mut transaction,
                &budget_id,
                &account_id,
                12345,
                MovementType::Credit,
                Some("Movement".to_string()),
            )
        };

        assert_eq!(result, Err(BudgetMovementError::BudgetArchived));
    }

    #[test]
    fn test_create_movement_error_when_account_mismatch_currency() {
        let mut db = InMemoryDatabase::new();

        let account = new_account(Currency::can());
        let account_id = account.id().clone();

        let budget = new_budget_brl();
        let budget_id = budget.id().clone();

        let result = {
            let mut transaction = InMemoryUnitOfWork::begin(&mut db);

            transaction.database_mut().add_account(account);

            transaction.database_mut().add_budget(budget);

            create_movement_in_budget(
                &mut transaction,
                &budget_id,
                &account_id,
                12345,
                MovementType::Credit,
                Some("Movement".to_string()),
            )
        };

        assert_eq!(result, Err(BudgetMovementError::CurrencyMismatch));
    }
}

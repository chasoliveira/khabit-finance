use crate::{
    AccountError,
    finance::{
        application::unit_of_work_trait::{UnitOfWork, UnitOfWorkError},
        domain::{account::AccountId, shared::money::Money},
    },
};

#[derive(PartialEq, Debug)]
pub enum WithdrawError {
    AccountNotFound,
    InvalidAmount,
    AccountArchived,
    InsufficientFunds,
    RepositoryFailure,
    CurrencyMismatch,
}

impl From<AccountError> for WithdrawError {
    fn from(error: AccountError) -> Self {
        match error {
            AccountError::AccountArchived => Self::AccountArchived,
            AccountError::InsufficientFunds => Self::InsufficientFunds,
            AccountError::InvalidAmount => Self::InvalidAmount,
            AccountError::CurrencyMismatch => Self::CurrencyMismatch,
        }
    }
}
impl From<UnitOfWorkError> for WithdrawError {
    fn from(error: UnitOfWorkError) -> Self {
        match error {
            UnitOfWorkError::ReadFailed
            | UnitOfWorkError::UpdateFailed
            | UnitOfWorkError::CommitFailed => Self::RepositoryFailure,
        }
    }
}

pub fn withdraw_from_account<U>(
    transaction: &mut U,
    account_id: &AccountId,
    amount: Money,
) -> Result<Money, WithdrawError>
where
    U: UnitOfWork<Error = UnitOfWorkError>,
{
    let Some(mut account) = transaction.find_account(account_id)? else {
        return Err(WithdrawError::AccountNotFound);
    };
    account.withdraw(&amount)?;
    let balance = account.balance().to_owned();
    transaction.update_account(account)?;

    Ok(balance)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::finance::application::test_support::fake_unit_of_work::FakeUnitOfWork;
    use crate::finance::application::test_support::test_helper::new_account_brl;
    use crate::finance::domain::account::AccountId;
    use crate::finance::domain::shared::currency::Currency;
    use crate::finance::infrastructure::contexts::in_memory_database::InMemoryDatabase;
    use crate::finance::infrastructure::contexts::in_memory_unit_of_work::InMemoryUnitOfWork;

    #[test]
    fn test_withdraw_from_account() {
        let mut db = InMemoryDatabase::new();

        let mut account = new_account_brl();
        account.deposit(&Money::new(1000, Currency::brl())).unwrap();

        let account_id = account.id().clone();

        let mut transaction = InMemoryUnitOfWork::begin(&mut db);
        transaction.database_mut().add_account(account);

        let result = withdraw_from_account(
            &mut transaction,
            &account_id,
            Money::new(500, Currency::brl()),
        );

        assert_eq!(result, Ok(Money::new(500, Currency::brl())));

        transaction.commit().unwrap();

        let stored_account = db
            .accounts()
            .iter()
            .find(|account| account.id() == &account_id)
            .expect("account should have persisted");

        assert_eq!(stored_account.balance(), &Money::new(500, Currency::brl()));
    }

    #[test]
    fn test_withdraw_from_account_insufficient_funds() {
        let mut db = InMemoryDatabase::new();

        let mut account = new_account_brl();
        account.deposit(&Money::new(1000, Currency::brl())).unwrap();

        let account_id = account.id().clone();

        let result = {
            let mut transaction = InMemoryUnitOfWork::begin(&mut db);

            transaction.database_mut().add_account(account);

            withdraw_from_account(
                &mut transaction,
                &account_id,
                Money::new(1500, Currency::brl()),
            )
        };

        assert_eq!(result, Err(WithdrawError::InsufficientFunds));

        assert!(
            db.accounts().is_empty(),
            "failed transactions must not persist changes"
        );
    }

    #[test]
    fn test_withdraw_from_account_invalid_amount() {
        let mut db = InMemoryDatabase::new();

        let mut account = new_account_brl();
        account.deposit(&Money::new(1000, Currency::brl())).unwrap();

        let account_id = account.id().clone();

        let result = {
            let mut transaction = InMemoryUnitOfWork::begin(&mut db);

            transaction.database_mut().add_account(account);

            withdraw_from_account(
                &mut transaction,
                &account_id,
                Money::new(-500, Currency::brl()),
            )
        };

        assert_eq!(result, Err(WithdrawError::InvalidAmount));

        assert!(
            db.accounts().is_empty(),
            "failed transactions must not persist changes"
        );
    }

    #[test]
    fn test_withdraw_from_account_account_archived() {
        let mut db = InMemoryDatabase::new();

        let mut account = new_account_brl();
        account.deposit(&Money::new(1000, Currency::brl())).unwrap();
        account.archive();

        let account_id = account.id().clone();

        let result = {
            let mut transaction = InMemoryUnitOfWork::begin(&mut db);

            transaction.database_mut().add_account(account);

            withdraw_from_account(
                &mut transaction,
                &account_id,
                Money::new(500, Currency::brl()),
            )
        };

        assert_eq!(result, Err(WithdrawError::AccountArchived));
    }

    #[test]
    fn test_withdraw_from_account_account_not_found() {
        let mut db = InMemoryDatabase::new();
        let account_id = AccountId::new();

        let result = {
            let mut transaction = InMemoryUnitOfWork::begin(&mut db);

            withdraw_from_account(
                &mut transaction,
                &account_id,
                Money::new(500, Currency::brl()),
            )
        };

        assert_eq!(result, Err(WithdrawError::AccountNotFound));
    }
    #[test]
    fn test_withdraw_from_account_simulated_read_error() {
        let mut account = new_account_brl();
        account.deposit(&Money::new(1000, Currency::brl())).unwrap();

        let account_id = account.id().clone();

        let mut transaction = FakeUnitOfWork::new(Some(account), None)
            .with_account_read_error(UnitOfWorkError::ReadFailed);

        let result = withdraw_from_account(
            &mut transaction,
            &account_id,
            Money::new(500, Currency::brl()),
        );

        assert_eq!(result, Err(WithdrawError::RepositoryFailure));

        assert!(!transaction.committed());
    }

    #[test]
    fn test_withdraw_from_account_simulated_update_error() {
        let mut account = new_account_brl();
        account.deposit(&Money::new(1000, Currency::brl())).unwrap();

        let account_id = account.id().clone();

        let mut transaction = FakeUnitOfWork::new(Some(account), None)
            .with_account_update_error(UnitOfWorkError::UpdateFailed);

        let result = withdraw_from_account(
            &mut transaction,
            &account_id,
            Money::new(500, Currency::brl()),
        );

        assert_eq!(result, Err(WithdrawError::RepositoryFailure));

        assert!(!transaction.committed());
    }
}

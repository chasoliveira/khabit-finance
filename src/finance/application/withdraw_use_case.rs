use crate::{
    FinanceAccountError,
    finance::{
        application::account_repository_trait::{AccountRepository, AccountRepositoryError},
        domain::account::FinanceAccountId,
    },
};

#[derive(PartialEq, Debug)]
pub enum WithdrawError {
    AccountNotFound,
    InvalidAmount,
    AccountArchived,
    InsufficientFunds,
    RepositoryFailure,
}

impl From<FinanceAccountError> for WithdrawError {
    fn from(error: FinanceAccountError) -> Self {
        match error {
            FinanceAccountError::AccountArchived => Self::AccountArchived,
            FinanceAccountError::InsufficientFunds => Self::InsufficientFunds,
            FinanceAccountError::InvalidAmount => Self::InvalidAmount,
        }
    }
}
impl From<AccountRepositoryError> for WithdrawError {
    fn from(error: AccountRepositoryError) -> Self {
        match error {
            AccountRepositoryError::ReadFailed => Self::RepositoryFailure,
            AccountRepositoryError::UpdateFailed => Self::RepositoryFailure,
        }
    }
}

pub fn withdraw_from_account<R>(
    repository: &mut R,
    account_id: &FinanceAccountId,
    amount_in_cents: i64,
) -> Result<i64, WithdrawError>
where
    R: AccountRepository,
{
    let Some(mut account) = repository.find_by_id(account_id)? else {
        return Err(WithdrawError::AccountNotFound);
    };
    account.withdraw(amount_in_cents)?;
    let balance = account.balance_in_cents();
    repository.update(account)?;

    Ok(balance)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::finance::application::account_repository_trait::{
        AccountRepository, AccountRepositoryError,
    };
    use crate::finance::domain::account::{FinanceAccount, FinanceAccountId, FinanceAccountType};

    struct FakeAccountRepository {
        account: Option<FinanceAccount>,
        with_error: bool,
    }
    impl FakeAccountRepository {
        fn new(account: Option<FinanceAccount>, with_error: bool) -> Self {
            FakeAccountRepository {
                account,
                with_error,
            }
        }
    }
    impl AccountRepository for FakeAccountRepository {
        fn find_by_id(
            &self,
            id: &FinanceAccountId,
        ) -> Result<Option<FinanceAccount>, AccountRepositoryError> {
            if self.with_error {
                return Err(AccountRepositoryError::ReadFailed);
            }
            match &self.account {
                Some(acc) => {
                    if acc.id() == id {
                        Ok(self.account.clone())
                    } else {
                        Ok(None)
                    }
                }
                None => Ok(None),
            }
        }

        fn update(&mut self, account: FinanceAccount) -> Result<(), AccountRepositoryError> {
            match &self.account {
                Some(acc) => {
                    if acc.id() == account.id() {
                        self.account = Some(account);
                        Ok(())
                    } else {
                        Err(AccountRepositoryError::UpdateFailed)
                    }
                }
                None => Err(AccountRepositoryError::UpdateFailed),
            }
        }
    }

    #[test]
    fn test_withdraw_from_account() {
        let mut account = FinanceAccount::new(
            "My Checking Account".to_string(),
            FinanceAccountType::Checking,
        );
        account.deposit(1000).unwrap();
        let id = &account.id().clone();

        let mut repository = FakeAccountRepository::new(Some(account), false);

        let result = withdraw_from_account(&mut repository, &id, 500);
        assert_eq!(result, Ok(500));
        assert_eq!(
            repository
                .find_by_id(&id)
                .unwrap()
                .unwrap()
                .balance_in_cents(),
            500
        );
    }

    #[test]
    fn test_withdraw_from_account_insufficient_funds() {
        let mut account = FinanceAccount::new(
            "My Checking Account".to_string(),
            FinanceAccountType::Checking,
        );
        account.deposit(1000).unwrap();
        let id = &account.id().clone();

        let mut repository = FakeAccountRepository::new(Some(account), false);

        let result = withdraw_from_account(&mut repository, &id, 1500);
        assert_eq!(result, Err(WithdrawError::InsufficientFunds));
        let account = repository.find_by_id(&id).unwrap().unwrap();
        assert_eq!(account.balance_in_cents(), 1000);
    }

    #[test]
    fn test_withdraw_from_account_invalid_amount() {
        let mut account = FinanceAccount::new(
            "My Checking Account".to_string(),
            FinanceAccountType::Checking,
        );
        account.deposit(1000).unwrap();
        let id = &account.id().clone();
        let mut repository = FakeAccountRepository::new(Some(account), false);

        let result = withdraw_from_account(&mut repository, &id, -500);
        assert_eq!(result, Err(WithdrawError::InvalidAmount));
        assert_eq!(
            repository
                .find_by_id(&id)
                .unwrap()
                .unwrap()
                .balance_in_cents(),
            1000
        );
    }

    #[test]
    fn test_withdraw_from_account_account_archived() {
        let mut account = FinanceAccount::new(
            "My Checking Account".to_string(),
            FinanceAccountType::Checking,
        );
        account.deposit(1000).unwrap();
        account.archive();
        let id = &account.id().clone();
        let mut repository = FakeAccountRepository::new(Some(account), false);

        let result = withdraw_from_account(&mut repository, &id, 500);
        assert_eq!(result, Err(WithdrawError::AccountArchived));
        assert_eq!(
            repository
                .find_by_id(&id)
                .unwrap()
                .unwrap()
                .balance_in_cents(),
            1000
        );
    }

    #[test]
    fn test_withdraw_from_account_account_not_found() {
        let mut account = FinanceAccount::new(
            "My Checking Account".to_string(),
            FinanceAccountType::Checking,
        );
        account.deposit(1000).unwrap();

        let mut repository = FakeAccountRepository::new(Some(account), false);
        let result = withdraw_from_account(&mut repository, &FinanceAccountId::new(), 500);
        assert_eq!(result, Err(WithdrawError::AccountNotFound));
    }

    #[test]
    fn test_withdraw_from_account_simulated_error() {
        let mut account = FinanceAccount::new(
            "My Checking Account".to_string(),
            FinanceAccountType::Checking,
        );
        account.deposit(1000).unwrap();
        let id = &account.id().clone();
        let mut repository = FakeAccountRepository::new(Some(account), true);
        let result = withdraw_from_account(&mut repository, id, 500);
        assert_eq!(result, Err(WithdrawError::RepositoryFailure));
    }
}

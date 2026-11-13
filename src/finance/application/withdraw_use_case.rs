use crate::{
    AccountError,
    finance::{
        application::account_repository_trait::{AccountRepository, AccountRepositoryError},
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
    account_id: &AccountId,
    amount: Money,
) -> Result<Money, WithdrawError>
where
    R: AccountRepository,
{
    let Some(mut account) = repository.find_by_id(account_id)? else {
        return Err(WithdrawError::AccountNotFound);
    };
    account.withdraw(&amount)?;
    let balance = account.balance().to_owned();
    repository.update(account)?;

    Ok(balance)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::finance::application::account_repository_trait::{
        AccountRepository, AccountRepositoryError,
    };
    use crate::finance::domain::account::{Account, AccountId, AccountType};
    use crate::finance::domain::shared::currency::Currency;

    struct FakeAccountRepository {
        account: Option<Account>,
        with_error: bool,
    }
    impl FakeAccountRepository {
        fn new(account: Option<Account>, with_error: bool) -> Self {
            FakeAccountRepository {
                account,
                with_error,
            }
        }
    }
    impl AccountRepository for FakeAccountRepository {
        fn find_by_id(&self, id: &AccountId) -> Result<Option<Account>, AccountRepositoryError> {
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

        fn update(&mut self, account: Account) -> Result<(), AccountRepositoryError> {
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
        let mut account = Account::new(
            "My Checking Account".to_string(),
            AccountType::Checking,
            Currency::brl(),
        );
        account.deposit(&Money::new(1000, Currency::brl())).unwrap();
        let id = account.id().clone();

        let mut repository = FakeAccountRepository::new(Some(account), false);

        let result = withdraw_from_account(&mut repository, &id, Money::new(500, Currency::brl()));
        assert_eq!(result, Ok(Money::new(500, Currency::brl())));
        assert_eq!(
            repository.find_by_id(&id).unwrap().unwrap().balance(),
            &Money::new(500, Currency::brl())
        );
    }

    #[test]
    fn test_withdraw_from_account_insufficient_funds() {
        let mut account = Account::new(
            "My Checking Account".to_string(),
            AccountType::Checking,
            Currency::brl(),
        );
        account.deposit(&Money::new(1000, Currency::brl())).unwrap();
        let id = account.id().clone();

        let mut repository = FakeAccountRepository::new(Some(account), false);

        let result = withdraw_from_account(&mut repository, &id, Money::new(1500, Currency::brl()));
        assert_eq!(result, Err(WithdrawError::InsufficientFunds));
        let account = repository.find_by_id(&id).unwrap().unwrap();
        assert_eq!(account.balance(), &Money::new(1000, Currency::brl()));
    }

    #[test]
    fn test_withdraw_from_account_invalid_amount() {
        let mut account = Account::new(
            "My Checking Account".to_string(),
            AccountType::Checking,
            Currency::brl(),
        );
        account.deposit(&Money::new(1000, Currency::brl())).unwrap();
        let id = account.id().clone();
        let mut repository = FakeAccountRepository::new(Some(account), false);

        let result = withdraw_from_account(&mut repository, &id, Money::new(-500, Currency::brl()));
        assert_eq!(result, Err(WithdrawError::InvalidAmount));
        assert_eq!(
            repository.find_by_id(&id).unwrap().unwrap().balance(),
            &Money::new(1000, Currency::brl())
        );
    }

    #[test]
    fn test_withdraw_from_account_account_archived() {
        let mut account = Account::new(
            "My Checking Account".to_string(),
            AccountType::Checking,
            Currency::brl(),
        );
        account.deposit(&Money::new(1000, Currency::brl())).unwrap();
        account.archive();
        let id = account.id().clone();
        let mut repository = FakeAccountRepository::new(Some(account), false);

        let result = withdraw_from_account(&mut repository, &id, Money::new(500, Currency::brl()));
        assert_eq!(result, Err(WithdrawError::AccountArchived));
        assert_eq!(
            repository.find_by_id(&id).unwrap().unwrap().balance(),
            &Money::new(1000, Currency::brl())
        );
    }

    #[test]
    fn test_withdraw_from_account_account_not_found() {
        let mut account = Account::new(
            "My Checking Account".to_string(),
            AccountType::Checking,
            Currency::brl(),
        );
        account.deposit(&Money::new(1000, Currency::brl())).unwrap();

        let mut repository = FakeAccountRepository::new(Some(account), false);
        let result = withdraw_from_account(
            &mut repository,
            &AccountId::new(),
            Money::new(500, Currency::brl()),
        );
        assert_eq!(result, Err(WithdrawError::AccountNotFound));
    }

    #[test]
    fn test_withdraw_from_account_simulated_error() {
        let mut account = Account::new(
            "My Checking Account".to_string(),
            AccountType::Checking,
            Currency::brl(),
        );
        account.deposit(&Money::new(1000, Currency::brl())).unwrap();
        let id = account.id().clone();
        let mut repository = FakeAccountRepository::new(Some(account), true);
        let result = withdraw_from_account(&mut repository, &id, Money::new(500, Currency::brl()));
        assert_eq!(result, Err(WithdrawError::RepositoryFailure));
    }
}

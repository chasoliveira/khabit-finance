use crate::{
    FinanceAccountError, finance::application::account_repository_trait::AccountRepository,
};

#[derive(PartialEq, Debug)]
pub enum WithdrawError {
    AccountNotFound,
    InvalidAmount,
    AccountArchived,
    InsufficientFunds,
}

pub fn withdraw_from_account<R>(
    repository: &mut R,
    account_id: &str,
    amount_in_cents: i64,
) -> Result<i64, WithdrawError>
where
    R: AccountRepository,
{
    let Some(account) = repository.find_by_id_mut(account_id) else {
        return Err(WithdrawError::AccountNotFound);
    };

    match account.withdraw(amount_in_cents) {
        Ok(()) => Ok(account.balance_in_cents()),
        Err(e) => match e {
            FinanceAccountError::AccountArchived => Err(WithdrawError::AccountArchived),
            FinanceAccountError::InsufficientFunds => Err(WithdrawError::InsufficientFunds),
            FinanceAccountError::InvalidAmount => Err(WithdrawError::InvalidAmount),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::finance::application::account_repository_trait::AccountRepository;
    use crate::finance::domain::account::{FinanceAccount, FinanceAccountType};

    struct FakeAccountRepository {
        account: Option<FinanceAccount>,
    }
    impl FakeAccountRepository {
        fn new(account: Option<FinanceAccount>) -> Self {
            FakeAccountRepository { account }
        }
    }
    impl AccountRepository for FakeAccountRepository {
        fn find_by_id_mut(&mut self, id: &str) -> Option<&mut FinanceAccount> {
            match self.account.as_mut() {
                Some(acc) => {
                    if acc.id() == id {
                        Some(acc)
                    } else {
                        None
                    }
                }
                None => None,
            }
        }
    }

    #[test]
    fn test_withdraw_from_account() {
        let mut account = FinanceAccount::new(
            "1".to_string(),
            "My Checking Account".to_string(),
            FinanceAccountType::Checking,
        );
        account.deposit(1000).unwrap();

        let mut repository = FakeAccountRepository::new(Some(account));

        let result = withdraw_from_account(&mut repository, "1", 500);
        assert_eq!(result, Ok(500));
        assert_eq!(
            repository.find_by_id_mut("1").unwrap().balance_in_cents(),
            500
        );
    }

    #[test]
    fn test_withdraw_from_account_insufficient_funds() {
        let mut account = FinanceAccount::new(
            "1".to_string(),
            "My Checking Account".to_string(),
            FinanceAccountType::Checking,
        );
        account.deposit(1000).unwrap();

        let mut repository = FakeAccountRepository::new(Some(account));

        let result = withdraw_from_account(&mut repository, "1", 1500);
        assert_eq!(result, Err(WithdrawError::InsufficientFunds));
        assert_eq!(
            repository.find_by_id_mut("1").unwrap().balance_in_cents(),
            1000
        );
    }

    #[test]
    fn test_withdraw_from_account_invalid_amount() {
        let mut account = FinanceAccount::new(
            "1".to_string(),
            "My Checking Account".to_string(),
            FinanceAccountType::Checking,
        );
        account.deposit(1000).unwrap();
        let mut repository = FakeAccountRepository::new(Some(account));

        let result = withdraw_from_account(&mut repository, "1", -500);
        assert_eq!(result, Err(WithdrawError::InvalidAmount));
        assert_eq!(
            repository.find_by_id_mut("1").unwrap().balance_in_cents(),
            1000
        );
    }

    #[test]
    fn test_withdraw_from_account_account_archived() {
        let mut account = FinanceAccount::new(
            "1".to_string(),
            "My Checking Account".to_string(),
            FinanceAccountType::Checking,
        );
        account.deposit(1000).unwrap();
        account.archive();
        let mut repository = FakeAccountRepository::new(Some(account));

        let result = withdraw_from_account(&mut repository, "1", 500);
        assert_eq!(result, Err(WithdrawError::AccountArchived));
        assert_eq!(
            repository.find_by_id_mut("1").unwrap().balance_in_cents(),
            1000
        );
    }

    #[test]
    fn test_withdraw_from_account_account_not_found() {
        let mut account = FinanceAccount::new(
            "1".to_string(),
            "My Checking Account".to_string(),
            FinanceAccountType::Checking,
        );
        account.deposit(1000).unwrap();

        let mut repository = FakeAccountRepository::new(Some(account));
        let result = withdraw_from_account(&mut repository, "999", 500);
        assert_eq!(result, Err(WithdrawError::AccountNotFound));
    }
}

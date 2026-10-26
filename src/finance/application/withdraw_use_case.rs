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
    use crate::finance::domain::account::{FinanceAccount, FinanceAccountType};
    use crate::finance::infrastructure::in_memory_account_repository::InMemoryAccountRepository;

    #[test]
    fn test_withdraw_from_account() {
        let mut repository = InMemoryAccountRepository::new();
        let mut account = FinanceAccount::new(
            "1".to_string(),
            "My Checking Account".to_string(),
            FinanceAccountType::Checking,
        );
        account.deposit(1000).unwrap();
        repository.add(account);

        let result = withdraw_from_account(&mut repository, "1", 500);
        assert_eq!(result, Ok(500));
        assert_eq!(
            repository.find_by_id_mut("1").unwrap().balance_in_cents(),
            500
        );
    }

    #[test]
    fn test_withdraw_from_account_insufficient_funds() {
        let mut repository = InMemoryAccountRepository::new();
        let mut account = FinanceAccount::new(
            "1".to_string(),
            "My Checking Account".to_string(),
            FinanceAccountType::Checking,
        );
        account.deposit(1000).unwrap();
        repository.add(account);

        let result = withdraw_from_account(&mut repository, "1", 1500);
        assert_eq!(result, Err(WithdrawError::InsufficientFunds));
        assert_eq!(
            repository.find_by_id_mut("1").unwrap().balance_in_cents(),
            1000
        );
    }

    #[test]
    fn test_withdraw_from_account_invalid_amount() {
        let mut repository = InMemoryAccountRepository::new();
        let mut account = FinanceAccount::new(
            "1".to_string(),
            "My Checking Account".to_string(),
            FinanceAccountType::Checking,
        );
        account.deposit(1000).unwrap();
        repository.add(account);

        let result = withdraw_from_account(&mut repository, "1", -500);
        assert_eq!(result, Err(WithdrawError::InvalidAmount));
        assert_eq!(
            repository.find_by_id_mut("1").unwrap().balance_in_cents(),
            1000
        );
    }

    #[test]
    fn test_withdraw_from_account_account_archived() {
        let mut repository = InMemoryAccountRepository::new();
        let mut account = FinanceAccount::new(
            "1".to_string(),
            "My Checking Account".to_string(),
            FinanceAccountType::Checking,
        );
        account.deposit(1000).unwrap();
        account.archive();
        repository.add(account);

        let result = withdraw_from_account(&mut repository, "1", 500);
        assert_eq!(result, Err(WithdrawError::AccountArchived));
        assert_eq!(
            repository.find_by_id_mut("1").unwrap().balance_in_cents(),
            1000
        );
    }
}

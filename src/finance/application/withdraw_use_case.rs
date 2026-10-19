use crate::{
    FinanceAccountError,
    finance::infrastructure::in_memory_account_repository::InMemoryAccountRepository,
};

#[derive(PartialEq, Debug)]
pub enum WithdrawTypeError {
    AccountNotFound,
    InvalidAmount,
    AccountArchived,
    InsufficientFunds,
}

pub fn withdraw_from_account(
    repository: &mut InMemoryAccountRepository,
    account_id: &str,
    amount_in_cents: i64,
) -> Result<i64, WithdrawTypeError> {
    let Some(account) = repository.find_by_id_mut(account_id) else {
        return Err(WithdrawTypeError::AccountNotFound);
    };

    match account.withdraw(amount_in_cents) {
        Ok(()) => Ok(account.balance_in_cents()),
        Err(e) => match e {
            FinanceAccountError::AccountArchived => Err(WithdrawTypeError::AccountArchived),
            FinanceAccountError::InsufficientFunds => Err(WithdrawTypeError::InsufficientFunds),
            FinanceAccountError::InvalidAmount => Err(WithdrawTypeError::InvalidAmount),
        },
    }
}

#[cfg(test)]
mod tests {
    use crate::finance::domain::account::{FinanceAccount, FinanceAccountType};

    use super::*;

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
        assert_eq!(result, Err(WithdrawTypeError::InsufficientFunds));
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
        assert_eq!(result, Err(WithdrawTypeError::InvalidAmount));
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
        assert_eq!(result, Err(WithdrawTypeError::AccountArchived));
        assert_eq!(
            repository.find_by_id_mut("1").unwrap().balance_in_cents(),
            1000
        );
    }
}

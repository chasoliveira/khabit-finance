use crate::finance::domain::account::{FinanceAccount, FinanceAccountError};

pub fn withdraw_from_account(
    account: &mut FinanceAccount,
    amount_in_cents: i64,
) -> Result<i64, FinanceAccountError> {
    account.withdraw(amount_in_cents)?;
    Ok(account.balance_in_cents())
}

#[cfg(test)]
mod tests {
    use crate::finance::domain::account::FinanceAccountType;

    use super::*;

    #[test]
    fn test_withdraw_from_account() {
        let mut account = FinanceAccount::new(
            "1".to_string(),
            "My Checking Account".to_string(),
            FinanceAccountType::Checking,
        );
        account.deposit(1000).unwrap();

        let result = withdraw_from_account(&mut account, 500);
        assert_eq!(result.unwrap(), 500);
        assert_eq!(account.balance_in_cents(), 500);
    }

    #[test]
    fn test_withdraw_from_account_insufficient_funds() {
        let mut account = FinanceAccount::new(
            "1".to_string(),
            "My Checking Account".to_string(),
            FinanceAccountType::Checking,
        );
        account.deposit(1000).unwrap();

        let result = withdraw_from_account(&mut account, 1500);
        assert_eq!(result.unwrap_err(), FinanceAccountError::InsufficientFunds);
        assert_eq!(account.balance_in_cents(), 1000);
    }

    #[test]
    fn test_withdraw_from_account_invalid_amount() {
        let mut account = FinanceAccount::new(
            "1".to_string(),
            "My Checking Account".to_string(),
            FinanceAccountType::Checking,
        );
        account.deposit(1000).unwrap();

        let result = withdraw_from_account(&mut account, -500);
        assert_eq!(result.unwrap_err(), FinanceAccountError::InvalidAmount);
        assert_eq!(account.balance_in_cents(), 1000);
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

        let result = withdraw_from_account(&mut account, 500);
        assert_eq!(result.unwrap_err(), FinanceAccountError::AccountArchived);
        assert_eq!(account.balance_in_cents(), 1000);
    }
}

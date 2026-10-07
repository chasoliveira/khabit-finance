pub mod finance;

pub use finance::application::withdraw_use_case::withdraw_from_account;
pub use finance::domain::account::FinanceAccountError;
pub use finance::domain::account::FinanceAccountType;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::finance::domain::account::FinanceAccount;

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
}

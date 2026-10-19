pub mod finance;

pub use finance::application::withdraw_use_case::withdraw_from_account;
pub use finance::domain::account::FinanceAccount;
pub use finance::domain::account::FinanceAccountError;
pub use finance::domain::account::FinanceAccountType;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::finance::{
        domain::account::FinanceAccount,
        infrastructure::in_memory_account_repository::InMemoryAccountRepository,
    };

    #[test]
    fn test_withdraw_from_account() {
        let mut repository = &mut InMemoryAccountRepository::new();
        {
            let mut account = FinanceAccount::new(
                "1".to_string(),
                "My Checking Account".to_string(),
                FinanceAccountType::Checking,
            );
            account.deposit(1000).unwrap();
            repository.add(account);
        }

        let result = withdraw_from_account(&mut repository, "1", 500);
        assert_eq!(result.unwrap(), 500);

        {
            let existing_account = repository.find_by_id_mut("1").unwrap();

            assert_eq!(existing_account.balance_in_cents(), 500);
        }
    }
}

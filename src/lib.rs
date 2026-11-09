pub mod finance;

pub use finance::application::withdraw_use_case::withdraw_from_account;
pub use finance::domain::account::FinanceAccount;
pub use finance::domain::account::FinanceAccountError;
pub use finance::domain::account::FinanceAccountType;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::finance::{
        application::account_repository_trait::AccountRepository,
        domain::account::{FinanceAccount, FinanceAccountId},
        infrastructure::in_memory_account_repository::InMemoryAccountRepository,
    };

    #[test]
    fn test_withdraw_from_account() {
        let id = &FinanceAccountId::new();
        let repository = &mut InMemoryAccountRepository::new();
        {
            let mut account = FinanceAccount::new(
                id.clone(),
                "My Checking Account".to_string(),
                FinanceAccountType::Checking,
            );
            account.deposit(1000).unwrap();
            repository.add(account);
        }

        let result = withdraw_from_account(repository, &id, 500);
        assert_eq!(result.unwrap(), 500);

        {
            let existing_account = repository.find_by_id(&id).unwrap().unwrap();
            assert_eq!(existing_account.balance_in_cents(), 500);
        }
    }
}

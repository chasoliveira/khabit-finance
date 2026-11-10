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
        domain::{
            account::FinanceAccount,
            shared::{currency::Currency, money::Money},
        },
        infrastructure::in_memory_account_repository::InMemoryAccountRepository,
    };

    #[test]
    fn test_withdraw_from_account() {
        let mut account = FinanceAccount::new(
            "My Checking Account".to_string(),
            FinanceAccountType::Checking,
        );
        let id = &account.id().clone();
        let repository = &mut InMemoryAccountRepository::new();
        {
            account.deposit(&Money::new(1000, Currency::brl())).unwrap();
            repository.add(account);
        }

        let result = withdraw_from_account(repository, &id, Money::new(500, Currency::brl()));
        assert_eq!(result, Ok(Money::new(500, Currency::brl())));

        {
            let existing_account = repository.find_by_id(&id).unwrap().unwrap();
            assert_eq!(existing_account.balance().amount_in_cents(), 500);
        }
    }
}

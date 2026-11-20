pub mod finance;

pub use finance::application::use_cases::withdraw_use_case::withdraw_from_account;
pub use finance::domain::account::{AccountError, AccountType};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::finance::{
        application::unit_of_work_trait::UnitOfWork,
        domain::{
            account::Account,
            shared::{currency::Currency, money::Money},
        },
        infrastructure::contexts::{
            in_memory_database::InMemoryDatabase, in_memory_unit_of_work::InMemoryUnitOfWork,
        },
    };

    #[test]
    fn test_withdraw_from_account() {
        let mut account = Account::new(
            "My Checking Account".to_string(),
            AccountType::Checking,
            Currency::brl(),
        );
        account.deposit(&Money::new(1000, Currency::brl())).unwrap();

        let account_id = account.id().clone();
        let mut db = InMemoryDatabase::new();

        let mut transaction = InMemoryUnitOfWork::begin(&mut db);

        transaction.database_mut().add_account(account);
        let result = withdraw_from_account(
            &mut transaction,
            &account_id,
            Money::new(500, Currency::brl()),
        );
        assert_eq!(result, Ok(Money::new(500, Currency::brl())));

        transaction.commit().unwrap();

        let stored_account = db
            .accounts()
            .iter()
            .find(|stored| stored.id() == &account_id)
            .expect("account should have been persisted");

        assert_eq!(stored_account.balance().amount_in_cents(), 500);
    }
}

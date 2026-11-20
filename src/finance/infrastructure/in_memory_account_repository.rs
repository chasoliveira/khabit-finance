use crate::finance::application::account_repository_trait::{
    AccountRepository, AccountRepositoryError,
};
use crate::finance::domain::account::{Account, AccountId};
use crate::finance::infrastructure::contexts::in_memory_database::InMemoryDatabase;

pub struct InMemoryAccountRepository<'db> {
    db: &'db mut InMemoryDatabase,
}

impl<'db> InMemoryAccountRepository<'db> {
    pub fn new(db: &'db mut InMemoryDatabase) -> Self {
        InMemoryAccountRepository { db }
    }

    pub fn add(&mut self, account: Account) {
        self.db.add_account(account);
    }
    pub fn list(&self) -> &[Account] {
        self.db.accounts()
    }
}

impl<'db> AccountRepository for InMemoryAccountRepository<'db> {
    type Error = AccountRepositoryError;

    fn find_by_id(&self, id: &AccountId) -> Result<Option<Account>, Self::Error> {
        Ok(self
            .db
            .accounts()
            .iter()
            .find(|account| account.id() == id)
            .cloned())
    }

    fn update(&mut self, account: Account) -> Result<(), Self::Error> {
        if let Some(pos) = self
            .db
            .accounts()
            .iter()
            .position(|acc| acc.id() == account.id())
        {
            self.db.update_account(pos, account);
            Ok(())
        } else {
            return Err(AccountRepositoryError::UpdateFailed);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::finance::application::account_repository_trait::AccountRepository;
    use crate::finance::domain::account::{Account, AccountType};
    use crate::finance::domain::shared::currency::Currency;

    #[test]
    fn test_add_account() {
        let mut db = InMemoryDatabase::new();
        let mut repository = InMemoryAccountRepository::new(&mut db);
        let new_account = Account::new(
            "Test Account".to_string(),
            AccountType::Checking,
            Currency::brl(),
        );

        repository.add(new_account);
        assert_eq!(repository.list().len(), 1);
    }

    #[test]
    fn test_find_by_id() {
        let mut db = InMemoryDatabase::new();
        let mut repository = InMemoryAccountRepository::new(&mut db);
        let new_account = Account::new(
            "Test Account".to_string(),
            AccountType::Checking,
            Currency::brl(),
        );
        let id = new_account.id().clone();

        repository.add(new_account);
        let account = repository.find_by_id(&id).unwrap();
        assert!(account.is_some());
    }

    #[test]
    fn test_find_by_id_returns_some() {
        let mut db = InMemoryDatabase::new();
        let mut repository = InMemoryAccountRepository::new(&mut db);
        let new_account = Account::new(
            "Test Account".to_string(),
            AccountType::Checking,
            Currency::brl(),
        );
        let id = new_account.id().clone();

        repository.add(new_account);

        let account = repository.find_by_id(&id).unwrap();
        assert!(account.is_some());
    }

    #[test]
    fn test_find_by_id_returns_none() {
        let mut db = InMemoryDatabase::new();
        let repository = InMemoryAccountRepository::new(&mut db);
        let account = repository.find_by_id(&AccountId::new()).unwrap();
        assert!(account.is_none());
    }

    #[test]
    fn test_update_ok() {
        let mut db = InMemoryDatabase::new();
        let mut repository = InMemoryAccountRepository::new(&mut db);
        let account = Account::new(
            "Test Account".to_string(),
            AccountType::Checking,
            Currency::brl(),
        );
        repository.add(account.clone());
        let result = repository.update(account);

        assert!(result.is_ok());
    }

    #[test]
    fn test_update_error() {
        let mut db = InMemoryDatabase::new();
        let mut repository = InMemoryAccountRepository::new(&mut db);
        let account = Account::new(
            "Test Account".to_string(),
            AccountType::Checking,
            Currency::brl(),
        );
        let result = repository.update(account);

        assert!(result.is_err());
        assert_eq!(result, Err(AccountRepositoryError::UpdateFailed))
    }
}

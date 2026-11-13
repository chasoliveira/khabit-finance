use crate::finance::application::account_repository_trait::{
    AccountRepository, AccountRepositoryError,
};
use crate::finance::domain::account::{Account, AccountId};

pub struct InMemoryAccountRepository {
    // This is a placeholder for the in-memory storage of accounts.
    // In a real implementation, you might use a HashMap or another data structure.
    // For simplicity, we'll just use a vector here.
    accounts: Vec<Account>,
}

impl InMemoryAccountRepository {
    pub fn new() -> Self {
        InMemoryAccountRepository {
            accounts: Vec::new(),
        }
    }

    pub fn add(&mut self, account: Account) {
        self.accounts.push(account);
    }
}

impl AccountRepository for InMemoryAccountRepository {
    fn find_by_id(&self, id: &AccountId) -> Result<Option<Account>, AccountRepositoryError> {
        Ok(self
            .accounts
            .iter()
            .find(|account| account.id() == id && !account.is_archived())
            .cloned())
    }

    fn exists_by_id(&self, id: &AccountId) -> Result<bool, AccountRepositoryError> {
        Ok(self
            .accounts
            .iter()
            .any(|account| account.id() == id && !account.is_archived()))
    }

    fn update(&mut self, account: Account) -> Result<(), AccountRepositoryError> {
        if let Some(pos) = self
            .accounts
            .iter()
            .position(|acc| acc.id() == account.id() && !account.is_archived())
        {
            self.accounts[pos] = account;
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
        let mut repository = InMemoryAccountRepository::new();
        let new_account = Account::new(
            "Test Account".to_string(),
            AccountType::Checking,
            Currency::brl(),
        );

        repository.add(new_account);
        assert_eq!(repository.accounts.len(), 1);
    }

    #[test]
    fn test_find_by_id() {
        let mut repository = InMemoryAccountRepository::new();
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
        let mut repository = InMemoryAccountRepository::new();
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
        let repository = InMemoryAccountRepository::new();
        let account = repository.find_by_id(&AccountId::new()).unwrap();
        assert!(account.is_none());
    }

    #[test]
    fn test_existing_by_id() {
        let mut repository = InMemoryAccountRepository::new();
        let new_account = Account::new(
            "Test Account".to_string(),
            AccountType::Checking,
            Currency::brl(),
        );
        let id = new_account.id().clone();

        repository.add(new_account);
        let account = repository.exists_by_id(&id).unwrap();
        assert!(account);
    }

    #[test]
    fn test_existing_by_id_returns_none() {
        let repository = InMemoryAccountRepository::new();
        let account = repository.exists_by_id(&AccountId::new()).unwrap();
        assert!(!account);
    }

    #[test]
    fn test_update_ok() {
        let mut repository = InMemoryAccountRepository::new();
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
        let mut repository = InMemoryAccountRepository::new();
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

use crate::finance::application::account_repository_trait::{
    AccountRepository, AccountRepositoryError,
};
use crate::finance::domain::account::FinanceAccount;

pub struct InMemoryAccountRepository {
    // This is a placeholder for the in-memory storage of accounts.
    // In a real implementation, you might use a HashMap or another data structure.
    // For simplicity, we'll just use a vector here.
    accounts: Vec<FinanceAccount>,
}

impl InMemoryAccountRepository {
    pub fn new() -> Self {
        InMemoryAccountRepository {
            accounts: Vec::new(),
        }
    }

    pub fn add(&mut self, account: FinanceAccount) {
        self.accounts.push(account);
    }
}

impl AccountRepository for InMemoryAccountRepository {
    fn find_by_id(&self, id: &str) -> Result<Option<FinanceAccount>, AccountRepositoryError> {
        Ok(self
            .accounts
            .iter()
            .find(|account| account.id() == id)
            .cloned())
    }
    fn save(&mut self, account: FinanceAccount) -> Result<(), AccountRepositoryError> {
        if let Some(pos) = self
            .accounts
            .iter()
            .position(|acc| acc.id() == account.id())
        {
            self.accounts[pos] = account;
            Ok(())
        } else {
            return Err(AccountRepositoryError::WriteFailed);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::finance::application::account_repository_trait::AccountRepository;
    use crate::{FinanceAccount, FinanceAccountType};

    #[test]
    fn test_add_account() {
        let mut repository = InMemoryAccountRepository::new();
        repository.add(FinanceAccount::new(
            "1".to_string(),
            "Test Account".to_string(),
            FinanceAccountType::Checking,
        ));
        assert_eq!(repository.accounts.len(), 1);
    }

    #[test]
    fn test_find_by_id() {
        let mut repository = InMemoryAccountRepository::new();
        repository.add(FinanceAccount::new(
            "1".to_string(),
            "Test Account".to_string(),
            FinanceAccountType::Checking,
        ));
        let account = repository.find_by_id("1").unwrap();
        assert!(account.is_some());
        assert_eq!(account.unwrap().id(), "1");
    }

    #[test]
    fn test_find_by_id_returns_some() {
        let mut repository = InMemoryAccountRepository::new();
        repository.add(FinanceAccount::new(
            "1".to_string(),
            "Test Account".to_string(),
            FinanceAccountType::Checking,
        ));
        let account = repository.find_by_id("1").unwrap();
        assert!(account.is_some());
    }

    #[test]
    fn test_find_by_id_returns_none() {
        let repository = InMemoryAccountRepository::new();
        let account = repository.find_by_id("nonexistent").unwrap();
        assert!(account.is_none());
    }

    #[test]
    fn test_save_ok() {
        let mut repository = InMemoryAccountRepository::new();
        let account = FinanceAccount::new(
            "1".to_string(),
            "Test Account".to_string(),
            FinanceAccountType::Checking,
        );
        repository.add(account.clone());
        let result = repository.save(account);

        assert!(result.is_ok());
    }

    #[test]
    fn test_save_error() {
        let mut repository = InMemoryAccountRepository::new();
        let account = FinanceAccount::new(
            "1".to_string(),
            "Test Account".to_string(),
            FinanceAccountType::Checking,
        );
        let result = repository.save(account);

        assert!(result.is_err());
        assert_eq!(result, Err(AccountRepositoryError::WriteFailed))
    }
}

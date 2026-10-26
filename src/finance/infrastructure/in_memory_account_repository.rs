use crate::finance::application::account_repository_trait::AccountRepository;
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
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut FinanceAccount> {
        self.accounts.iter_mut().find(|account| account.id() == id)
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
    fn test_find_by_id_mut() {
        let mut repository = InMemoryAccountRepository::new();
        repository.add(FinanceAccount::new(
            "1".to_string(),
            "Test Account".to_string(),
            FinanceAccountType::Checking,
        ));
        let account = repository.find_by_id_mut("1");
        assert!(account.is_some());
        assert_eq!(account.unwrap().id(), "1");
    }

    #[test]
    fn test_find_by_id_mut_withdraw() {
        let mut repository = InMemoryAccountRepository::new();
        repository.add(FinanceAccount::new(
            "1".to_string(),
            "Test Account".to_string(),
            FinanceAccountType::Checking,
        ));
        let account = repository.find_by_id_mut("1");
        assert!(account.is_some());
    }

    #[test]
    fn test_find_by_id_mut_not_found() {
        let mut repository = InMemoryAccountRepository::new();
        let account = repository.find_by_id_mut("nonexistent");
        assert!(account.is_none());
    }
}

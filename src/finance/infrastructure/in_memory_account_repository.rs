use crate::finance::domain::account::{FinanceAccount, FinanceAccountError, FinanceAccountType};

#[warn(private_interfaces)]
pub(crate) struct InMemoryAccountRepository {
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

    pub fn add_account(&mut self, id: String, name: String, account_type: FinanceAccountType) {
        let account = FinanceAccount::new(id, name, account_type);
        self.accounts.push(account);
    }

    pub fn get_account_by_id_mut(&mut self, id: &str) -> Option<&mut FinanceAccount> {
        self.accounts.iter_mut().find(|account| account.id() == id)
    }

    pub fn deposit_to_account(
        &mut self,
        id: &str,
        amount_in_cents: i64,
    ) -> Result<(), FinanceAccountError> {
        if let Some(account) = self.get_account_by_id_mut(id) {
            account.deposit(amount_in_cents)
        } else {
            Err(FinanceAccountError::AccountNotFound)
        }
    }

    pub fn withdraw_from_account(
        &mut self,
        id: &str,
        amount_in_cents: i64,
    ) -> Result<(), FinanceAccountError> {
        if let Some(account) = self.get_account_by_id_mut(id) {
            account.withdraw(amount_in_cents)
        } else {
            Err(FinanceAccountError::AccountNotFound)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_account() {
        let mut repository = InMemoryAccountRepository::new();
        repository.add_account(
            "1".to_string(),
            "Test Account".to_string(),
            FinanceAccountType::Checking,
        );
        assert_eq!(repository.accounts.len(), 1);
    }

    #[test]
    fn test_deposit_to_account() {
        let mut repository = InMemoryAccountRepository::new();
        repository.add_account(
            "1".to_string(),
            "Test Account".to_string(),
            FinanceAccountType::Checking,
        );
        let result = repository.deposit_to_account("1", 1000);
        assert!(result.is_ok());
        let account = repository.get_account_by_id_mut("1").unwrap();
        assert_eq!(account.balance_in_cents(), 1000);
    }

    #[test]
    fn test_withdraw_from_account() {
        let mut repository = InMemoryAccountRepository::new();
        repository.add_account(
            "1".to_string(),
            "Test Account".to_string(),
            FinanceAccountType::Checking,
        );
        let _ = repository.deposit_to_account("1", 1000);
        let result = repository.withdraw_from_account("1", 500);
        assert!(result.is_ok());
        let account = repository.get_account_by_id_mut("1").unwrap();
        assert_eq!(account.balance_in_cents(), 500);
    }
}

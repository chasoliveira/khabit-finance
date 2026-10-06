#[derive(Debug, PartialEq)]
pub enum FinanceAccountType {
    Cash,
    Checking,
    Savings,
    Investment,
}

impl FinanceAccountType {
    pub fn is_spendable(&self) -> bool {
        match self {
            FinanceAccountType::Cash => true,
            FinanceAccountType::Checking => true,
            FinanceAccountType::Savings => false,
            FinanceAccountType::Investment => false,
        }
    }
}

pub struct FinanceAccount {
    id: String,
    name: String,
    account_type: FinanceAccountType,
    institution_name: Option<String>,
    description: Option<String>,
    balance_in_cents: i64,
    is_archived: bool,
}

impl FinanceAccount {
    pub fn new(id: String, name: String, account_type: FinanceAccountType) -> Self {
        FinanceAccount {
            id,
            name,
            account_type,
            institution_name: None,
            description: None,
            balance_in_cents: 0,
            is_archived: false,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn balance_in_cents(&self) -> i64 {
        self.balance_in_cents
    }

    pub fn deposit(&mut self, amount_in_cents: i64) {
        if amount_in_cents < 0 {
            panic!("Deposit amount must be positive")
        }
        if amount_in_cents == 0 {
            panic!("Deposit amount must be greater than zero")
        }
        self.balance_in_cents += amount_in_cents;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_finance_account_type() {
        for account_type in [FinanceAccountType::Cash, FinanceAccountType::Checking] {
            println!("Testing expandable account type: {:?}", account_type);
            assert!(account_type.is_spendable());
        }

        for account_type in [FinanceAccountType::Savings, FinanceAccountType::Investment] {
            println!("Testing non-expandable account type: {:?}", account_type);
            assert!(!account_type.is_spendable());
        }
    }

    #[test]
    fn test_account_creation() {
        let account = FinanceAccount::new(
            "1".to_string(),
            "My Checking Account".to_string(),
            FinanceAccountType::Checking,
        );

        assert_eq!(account.id(), "1");
        assert_eq!(account.name(), "My Checking Account");
        assert_eq!(account.account_type, FinanceAccountType::Checking);
        assert_eq!(account.institution_name.as_deref(), None);
        assert_eq!(account.description.as_deref(), None);
        assert_eq!(account.balance_in_cents(), 0);
        assert!(!account.is_archived);
    }

    #[test]
    fn test_account_deposit() {
        let mut account = FinanceAccount::new(
            "1".to_string(),
            "My Checking Account".to_string(),
            FinanceAccountType::Checking,
        );

        account.deposit(1000);
        account.deposit(1000);
        assert_eq!(account.balance_in_cents(), 2000);
    }

    #[test]
    #[should_panic(expected = "Deposit amount must be positive")]
    fn test_account_deposit_negative() {
        let mut account = FinanceAccount::new(
            "1".to_string(),
            "My Checking Account".to_string(),
            FinanceAccountType::Checking,
        );
        account.deposit(-1000);
    }

    #[test]
    #[should_panic(expected = "Deposit amount must be greater than zero")]
    fn test_account_deposit_zero() {
        let mut account = FinanceAccount::new(
            "1".to_string(),
            "My Checking Account".to_string(),
            FinanceAccountType::Checking,
        );
        account.deposit(0);
    }
}

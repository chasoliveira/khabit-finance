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

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_balance_in_cents(&self) -> i64 {
        self.balance_in_cents
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

        assert_eq!(account.get_id(), "1");
        assert_eq!(account.get_name(), "My Checking Account");
        assert_eq!(account.account_type, FinanceAccountType::Checking);
        assert_eq!(account.institution_name.as_deref(), None);
        assert_eq!(account.description.as_deref(), None);
        assert_eq!(account.get_balance_in_cents(), 0);
        assert!(!account.is_archived);
    }
}

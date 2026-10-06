#[derive(Debug, PartialEq)]
pub enum FinanceAccountType {
    Cash,
    Checking,
    Savings,
    Investment,
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
        let account = FinanceAccount {
            id: "1".to_string(),
            name: "My Checking Account".to_string(),
            account_type: FinanceAccountType::Checking,
            institution_name: Some("Bank of Rust".to_string()),
            description: Some("Primary checking account".to_string()),
            balance_in_cents: 100_00, // $100.00
            is_archived: false,
        };

        assert_eq!(account.id, "1");
        assert_eq!(account.name, "My Checking Account");
        assert_eq!(account.account_type, FinanceAccountType::Checking);
        assert_eq!(account.institution_name.as_deref(), Some("Bank of Rust"));
        assert_eq!(
            account.description.as_deref(),
            Some("Primary checking account")
        );
        assert_eq!(account.balance_in_cents, 100_00);
        assert!(!account.is_archived);
    }
}

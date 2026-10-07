#[derive(Debug, PartialEq)]
pub enum FinanceAccountType {
    Cash,
    Checking,
    Savings,
    Investment,
}

#[derive(Debug, PartialEq)]
pub enum FinanceAccountError {
    InvalidAmount,
    InsufficientFunds,
    AccountArchived,
    AccountNotFound,
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

#[warn(private_interfaces)]
pub(crate) struct FinanceAccount {
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

    fn validate_amount(&self, amount_in_cents: i64) -> Result<(), FinanceAccountError> {
        if amount_in_cents <= 0 {
            Err(FinanceAccountError::InvalidAmount)
        } else {
            Ok(())
        }
    }

    fn ensure_active(&self) -> Result<(), FinanceAccountError> {
        if self.is_archived {
            Err(FinanceAccountError::AccountArchived)
        } else {
            Ok(())
        }
    }

    pub fn deposit(&mut self, amount_in_cents: i64) -> Result<(), FinanceAccountError> {
        self.ensure_active()?;
        self.validate_amount(amount_in_cents)?;

        self.balance_in_cents += amount_in_cents;
        Ok(())
    }

    pub fn withdraw(&mut self, amount_in_cents: i64) -> Result<(), FinanceAccountError> {
        self.ensure_active()?;
        self.validate_amount(amount_in_cents)?;

        if self.balance_in_cents < amount_in_cents {
            return Err(FinanceAccountError::InsufficientFunds);
        }
        self.balance_in_cents -= amount_in_cents;
        Ok(())
    }

    pub fn archive(&mut self) {
        self.is_archived = true;
    }

    pub fn is_archived(&self) -> bool {
        self.is_archived
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
        assert!(!account.is_archived());
    }

    #[test]
    fn test_account_deposit() {
        let mut account = FinanceAccount::new(
            "1".to_string(),
            "My Checking Account".to_string(),
            FinanceAccountType::Checking,
        );

        let result = account.deposit(1000);
        assert!(result.is_ok());
        assert_eq!(account.balance_in_cents(), 1000);
    }

    #[test]
    fn test_account_archived_deposit() {
        let mut account = FinanceAccount::new(
            "1".to_string(),
            "My Checking Account".to_string(),
            FinanceAccountType::Checking,
        );
        account.archive();

        let result = account.deposit(1000);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), FinanceAccountError::AccountArchived);

        assert_eq!(account.balance_in_cents(), 0);
        assert!(account.is_archived());
    }

    #[test]
    fn test_account_archived_deposit_invalid() {
        let mut account = FinanceAccount::new(
            "1".to_string(),
            "My Checking Account".to_string(),
            FinanceAccountType::Checking,
        );
        account.archive();

        let result = account.deposit(-1000);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), FinanceAccountError::AccountArchived);

        assert_eq!(account.balance_in_cents(), 0);
        assert!(account.is_archived());
    }

    #[test]
    fn test_account_deposit_negative() {
        let mut account = FinanceAccount::new(
            "1".to_string(),
            "My Checking Account".to_string(),
            FinanceAccountType::Checking,
        );
        let result = account.deposit(-1000);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), FinanceAccountError::InvalidAmount);

        let balance = account.balance_in_cents();
        assert_eq!(balance, 0);
    }

    #[test]
    fn test_account_deposit_zero() {
        let mut account = FinanceAccount::new(
            "1".to_string(),
            "My Checking Account".to_string(),
            FinanceAccountType::Checking,
        );

        let result = account.deposit(0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), FinanceAccountError::InvalidAmount);

        let balance = account.balance_in_cents();
        assert_eq!(balance, 0);
    }

    #[test]
    fn test_account_withdraw() {
        let mut account = FinanceAccount::new(
            "1".to_string(),
            "My Checking Account".to_string(),
            FinanceAccountType::Checking,
        );
        let result_deposit = account.deposit(1000);
        assert!(result_deposit.is_ok());

        let result = account.withdraw(500);
        assert!(result.is_ok());
        assert_eq!(account.balance_in_cents(), 500);
    }

    #[test]
    fn test_account_withdraw_archived() {
        let mut account = FinanceAccount::new(
            "1".to_string(),
            "My Checking Account".to_string(),
            FinanceAccountType::Checking,
        );
        account.archive();

        let result = account.withdraw(500);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), FinanceAccountError::AccountArchived);
        assert_eq!(account.balance_in_cents(), 0);
        assert!(account.is_archived());
    }

    #[test]
    fn test_account_withdraw_archived_negative() {
        let mut account = FinanceAccount::new(
            "1".to_string(),
            "My Checking Account".to_string(),
            FinanceAccountType::Checking,
        );
        account.archive();

        let result = account.withdraw(-500);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), FinanceAccountError::AccountArchived);
        assert_eq!(account.balance_in_cents(), 0);
        assert!(account.is_archived());
    }

    #[test]
    fn test_account_withdraw_insufficient_funds() {
        let mut account = FinanceAccount::new(
            "1".to_string(),
            "My Checking Account".to_string(),
            FinanceAccountType::Checking,
        );
        account.deposit(1000).unwrap();

        let result = account.withdraw(1500);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), FinanceAccountError::InsufficientFunds);
        assert_eq!(account.balance_in_cents(), 1000);
    }

    #[test]
    fn test_account_withdraw_negative() {
        let mut account = FinanceAccount::new(
            "1".to_string(),
            "My Checking Account".to_string(),
            FinanceAccountType::Checking,
        );
        let result_deposit = account.deposit(1000);
        assert!(result_deposit.is_ok());

        let result = account.withdraw(-500);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), FinanceAccountError::InvalidAmount);
        assert_eq!(account.balance_in_cents(), 1000);
    }

    #[test]
    fn test_account_withdraw_zero() {
        let mut account = FinanceAccount::new(
            "1".to_string(),
            "My Checking Account".to_string(),
            FinanceAccountType::Checking,
        );
        let result_deposit = account.deposit(1000);
        assert!(result_deposit.is_ok());

        let result = account.withdraw(0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), FinanceAccountError::InvalidAmount);
        assert_eq!(account.balance_in_cents(), 1000);
    }
}

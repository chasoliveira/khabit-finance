use crate::finance::domain::shared::{
    currency::Currency,
    id::Id,
    money::{Money, MoneyError},
};

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct FinanceAccountId(Id);

impl FinanceAccountId {
    pub fn new() -> Self {
        FinanceAccountId(Id::new())
    }
}

#[derive(Debug, PartialEq, Clone)]
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
    CurrencyMismatch,
}

impl From<MoneyError> for FinanceAccountError {
    fn from(error: MoneyError) -> Self {
        match error {
            MoneyError::CurrencyMismatch => Self::CurrencyMismatch,
        }
    }
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

#[derive(Clone)]
pub struct FinanceAccount {
    id: FinanceAccountId,
    name: String,
    account_type: FinanceAccountType,
    institution_name: Option<String>,
    description: Option<String>,
    balance: Money,
    is_archived: bool,
}

impl FinanceAccount {
    pub fn new(name: String, account_type: FinanceAccountType, currency: Currency) -> Self {
        FinanceAccount {
            id: FinanceAccountId(Id::new()),
            name,
            account_type,
            institution_name: None,
            description: None,
            balance: Money::zero(currency),
            is_archived: false,
        }
    }

    pub fn id(&self) -> &FinanceAccountId {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn balance(&self) -> &Money {
        &self.balance
    }

    fn validate_amount(&self, amount: &Money) -> Result<(), FinanceAccountError> {
        if amount.amount_in_cents() <= 0 {
            Err(FinanceAccountError::InvalidAmount)
        } else if self.balance.currency() != amount.currency() {
            Err(FinanceAccountError::CurrencyMismatch)
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

    pub fn deposit(&mut self, amount: &Money) -> Result<(), FinanceAccountError> {
        self.ensure_active()?;
        self.validate_amount(&amount)?;

        let balance = self.balance.add(&amount)?;

        self.balance = balance;

        Ok(())
    }

    pub fn withdraw(&mut self, amount: &Money) -> Result<(), FinanceAccountError> {
        self.ensure_active()?;
        self.validate_amount(amount)?;

        if self.balance.amount_in_cents() < amount.amount_in_cents() {
            return Err(FinanceAccountError::InsufficientFunds);
        }
        let balance = self.balance.subtract(&amount)?;
        self.balance = balance;
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

    use uuid::Version;

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
            "My Checking Account".to_string(),
            FinanceAccountType::Checking,
            Currency::brl(),
        );

        assert_eq!(account.id().0.value().get_version(), Some(Version::Random));
        assert_eq!(account.name(), "My Checking Account");
        assert_eq!(account.account_type, FinanceAccountType::Checking);
        assert_eq!(account.institution_name.as_deref(), None);
        assert_eq!(account.description.as_deref(), None);
        assert_eq!(account.balance.amount_in_cents(), 0);
        assert!(!account.is_archived());
    }

    #[test]
    fn test_account_deposit() {
        let mut account = FinanceAccount::new(
            "My Checking Account".to_string(),
            FinanceAccountType::Checking,
            Currency::brl(),
        );

        let result = account.deposit(&Money::new(1000, Currency::brl()));
        assert!(result.is_ok());
        assert_eq!(account.balance.amount_in_cents(), 1000);
    }

    #[test]
    fn test_account_archived_deposit() {
        let mut account = FinanceAccount::new(
            "My Checking Account".to_string(),
            FinanceAccountType::Checking,
            Currency::brl(),
        );
        account.archive();

        let result = account.deposit(&Money::new(1000, Currency::brl()));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), FinanceAccountError::AccountArchived);

        assert_eq!(account.balance.amount_in_cents(), 0);
        assert!(account.is_archived());
    }

    #[test]
    fn test_account_archived_deposit_invalid() {
        let mut account = FinanceAccount::new(
            "My Checking Account".to_string(),
            FinanceAccountType::Checking,
            Currency::brl(),
        );
        account.archive();

        let result = account.deposit(&Money::new(-1000, Currency::brl()));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), FinanceAccountError::AccountArchived);

        assert_eq!(account.balance.amount_in_cents(), 0);
        assert!(account.is_archived());
    }

    #[test]
    fn test_account_deposit_negative() {
        let mut account = FinanceAccount::new(
            "My Checking Account".to_string(),
            FinanceAccountType::Checking,
            Currency::brl(),
        );
        let result = account.deposit(&Money::new(-1000, Currency::brl()));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), FinanceAccountError::InvalidAmount);

        let balance = account.balance.amount_in_cents();
        assert_eq!(balance, 0);
    }

    #[test]
    fn test_account_deposit_invalid_currency() {
        let mut account = FinanceAccount::new(
            "My Checking Account".to_string(),
            FinanceAccountType::Checking,
            Currency::brl(),
        );
        let result = account.deposit(&Money::new(800, Currency::usa()));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), FinanceAccountError::CurrencyMismatch);

        assert_eq!(account.balance.amount_in_cents(), 0);
    }

    #[test]
    fn test_account_deposit_zero() {
        let mut account = FinanceAccount::new(
            "My Checking Account".to_string(),
            FinanceAccountType::Checking,
            Currency::brl(),
        );

        let result = account.deposit(&Money::new(0, Currency::brl()));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), FinanceAccountError::InvalidAmount);

        let balance = account.balance.amount_in_cents();
        assert_eq!(balance, 0);
    }

    #[test]
    fn test_account_withdraw() {
        let mut account = FinanceAccount::new(
            "My Checking Account".to_string(),
            FinanceAccountType::Checking,
            Currency::brl(),
        );
        let result_deposit = account.deposit(&Money::new(1000, Currency::brl()));
        assert!(result_deposit.is_ok());

        let result = account.withdraw(&Money::new(500, Currency::brl()));
        assert!(result.is_ok());
        assert_eq!(account.balance.amount_in_cents(), 500);
    }

    #[test]
    fn test_account_withdraw_archived() {
        let mut account = FinanceAccount::new(
            "My Checking Account".to_string(),
            FinanceAccountType::Checking,
            Currency::brl(),
        );
        account.archive();

        let result = account.withdraw(&Money::new(500, Currency::brl()));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), FinanceAccountError::AccountArchived);
        assert_eq!(account.balance.amount_in_cents(), 0);
        assert!(account.is_archived());
    }

    #[test]
    fn test_account_withdraw_archived_negative() {
        let mut account = FinanceAccount::new(
            "My Checking Account".to_string(),
            FinanceAccountType::Checking,
            Currency::brl(),
        );
        account.archive();

        let result = account.withdraw(&Money::new(-500, Currency::brl()));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), FinanceAccountError::AccountArchived);
        assert_eq!(account.balance.amount_in_cents(), 0);
        assert!(account.is_archived());
    }

    #[test]
    fn test_account_withdraw_insufficient_funds() {
        let mut account = FinanceAccount::new(
            "My Checking Account".to_string(),
            FinanceAccountType::Checking,
            Currency::brl(),
        );
        account.deposit(&Money::new(1000, Currency::brl())).unwrap();

        let result = account.withdraw(&Money::new(1500, Currency::brl()));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), FinanceAccountError::InsufficientFunds);
        assert_eq!(account.balance.amount_in_cents(), 1000);
    }

    #[test]
    fn test_account_withdraw_negative() {
        let mut account = FinanceAccount::new(
            "My Checking Account".to_string(),
            FinanceAccountType::Checking,
            Currency::brl(),
        );
        let result_deposit = account.deposit(&Money::new(1000, Currency::brl()));
        assert!(result_deposit.is_ok());

        let result = account.withdraw(&Money::new(-500, Currency::brl()));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), FinanceAccountError::InvalidAmount);
        assert_eq!(account.balance.amount_in_cents(), 1000);
    }

    #[test]
    fn test_account_withdraw_invalid_currency() {
        let mut account = FinanceAccount::new(
            "My Checking Account".to_string(),
            FinanceAccountType::Checking,
            Currency::brl(),
        );
        let result = account.withdraw(&Money::new(800, Currency::usa()));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), FinanceAccountError::CurrencyMismatch);

        assert_eq!(account.balance.amount_in_cents(), 0);
    }

    #[test]
    fn test_account_withdraw_zero() {
        let mut account = FinanceAccount::new(
            "My Checking Account".to_string(),
            FinanceAccountType::Checking,
            Currency::brl(),
        );
        let result_deposit = account.deposit(&Money::new(1000, Currency::brl()));
        assert!(result_deposit.is_ok());

        let result = account.withdraw(&Money::new(0, Currency::brl()));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), FinanceAccountError::InvalidAmount);
        assert_eq!(account.balance.amount_in_cents(), 1000);
    }
}

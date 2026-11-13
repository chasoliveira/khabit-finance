use crate::finance::domain::shared::{
    currency::Currency,
    id::Id,
    money::{Money, MoneyError},
};

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct AccountId(Id);

impl AccountId {
    pub fn new() -> Self {
        AccountId(Id::new())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum AccountType {
    Cash,
    Checking,
    Savings,
    Investment,
}

#[derive(Debug, PartialEq)]
pub enum AccountError {
    InvalidAmount,
    InsufficientFunds,
    AccountArchived,
    CurrencyMismatch,
}

impl From<MoneyError> for AccountError {
    fn from(error: MoneyError) -> Self {
        match error {
            MoneyError::CurrencyMismatch => Self::CurrencyMismatch,
        }
    }
}

impl AccountType {
    pub fn is_spendable(&self) -> bool {
        match self {
            AccountType::Cash => true,
            AccountType::Checking => true,
            AccountType::Savings => false,
            AccountType::Investment => false,
        }
    }
}

#[derive(Clone)]
pub struct Account {
    id: AccountId,
    name: String,
    account_type: AccountType,
    institution_name: Option<String>,
    description: Option<String>,
    balance: Money,
    is_archived: bool,
}

impl Account {
    pub fn new(name: String, account_type: AccountType, currency: Currency) -> Self {
        Account {
            id: AccountId(Id::new()),
            name,
            account_type,
            institution_name: None,
            description: None,
            balance: Money::zero(currency),
            is_archived: false,
        }
    }

    pub fn id(&self) -> &AccountId {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn account_type(&self) -> &AccountType {
        &self.account_type
    }
    pub fn institution_name(&self) -> &Option<String> {
        &self.institution_name
    }
    pub fn description(&self) -> &Option<String> {
        &self.description
    }

    pub fn balance(&self) -> &Money {
        &self.balance
    }

    pub fn is_archived(&self) -> bool {
        self.is_archived
    }

    pub fn archive(&mut self) {
        self.is_archived = true;
    }

    pub fn matches_currency(&self, currency: &Currency) -> bool {
        self.balance().currency() == currency
    }

    pub fn deposit(&mut self, amount: &Money) -> Result<(), AccountError> {
        self.ensure_active()?;
        self.validate_amount(&amount)?;

        let balance = self.balance.add(&amount)?;

        self.balance = balance;

        Ok(())
    }

    pub fn withdraw(&mut self, amount: &Money) -> Result<(), AccountError> {
        self.ensure_active()?;
        self.validate_amount(amount)?;

        if self.balance.amount_in_cents() < amount.amount_in_cents() {
            return Err(AccountError::InsufficientFunds);
        }
        let balance = self.balance.subtract(&amount)?;
        self.balance = balance;
        Ok(())
    }

    fn validate_amount(&self, amount: &Money) -> Result<(), AccountError> {
        if amount.amount_in_cents() <= 0 {
            Err(AccountError::InvalidAmount)
        } else if self.balance.currency() != amount.currency() {
            Err(AccountError::CurrencyMismatch)
        } else {
            Ok(())
        }
    }

    fn ensure_active(&self) -> Result<(), AccountError> {
        if self.is_archived {
            Err(AccountError::AccountArchived)
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::finance::application::test_support::fake_account_repository::new_account_brl;
    use uuid::Version;

    #[test]
    fn test_finance_account_type() {
        for account_type in [AccountType::Cash, AccountType::Checking] {
            assert!(account_type.is_spendable());
        }

        for account_type in [AccountType::Savings, AccountType::Investment] {
            assert!(!account_type.is_spendable());
        }
    }

    #[test]
    fn test_account_creation() {
        let account = new_account_brl();

        assert_eq!(account.id().0.value().get_version(), Some(Version::Random));
        assert_eq!(account.name(), "Generic");
        assert_eq!(account.account_type, AccountType::Cash);
        assert_eq!(account.institution_name.as_deref(), None);
        assert_eq!(account.description.as_deref(), None);
        assert_eq!(account.balance.amount_in_cents(), 0);
        assert!(!account.is_archived());
    }

    #[test]
    fn test_account_deposit() {
        let mut account = new_account_brl();

        let result = account.deposit(&Money::new(1000, Currency::brl()));
        assert!(result.is_ok());
        assert_eq!(account.balance.amount_in_cents(), 1000);
    }

    #[test]
    fn test_account_archived_deposit() {
        let mut account = new_account_brl();
        account.archive();

        let result = account.deposit(&Money::new(1000, Currency::brl()));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), AccountError::AccountArchived);

        assert_eq!(account.balance.amount_in_cents(), 0);
        assert!(account.is_archived());
    }

    #[test]
    fn test_account_archived_deposit_invalid() {
        let mut account = new_account_brl();
        account.archive();

        let result = account.deposit(&Money::new(-1000, Currency::brl()));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), AccountError::AccountArchived);

        assert_eq!(account.balance.amount_in_cents(), 0);
        assert!(account.is_archived());
    }

    #[test]
    fn test_account_deposit_negative() {
        let mut account = new_account_brl();
        let result = account.deposit(&Money::new(-1000, Currency::brl()));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), AccountError::InvalidAmount);

        let balance = account.balance.amount_in_cents();
        assert_eq!(balance, 0);
    }

    #[test]
    fn test_account_deposit_invalid_currency() {
        let mut account = new_account_brl();
        let result = account.deposit(&Money::new(800, Currency::usa()));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), AccountError::CurrencyMismatch);

        assert_eq!(account.balance.amount_in_cents(), 0);
    }

    #[test]
    fn test_account_deposit_zero() {
        let mut account = new_account_brl();

        let result = account.deposit(&Money::new(0, Currency::brl()));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), AccountError::InvalidAmount);

        let balance = account.balance.amount_in_cents();
        assert_eq!(balance, 0);
    }

    #[test]
    fn test_account_withdraw() {
        let mut account = new_account_brl();
        let result_deposit = account.deposit(&Money::new(1000, Currency::brl()));
        assert!(result_deposit.is_ok());

        let result = account.withdraw(&Money::new(500, Currency::brl()));
        assert!(result.is_ok());
        assert_eq!(account.balance.amount_in_cents(), 500);
    }

    #[test]
    fn test_account_withdraw_archived() {
        let mut account = new_account_brl();
        account.archive();

        let result = account.withdraw(&Money::new(500, Currency::brl()));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), AccountError::AccountArchived);
        assert_eq!(account.balance.amount_in_cents(), 0);
        assert!(account.is_archived());
    }

    #[test]
    fn test_account_withdraw_archived_negative() {
        let mut account = new_account_brl();
        account.archive();

        let result = account.withdraw(&Money::new(-500, Currency::brl()));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), AccountError::AccountArchived);
        assert_eq!(account.balance.amount_in_cents(), 0);
        assert!(account.is_archived());
    }

    #[test]
    fn test_account_withdraw_insufficient_funds() {
        let mut account = new_account_brl();
        account.deposit(&Money::new(1000, Currency::brl())).unwrap();

        let result = account.withdraw(&Money::new(1500, Currency::brl()));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), AccountError::InsufficientFunds);
        assert_eq!(account.balance.amount_in_cents(), 1000);
    }

    #[test]
    fn test_account_withdraw_negative() {
        let mut account = new_account_brl();
        let result_deposit = account.deposit(&Money::new(1000, Currency::brl()));
        assert!(result_deposit.is_ok());

        let result = account.withdraw(&Money::new(-500, Currency::brl()));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), AccountError::InvalidAmount);
        assert_eq!(account.balance.amount_in_cents(), 1000);
    }

    #[test]
    fn test_account_withdraw_invalid_currency() {
        let mut account = new_account_brl();
        let result = account.withdraw(&Money::new(800, Currency::usa()));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), AccountError::CurrencyMismatch);

        assert_eq!(account.balance.amount_in_cents(), 0);
    }

    #[test]
    fn test_account_withdraw_zero() {
        let mut account = new_account_brl();
        let result_deposit = account.deposit(&Money::new(1000, Currency::brl()));
        assert!(result_deposit.is_ok());

        let result = account.withdraw(&Money::new(0, Currency::brl()));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), AccountError::InvalidAmount);
        assert_eq!(account.balance.amount_in_cents(), 1000);
    }

    #[test]
    fn test_account_matches_currency() {
        let account = new_account_brl();

        assert!(account.matches_currency(&Currency::brl()));
        assert!(!account.matches_currency(&Currency::usa()));
    }
}

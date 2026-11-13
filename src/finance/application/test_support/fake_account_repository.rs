use crate::{
    AccountType,
    finance::{
        application::account_repository_trait::{AccountRepository, AccountRepositoryError},
        domain::{
            account::{Account, AccountId},
            shared::currency::Currency,
        },
    },
};

pub fn new_account_brl() -> Account {
    Account::new("Generic".to_string(), AccountType::Cash, Currency::brl())
}
pub struct FakeAccountRepository {
    account: Option<Account>,
    read_error: Option<AccountRepositoryError>,
    update_error: Option<AccountRepositoryError>,
}
impl FakeAccountRepository {
    pub fn new(account: Option<Account>) -> Self {
        FakeAccountRepository {
            account,
            read_error: None,
            update_error: None,
        }
    }
    pub fn with_read_error(
        account: Option<Account>,
        read_error: Option<AccountRepositoryError>,
    ) -> Self {
        FakeAccountRepository {
            account,
            read_error,
            update_error: None,
        }
    }

    pub fn with_update_error(
        account: Option<Account>,
        update_error: Option<AccountRepositoryError>,
    ) -> Self {
        FakeAccountRepository {
            account,
            read_error: None,
            update_error,
        }
    }
}

impl AccountRepository for FakeAccountRepository {
    fn find_by_id(&self, id: &AccountId) -> Result<Option<Account>, AccountRepositoryError> {
        if self.read_error.is_some() {
            return Err(self.read_error.clone().unwrap());
        }
        match &self.account {
            Some(acc) => {
                if acc.id() == id && !acc.is_archived() {
                    Ok(self.account.clone())
                } else {
                    Ok(None)
                }
            }
            None => Ok(None),
        }
    }

    fn exists_by_id(&self, id: &AccountId) -> Result<bool, AccountRepositoryError> {
        if self.read_error.is_some() {
            return Err(self.read_error.clone().unwrap());
        }
        match &self.account {
            Some(acc) => {
                if acc.id() == id && !acc.is_archived() {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            None => Ok(false),
        }
    }

    fn update(&mut self, account: Account) -> Result<(), AccountRepositoryError> {
        if self.update_error.is_some() {
            return Err(self.update_error.clone().unwrap());
        }
        match &self.account {
            Some(acc) => {
                if acc.id() == account.id() && !account.is_archived() {
                    self.account = Some(account);
                    Ok(())
                } else {
                    Err(AccountRepositoryError::UpdateFailed)
                }
            }
            None => Err(AccountRepositoryError::UpdateFailed),
        }
    }
}

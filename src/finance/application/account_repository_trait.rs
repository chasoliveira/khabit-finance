use crate::finance::domain::account::{Account, AccountId};

#[derive(Debug, PartialEq, Clone)]
pub enum AccountRepositoryError {
    ReadFailed,
    UpdateFailed,
}

pub trait AccountRepository {
    fn find_by_id(&self, id: &AccountId) -> Result<Option<Account>, AccountRepositoryError>;
    fn exists_by_id(&self, id: &AccountId) -> Result<bool, AccountRepositoryError>;
    fn update(&mut self, account: Account) -> Result<(), AccountRepositoryError>;
}

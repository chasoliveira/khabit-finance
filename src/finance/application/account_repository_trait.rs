use crate::finance::domain::account::{Account, AccountId};

#[derive(Debug, PartialEq, Clone)]
pub enum AccountRepositoryError {
    ReadFailed,
    UpdateFailed,
}

pub trait AccountRepository {
    type Error;

    fn find_by_id(&self, id: &AccountId) -> Result<Option<Account>, Self::Error>;
    fn update(&mut self, account: Account) -> Result<(), Self::Error>;
}

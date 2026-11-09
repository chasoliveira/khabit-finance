use crate::finance::domain::account::{FinanceAccount, FinanceAccountId};

#[derive(Debug, PartialEq)]
pub enum AccountRepositoryError {
    ReadFailed,
    UpdateFailed,
}

pub trait AccountRepository {
    fn find_by_id(
        &self,
        id: &FinanceAccountId,
    ) -> Result<Option<FinanceAccount>, AccountRepositoryError>;
    fn update(&mut self, account: FinanceAccount) -> Result<(), AccountRepositoryError>;
}

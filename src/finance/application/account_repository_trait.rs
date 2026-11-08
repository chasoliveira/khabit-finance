use crate::finance::domain::account::FinanceAccount;

#[derive(Debug, PartialEq)]
pub enum AccountRepositoryError {
    ReadFailed,
    WriteFailed,
}

pub trait AccountRepository {
    fn find_by_id(&self, id: &str) -> Result<Option<FinanceAccount>, AccountRepositoryError>;
    fn save(&mut self, account: FinanceAccount) -> Result<(), AccountRepositoryError>;
}

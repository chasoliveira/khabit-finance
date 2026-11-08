use crate::finance::domain::account::FinanceAccount;

#[derive(Debug, PartialEq)]
pub enum AccountRepositoryError {
    AccountNotFound,
}

pub trait AccountRepository {
    fn find_by_id(&self, id: &str) -> Option<FinanceAccount>;
    fn save(&mut self, account: FinanceAccount) -> Result<(), AccountRepositoryError>;
}

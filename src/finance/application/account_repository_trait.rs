use crate::FinanceAccount;

pub trait AccountRepository {
    fn new() -> Self;
    fn add(&mut self, account: FinanceAccount);
    fn find_by_id_mut(&mut self, id: &str) -> Option<&mut FinanceAccount>;
}

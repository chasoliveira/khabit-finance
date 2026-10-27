use crate::finance::domain::account::FinanceAccount;

pub trait AccountRepository {
    fn find_by_id_mut(&mut self, id: &str) -> Option<FinanceAccount>;
}

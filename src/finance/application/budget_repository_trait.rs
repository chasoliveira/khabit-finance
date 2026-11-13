use crate::finance::domain::budget::{Budget, BudgetId};

#[derive(Debug, PartialEq)]
pub enum BudgetRepositoryError {
    ReadFailed,
    UpdateFailed,
}

pub trait BudgetRepository {
    fn find_by_id(&self, id: &BudgetId) -> Result<Option<Budget>, BudgetRepositoryError>;
    fn update(&mut self, budget: Budget) -> Result<(), BudgetRepositoryError>;
}

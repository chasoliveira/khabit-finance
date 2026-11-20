use crate::finance::domain::budgets::budget::{Budget, BudgetId};

#[derive(Debug, PartialEq, Clone)]
pub enum BudgetRepositoryError {
    ReadFailed,
    UpdateFailed,
}

pub trait BudgetRepository {
    type Error;

    fn find_by_id(&self, id: &BudgetId) -> Result<Option<Budget>, Self::Error>;
    fn update(&mut self, budget: Budget) -> Result<(), Self::Error>;
}

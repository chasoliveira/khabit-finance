use crate::finance::{
    application::budget_repository_trait::{BudgetRepository, BudgetRepositoryError},
    domain::{
        budgets::budget::{Budget, BudgetId},
        shared::currency::Currency,
    },
};

pub fn new_budget_brl() -> Budget {
    Budget::new("My Wallet".to_string(), Currency::brl())
}
pub struct FakeBudgetRepository {
    budget: Option<Budget>,
    read_error: Option<BudgetRepositoryError>,
    update_error: Option<BudgetRepositoryError>,
}

impl FakeBudgetRepository {
    pub fn new(budget: Option<Budget>) -> Self {
        FakeBudgetRepository {
            budget,
            read_error: None,
            update_error: None,
        }
    }

    pub fn with_read_error(
        budget: Option<Budget>,
        read_error: Option<BudgetRepositoryError>,
    ) -> Self {
        FakeBudgetRepository {
            budget,
            read_error,
            update_error: None,
        }
    }

    pub fn with_update_error(
        budget: Option<Budget>,
        update_error: Option<BudgetRepositoryError>,
    ) -> Self {
        FakeBudgetRepository {
            budget,
            read_error: None,
            update_error,
        }
    }
}

impl BudgetRepository for FakeBudgetRepository {
    fn find_by_id(&self, id: &BudgetId) -> Result<Option<Budget>, BudgetRepositoryError> {
        if self.read_error.is_some() {
            return Err(self.read_error.clone().unwrap());
        }
        match &self.budget {
            Some(bgt) => {
                if bgt.id() == id {
                    Ok(self.budget.clone())
                } else {
                    Ok(None)
                }
            }
            None => Ok(None),
        }
    }

    fn update(&mut self, budget: Budget) -> Result<(), BudgetRepositoryError> {
        if self.update_error.is_some() {
            return Err(self.update_error.clone().unwrap());
        }
        match &self.budget {
            Some(bgt) => {
                if bgt.id() == budget.id() {
                    self.budget = Some(budget);
                    Ok(())
                } else {
                    Err(BudgetRepositoryError::UpdateFailed)
                }
            }
            None => Err(BudgetRepositoryError::UpdateFailed),
        }
    }
}

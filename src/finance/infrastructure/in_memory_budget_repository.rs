use crate::finance::{
    application::budget_repository_trait::{BudgetRepository, BudgetRepositoryError},
    domain::budgets::budget::{Budget, BudgetId},
};

pub struct InMemoryBudgetRepository {
    // This is a placeholder for the in-memory storage of Budgets.
    // In a real implementation, you might use a HashMap or another data structure.
    // For simplicity, we'll just use a vector here.
    budgets: Vec<Budget>,
}

impl InMemoryBudgetRepository {
    pub fn new() -> Self {
        InMemoryBudgetRepository {
            budgets: Vec::new(),
        }
    }

    pub fn add(&mut self, busget: Budget) {
        self.budgets.push(busget);
    }
}

impl BudgetRepository for InMemoryBudgetRepository {
    fn find_by_id(&self, id: &BudgetId) -> Result<Option<Budget>, BudgetRepositoryError> {
        Ok(self
            .budgets
            .iter()
            .find(|budget| budget.id() == id && !budget.is_archived())
            .cloned())
    }

    fn update(&mut self, budget: Budget) -> Result<(), BudgetRepositoryError> {
        if let Some(pos) = self
            .budgets
            .iter()
            .position(|bgt| bgt.id() == budget.id() && !bgt.is_archived())
        {
            self.budgets[pos] = budget;
            Ok(())
        } else {
            Err(BudgetRepositoryError::UpdateFailed)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::finance::domain::{
        account::AccountId,
        movement::MovementType,
        shared::{currency::Currency, money::Money},
    };

    #[test]
    fn test_add_budget() {
        let mut repository = InMemoryBudgetRepository::new();
        let new_budget = Budget::new("My Wallet".to_string(), Currency::brl());
        let new_budget_id = new_budget.id().clone();

        repository.add(new_budget);

        assert_eq!(repository.budgets.len(), 1);
        let found_result = repository.find_by_id(&new_budget_id).unwrap();
        assert!(found_result.is_some());

        let existing = found_result.unwrap();
        assert_eq!(existing.id(), &new_budget_id);
        assert_eq!(existing.name(), "My Wallet".to_string());
        assert_eq!(existing.currency(), &Currency::brl());
    }

    #[test]
    fn test_find_by_id_returns_none() {
        let repository = InMemoryBudgetRepository::new();
        let exists = repository.find_by_id(&BudgetId::new()).unwrap();
        assert!(exists.is_none());
    }

    #[test]
    fn test_update_ok() {
        let mut repository = InMemoryBudgetRepository::new();
        let mut new_item = Budget::new("Test Budget".to_string(), Currency::brl());
        repository.add(new_item.clone());

        let new_item_id = new_item.id().clone();
        let result_added_movment = new_item.create_movement(
            &AccountId::new(),
            MovementType::Debit,
            Money::new(123, Currency::brl()),
            Some("Bought a Coffe".to_string()),
        );
        assert!(result_added_movment.is_ok());

        let result = repository.update(new_item);
        assert!(result.is_ok());
        let exists = repository.find_by_id(&new_item_id).unwrap();
        assert!(exists.is_some());
        assert_eq!(exists.unwrap().movements().len(), 1);
    }
}

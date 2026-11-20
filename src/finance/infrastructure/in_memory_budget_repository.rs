use crate::finance::{
    application::budget_repository_trait::{BudgetRepository, BudgetRepositoryError},
    domain::budgets::budget::{Budget, BudgetId},
    infrastructure::contexts::in_memory_database::InMemoryDatabase,
};

pub struct InMemoryBudgetRepository<'db> {
    db: &'db mut InMemoryDatabase,
}

impl<'db> InMemoryBudgetRepository<'db> {
    pub fn new(db: &'db mut InMemoryDatabase) -> Self {
        InMemoryBudgetRepository { db }
    }

    pub fn add(&mut self, budget: Budget) {
        self.db.add_budget(budget);
    }
    pub fn list(&self) -> &[Budget] {
        self.db.budgets()
    }
}

impl<'db> BudgetRepository for InMemoryBudgetRepository<'db> {
    type Error = BudgetRepositoryError;

    fn find_by_id(&self, id: &BudgetId) -> Result<Option<Budget>, Self::Error> {
        Ok(self
            .db
            .budgets()
            .iter()
            .find(|budget| budget.id() == id)
            .cloned())
    }

    fn update(&mut self, budget: Budget) -> Result<(), Self::Error> {
        if let Some(pos) = self
            .db
            .budgets()
            .iter()
            .position(|bgt| bgt.id() == budget.id())
        {
            self.db.update_budget(pos, budget);
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
        budgets::movements::movement::MovementType,
        shared::{currency::Currency, money::Money},
    };

    #[test]
    fn test_add_budget() {
        let mut db = InMemoryDatabase::new();
        let mut repository = InMemoryBudgetRepository::new(&mut db);
        let new_budget = Budget::new("My Wallet".to_string(), Currency::brl());
        let new_budget_id = new_budget.id().clone();

        repository.add(new_budget);

        assert_eq!(repository.list().len(), 1);
        let found_result = repository.find_by_id(&new_budget_id).unwrap();
        assert!(found_result.is_some());

        let existing = found_result.unwrap();
        assert_eq!(existing.id(), &new_budget_id);
        assert_eq!(existing.name(), "My Wallet".to_string());
        assert_eq!(existing.currency(), &Currency::brl());
    }

    #[test]
    fn test_find_by_id_returns_none() {
        let mut db = InMemoryDatabase::new();
        let repository = InMemoryBudgetRepository::new(&mut db);
        let exists = repository.find_by_id(&BudgetId::new()).unwrap();
        assert!(exists.is_none());
    }

    #[test]
    fn test_update_ok() {
        let mut db = InMemoryDatabase::new();
        let mut repository = InMemoryBudgetRepository::new(&mut db);
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

use crate::finance::{
    application::{
        account_repository_trait::AccountRepository,
        budget_repository_trait::BudgetRepository,
        outbox_repository_trait::OutboxRepository,
        unit_of_work_trait::{UnitOfWork, UnitOfWorkError},
    },
    domain::{
        account::{Account, AccountId},
        budgets::budget::{Budget, BudgetId},
        outbox::outbox_message::OutboxMessage,
    },
    infrastructure::{
        contexts::in_memory_database::InMemoryDatabase,
        in_memory_account_repository::InMemoryAccountRepository,
        in_memory_budget_repository::InMemoryBudgetRepository,
        in_memory_outbox_repository::InMemoryOutboxRepository,
    },
};

pub struct InMemoryUnitOfWork<'db> {
    original: &'db mut InMemoryDatabase,
    working: InMemoryDatabase,
}

impl<'db> InMemoryUnitOfWork<'db> {
    pub fn begin(original: &'db mut InMemoryDatabase) -> Self {
        Self {
            working: original.clone(),
            original,
        }
    }

    pub(crate) fn database_mut(&mut self) -> &mut InMemoryDatabase {
        &mut self.working
    }

    pub fn apply_changes(&mut self) {
        *self.original = self.working.clone();
    }
}

impl<'db> UnitOfWork for InMemoryUnitOfWork<'db> {
    type Error = UnitOfWorkError;

    fn find_account(&mut self, id: &AccountId) -> Result<Option<Account>, Self::Error> {
        let repository = InMemoryAccountRepository::new(self.database_mut());
        repository
            .find_by_id(id)
            .map_err(|_| UnitOfWorkError::ReadFailed)
    }
    fn update_account(&mut self, account: Account) -> Result<(), Self::Error> {
        let mut repository = InMemoryAccountRepository::new(self.database_mut());
        repository
            .update(account)
            .map_err(|_| UnitOfWorkError::UpdateFailed)
    }

    fn find_budget(&mut self, id: &BudgetId) -> Result<Option<Budget>, Self::Error> {
        let repository = InMemoryBudgetRepository::new(self.database_mut());
        repository
            .find_by_id(id)
            .map_err(|_| UnitOfWorkError::ReadFailed)
    }
    fn update_budget(&mut self, budget: Budget) -> Result<(), Self::Error> {
        let mut repository = InMemoryBudgetRepository::new(self.database_mut());
        repository
            .update(budget)
            .map_err(|_| UnitOfWorkError::UpdateFailed)
    }

    fn append_outbox_message(&mut self, outbox_message: OutboxMessage) -> Result<(), Self::Error> {
        let mut repository = InMemoryOutboxRepository::new(self.database_mut());

        repository
            .add(outbox_message)
            .map_err(|_| UnitOfWorkError::UpdateFailed)
    }

    fn commit(&mut self) -> Result<(), Self::Error> {
        self.apply_changes();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{AccountType, finance::domain::shared::currency::Currency};

    use super::*;

    fn new_account() -> Account {
        Account::new(
            "Test Account".to_string(),
            AccountType::Checking,
            Currency::brl(),
        )
    }

    #[test]
    fn commit_persists_working_changes() {
        let mut database = InMemoryDatabase::new();
        let account = new_account();
        let account_id = account.id().clone();
        {
            let mut transaction = InMemoryUnitOfWork::begin(&mut database);
            transaction.database_mut().add_account(account);
            transaction.apply_changes();
        }

        assert!(
            database
                .accounts()
                .iter()
                .any(|account| account.id() == &account_id)
        );
    }

    #[test]
    fn dropping_transaction_discards_working_changes() {
        let mut database = InMemoryDatabase::new();

        {
            let mut transaction = InMemoryUnitOfWork::begin(&mut database);

            transaction.database_mut().add_account(new_account());

            // No commit.
        }

        assert!(database.accounts().is_empty());
    }

    #[test]
    fn commit_persists_outbox_messages() {
        let mut database = InMemoryDatabase::new();

        {
            let mut transaction = InMemoryUnitOfWork::begin(&mut database);

            transaction
                .append_outbox_message(OutboxMessage::new(
                    "MovementCreated".to_string(),
                    r#"{"movement_id":"test"}"#.to_string(),
                ))
                .unwrap();

            transaction.apply_changes();
        }

        assert_eq!(database.outbox_messages().len(), 1);
        assert_eq!(
            database.outbox_messages()[0].event_name(),
            "MovementCreated"
        );
    }

    #[test]
    fn dropping_transaction_discards_outbox_messages() {
        let mut database = InMemoryDatabase::new();

        {
            let mut transaction = InMemoryUnitOfWork::begin(&mut database);

            transaction
                .append_outbox_message(OutboxMessage::new(
                    "MovementCreated".to_string(),
                    r#"{"movement_id":"test"}"#.to_string(),
                ))
                .unwrap();

            // No commit.
        }

        assert!(database.outbox_messages().is_empty());
    }
}

use crate::finance::{
    domain::{
        account::{Account, AccountId},
        budgets::budget::{Budget, BudgetId},
    },
    infrastructure::outbox::outbox_message::OutboxMessage,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UnitOfWorkError {
    ReadFailed,
    UpdateFailed,
    CommitFailed,
}

pub trait UnitOfWork {
    type Error;

    fn find_account(&mut self, id: &AccountId) -> Result<Option<Account>, Self::Error>;
    fn update_account(&mut self, account: Account) -> Result<(), Self::Error>;

    fn find_budget(&mut self, id: &BudgetId) -> Result<Option<Budget>, Self::Error>;
    fn update_budget(&mut self, budget: Budget) -> Result<(), Self::Error>;

    fn append_outbox_message(&mut self, outbox_message: OutboxMessage) -> Result<(), Self::Error>;

    fn commit(&mut self) -> Result<(), Self::Error>;
}

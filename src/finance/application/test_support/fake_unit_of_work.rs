use crate::finance::{
    application::unit_of_work_trait::{UnitOfWork, UnitOfWorkError},
    domain::{
        account::{Account, AccountId},
        budgets::budget::{Budget, BudgetId},
        outbox::outbox_message::OutboxMessage,
    },
};

pub struct FakeUnitOfWork {
    account: Option<Account>,
    budget: Option<Budget>,
    account_read_error: Option<UnitOfWorkError>,
    account_update_error: Option<UnitOfWorkError>,
    budget_read_error: Option<UnitOfWorkError>,
    budget_update_error: Option<UnitOfWorkError>,
    committed: bool,
}

impl FakeUnitOfWork {
    pub fn new(account: Option<Account>, budget: Option<Budget>) -> Self {
        Self {
            account,
            budget,
            account_read_error: None,
            account_update_error: None,
            budget_read_error: None,
            budget_update_error: None,
            committed: false,
        }
    }

    pub fn with_account_read_error(mut self, error: UnitOfWorkError) -> Self {
        self.account_read_error = Some(error);
        self
    }
    pub fn with_account_update_error(mut self, error: UnitOfWorkError) -> Self {
        self.account_update_error = Some(error);
        self
    }

    pub fn with_budget_read_error(mut self, error: UnitOfWorkError) -> Self {
        self.budget_read_error = Some(error);
        self
    }
    pub fn with_budget_update_error(mut self, error: UnitOfWorkError) -> Self {
        self.budget_update_error = Some(error);
        self
    }

    pub fn committed(&self) -> bool {
        self.committed
    }
}

impl UnitOfWork for FakeUnitOfWork {
    type Error = UnitOfWorkError;

    fn find_account(&mut self, id: &AccountId) -> Result<Option<Account>, Self::Error> {
        if let Some(error) = self.account_read_error.clone() {
            return Err(error);
        }

        Ok(self
            .account
            .as_ref()
            .filter(|account| account.id() == id)
            .cloned())
    }
    fn update_account(&mut self, account: Account) -> Result<(), Self::Error> {
        if let Some(error) = self.account_update_error.clone() {
            return Err(error);
        }

        if self
            .account
            .as_ref()
            .is_some_and(|stored| stored.id() == account.id())
        {
            self.account = Some(account);
            Ok(())
        } else {
            Err(UnitOfWorkError::UpdateFailed)
        }
    }

    fn find_budget(&mut self, id: &BudgetId) -> Result<Option<Budget>, Self::Error> {
        if let Some(error) = self.budget_read_error.clone() {
            return Err(error);
        }

        Ok(self
            .budget
            .as_ref()
            .filter(|budget| budget.id() == id)
            .cloned())
    }

    fn update_budget(&mut self, budget: Budget) -> Result<(), Self::Error> {
        if let Some(error) = self.budget_update_error.clone() {
            return Err(error);
        }

        if self
            .budget
            .as_ref()
            .is_some_and(|stored| stored.id() == budget.id())
        {
            self.budget = Some(budget);
            Ok(())
        } else {
            Err(UnitOfWorkError::UpdateFailed)
        }
    }

    fn append_outbox_message(&mut self, _outbox_message: OutboxMessage) -> Result<(), Self::Error> {
        Ok(())
    }

    fn commit(&mut self) -> Result<(), Self::Error> {
        self.committed = true;
        Ok(())
    }
}

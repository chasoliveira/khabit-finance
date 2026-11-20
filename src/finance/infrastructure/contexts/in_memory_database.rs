use crate::finance::{
    domain::{account::Account, budgets::budget::Budget},
    infrastructure::outbox::outbox_message::OutboxMessage,
};

#[derive(Clone, Debug)]
pub struct InMemoryDatabase {
    accounts: Vec<Account>,
    budgets: Vec<Budget>,
    outbox_messages: Vec<OutboxMessage>,
}

impl InMemoryDatabase {
    pub fn new() -> Self {
        Self {
            accounts: Vec::new(),
            budgets: Vec::new(),
            outbox_messages: Vec::new(),
        }
    }
    pub fn accounts(&self) -> &[Account] {
        &self.accounts
    }
    pub fn add_account(&mut self, account: Account) {
        self.accounts.push(account)
    }
    pub fn update_account(&mut self, position: usize, account: Account) {
        self.accounts[position] = account;
    }

    pub fn budgets(&self) -> &[Budget] {
        &self.budgets
    }
    pub fn add_budget(&mut self, budget: Budget) {
        self.budgets.push(budget)
    }
    pub fn update_budget(&mut self, position: usize, budget: Budget) {
        self.budgets[position] = budget;
    }

    pub fn outbox_messages(&self) -> &[OutboxMessage] {
        &self.outbox_messages
    }
    pub fn add_outbox_message(&mut self, outbox_message: OutboxMessage) {
        self.outbox_messages.push(outbox_message)
    }
    pub fn update_outbox_message(&mut self, position: usize, outbox_message: OutboxMessage) {
        self.outbox_messages[position] = outbox_message;
    }
}

use crate::finance::domain::{budgets::budget::BudgetId, events::domain_events::DomainEvent};

#[derive(Debug, Clone)]
pub struct BudgetCreatedDomainEvent {
    pub budget_id: BudgetId,
}

impl BudgetCreatedDomainEvent {
    pub(in super::super) fn new(budget_id: BudgetId) -> Self {
        Self { budget_id }
    }
}
impl DomainEvent for BudgetCreatedDomainEvent {
    fn event_name(&self) -> &'static str {
        "BudgetCreatedDomainEvent"
    }

    fn clone_box(&self) -> Box<dyn DomainEvent> {
        Box::new(self.clone())
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

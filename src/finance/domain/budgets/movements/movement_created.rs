use crate::finance::domain::{
    budgets::{budget::BudgetId, movements::movement::MovementId},
    events::domain_events::DomainEvent,
};

#[derive(Debug, Clone)]
pub struct MovementCreatedDomainEvent {
    pub budget_id: BudgetId,
    pub movement_id: MovementId,
}

impl MovementCreatedDomainEvent {
    pub(in super::super) fn new(budget_id: BudgetId, movement_id: MovementId) -> Self {
        Self {
            budget_id,
            movement_id,
        }
    }
}

impl DomainEvent for MovementCreatedDomainEvent {
    fn event_name(&self) -> &'static str {
        "MovementCreatedDomainEvent"
    }

    fn clone_box(&self) -> Box<dyn DomainEvent> {
        Box::new(self.clone())
    }
}

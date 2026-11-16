use crate::finance::domain::{
    budgets::budget_created::BudgetCreatedDomainEvent, events::domain_events::DomainEvent,
};

#[derive(Debug)]
pub enum SerdeBudgetError {
    UnsupportedEvent,
    SerializationFailed,
}

pub fn serialize_budget_created(event: &dyn DomainEvent) -> Result<String, SerdeBudgetError> {
    let event = event
        .as_any()
        .downcast_ref::<BudgetCreatedDomainEvent>()
        .ok_or(SerdeBudgetError::UnsupportedEvent)?;

    serde_json::to_string(event).map_err(|_| SerdeBudgetError::SerializationFailed)
}

#[cfg(test)]
mod tests {
    use crate::finance::{
        domain::{
            account::AccountId,
            budgets::{budget::Budget, movements::movement::MovementType},
            events::domain_event_source::DomainEventSource,
            shared::{currency::Currency, money::Money},
        },
        infrastructure::serializers::serializer_budget_created::serialize_budget_created,
    };

    #[test]
    fn serialize_budget_created_should_work_properly() {
        let mut budget = Budget::new("Monthly expenses".to_string(), Currency::usa());
        let account_id = AccountId::new();
        let movement_type = MovementType::Credit;
        let amount = Money::new(100, Currency::usa());
        let decription = Some("text".to_string());
        let result = budget.create_movement(
            &account_id,
            movement_type.clone(),
            amount.clone(),
            decription.clone(),
        );

        assert!(result.is_ok());
        assert_eq!(budget.movements().len(), 1);

        let event = budget.domain_events()[0].as_ref();
        let result = serialize_budget_created(event);
        assert!(result.is_ok());
        let json = result.unwrap();

        let value: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(value["budget_id"], serde_json::json!(&budget.id()));
    }
}

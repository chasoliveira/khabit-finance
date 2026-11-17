use crate::finance::{
    domain::{
        budgets::budget_created::BudgetCreatedDomainEvent, events::domain_events::DomainEvent,
    },
    infrastructure::serializers::domain_event_serializer::{
        DomainEventSerializer, DomainEventSerializerError, serialize_event,
    },
};

pub struct BudgetCreatedEventSerializer;

impl DomainEventSerializer for BudgetCreatedEventSerializer {
    fn can_serialize(&self, event: &dyn DomainEvent) -> bool {
        event.as_any().is::<BudgetCreatedDomainEvent>()
    }
    fn serialize(&self, event: &dyn DomainEvent) -> Result<String, DomainEventSerializerError> {
        let event = event
            .as_any()
            .downcast_ref::<BudgetCreatedDomainEvent>()
            .ok_or(DomainEventSerializerError::UnsupportedEvent)?;

        serialize_event(event)
    }
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
        infrastructure::serializers::{
            budget_created_serializer::BudgetCreatedEventSerializer,
            domain_event_serializer::{
                DomainEventSerializer, DomainEventSerializerError, serialize_event,
            },
            test_support::domain_failing_event::FailingEvent,
        },
    };

    #[test]
    fn serialize_budget_created_should_work_properly() {
        let serializer = BudgetCreatedEventSerializer;
        let mut budget = Budget::new("Monthly expenses".to_string(), Currency::usa());
        let movement = budget.create_movement(
            &AccountId::new(),
            MovementType::Credit,
            Money::new(100, Currency::usa()),
            Some("text".to_string()),
        );

        assert!(movement.is_ok());
        assert_eq!(budget.movements().len(), 1);

        let event = budget.domain_events()[0].as_ref();
        let result = serializer.can_serialize(event);
        assert!(result);
        let json = serializer.serialize(event).unwrap();

        let value: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(value["budget_id"], serde_json::json!(&budget.id()));
    }

    #[test]
    fn serialize_event_returns_serialization_failed_when_json_serializer_fails() {
        let result = serialize_event(&FailingEvent);

        assert!(matches!(
            result,
            Err(DomainEventSerializerError::SerializationFailed)
        ));
    }

    #[test]
    fn serialize_budget_created_rejects_unsupported_event() {
        let serializer = BudgetCreatedEventSerializer;
        let mut budget = Budget::new("Monthly expenses".to_string(), Currency::usa());
        budget
            .create_movement(
                &AccountId::new(),
                MovementType::Credit,
                Money::new(100, Currency::usa()),
                None,
            )
            .unwrap();

        let movement_event = budget.movements()[0].domain_events()[0].as_ref();
        let result = serializer.serialize(movement_event);

        assert!(matches!(
            result,
            Err(DomainEventSerializerError::UnsupportedEvent)
        ));
    }
}

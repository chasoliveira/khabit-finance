use crate::finance::{
    domain::events::domain_event_source::DomainEventSource,
    infrastructure::{
        outbox::outbox_message::OutboxMessage,
        serializers::{
            domain_event_serializer::DomainEventSerializerError,
            domain_event_serializer_registry::{DomainEventSerializerRegistry, SerializerRegistry},
        },
    },
};

pub trait DomainEventExtractor {
    fn extract_outbox_messages(
        &self,
        domain_event: &dyn DomainEventSource,
        registry: &DomainEventSerializerRegistry,
    ) -> Result<Vec<OutboxMessage>, DomainEventSerializerError>;
}

pub struct OutboxDomainEventExtractor;

impl DomainEventExtractor for OutboxDomainEventExtractor {
    fn extract_outbox_messages(
        &self,
        domain_event: &dyn DomainEventSource,
        registry: &DomainEventSerializerRegistry,
    ) -> Result<Vec<OutboxMessage>, DomainEventSerializerError> {
        let mut error = None;
        let mut outbox_messages: Vec<OutboxMessage> = Vec::new();
        domain_event.visit_event_sources(&mut |event_source| {
            if error.is_some() {
                return;
            }
            for event in event_source.domain_events() {
                let item = registry.serialize(event.as_ref());
                match item {
                    Ok(json) => {
                        outbox_messages
                            .push(OutboxMessage::new(event.event_name().to_string(), json));
                    }
                    Err(err) => {
                        error = Some(err);
                        break;
                    }
                }
            }
        });

        if let Some(err) = error {
            return Err(err);
        }

        Ok(outbox_messages)
    }
}

#[cfg(test)]
mod tests {
    use crate::finance::{
        domain::{
            account::AccountId,
            budgets::{budget::Budget, movements::movement::MovementType},
            shared::{currency::Currency, money::Money},
        },
        infrastructure::{
            outbox::outbox_traverser::{DomainEventExtractor, OutboxDomainEventExtractor},
            serializers::{
                budget_created_serializer::BudgetCreatedEventSerializer,
                domain_event_serializer::DomainEventSerializerError,
                domain_event_serializer_registry::{
                    DomainEventSerializerRegistry, SerializerRegistry,
                },
                movement_created_serializer::MovementCreatedEventSerializer,
            },
        },
    };

    #[test]
    fn extract_outbox_messages_success_when_multiple_visitor_locations() {
        let extractor = OutboxDomainEventExtractor;
        let mut registry = DomainEventSerializerRegistry::new();
        registry.register(BudgetCreatedEventSerializer);
        registry.register(MovementCreatedEventSerializer);

        let mut budget = Budget::new("Monthly expenses".to_string(), Currency::usa());
        let movement_id = budget
            .create_movement(
                &AccountId::new(),
                MovementType::Credit,
                Money::new(100, Currency::usa()),
                None,
            )
            .unwrap();

        let result = extractor.extract_outbox_messages(&budget, &registry);

        assert!(result.is_ok());
        let outbox_messages = result.unwrap();
        assert_eq!(outbox_messages.len(), 2);

        assert_eq!(outbox_messages[0].event_name(), "BudgetCreatedDomainEvent");
        let value: serde_json::Value = serde_json::from_str(&outbox_messages[0].payload()).unwrap();
        assert_eq!(value["budget_id"], serde_json::json!(&budget.id()));

        assert_eq!(
            outbox_messages[1].event_name(),
            "MovementCreatedDomainEvent"
        );
        let value: serde_json::Value = serde_json::from_str(&outbox_messages[1].payload()).unwrap();
        assert_eq!(value["budget_id"], serde_json::json!(&budget.id()));
        assert_eq!(value["movement_id"], serde_json::json!(&movement_id));
    }
    #[test]
    fn extract_outbox_messages_success_when_only_one_visitor_location() {
        let extractor = OutboxDomainEventExtractor;
        let mut registry = DomainEventSerializerRegistry::new();
        registry.register(BudgetCreatedEventSerializer);
        registry.register(MovementCreatedEventSerializer);

        let budget = Budget::new("Monthly expenses".to_string(), Currency::usa());

        let result = extractor.extract_outbox_messages(&budget, &registry);

        assert!(result.is_ok());
        let outbox_messages = result.unwrap();
        assert_eq!(outbox_messages.len(), 1);

        assert_eq!(outbox_messages[0].event_name(), "BudgetCreatedDomainEvent");
        let value: serde_json::Value = serde_json::from_str(&outbox_messages[0].payload()).unwrap();
        assert_eq!(value["budget_id"], serde_json::json!(&budget.id()));
    }
    #[test]
    fn extract_outbox_messages_fails_when_no_registers() {
        let extractor = OutboxDomainEventExtractor;
        let registry = DomainEventSerializerRegistry::new();
        let budget = Budget::new("Monthly expenses".to_string(), Currency::usa());

        let result = extractor.extract_outbox_messages(&budget, &registry);

        assert!(result.is_err());
        let outbox_message_error = result.unwrap_err();
        assert_eq!(
            outbox_message_error,
            DomainEventSerializerError::UnsupportedEvent
        );
    }
    #[test]
    fn extract_outbox_messages_fails_when_only_one_registers() {
        let extractor = OutboxDomainEventExtractor;
        let mut registry = DomainEventSerializerRegistry::new();
        registry.register(BudgetCreatedEventSerializer);

        let mut budget = Budget::new("Monthly expenses".to_string(), Currency::usa());
        budget
            .create_movement(
                &AccountId::new(),
                MovementType::Credit,
                Money::new(100, Currency::usa()),
                None,
            )
            .unwrap();
        let result = extractor.extract_outbox_messages(&budget, &registry);

        assert!(result.is_err());
        let outbox_message_error = result.unwrap_err();
        assert_eq!(
            outbox_message_error,
            DomainEventSerializerError::UnsupportedEvent
        );
    }
}

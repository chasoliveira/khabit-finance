use crate::finance::{
    domain::events::domain_events::DomainEvent,
    infrastructure::serializers::domain_event_serializer::{
        DomainEventSerializer, DomainEventSerializerError,
    },
};

pub struct DomainEventSerializerRegistry {
    serializers: Vec<Box<dyn DomainEventSerializer>>,
}

pub trait SerializerRegistry {
    fn register<T>(&mut self, event: T)
    where
        T: DomainEventSerializer + 'static;
    fn serialize(&self, event: &dyn DomainEvent) -> Result<String, DomainEventSerializerError>;
}
impl DomainEventSerializerRegistry {
    pub fn new() -> Self {
        Self {
            serializers: Vec::new(),
        }
    }
}
impl SerializerRegistry for DomainEventSerializerRegistry {
    fn register<T>(&mut self, serializer: T)
    where
        T: DomainEventSerializer + 'static,
    {
        self.serializers.push(Box::new(serializer));
    }
    fn serialize(&self, event: &dyn DomainEvent) -> Result<String, DomainEventSerializerError> {
        let serializer_option = self
            .serializers
            .iter()
            .find(|domain_event| domain_event.can_serialize(event));

        match serializer_option {
            None => Err(DomainEventSerializerError::UnsupportedEvent),
            Some(serializer) => serializer.serialize(event),
        }
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
            domain_event_serializer::DomainEventSerializerError,
            domain_event_serializer_registry::{DomainEventSerializerRegistry, SerializerRegistry},
            movement_created_serializer::MovementCreatedEventSerializer,
        },
    };

    #[test]
    fn register_should_register_success() {
        let mut registry = DomainEventSerializerRegistry::new();
        registry.register(BudgetCreatedEventSerializer);
        assert_eq!(registry.serializers.len(), 1);
        registry.register(MovementCreatedEventSerializer);
        assert_eq!(registry.serializers.len(), 2);
    }

    #[test]
    fn register_solve_serializer() {
        let mut registry = DomainEventSerializerRegistry::new();
        registry.register(BudgetCreatedEventSerializer);
        registry.register(MovementCreatedEventSerializer);

        let mut budget = Budget::new("Monthly expenses".to_string(), Currency::usa());
        let budget_event = budget.domain_events()[0].as_ref();
        let budget_event_json = registry.serialize(budget_event).unwrap();

        let value: serde_json::Value = serde_json::from_str(&budget_event_json).unwrap();

        assert_eq!(value["budget_id"], serde_json::json!(&budget.id()));

        let movement_id = budget
            .create_movement(
                &AccountId::new(),
                MovementType::Credit,
                Money::new(100, Currency::usa()),
                None,
            )
            .unwrap();

        let movement_event = budget.movements()[0].domain_events()[0].as_ref();
        let movement_event_json = registry.serialize(movement_event).unwrap();
        let value: serde_json::Value = serde_json::from_str(&movement_event_json).unwrap();

        assert_eq!(value["budget_id"], serde_json::json!(&budget.id()));
        assert_eq!(value["movement_id"], serde_json::json!(&movement_id));
    }

    #[test]
    fn register_returns_unsuported_event() {
        let mut registry = DomainEventSerializerRegistry::new();
        registry.register(MovementCreatedEventSerializer);
        let mut budget = Budget::new("Monthly expenses".to_string(), Currency::usa());
        budget
            .create_movement(
                &AccountId::new(),
                MovementType::Credit,
                Money::new(100, Currency::usa()),
                None,
            )
            .unwrap();

        let budget_created_event = budget.domain_events()[0].as_ref();
        let result = registry.serialize(budget_created_event);

        assert!(matches!(
            result,
            Err(DomainEventSerializerError::UnsupportedEvent)
        ));
    }
}

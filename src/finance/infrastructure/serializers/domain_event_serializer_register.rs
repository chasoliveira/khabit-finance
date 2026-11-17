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
    fn register<T>(&mut self, domain_event: T)
    where
        T: DomainEventSerializer + 'static,
    {
        self.serializers.push(Box::new(domain_event));
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
            budgets::{
                budget::{Budget, BudgetId},
                budget_created::BudgetCreatedDomainEvent,
            },
            events::domain_event_source::DomainEventSource,
            shared::currency::Currency,
        },
        infrastructure::serializers::{
            budget_created_serializer::BudgetCreatedEventSerializer,
            domain_event_serializer_register::{DomainEventSerializerRegistry, SerializerRegistry},
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

        let budget = Budget::new("Monthly expenses".to_string(), Currency::usa());
        let budget_event = budget.domain_events()[0].as_ref();
        let result = registry.serialize(budget_event);

        let json = result.unwrap();
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(value["budget_id"], serde_json::json!(&budget.id()));
    }
}

use serde::Serialize;

use crate::finance::domain::events::domain_events::DomainEvent;

#[derive(Debug, PartialEq, Eq)]
pub enum DomainEventSerializerError {
    UnsupportedEvent,
    SerializationFailed,
}
pub trait DomainEventSerializer {
    fn can_serialize(&self, event: &dyn DomainEvent) -> bool;
    fn serialize(&self, event: &dyn DomainEvent) -> Result<String, DomainEventSerializerError>;
}

pub fn serialize_event<T: Serialize>(event: &T) -> Result<String, DomainEventSerializerError> {
    serde_json::to_string(event).map_err(|_| DomainEventSerializerError::SerializationFailed)
}

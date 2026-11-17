use serde::{Serialize, ser::Error};

use crate::finance::domain::events::domain_events::DomainEvent;

#[derive(Debug, Clone)]
pub struct FailingEvent;

impl Serialize for FailingEvent {
    fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        Err(S::Error::custom("forced serialization failure"))
    }
}

impl DomainEvent for FailingEvent {
    fn event_name(&self) -> &'static str {
        "FailingEvent"
    }
    fn clone_box(&self) -> Box<dyn DomainEvent> {
        Box::new(self.clone())
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

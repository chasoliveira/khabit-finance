use crate::finance::domain::events::domain_events::DomainEvent;

pub trait DomainEventSource {
    fn domain_events(&self) -> &[Box<dyn DomainEvent>];

    fn clear_domain_events(&mut self);
}

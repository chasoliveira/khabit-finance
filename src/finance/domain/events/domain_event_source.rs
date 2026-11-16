use crate::finance::domain::events::domain_events::DomainEvent;

pub trait DomainEventSource {
    fn domain_events(&self) -> &[Box<dyn DomainEvent>];
    fn clear_domain_events(&mut self);
    fn visit_event_sources(&self, visitor: &mut dyn FnMut(&dyn DomainEventSource));
    fn visit_event_sources_mut(&mut self, visitor: &mut dyn FnMut(&mut dyn DomainEventSource));
}

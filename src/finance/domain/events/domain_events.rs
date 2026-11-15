use std::fmt::Debug;

#[derive(Clone, Debug)]
pub struct DomainEvents {
    events: Vec<Box<dyn DomainEvent>>,
}

impl DomainEvents {
    pub fn new() -> Self {
        DomainEvents { events: Vec::new() }
    }

    pub(in crate::finance::domain) fn raise(&mut self, event: Box<dyn DomainEvent>) {
        self.events.push(event);
    }
    pub fn events(&self) -> &[Box<dyn DomainEvent>] {
        &self.events
    }
    pub fn clear(&mut self) {
        self.events = Vec::new()
    }
}

pub trait DomainEvent: Debug + Send + Sync {
    fn event_name(&self) -> &'static str;
    fn clone_box(&self) -> Box<dyn DomainEvent>;
}

impl Clone for Box<dyn DomainEvent> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

#[cfg(test)]
mod tests {
    use super::{DomainEvent, DomainEvents};

    #[derive(Clone, Debug)]
    struct TestEvent {
        name: &'static str,
    }

    impl DomainEvent for TestEvent {
        fn event_name(&self) -> &'static str {
            self.name
        }

        fn clone_box(&self) -> Box<dyn DomainEvent> {
            Box::new(self.clone())
        }
    }

    #[test]
    fn creates_an_empty_event_collection() {
        let events = DomainEvents::new();

        assert!(events.events().is_empty());
    }

    #[test]
    fn raises_and_exposes_events() {
        let mut events = DomainEvents::new();

        events.raise(Box::new(TestEvent {
            name: "MovementCreated",
        }));

        assert_eq!(events.events().len(), 1);
        assert_eq!(events.events()[0].event_name(), "MovementCreated");
    }

    #[test]
    fn clones_boxed_events() {
        let mut events = DomainEvents::new();
        events.raise(Box::new(TestEvent {
            name: "MovementCreated",
        }));

        let cloned_events = events.clone();
        events.clear();

        assert!(events.events().is_empty());
        assert_eq!(cloned_events.events().len(), 1);
        assert_eq!(cloned_events.events()[0].event_name(), "MovementCreated");
    }

    #[test]
    fn clears_all_events() {
        let mut events = DomainEvents::new();
        events.raise(Box::new(TestEvent { name: "FirstEvent" }));
        events.raise(Box::new(TestEvent {
            name: "SecondEvent",
        }));

        events.clear();

        assert!(events.events().is_empty());
    }
}

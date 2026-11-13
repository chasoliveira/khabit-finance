use std::fmt::Debug;

pub trait DomainEvent: Debug + Send + Sync {
    fn event_name(&self) -> &'static str;
    fn clone_box(&self) -> Box<dyn DomainEvent>;
}

impl Clone for Box<dyn DomainEvent> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

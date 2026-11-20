use crate::finance::domain::outbox::outbox_message::OutboxMessage;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OutboxRepositoryError {
    AddFailed,
}
pub trait OutboxRepository {
    type Error;

    fn add(&mut self, message: OutboxMessage) -> Result<(), Self::Error>;
}

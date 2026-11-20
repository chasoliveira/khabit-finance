use crate::finance::{
    application::outbox_repository_trait::{OutboxRepository, OutboxRepositoryError},
    infrastructure::{
        contexts::in_memory_database::InMemoryDatabase, outbox::outbox_message::OutboxMessage,
    },
};

pub struct InMemoryOutboxRepository<'db> {
    db: &'db mut InMemoryDatabase,
}

impl<'db> InMemoryOutboxRepository<'db> {
    pub fn new(db: &'db mut InMemoryDatabase) -> Self {
        Self { db }
    }

    pub fn list(&self) -> &[OutboxMessage] {
        self.db.outbox_messages()
    }
}

impl<'db> OutboxRepository for InMemoryOutboxRepository<'db> {
    type Error = OutboxRepositoryError;
    fn add(&mut self, message: OutboxMessage) -> Result<(), Self::Error> {
        self.db.add_outbox_message(message);
        Ok(())
    }
}

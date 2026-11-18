#[derive(Debug)]
pub struct OutboxMessage {
    event_name: String,
    payload: String,
}

impl OutboxMessage {
    pub fn new(event_name: String, payload: String) -> Self {
        Self {
            event_name,
            payload,
        }
    }

    pub fn event_name(&self) -> &str {
        &self.event_name
    }
    pub fn payload(&self) -> &str {
        &self.payload
    }
}

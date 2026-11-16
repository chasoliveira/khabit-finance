use serde::Serialize;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
pub struct Id {
    value: Uuid,
}

impl Id {
    pub fn new() -> Self {
        Id {
            value: Uuid::new_v4(),
        }
    }

    pub fn value(&self) -> &Uuid {
        &self.value
    }
}

use uuid::Uuid;

#[derive(Clone, Debug, PartialEq)]
pub struct Id {
    value: Uuid,
}

impl Id {
    pub fn new() -> Self {
        Id {
            value: Uuid::new_v4(),
        }
    }
}

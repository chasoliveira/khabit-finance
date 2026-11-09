use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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

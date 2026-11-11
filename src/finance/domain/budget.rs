use crate::finance::domain::shared::{currency::Currency, id::Id};

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct BudgetId(Id);

#[derive(Debug, PartialEq)]
pub enum BudgetError {}

impl BudgetId {
    pub fn new() -> Self {
        BudgetId(Id::new())
    }
}

pub struct Budget {
    id: BudgetId,
    name: String,
    currency: Currency,
    is_archived: bool,
}

impl Budget {
    pub fn new(name: String, currency: Currency) -> Self {
        Budget {
            id: BudgetId(Id::new()),
            name,
            currency,
            is_archived: true,
        }
    }

    pub fn id(&self) -> &BudgetId {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn curency(&self) -> &Currency {
        &self.currency
    }

    pub fn is_archived(&self) -> &bool {
        &self.is_archived
    }

    pub fn archive(&mut self) -> Result<(), BudgetError> {
        self.is_archived = true;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use uuid::Version;

    use super::*;

    #[test]
    fn creates_budget_with_expected_values() {
        let budget = Budget::new("Monthly expenses".to_string(), Currency::brl());

        assert_eq!(budget.id().0.value().get_version(), Some(Version::Random));
        assert_eq!(budget.name(), "Monthly expenses");
        assert_eq!(budget.curency(), &Currency::brl());
        assert_eq!(budget.is_archived(), &true);
    }

    #[test]
    fn creates_budget_id_with_random_uuid() {
        let id = BudgetId::new();

        assert_eq!(id.0.value().get_version(), Some(Version::Random));
    }

    #[test]
    fn archive_returns_ok_and_keeps_budget_archived() {
        let mut budget = Budget::new("Monthly expenses".to_string(), Currency::usa());

        assert_eq!(budget.archive(), Ok(()));
        assert_eq!(budget.is_archived(), &true);
    }
}

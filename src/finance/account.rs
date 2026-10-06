#[derive(Debug)]
pub enum FinanceAccountType {
    Cash,
    Checking,
    Savings,
    Investment,
}

impl FinanceAccountType {
    pub fn is_expandable(&self) -> bool {
        match self {
            FinanceAccountType::Cash => true,
            FinanceAccountType::Checking => true,
            FinanceAccountType::Savings => false,
            FinanceAccountType::Investment => false,
        }
    }
}

pub mod finance;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_finance_account_type() {
        use finance::account::FinanceAccountType;

        for account_type in [FinanceAccountType::Cash, FinanceAccountType::Checking] {
            println!("Testing expandable account type: {:?}", account_type);
            assert_eq!(account_type.is_expandable(), true);
        }

        for account_type in [FinanceAccountType::Savings, FinanceAccountType::Investment] {
            println!("Testing non-expandable account type: {:?}", account_type);
            assert_eq!(account_type.is_expandable(), false);
        }
    }
}

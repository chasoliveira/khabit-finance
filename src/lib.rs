pub mod finance;

pub fn call_finance() {
    finance::call_me();
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_call_finance() {
        call_finance();
    }

    #[test]
    fn test_finance_account_type() {
        use finance::account::FinanceAccountType;
        let account_type = FinanceAccountType::Checking;
        match account_type {
            FinanceAccountType::Cash => println!("This is a cash account."),
            FinanceAccountType::Checking => println!("This is a checking account."),
            FinanceAccountType::Savings => println!("This is a savings account."),
            FinanceAccountType::Investment => println!("This is an investment account."),
        }
    }
}

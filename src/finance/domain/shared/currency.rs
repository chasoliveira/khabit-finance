#[derive(Debug, Clone, PartialEq)]
pub enum CurrencyType {
    BRL,
    USA,
    CAN,
    EUR,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Currency {
    code: CurrencyType,
    description: String,
    symbol: String,
}

impl Currency {
    pub fn brl() -> Self {
        Self {
            code: CurrencyType::BRL,
            description: "Brazilian Real".to_string(),
            symbol: "R$".to_string(),
        }
    }
    pub fn usa() -> Self {
        Self {
            code: CurrencyType::USA,
            description: "American Dollar".to_string(),
            symbol: "$".to_string(),
        }
    }
    pub fn can() -> Self {
        Self {
            code: CurrencyType::CAN,
            description: "Canadian Dollar".to_string(),
            symbol: "CA$".to_string(),
        }
    }
    pub fn eur() -> Self {
        Self {
            code: CurrencyType::EUR,
            description: "Euro".to_string(),
            symbol: "€".to_string(),
        }
    }

    pub fn code(&self) -> &CurrencyType {
        &self.code
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn symbol(&self) -> &str {
        &self.symbol
    }
}

#[cfg(test)]
mod tests {
    use super::{Currency, CurrencyType};

    #[test]
    fn creates_brl_currency() {
        let currency = Currency::brl();

        assert_eq!(currency.code(), &CurrencyType::BRL);
        assert_eq!(currency.description(), "Brazilian Real");
        assert_eq!(currency.symbol(), "R$");
    }

    #[test]
    fn creates_usa_currency() {
        let currency = Currency::usa();

        assert_eq!(currency.code(), &CurrencyType::USA);
        assert_eq!(currency.description(), "American Dollar");
        assert_eq!(currency.symbol(), "$");
    }

    #[test]
    fn creates_can_currency() {
        let currency = Currency::can();

        assert_eq!(currency.code(), &CurrencyType::CAN);
        assert_eq!(currency.description(), "Canadian Dollar");
        assert_eq!(currency.symbol(), "CA$");
    }

    #[test]
    fn creates_eur_currency() {
        let currency = Currency::eur();

        assert_eq!(currency.code(), &CurrencyType::EUR);
        assert_eq!(currency.description(), "Euro");
        assert_eq!(currency.symbol(), "€");
    }
}

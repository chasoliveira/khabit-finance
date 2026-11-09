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
}

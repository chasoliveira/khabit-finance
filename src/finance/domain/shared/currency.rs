#[derive(Clone)]
pub struct Currency {
    code: String,
    description: String,
    symbol: String,
}

impl Currency {
    pub fn new(code: String, description: String, symbol: String) -> Self {
        Self {
            code,
            description,
            symbol,
        }
    }
}

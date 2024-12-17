use std::fmt;
/// Generic Currency Struct
#[derive(Debug, Clone, PartialEq)]
pub struct Currency<T> {
    pub amount: T,
    pub code: String,
}

impl<T> Currency<T> {
    pub fn new(amount: T, code: &str) -> Self {
        Self {
            amount,
            code: code.to_uppercase(),
        }
    }
}

impl<T: fmt::Display> fmt::Display for Currency<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.amount, self.code)
    }
}

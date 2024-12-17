use std::ops::{Add, Sub, Mul, Div};
use crate::currency::Currency;

/// Implementation of addition operation on Currency Struct
impl<T> Add for Currency<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.code != other.code {
            panic!("Cannot add different currencies: {} and {}", self.code, other.code);
        }
        Self::new(self.amount + other.amount, &self.code)
    }
}
/// Implementation of Subtraction operation on Currency Struct
impl<T> Sub for Currency<T>
where
    T: Sub<Output = T> + PartialOrd + Copy,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        if self.code != other.code {
            panic!("Cannot subtract different currencies: {} and {}", self.code, other.code);
        }
        if self.amount < other.amount {
            panic!("Resulting amount cannot be negative!");
        }
        Self::new(self.amount - other.amount, &self.code)
    }
}
/// Implementation of Multiplaction operation on Currency Struct
impl<T> Mul<T> for Currency<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, scalar: T) -> Self {
        Self::new(self.amount * scalar, &self.code)
    }
}
/// Implementation of Division operation on Currency Struct
impl<T> Div<T> for Currency<T>
where
    T: Div<Output = T> + PartialOrd + Copy,
{
    type Output = Self;

    fn div(self, scalar: T) -> Self {
        if scalar <= T::from(0) {
            panic!("Scalar divisor must be greater than zero!");
        }
        Self::new(self.amount / scalar, &self.code)
    }
}

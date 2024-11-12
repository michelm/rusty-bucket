use num_traits::Zero;
use std::ops::{Add, Div, Mul, Sub};
use std::result::Result;

#[derive(Debug, Clone)]
pub struct DivideByZeroError;

/// Adds two numbers together
pub fn add<T>(left: T, right: T) -> T
where
    T: Add<Output = T>,
{
    left + right
}

/// Subtracts one number from another
pub fn subtract<T>(left: T, right: T) -> T
where
    T: Sub<Output = T>,
{
    left - right
}

/// Multiplies two numbers together
pub fn multiply<T>(left: T, right: T) -> T
where
    T: Mul<Output = T>,
{
    left * right
}

/// Divides one number by another
pub fn divide<T>(left: T, right: T) -> Result<T, DivideByZeroError>
where
    T: Div<Output = T> + Zero + PartialEq,
{
    if right == T::zero() {
        return Err(DivideByZeroError);
    }

    Ok(left / right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let result: u32 = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_subtract() {
        let result = subtract(2, 2);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_divide() {
        let result = divide(4, 2).unwrap();
        assert_eq!(result, 2);
    }

    #[test]
    fn test_divide_f32() {
        let result: f32 = divide(4.0, 2.0).unwrap();
        assert_eq!(result, 2.0);
    }

    #[test]
    fn test_divide_f64() {
        let result: f64 = divide(4.0, 2.0).unwrap();
        assert_eq!(result, 2.0);
    }

    #[test]
    fn test_divide_by_zero() {
        assert!(divide(4, 0).is_err());
    }

    #[test]
    fn test_divide_by_zero_f32() {
        assert!(divide(4.0f32, 0.0f32).is_err());
    }

    #[test]
    fn test_divide_by_zero_f64() {
        assert!(divide(4.0f64, 0.0f64).is_err());
    }
}

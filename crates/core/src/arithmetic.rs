//! Checked U256 division with exact intermediate products.

use crate::errors::ArithmeticError;
use fastnum::{Cast, TryCast, U256, U512};

/// Returns `floor(lhs * rhs / denominator)` with an exact 512-bit product.
pub fn mul_div_floor(lhs: U256, rhs: U256, denominator: U256) -> Result<U256, ArithmeticError> {
    if denominator.is_zero() {
        return Err(ArithmeticError::DivisionByZero);
    }
    let product = Cast::<U512>::cast(lhs) * Cast::<U512>::cast(rhs);
    TryCast::<U256>::try_cast(product / Cast::<U512>::cast(denominator)).map_err(|_| ArithmeticError::Overflow)
}

/// Returns `ceil(lhs * rhs / denominator)` with an exact 512-bit product.
pub fn mul_div_ceil(lhs: U256, rhs: U256, denominator: U256) -> Result<U256, ArithmeticError> {
    if denominator.is_zero() {
        return Err(ArithmeticError::DivisionByZero);
    }
    let product = Cast::<U512>::cast(lhs) * Cast::<U512>::cast(rhs);
    let divisor = Cast::<U512>::cast(denominator);
    let quotient = product / divisor;
    let rounded = if product % divisor == U512::ZERO {
        quotient
    } else {
        quotient.checked_add(U512::ONE).ok_or(ArithmeticError::Overflow)?
    };
    TryCast::<U256>::try_cast(rounded).map_err(|_| ArithmeticError::Overflow)
}

/// Returns `ceil(numerator / denominator)` without adding to the numerator.
pub fn div_ceil(numerator: U256, denominator: U256) -> Result<U256, ArithmeticError> {
    if denominator.is_zero() {
        return Err(ArithmeticError::DivisionByZero);
    }
    let quotient = numerator / denominator;
    if numerator % denominator == U256::ZERO {
        Ok(quotient)
    } else {
        quotient.checked_add(U256::ONE).ok_or(ArithmeticError::Overflow)
    }
}

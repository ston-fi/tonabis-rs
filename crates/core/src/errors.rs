//! Typed errors for shared checked operations.

/// Arithmetic failures retain the distinction between invalid divisors and range errors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum ArithmeticError {
    /// The divisor is zero.
    #[error("division by zero")]
    DivisionByZero,
    /// The result does not fit the return type.
    #[error("arithmetic result exceeds U256")]
    Overflow,
}

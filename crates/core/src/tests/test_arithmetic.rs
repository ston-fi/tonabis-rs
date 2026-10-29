use crate::{
    arithmetic::{div_ceil, mul_div_ceil, mul_div_floor},
    errors::ArithmeticError,
};
use fastnum::U256;
use fastnum::bint::ParseError;
use num_bigint::BigUint;
use std::str::FromStr;

#[test]
fn test_wide_product_and_rounding_boundaries() -> anyhow::Result<()> {
    let max = U256::MAX;
    assert_eq!(mul_div_floor(max, max, max)?, max);
    assert_eq!(mul_div_ceil(max, max, max)?, max);
    assert_eq!(div_ceil(max, U256::TWO)?, (max / U256::TWO) + U256::ONE);
    assert_eq!(mul_div_floor(max, max, U256::ONE), Err(ArithmeticError::Overflow));
    // The floor fits exactly, but rounding up must reject 2^256.
    assert_eq!(mul_div_floor(max - U256::ONE, max - U256::ONE, max - U256::TWO)?, max);
    assert_eq!(
        mul_div_ceil(max - U256::ONE, max - U256::ONE, max - U256::TWO),
        Err(ArithmeticError::Overflow)
    );
    {
        let divisor = U256::ZERO;
        assert_eq!(
            mul_div_floor(U256::ZERO, U256::ONE, divisor),
            Err(ArithmeticError::DivisionByZero)
        );
        assert_eq!(
            mul_div_ceil(U256::ONE, U256::ONE, divisor),
            Err(ArithmeticError::DivisionByZero)
        );
        assert_eq!(div_ceil(U256::ONE, divisor), Err(ArithmeticError::DivisionByZero));
    }
    Ok(())
}

#[test]
fn test_small_domain_exhaustive_rounding() -> anyhow::Result<()> {
    for lhs in 0u64..32 {
        for rhs in 0u64..32 {
            for divisor in 1u64..32 {
                let product = lhs * rhs;
                assert_eq!(
                    mul_div_floor(U256::from(lhs), U256::from(rhs), U256::from(divisor))?,
                    U256::from(product / divisor)
                );
                assert_eq!(
                    mul_div_ceil(U256::from(lhs), U256::from(rhs), U256::from(divisor))?,
                    U256::from(product.div_ceil(divisor))
                );
            }
        }
    }
    Ok(())
}

#[test]
fn test_fixed_seed_against_unbounded_integer_reference() -> anyhow::Result<()> {
    let mut seed = 0xb1d_2026_1026_u64;
    for _ in 0..512 {
        let mut next = || {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            seed
        };
        let limbs: Vec<_> = (0..12).map(|_| next()).collect();
        let value = |offset: usize| -> Result<U256, ParseError> {
            U256::from_str_radix(
                &format!(
                    "{:016x}{:016x}{:016x}{:016x}",
                    limbs[offset],
                    limbs[offset + 1],
                    limbs[offset + 2],
                    limbs[offset + 3]
                ),
                16,
            )
        };
        let lhs = value(0)?;
        let rhs = value(4)?;
        let divisor = value(8)?.max(U256::ONE);
        let product = BigUint::from_str(&lhs.to_string())? * BigUint::from_str(&rhs.to_string())?;
        let d = BigUint::from_str(&divisor.to_string())?;
        let floor = &product / &d;
        let ceil = (&product + &d - BigUint::from(1u8)) / &d;
        let max = BigUint::from_str(&U256::MAX.to_string())?;
        for (actual, expected) in [
            (mul_div_floor(lhs, rhs, divisor), floor),
            (mul_div_ceil(lhs, rhs, divisor), ceil),
        ] {
            if expected > max {
                assert_eq!(actual, Err(ArithmeticError::Overflow));
            } else {
                assert_eq!(actual?.to_string(), expected.to_string());
            }
        }
    }
    Ok(())
}

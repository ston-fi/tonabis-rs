//! Proportional integer share calculation; protocol-specific LP rules are caller-owned.
use fastnum::U256;
use tonabis_core::arithmetic::mul_div_floor;
use tonabis_core::errors::ArithmeticError;

fn main() -> Result<(), ArithmeticError> {
    let amount = U256::from(1_000u64);
    let shares = U256::from(3u64);
    let supply = U256::from(10u64);
    println!("{}", mul_div_floor(amount, shares, supply)?);
    Ok(())
}

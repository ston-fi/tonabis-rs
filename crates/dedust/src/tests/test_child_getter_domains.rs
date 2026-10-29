//! Stored unsigned wire quantities and computed signed outputs are distinct.
use crate::contracts::cpmm_v2::{deposit, position};
use fastnum::{I512, U256};
use ton::ton_core::types::Coins;
use ton::{
    block_tlb::{FromTVMStack, TVMInt, TVMNull, TVMStack, TVMStackValue},
    ton_core::{traits::tlb::TLB, types::tlb_core::MsgAddress},
};

fn position_stack() -> anyhow::Result<TVMStack> {
    let mut stack = TVMStack::EMPTY;
    stack.push_cell_slice(MsgAddress::NONE.to_cell()?);
    stack.push_cell_slice(MsgAddress::NONE.to_cell()?);
    stack.push_tiny_int(19);
    stack.push_tiny_int(5);
    stack.push(TVMStackValue::Null(TVMNull));
    stack.push_tiny_int(7);
    stack.push_tiny_int(11);
    stack.push_int(I512::ONE << 120u32);
    stack.push_int(I512::ONE << 121u32);
    Ok(stack)
}
#[test]
fn test_position_wire_widths_do_not_restrict_computed_negative_available_fees() -> anyhow::Result<()> {
    let mut stack = position_stack()?;
    stack[7] = TVMStackValue::Int(TVMInt {
        value: I512::ONE << 248u32,
    });
    stack[8] = TVMStackValue::Int(TVMInt {
        value: I512::ONE << 255u32,
    });
    let data = position::method_results::DedustV2CpmmPositionDataResult::from_stack(&mut stack)?;
    assert_eq!(data.x_checkpoint, U256::ONE << 248u32);
    assert_eq!(data.y_checkpoint, U256::ONE << 255u32);
    assert_eq!(
        (data.total_liquidity, data.locked_liquidity, data.x_pending, data.y_pending),
        (Coins::new(19), Coins::new(5), Coins::new(7), Coins::new(11))
    );
    for (index, value) in [(8, -I512::ONE), (5, -I512::ONE), (2, I512::ONE << 128u32)] {
        let mut bad = position_stack()?;
        bad[index] = TVMStackValue::Int(TVMInt { value });
        assert!(position::method_results::DedustV2CpmmPositionDataResult::from_stack(&mut bad).is_err());
    }
    // Pinned library getter80110 at input checkpoints0/0 returns these negatives.
    let mut stack = TVMStack::EMPTY;
    stack.push_tiny_int(-12);
    stack.push_tiny_int(-27);
    let available = position::method_results::DedustV2CpmmPositionAvailableFees::from_stack(&mut stack)?;
    assert_eq!((available.x_fee, available.y_fee), (I512::from_i64(-12), I512::from_i64(-27)));
    Ok(())
}
fn deposit_stack() -> anyhow::Result<TVMStack> {
    let mut stack = TVMStack::EMPTY;
    for _ in 0..3 {
        stack.push_cell_slice(MsgAddress::NONE.to_cell()?);
    }
    for value in [100, 200, 65535, 17, 11, 23] {
        stack.push_tiny_int(value);
    }
    Ok(stack)
}
#[test]
fn test_deposit_getter_keeps_uint16_lock_share_and_rejects_non_coin_quantities() -> anyhow::Result<()> {
    let reply = deposit::method_results::DedustV2CpmmDepositDataResult::from_stack(&mut deposit_stack()?)?;
    assert_eq!(reply.locked_liquidity_share, 65535);
    assert_eq!(
        (reply.target_x, reply.target_y, reply.amount_x, reply.amount_y),
        (Coins::new(100), Coins::new(200), Coins::new(11), Coins::new(23))
    );
    for (index, value) in [(8, -I512::ONE), (3, I512::ONE << 128u32)] {
        let mut stack = deposit_stack()?;
        stack[index] = TVMStackValue::Int(TVMInt { value });
        assert!(deposit::method_results::DedustV2CpmmDepositDataResult::from_stack(&mut stack).is_err());
    }
    Ok(())
}

use crate::contracts::classic::common::method_results::DedustClassicNullableVersionedCode;
use crate::contracts::classic::lp_wallet::method_results::DedustClassicLpWalletWalletData;
use crate::contracts::uranus::meme_v3::method_results::DedustUranusMemeV3GetJettonDataResult;
use fastnum::I512;
use ton::{
    block_tlb::{FromTVMStack, TVMNull, TVMStack, TVMStackValue},
    ton_core::{
        cell::TonCell,
        traits::tlb::TLB,
        types::{Coins, TonAddress, tlb_core::MsgAddress},
    },
};

#[test]
fn test_wallet_code_getter_accepts_cell_and_slice() -> anyhow::Result<()> {
    let a = MsgAddress::Int(TonAddress::ZERO.to_msg_address_int()).to_cell()?;
    for kind in 0..3 {
        let mut stack = TVMStack::EMPTY;
        // Getter amounts use native Coins conversion, without a separate uint120 guard.
        stack.push_int(I512::ONE << 120u32);
        stack.push_cell_slice(MsgAddress::NONE.to_cell()?);
        stack.push_cell_slice(a.clone());
        match kind {
            0 => {
                stack.push_cell(TonCell::empty().clone());
            },
            1 => {
                stack.push_cell_slice(TonCell::empty().clone());
            },
            _ => {
                stack.push(TVMStackValue::Null(TVMNull));
            },
        }
        let result = DedustClassicLpWalletWalletData::from_stack(&mut stack);
        if kind < 2 {
            let decoded = result?;
            assert_eq!(decoded.balance, Coins::new(1u128 << 120));
            assert_eq!(decoded.owner, None);
            assert_eq!(
                decoded.pool,
                tonabis_core::adapters::nullable_internal_address::NullableInternalAddress::from_address(
                    MsgAddress::Int(TonAddress::ZERO.to_msg_address_int())
                )?
            );
            assert_eq!(decoded.wallet_code, *TonCell::empty());
        } else {
            assert!(result.is_err(), "required cell field accepted NULL");
        }
    }
    Ok(())
}
#[test]
fn test_jetton_admin_address_preserves_zero_with_suffix() -> anyhow::Result<()> {
    let a = MsgAddress::Int(TonAddress::ZERO.to_msg_address_int()).to_cell()?;
    for malformed in [false, true] {
        let mut b = TonCell::builder();
        b.write_cell(&a)?;
        if malformed {
            b.write_bit(true)?;
        }
        let mut stack = TVMStack::EMPTY;
        stack.push_tiny_int(1);
        stack.push_tiny_int(-1);
        stack.push_cell_slice(b.build()?);
        stack.push_cell(TonCell::empty().clone());
        stack.push_cell(TonCell::empty().clone());
        let result = DedustUranusMemeV3GetJettonDataResult::from_stack(&mut stack);
        assert_eq!(
            result?.admin_address,
            tonabis_core::adapters::nullable_internal_address::NullableInternalAddress::from_address(MsgAddress::Int(
                TonAddress::ZERO.to_msg_address_int()
            ))?
        );
    }
    Ok(())
}

#[test]
fn test_nullable_code_preserves_null_and_accepts_slice() -> anyhow::Result<()> {
    for kind in 0..3 {
        let mut stack = TVMStack::EMPTY;
        stack.push_tiny_int(0);
        match kind {
            0 => {
                stack.push(TVMStackValue::Null(TVMNull));
            },
            1 => {
                stack.push_cell(TonCell::empty().clone());
            },
            _ => {
                stack.push_cell_slice(TonCell::empty().clone());
            },
        }
        let result = DedustClassicNullableVersionedCode::from_stack(&mut stack);
        assert_eq!(result?.code.is_some(), kind != 0);
    }
    Ok(())
}

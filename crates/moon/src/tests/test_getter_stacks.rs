use crate::contracts::{legacy_pool, order_new, order_old};
use ton::{
    block_tlb::{FromTVMStack, TVMNull, TVMStack, TVMStackValue},
    ton_core::{cell::TonCell, traits::tlb::TLB, types::tlb_core::MsgAddress},
};
#[test]
fn test_all_catalog_getter_stack_layouts() -> anyhow::Result<()> {
    {
        let mut stack = TVMStack::default();
        stack.push_cell_slice(MsgAddress::NONE.to_cell()?);
        stack.push_tiny_int(4);
        stack.push_cell_slice(MsgAddress::NONE.to_cell()?);
        stack.push_tiny_int(6);
        let mut extra = stack.clone();
        extra.insert(0, TVMStackValue::Null(TVMNull));
        let value = legacy_pool::method_results::MoonLegacyPoolGetReservesResult::from_stack(&mut stack)?;
        assert_eq!(value.asset1_id, None);
        assert_eq!(value.asset1_reserve.to_string(), "4");
        assert_eq!(value.asset2_id, None);
        assert_eq!(value.asset2_reserve.to_string(), "6");
        assert!(legacy_pool::method_results::MoonLegacyPoolGetReservesResult::from_stack(&mut extra).is_err());
    }
    {
        let mut stack = TVMStack::default();
        stack.push_cell_slice(MsgAddress::NONE.to_cell()?);
        stack.push_cell_slice(MsgAddress::NONE.to_cell()?);
        let mut extra = stack.clone();
        extra.insert(0, TVMStackValue::Null(TVMNull));
        let value = legacy_pool::method_results::MoonLegacyPoolGetAssetsResult::from_stack(&mut stack)?;
        assert_eq!(value.asset1_id, None);
        assert_eq!(value.asset2_id, None);
        assert!(legacy_pool::method_results::MoonLegacyPoolGetAssetsResult::from_stack(&mut extra).is_err());
    }
    {
        let mut stack = TVMStack::default();
        stack.push_cell_slice(MsgAddress::NONE.to_cell()?);
        stack.push_cell_slice(MsgAddress::NONE.to_cell()?);
        let mut extra = stack.clone();
        extra.insert(0, TVMStackValue::Null(TVMNull));
        let value = legacy_pool::method_results::MoonLegacyPoolGetWalletsResult::from_stack(&mut stack)?;
        assert_eq!(value.asset1_wallet, None);
        assert_eq!(value.asset2_wallet, None);
        assert!(legacy_pool::method_results::MoonLegacyPoolGetWalletsResult::from_stack(&mut extra).is_err());
    }
    {
        let mut stack = TVMStack::default();
        stack.push_tiny_int(3);
        stack.push_tiny_int(4);
        stack.push_tiny_int(5);
        let mut extra = stack.clone();
        extra.insert(0, TVMStackValue::Null(TVMNull));
        let value = legacy_pool::method_results::MoonLegacyPoolGetFeesResult::from_stack(&mut stack)?;
        assert_eq!(value.swap_fee_bps.to_string(), "3");
        assert_eq!(value.protocol_fee_bps.to_string(), "4");
        assert_eq!(value.referral_fee_bps.to_string(), "5");
        assert!(legacy_pool::method_results::MoonLegacyPoolGetFeesResult::from_stack(&mut extra).is_err());
    }
    {
        let mut stack = TVMStack::default();
        stack.push_tiny_int(3);
        stack.push_tiny_int(4);
        let mut extra = stack.clone();
        extra.insert(0, TVMStackValue::Null(TVMNull));
        let value = legacy_pool::method_results::MoonLegacyPoolGetCollectedFeesResult::from_stack(&mut stack)?;
        assert_eq!(value.asset1_fees.to_string(), "3");
        assert_eq!(value.asset2_fees.to_string(), "4");
        assert!(legacy_pool::method_results::MoonLegacyPoolGetCollectedFeesResult::from_stack(&mut extra).is_err());
    }
    {
        let mut stack = TVMStack::default();
        stack.push_cell_slice(MsgAddress::NONE.to_cell()?);
        stack.push_tiny_int(4);
        stack.push_cell_slice(MsgAddress::NONE.to_cell()?);
        stack.push_tiny_int(6);
        let mut extra = stack.clone();
        extra.insert(0, TVMStackValue::Null(TVMNull));
        let value = legacy_pool::method_results::MoonLegacyPoolGetEstimatedLpBurnResult::from_stack(&mut stack)?;
        assert_eq!(value.asset1_id, None);
        assert_eq!(value.asset1_amount.to_string(), "4");
        assert_eq!(value.asset2_id, None);
        assert_eq!(value.asset2_amount.to_string(), "6");
        assert!(legacy_pool::method_results::MoonLegacyPoolGetEstimatedLpBurnResult::from_stack(&mut extra).is_err());
    }
    {
        let mut stack = TVMStack::default();
        stack.push_cell_slice(MsgAddress::NONE.to_cell()?);
        stack.push_tiny_int(4);
        let mut extra = stack.clone();
        extra.insert(0, TVMStackValue::Null(TVMNull));
        let value = legacy_pool::method_results::MoonLegacyPoolGetEstimatedSwapOutResult::from_stack(&mut stack)?;
        assert_eq!(value.output_asset_id, None);
        assert_eq!(value.output_amount.to_string(), "4");
        assert!(legacy_pool::method_results::MoonLegacyPoolGetEstimatedSwapOutResult::from_stack(&mut extra).is_err());
    }
    {
        let mut stack = TVMStack::default();
        stack.push_tiny_int(3);
        stack.push_tiny_int(-1);
        stack.push_cell_slice(MsgAddress::NONE.to_cell()?);
        stack.push_cell(TonCell::empty().clone());
        stack.push_cell(TonCell::empty().clone());
        let mut extra = stack.clone();
        extra.insert(0, TVMStackValue::Null(TVMNull));
        let value = legacy_pool::method_results::MoonLegacyPoolGetJettonDataResult::from_stack(&mut stack)?;
        assert_eq!(value.total_supply.to_string(), "3");
        assert!(value.mintable);
        assert_eq!(value.admin_address, None);
        assert_eq!(value.jetton_content, *TonCell::empty());
        assert_eq!(value.jetton_wallet_code, *TonCell::empty());
        assert!(legacy_pool::method_results::MoonLegacyPoolGetJettonDataResult::from_stack(&mut extra).is_err());
    }
    {
        let mut stack = TVMStack::default();
        stack.push_tiny_int(3);
        stack.push_tiny_int(4);
        stack.push_cell_slice(MsgAddress::NONE.to_cell()?);
        let mut extra = stack.clone();
        extra.insert(0, TVMStackValue::Null(TVMNull));
        let value = order_old::method_results::MoonOrderOldGetVestingDataResult::from_stack(&mut stack)?;
        assert_eq!(value.vesting_start_time.to_string(), "3");
        assert_eq!(value.vesting_duration.to_string(), "4");
        assert_eq!(value.recipient, None);
        assert!(order_old::method_results::MoonOrderOldGetVestingDataResult::from_stack(&mut extra).is_err());
    }
    {
        let mut stack = TVMStack::default();
        stack.push_tiny_int(3);
        stack.push_tiny_int(4);
        stack.push_cell_slice(MsgAddress::NONE.to_cell()?);
        let mut extra = stack.clone();
        extra.insert(0, TVMStackValue::Null(TVMNull));
        let value = order_new::method_results::MoonOrderNewGetVestingDataResult::from_stack(&mut stack)?;
        assert_eq!(value.vesting_start_time.to_string(), "3");
        assert_eq!(value.vesting_duration.to_string(), "4");
        assert_eq!(value.recipient, None);
        assert!(order_new::method_results::MoonOrderNewGetVestingDataResult::from_stack(&mut extra).is_err());
    }
    Ok(())
}

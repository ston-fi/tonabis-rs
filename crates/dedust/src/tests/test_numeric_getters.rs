#[test]
fn test_source_integer_getters_preserve_signed_valuation_and_coin_domain() -> anyhow::Result<()> {
    use super::test_getter_provider::{fixture_arguments, recording_client};
    use crate::contracts::{classic::vault_native_v2, cpmm_v2::pool};
    use fastnum::I512;
    use ton::{
        block_tlb::{FromTVMStack, TVMStack, ToTVMStack},
        contracts::TonContract,
        ton_core::types::TonAddress,
    };

    let mut stack = TVMStack::EMPTY;
    stack.push_tiny_int(-1);
    stack.push_tiny_int(-1);
    let value = pool::method_results::DedustV2CpmmPoolGetLiquidityAmountsResult::from_stack(&mut stack)?;
    assert_eq!((value.amount_x, value.amount_y), (-I512::ONE, -I512::ONE));
    let mut stack = TVMStack::EMPTY;
    (-(I512::ONE << 256u32)).to_stack(&mut stack)?;
    assert_eq!(
        tokio_test::block_on(
            vault_native_v2::DedustVaultNativeV2Contract::new(
                &recording_client(109505, fixture_arguments(&["17"])?, stack)?,
                &TonAddress::ZERO,
                None
            )
            .estimate_gas_one_arg(17.into())
        )?,
        -(I512::ONE << 256u32)
    );
    let mut stack = TVMStack::EMPTY;
    (I512::ONE << 128u32).to_stack(&mut stack)?;
    assert!(
        tokio_test::block_on(
            vault_native_v2::DedustVaultNativeV2Contract::new(
                &recording_client(109867, TVMStack::EMPTY, stack)?,
                &TonAddress::ZERO,
                None
            )
            .get_tracked_native_balance()
        )
        .is_err()
    );
    Ok(())
}

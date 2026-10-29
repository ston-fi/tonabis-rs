use super::test_getter_provider::recording_client;
use crate::contracts::classic::{operator_pool, pool_v2};
use ton::{
    block_tlb::TVMStack,
    contracts::TonContract,
    ton_core::{cell::TonCell, traits::tlb::TLB, types::TonAddress},
};

#[test]
fn test_managed_tail_preserves_absent_and_zero_ordinary_pool_accepts_prefix() -> anyhow::Result<()> {
    for (absent, zero) in [
        (
            include_bytes!("fixtures/operator-a-absent.boc").as_slice(),
            include_bytes!("fixtures/operator-a-zero.boc").as_slice(),
        ),
        (
            include_bytes!("fixtures/operator-b-absent.boc").as_slice(),
            include_bytes!("fixtures/operator-b-zero.boc").as_slice(),
        ),
    ] {
        let absent = TonCell::from_boc(absent.to_vec())?;
        let zero = TonCell::from_boc(zero.to_vec())?;
        let a: operator_pool::storage::DedustClassicOperatorPoolStorage = TLB::from_cell(&absent)?;
        let z: operator_pool::storage::DedustClassicOperatorPoolStorage = TLB::from_cell(&zero)?;
        assert_eq!(a.swap_active_since, None);
        assert_eq!(z.swap_active_since, Some(0));
        assert_eq!(a.to_cell()?.hash()?, absent.hash()?);
        assert_eq!(z.to_cell()?.hash()?, zero.hash()?);
        assert_eq!(pool_v2::storage::DedustPoolV2Storage::from_cell(&zero)?, z.pool);
    }
    let mut stack = TVMStack::EMPTY;
    stack.push_tiny_int(1100);
    assert_eq!(
        tokio_test::block_on(
            operator_pool::DedustClassicOperatorPoolContract::new(
                &recording_client(112861, TVMStack::EMPTY, stack)?,
                &TonAddress::ZERO,
                None
            )
            .get_start_at()
        )?,
        1100
    );
    Ok(())
}

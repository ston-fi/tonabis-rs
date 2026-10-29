//! Independent encodings for source-defined untagged suffixes.
use crate::contracts::{farming, v1, v2};
use fastnum::U256;
use std::fmt::Debug;
use ton::ton_core::{
    cell::TonCell,
    traits::tlb::TLB,
    types::tlb_core::{TLBCoins, TLBRef},
};

fn check<T: TLB + PartialEq + Debug>(value: &T, wire: &TonCell) -> anyhow::Result<()> {
    assert_eq!(value.to_cell()?, *wire);
    assert_eq!(*value, T::from_cell(wire)?);
    let mut extra = TonCell::builder();
    extra.write_cell(wire)?;
    extra.write_bit(true)?;
    assert!(
        T::from_cell(&extra.build()?).is_err(),
        "explicit suffix must reject an incomplete/extra field"
    );
    Ok(())
}

#[test]
fn test_untagged_sequence_and_legacy_claim_index() -> anyhow::Result<()> {
    use farming::msgs::ToncoFarmingLegacyIndex;
    use v2::router::storage::ToncoRouterV2OptionalU64;
    check(&ToncoRouterV2OptionalU64(None), TonCell::empty())?;
    let mut seq = TonCell::builder();
    seq.write_num(&17u64, 64)?;
    check(&ToncoRouterV2OptionalU64(Some(17)), &seq.build()?)?;
    check(&ToncoFarmingLegacyIndex(None), TonCell::empty())?;
    let mut index = TonCell::builder();
    index.write_num(&U256::MAX, 256)?;
    check(&ToncoFarmingLegacyIndex(Some(U256::MAX)), &index.build()?)?;
    Ok(())
}

#[test]
fn test_pool_and_router_timelocks_initial_and_pending_forms() -> anyhow::Result<()> {
    use v2::{pool::storage as pool, router::storage as router};
    let mut initial = TonCell::builder();
    initial.write_num(&17u64, 64)?;
    let initial = initial.build()?;
    check(
        &pool::ToncoPoolV2Timelock {
            delay: 17,
            pending: None,
        },
        &initial,
    )?;
    check(
        &router::ToncoRouterV2Timelock {
            delay: 17,
            pending: None,
        },
        &initial,
    )?;
    let mut p = TonCell::builder();
    p.write_cell(&initial)?;
    p.write_num(&1u64, 64)?;
    p.write_bit(false)?;
    p.write_num(&2u64, 64)?;
    p.write_num(&0u8, 2)?;
    p.write_num(&3u64, 64)?;
    p.write_num(&0u8, 2)?;
    p.write_num(&4u64, 64)?;
    p.write_num(&5u64, 64)?;
    check(
        &pool::ToncoPoolV2Timelock {
            delay: 17,
            pending: Some(pool::ToncoPoolV2PendingUpdates {
                code_time: 1,
                code: None,
                alm_time: 2,
                alm: None,
                arbiter_time: 3,
                arbiter: None,
                flags_time: 4,
                flags: 5,
            }),
        },
        &p.build()?,
    )?;
    let mut r = TonCell::builder();
    r.write_cell(&initial)?;
    r.write_num(&1u64, 64)?;
    r.write_num(&0u8, 2)?;
    r.write_num(&2u64, 64)?;
    r.write_num(&0u8, 2)?;
    r.write_num(&3u64, 64)?;
    r.write_num(&4u64, 64)?;
    r.write_num(&5u64, 64)?;
    r.write_bit(false)?;
    r.write_num(&6u64, 64)?;
    r.write_bit(false)?;
    check(
        &router::ToncoRouterV2Timelock {
            delay: 17,
            pending: Some(router::ToncoRouterV2PendingUpdates {
                admin_time: 1,
                admin: None,
                proxy_time: 2,
                proxy: None,
                flags_time: 3,
                flags: 4,
                code_time: 5,
                code: None,
                pool_code_time: 6,
                pool_code: None,
            }),
        },
        &r.build()?,
    )?;
    Ok(())
}

#[test]
fn test_router_untagged_role_references() -> anyhow::Result<()> {
    use v2::router::msgs::{ToncoRouterV2OracleRole, ToncoRouterV2RoleRefs, ToncoRouterV2SecondaryRoles};
    check(&ToncoRouterV2RoleRefs::None, TonCell::empty())?;
    let mut secondary = TonCell::builder();
    secondary.write_bit(false)?;
    secondary.write_num(&0u8, 2)?;
    secondary.write_bit(true)?;
    secondary.write_num(&0u8, 2)?;
    let secondary = secondary.build()?;
    let mut oracle = TonCell::builder();
    oracle.write_bit(true)?;
    oracle.write_num(&0u8, 2)?;
    let oracle = oracle.build()?;
    let s = TLBRef::new(ToncoRouterV2SecondaryRoles {
        has_arbiter: false,
        arbiter: None,
        has_alm: true,
        alm: None,
    });
    let o = TLBRef::new(ToncoRouterV2OracleRole {
        has_oracle: true,
        oracle: None,
    });
    let mut one = TonCell::builder();
    one.write_ref(secondary.clone())?;
    check(&ToncoRouterV2RoleRefs::Secondary(s.clone()), &one.build()?)?;
    let mut two = TonCell::builder();
    two.write_ref(secondary)?;
    two.write_ref(oracle)?;
    check(
        &ToncoRouterV2RoleRefs::Both {
            secondary: s,
            oracle: o,
        },
        &two.build()?,
    )?;
    Ok(())
}

#[test]
fn test_factory_extra_settings_preserve_ignored_sdk_tag() -> anyhow::Result<()> {
    use v2::factory::msgs::{ToncoFactoryV2CreateSettings, ToncoFactoryV2ExtraSettingsSuffix};
    let mut settings = TonCell::builder();
    settings.write_num(&30u16, 16)?;
    settings.write_num(&10u32, 24)?;
    settings.write_bit(true)?;
    settings.write_bit(false)?;
    let settings = settings.build()?;
    for tag in [None, Some(false), Some(true)] {
        for present in [false, true] {
            let mut b = TonCell::builder();
            if let Some(t) = tag {
                b.write_bit(t)?;
            }
            if present {
                b.write_ref(settings.clone())?;
            }
            let value = ToncoFactoryV2ExtraSettingsSuffix {
                sdk_tag: tag,
                settings: present.then(|| {
                    TLBRef::new(ToncoFactoryV2CreateSettings {
                        fee: 30,
                        spacing: 10,
                        active: true,
                        content: None,
                    })
                }),
            };
            // A missing tag followed by a bit is itself another supported SDK form.
            let wire = b.build()?;
            assert_eq!(value.to_cell()?, wire);
            assert_eq!(ToncoFactoryV2ExtraSettingsSuffix::from_cell(&wire)?, value);
        }
    }
    let mut malformed = TonCell::builder();
    malformed.write_num(&0u8, 2)?;
    assert!(ToncoFactoryV2ExtraSettingsSuffix::from_cell(&malformed.build()?).is_err());
    Ok(())
}

#[test]
fn test_funding_order_collect_target_and_nft_indexer_suffixes() -> anyhow::Result<()> {
    use v2::{
        account::msgs::ToncoAccountV2FundingOrderSuffix,
        msgs::{ToncoV2MintOrders, ToncoV2OrderBody},
        pool::msgs::{ToncoPoolV2CollectProtocolSuffix, ToncoPoolV2CollectProtocolTarget},
        position_nft::msgs::{ToncoPositionNftV2InitLog, ToncoPositionNftV2InitLogSuffix},
    };
    check(&ToncoAccountV2FundingOrderSuffix(None), TonCell::empty())?;
    let mut order = TonCell::builder();
    order.write_num(&0u8, 8)?;
    order.write_num(&2u64, 64)?;
    order.write_num(&3u8, 4)?;
    order.write_num(&4u32, 32)?;
    order.write_num(&0u8, 4)?;
    check(
        &ToncoAccountV2FundingOrderSuffix(Some(ToncoV2OrderBody {
            enough0: TLBCoins::new(0),
            enough1: TLBCoins::new(0),
            positions_needed: 2,
            passthrough: 3,
            action: 4,
            orders: ToncoV2MintOrders {
                slot0: None,
                slot1: None,
                slot2: None,
                slot3: None,
            },
        })),
        &order.build()?,
    )?;
    check(&ToncoPoolV2CollectProtocolSuffix(None), TonCell::empty())?;
    let mut target = TonCell::builder();
    target.write_num(&0u8, 2)?;
    target.write_num(&0u8, 8)?;
    check(
        &ToncoPoolV2CollectProtocolSuffix(Some(ToncoPoolV2CollectProtocolTarget {
            target: None,
            amount0: TLBCoins::new(0),
            amount1: TLBCoins::new(0),
        })),
        &target.build()?,
    )?;
    check(&ToncoPositionNftV2InitLogSuffix(None), TonCell::empty())?;
    let mut log = TonCell::builder();
    log.write_num(&1u64, 64)?;
    log.write_num(&2u64, 64)?;
    log.write_num(&0u8, 8)?;
    log.write_num(&-3i32, 24)?;
    log.write_num(&U256::ONE, 160)?;
    let mut body = TonCell::builder();
    body.write_ref(log.build()?)?;
    check(
        &ToncoPositionNftV2InitLogSuffix(Some(TLBRef::new(ToncoPositionNftV2InitLog {
            seqno: 1,
            index: 2,
            amount0: TLBCoins::new(0),
            amount1: TLBCoins::new(0),
            tick: -3,
            price: U256::ONE,
        }))),
        &body.build()?,
    )?;
    Ok(())
}

#[test]
fn test_v1_untagged_payout_forwarding_reference() -> anyhow::Result<()> {
    use v1::router::msgs::{ToncoRouterV1PayToPayloadCell, ToncoRouterV1PayloadSuffix};
    check(&ToncoRouterV1PayloadSuffix(None), TonCell::empty())?;
    let mut payload = TonCell::builder();
    payload.write_num(&0u8, 4)?;
    payload.write_bit(false)?;
    payload.write_num(&0u8, 4)?;
    payload.write_bit(false)?;
    let mut body = TonCell::builder();
    body.write_ref(payload.build()?)?;
    check(
        &ToncoRouterV1PayloadSuffix(Some(TLBRef::new(ToncoRouterV1PayToPayloadCell {
            payload_amount0: TLBCoins::new(0),
            payload_cell0: None,
            payload_amount1: TLBCoins::new(0),
            payload_cell1: None,
        }))),
        &body.build()?,
    )?;
    Ok(())
}

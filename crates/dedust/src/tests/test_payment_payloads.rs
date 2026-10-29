use crate::contracts::{
    classic::{vault_jetton_v1, vault_jetton_v2},
    cpmm_v2::pool::{self, msgs as payment},
};
use ton::ton_core::{cell::TonCell, traits::tlb::TLB, types::tlb_core::EitherRefLayout};

fn cell(bytes: &[u8]) -> anyhow::Result<TonCell> {
    Ok(TonCell::from_boc(bytes.to_vec())?)
}
fn check(payload: &payment::DedustV2CpmmPoolPayloadMsg, id: usize) -> anyhow::Result<()> {
    match (payload, id) {
        (payment::DedustV2CpmmPoolPayloadMsg::Swap(v), 0) => {
            assert_eq!((*v.minimal_amount_out, v.deadline), (17, 1000));
            assert!(v.next.is_none() && v.partner_config.is_none() && v.referrer_config.is_none());
        },
        (payment::DedustV2CpmmPoolPayloadMsg::Deposit(v), 1) => assert_eq!(
            (*v.amount_x, *v.amount_y, *v.minimal_liquidity, v.locked_liquidity_share),
            (100, 200, 17, 5000)
        ),
        (payment::DedustV2CpmmPoolPayloadMsg::FundReward(v), 2) => assert_eq!(v.till, 1000),
        _ => anyhow::bail!("wrong finite operation"),
    }
    Ok(())
}
#[test]
fn test_fixed_cpmm_payment_payload_is_typed_through_parent_and_both_either_forms() -> anyhow::Result<()> {
    for (id, bytes) in [
        include_bytes!("fixtures/cpmm-payment-0.boc").as_slice(),
        include_bytes!("fixtures/cpmm-payment-1.boc").as_slice(),
        include_bytes!("fixtures/cpmm-payment-2.boc").as_slice(),
    ]
    .into_iter()
    .enumerate()
    {
        let wire = cell(bytes)?;
        let native = pool::msgs::DedustV2CpmmPayNativeMsg::from_cell(&wire)?;
        check(&native.payment_payload, id)?;
        assert_eq!((native.query_id, *native.amount), (7, 123));
        assert_eq!(native.to_cell()?.hash()?, wire.hash()?);
    }
    for (id, inline, referenced) in [
        (
            0,
            include_bytes!("fixtures/cpmm-notification-0-inline.boc").as_slice(),
            include_bytes!("fixtures/cpmm-notification-0-ref.boc").as_slice(),
        ),
        (
            1,
            include_bytes!("fixtures/cpmm-notification-1-inline.boc").as_slice(),
            include_bytes!("fixtures/cpmm-notification-1-ref.boc").as_slice(),
        ),
        (
            2,
            include_bytes!("fixtures/cpmm-notification-2-inline.boc").as_slice(),
            include_bytes!("fixtures/cpmm-notification-2-ref.boc").as_slice(),
        ),
    ] {
        for (bytes, layout) in [(inline, EitherRefLayout::ToCell), (referenced, EitherRefLayout::ToRef)] {
            let wire = cell(bytes)?;
            let notification = pool::msgs::DedustV2CpmmTransferNotificationMsg::from_cell(&wire)?;
            assert_eq!(notification.forward_payload.layout, layout);
            let payment::DedustV2CpmmPoolJettonPayloadMsg::Payment(frame) = &notification.forward_payload.value else {
                anyhow::bail!("known payment frame opaque")
            };
            check(&frame.payment_payload, id)?;
            assert_eq!(notification.to_cell()?.hash()?, wire.hash()?);
        }
    }
    Ok(())
}
#[test]
fn test_payment_unknown_preserved_known_malformed_is_error() -> anyhow::Result<()> {
    let wire = cell(include_bytes!("fixtures/cpmm-payment-3.boc"))?;
    let unknown = pool::msgs::DedustV2CpmmPayNativeMsg::from_cell(&wire)?;
    assert!(
        matches!(&*unknown.payment_payload,payment::DedustV2CpmmPoolPayloadMsg::Unknown(c)if c.parser().read_num::<u32>(32)?==0x12345678)
    );
    assert_eq!(unknown.to_cell()?.hash()?, wire.hash()?);
    assert!(
        pool::msgs::DedustV2CpmmPayNativeMsg::from_cell(&cell(include_bytes!("fixtures/cpmm-payment-4.boc"))?).is_err()
    );
    Ok(())
}
#[test]
fn test_classic_forward_records_preserve_generation_and_reference_form() -> anyhow::Result<()> {
    for (id, inline, referenced) in [
        (
            0,
            include_bytes!("fixtures/classic-forward-0-inline.boc").as_slice(),
            include_bytes!("fixtures/classic-forward-0-ref.boc").as_slice(),
        ),
        (
            1,
            include_bytes!("fixtures/classic-forward-1-inline.boc").as_slice(),
            include_bytes!("fixtures/classic-forward-1-ref.boc").as_slice(),
        ),
    ] {
        for (bytes, layout) in [(inline, EitherRefLayout::ToCell), (referenced, EitherRefLayout::ToRef)] {
            let wire = cell(bytes)?;
            let one = vault_jetton_v1::msgs::DedustVaultJettonV1TransferNotificationMsg::from_cell(&wire)?;
            let two = vault_jetton_v2::msgs::DedustVaultJettonV2TransferNotificationMsg::from_cell(&wire)?;
            assert_eq!(one.forward_payload.layout, layout);
            assert_eq!(two.forward_payload.layout, layout);
            match (&one.forward_payload.value, &two.forward_payload.value, id) {
                (
                    vault_jetton_v1::msgs::DedustVaultJettonV1ForwardMsg::Swap(a),
                    vault_jetton_v2::msgs::DedustVaultJettonV2ForwardMsg::Swap(b),
                    0,
                ) => {
                    assert_eq!(
                        (
                            *a.step.params.limit,
                            a.swap_params.deadline,
                            *b.step.params.limit,
                            b.swap_params.deadline
                        ),
                        (17, 1000, 17, 1000)
                    );
                },
                (
                    vault_jetton_v1::msgs::DedustVaultJettonV1ForwardMsg::DepositLiquidity(a),
                    vault_jetton_v2::msgs::DedustVaultJettonV2ForwardMsg::DepositLiquidity(b),
                    1,
                ) => assert_eq!(
                    (
                        *a.min_lp_amount,
                        *a.asset0_target_balance,
                        *a.asset1_target_balance,
                        *b.min_lp_amount,
                        *b.asset0_target_balance,
                        *b.asset1_target_balance
                    ),
                    (17, 100, 200, 17, 100, 200)
                ),
                _ => anyhow::bail!("known classic forward command opaque"),
            }
            assert_eq!(one.to_cell()?.hash()?, wire.hash()?);
            assert_eq!(two.to_cell()?.hash()?, wire.hash()?);
        }
    }
    Ok(())
}

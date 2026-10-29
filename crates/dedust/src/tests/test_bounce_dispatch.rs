use crate::contracts::{classic::common::msgs::DedustLiquidityDepositBounceMsg as Bounce, operator::v2};
use ton::ton_core::{cell::TonCell, traits::tlb::TLB};

#[test]
fn test_classic_processing_bounce_known_prefix_and_ignored_suffix() -> anyhow::Result<()> {
    for bytes in [
        include_bytes!("fixtures/classic-deposit-v1-bounce.boc").as_slice(),
        include_bytes!("fixtures/classic-deposit-v2-bounce.boc").as_slice(),
    ] {
        let cell = TonCell::from_boc(bytes.to_vec())?;
        let bounce = Bounce::from_cell(&cell)?;
        assert!(
            matches!(&bounce,Bounce::DepositLiquidityAll{prefix:u32::MAX,query_id:99,ignored_suffix}if ignored_suffix.data_len_bits()==3&&ignored_suffix.parser().read_num::<u8>(3)?==5)
        );
        assert_eq!(bounce.to_cell()?.hash()?, cell.hash()?);
        let mut truncated = TonCell::builder();
        truncated.write_num(&u32::MAX, 32)?;
        truncated.write_num(&0xb56b9598u32, 32)?;
        assert!(Bounce::from_cell(&truncated.build()?).is_err());
    }
    let mut unknown = TonCell::builder();
    unknown.write_num(&u32::MAX, 32)?;
    unknown.write_num(&0x12345678u32, 32)?;
    let unknown = unknown.build()?;
    assert!(matches!(Bounce::from_cell(&unknown)?,Bounce::Unknown(c)if c.hash()?==unknown.hash()?));
    Ok(())
}
#[test]
fn test_operator_relay_observed_fixed_state_and_forwarded_command() -> anyhow::Result<()> {
    let parse = |bytes: &[u8]| -> anyhow::Result<v2::storage::DedustOperatorV2Storage> {
        Ok(v2::storage::DedustOperatorV2Storage::from_cell(&TonCell::from_boc(
            bytes.to_vec(),
        )?)?)
    };
    let before = parse(include_bytes!("fixtures/relay-forward-state.boc"))?;
    let user = parse(include_bytes!("fixtures/relay-user-state.boc"))?;
    let upgraded = parse(include_bytes!("fixtures/relay-upgraded-state.boc"))?;
    let bounced = parse(include_bytes!("fixtures/relay-bounce-state.boc"))?;
    assert_eq!((before.version, before.role, before.can_execute_after), (5, 10, 1100));
    assert_eq!((user.version, user.role, user.can_execute_after), (5, 10, 173800));
    assert_ne!(user.user, before.user);
    assert_eq!(user.factory, before.factory);
    assert_eq!(
        (upgraded.version, upgraded.user, upgraded.can_execute_after),
        (6, before.user, 1100)
    );
    assert_eq!(bounced.to_cell()?.hash()?, upgraded.to_cell()?.hash()?);
    let command = v2::msgs::DedustOperatorV2ConfigureStartTimeMsg::from_cell(&TonCell::from_boc(
        include_bytes!("fixtures/relay-forwarded.boc").to_vec(),
    )?)?;
    assert_eq!((command.query_id, command.start_at), (9, 1200));
    Ok(())
}
#[test]
fn test_lp_restore_bounce_source_skips_marker_and_ignores_suffix() -> anyhow::Result<()> {
    use crate::contracts::classic::lp_wallet::msgs::DedustLpWalletBounceMsg as Bounce;

    for (id, bytes) in [
        include_bytes!("fixtures/lp-bounce-0.boc").as_slice(),
        include_bytes!("fixtures/lp-bounce-1.boc").as_slice(),
        include_bytes!("fixtures/lp-bounce-2.boc").as_slice(),
    ]
    .into_iter()
    .enumerate()
    {
        let bounce = Bounce::from_cell(&TonCell::from_boc(bytes.to_vec())?)?;
        let (prefix, query_id, amount, suffix) = match bounce {
            Bounce::InternalTransfer {
                prefix,
                query_id,
                amount,
                suffix,
            } if id != 1 => (prefix, query_id, amount, suffix),
            Bounce::BurnNotification {
                prefix,
                query_id,
                amount,
                suffix,
            } if id == 1 => (prefix, query_id, amount, suffix),
            _ => anyhow::bail!("wrong restored operation"),
        };
        assert_eq!(prefix, if id == 2 { 0 } else { u32::MAX });
        assert_eq!((query_id, *amount), (99, 17));
        assert_eq!(suffix.parser().read_num::<u8>(3)?, 5);
    }
    Ok(())
}
#[test]
fn test_uranus_bounced_sell_transfer_burn_fixed_records() -> anyhow::Result<()> {
    use crate::contracts::uranus::wallet_msgs::{
        DedustUranusWalletBounceMsg as Bounce, DedustUranusWalletRestoredOperation,
    };

    for (id, bytes) in [
        include_bytes!("fixtures/uranus-wallet-bounce-0.boc").as_slice(),
        include_bytes!("fixtures/uranus-wallet-bounce-1.boc").as_slice(),
        include_bytes!("fixtures/uranus-wallet-bounce-2.boc").as_slice(),
        include_bytes!("fixtures/uranus-wallet-bounce-3.boc").as_slice(),
    ]
    .into_iter()
    .enumerate()
    {
        let cell = TonCell::from_boc(bytes.to_vec())?;
        let decoded = Bounce::from_cell(&cell)?;
        assert!(
            matches!(&decoded,Bounce::Restore{prefix,operation,query_id:99,amount,ignored_suffix}if *prefix==if id==3 {0}else{u32::MAX}&&*operation==match id {1=>DedustUranusWalletRestoredOperation::BurnNotification,2=>DedustUranusWalletRestoredOperation::SellTokens,_=>DedustUranusWalletRestoredOperation::InternalTransfer}&&**amount==17&&ignored_suffix.parser().read_num::<u8>(3)?==5)
        );
        assert_eq!(decoded.to_cell()?.hash()?, cell.hash()?);
    }
    Ok(())
}
#[test]
fn test_x1000_bounce_fixed_request_link_and_ignored_prefix() -> anyhow::Result<()> {
    use crate::contracts::custom_wallet::v2::{
        self, msgs::DedustX1000WalletV2BounceMsg as Bounce, storage::DedustX1000WalletV2ExecutionEntry,
    };

    for (id, bytes) in [
        include_bytes!("fixtures/x1000-v2-bounce-0.boc").as_slice(),
        include_bytes!("fixtures/x1000-v2-bounce-1.boc").as_slice(),
        include_bytes!("fixtures/x1000-v2-bounce-2.boc").as_slice(),
        include_bytes!("fixtures/x1000-v2-bounce-3.boc").as_slice(),
        include_bytes!("fixtures/x1000-v2-bounce-4.boc").as_slice(),
        include_bytes!("fixtures/x1000-v2-bounce-5.boc").as_slice(),
        include_bytes!("fixtures/x1000-v2-bounce-6.boc").as_slice(),
    ]
    .into_iter()
    .enumerate()
    {
        let cell = TonCell::from_boc(bytes.to_vec())?;
        let decoded = Bounce::from_cell(&cell)?;
        if id < 3 {
            assert!(matches!(&decoded, Bounce::Unknown(_)));
        } else {
            assert!(
                matches!(&decoded,Bounce::FailedRequest{original_opcode:0x12345678,outgoing_request_id,ignored_suffix}if *outgoing_request_id==match id {4=>999,5=>99,_=>100}&&ignored_suffix.data_len_bits()==if id==6 {3}else{0})
            );
        }
        assert_eq!(decoded.to_cell()?.hash()?, cell.hash()?);
    }
    let event = v2::msgs::DedustX1000WalletV2EventBeaf9617Msg::from_cell(&TonCell::from_boc(
        include_bytes!("fixtures/x1000-v2-bounce-event.boc").to_vec(),
    )?)?;
    assert_eq!(event.unknown0, 99);
    let post = v2::storage::DedustX1000WalletV2Storage::from_cell(&TonCell::from_boc(
        include_bytes!("fixtures/x1000-v2-bounce-post.boc").to_vec(),
    )?)?;
    assert!(post.runtime_state.request_links.is_empty());
    assert!(matches!(
        post.runtime_state.execution_entries.get(&99),
        Some(DedustX1000WalletV2ExecutionEntry::Marker)
    ));
    Ok(())
}
#[test]
fn test_jetton_discovery_bounces_skip_marker_and_preserve_ignored_tail() -> anyhow::Result<()> {
    use crate::contracts::classic::{vault_jetton_v1, vault_jetton_v2};

    let one = TonCell::from_boc(include_bytes!("fixtures/jetton-v1-discovery-bounce.boc").to_vec())?;
    let two = TonCell::from_boc(include_bytes!("fixtures/jetton-v2-discovery-bounce.boc").to_vec())?;
    let v1 = vault_jetton_v1::msgs::DedustVaultJettonV1BouncedProvideWalletAddressMsg::from_cell(&one)?;
    let v2 = vault_jetton_v2::msgs::DedustVaultJettonV2BouncedProvideWalletAddressMsg::from_cell(&two)?;
    assert_eq!(
        (v1.prefix, v1.original_opcode, v2.prefix, v2.original_opcode),
        (0, 0x2c76b973, 0, 0x2c76b973)
    );
    assert_eq!(v1.tail.parser().read_num::<u8>(3)?, 5);
    assert_eq!(v2.tail.parser().read_num::<u8>(3)?, 5);
    assert_eq!(v1.to_cell()?.hash()?, one.hash()?);
    assert_eq!(v2.to_cell()?.hash()?, two.hash()?);
    Ok(())
}
#[test]
fn test_position_source_skips_bounce_marker_and_retains_secondary_balances() -> anyhow::Result<()> {
    use crate::contracts::cpmm_v2::position::{
        msgs::DedustV2PositionBounceMsg as Bounce, storage::DedustV2CpmmPositionStorage,
    };

    let wire = TonCell::from_boc(include_bytes!("fixtures/position-zero-marker-bounce.boc").to_vec())?;
    assert!(
        matches!(Bounce::from_cell(&wire)?,Bounce::PositionFees{prefix:0,query_id:10,x_fees,y_fees,tail}if *x_fees==45&&*y_fees==68&&tail.parser().read_num::<u8>(3)?==5)
    );
    let post = DedustV2CpmmPositionStorage::from_cell(&TonCell::from_boc(
        include_bytes!("fixtures/position-zero-marker-restored.boc").to_vec(),
    )?)?;
    assert_eq!(
        (
            *post.fees.x_pending,
            *post.fees.y_pending,
            *post.fees.x_pending_secondary,
            *post.fees.y_pending_secondary
        ),
        (52, 79, 13, 17)
    );
    Ok(())
}
#[test]
fn test_lp_source_opaque_forward_tail_and_literal_zero_response_survive() -> anyhow::Result<()> {
    use crate::contracts::classic::lp_wallet;

    let wire = TonCell::from_boc(include_bytes!("fixtures/lp-forward-input-0.boc").to_vec())?;
    let transfer = lp_wallet::msgs::DedustClassicLpWalletTransferMsg::from_cell(&wire)?;
    assert_eq!((transfer.query_id, *transfer.amount), (77, 17));
    assert_eq!(transfer.forward_payload.parser().read_num::<u8>(4)?, 13);
    assert_eq!(transfer.forward_payload.refs().len(), 0);
    assert!(matches!(&transfer.response_destination,Some(v)if v.hash.as_slice()==[0;32]));
    assert_eq!(transfer.to_cell()?.hash()?, wire.hash()?);
    let wire = TonCell::from_boc(include_bytes!("fixtures/lp-forward-input-1.boc").to_vec())?;
    let internal = lp_wallet::msgs::DedustClassicLpWalletInternalTransferMsg::from_cell(&wire)?;
    assert_eq!(internal.from_address, None);
    assert_eq!(internal.forward_payload.data_len_bits(), 1);
    assert_eq!(internal.forward_payload.refs().len(), 0);
    assert_eq!(internal.to_cell()?.hash()?, wire.hash()?);
    let wire = TonCell::from_boc(include_bytes!("fixtures/lp-forward-notification.boc").to_vec())?;
    let notification = lp_wallet::msgs::DedustClassicLpWalletTransferNotificationMsg::from_cell(&wire)?;
    assert_eq!((notification.query_id, *notification.amount), (78, 17));
    assert_eq!(notification.sender, None);
    assert_eq!(notification.forward_payload.data_len_bits(), 1);
    assert_eq!(notification.to_cell()?.hash()?, wire.hash()?);
    let wire = TonCell::from_boc(include_bytes!("fixtures/lp-forward-internal.boc").to_vec())?;
    let forwarded = lp_wallet::msgs::DedustClassicLpWalletInternalTransferMsg::from_cell(&wire)?;
    assert_eq!((forwarded.query_id, *forwarded.amount), (77, 17));
    assert_eq!(forwarded.forward_payload.parser().read_num::<u8>(4)?, 13);
    assert_eq!(forwarded.to_cell()?.hash()?, wire.hash()?);
    Ok(())
}

/// These programs exit on the bounced envelope flag before looking at the body.
/// Their body codecs preserve an empty/unknown body for caller-owned envelope
/// dispatch; they intentionally invent no FFFFFFFF record for ignored data.
#[test]
fn test_early_ignored_bounce_has_no_fixed_body_and_short_unknown_is_preserved() -> anyhow::Result<()> {
    use crate::contracts::{classic, cpmm_v2, custom_wallet, uranus};

    macro_rules! preserved {
        ($path:path) => {{
use $path as Message;

            let empty=TonCell::empty().clone();
            let decoded=Message::from_cell(&empty)?;
            assert!(matches!(&decoded,Message::Unknown(v) if v.hash()?==empty.hash()?));
            assert_eq!(decoded.to_cell()?,empty);
        }};
    }
    preserved!(classic::factory_v1::msgs::DedustFactoryV1IncomingMsg);
    preserved!(classic::factory_v2::msgs::DedustFactoryV2IncomingMsg);
    preserved!(classic::pool_v1::msgs::DedustPoolV1IncomingMsg);
    preserved!(classic::pool_v2::msgs::DedustPoolV2IncomingMsg);
    preserved!(classic::vault_native_v1::msgs::DedustVaultNativeV1IncomingMsg);
    preserved!(classic::vault_native_v2::msgs::DedustVaultNativeV2IncomingMsg);
    preserved!(cpmm_v2::pool::msgs::DedustV2CpmmIncomingMsg);
    preserved!(uranus::factory_v3::msgs::DedustUranusFactoryV3IncomingMsg);
    preserved!(uranus::meme_v2::msgs::DedustUranusMemeV2IncomingMsg);
    preserved!(uranus::meme_v3::msgs::DedustUranusMemeV3IncomingMsg);
    preserved!(custom_wallet::v1::msgs::DedustX1000WalletV1IncomingMsg);
    preserved!(v2::msgs::DedustOperatorV2IncomingMsg);
    Ok(())
}

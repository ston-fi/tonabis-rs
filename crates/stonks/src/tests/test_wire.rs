use crate::contracts::custom_wallet::StonksCustomWalletContract;
use crate::contracts::custom_wallet::msgs::StonksCustomWalletJettonTransferMsg;
use crate::contracts::dedust_minter_v7::msgs::StonksDedustMinterV7MigrationCallbackMsg;
use crate::contracts::dedust_minter_v7::msgs::StonksDedustMinterV7MigrationRequestMsg;
use crate::contracts::dedust_wallet::msgs::StonksDedustWalletBouncedTransferMsg;
use crate::contracts::dedust_wallet::msgs::StonksDedustWalletSetFlagMsg;
use crate::contracts::dedust_wallet::storage::StonksDedustWalletStorage;
use crate::contracts::ston_minter::msgs::StonksStonMinterBuyFromPresaleMsg;
use crate::contracts::ston_minter::msgs::StonksStonMinterBuyMsg;
use crate::contracts::ston_minter::msgs::StonksStonMinterControllerInitMsg;
use crate::contracts::ston_minter::msgs::StonksStonMinterCurrentMessage;
use crate::contracts::ston_minter::msgs::StonksStonMinterCurrentSellNotificationMsg;
use crate::contracts::ston_minter::msgs::StonksStonMinterRetryMigrationMsg;
use crate::contracts::ston_minter::msgs::StonksStonMinterSellMsg;
use crate::contracts::ston_minter::msgs::StonksStonMinterSellNotificationMsg;
use crate::contracts::ston_minter::msgs::StonksStonMinterSellWithPayloadMsg;
use crate::contracts::ston_minter::msgs::StonksStonMinterWalletStatusRequestMsg;
use crate::contracts::ston_minter::msgs::StonksStonMinterWithdrawTonMsg;
use crate::contracts::ston_minter::storage::StonksVirtualMinterStorage;
use crate::contracts::ston_wallet::storage::StonksStonWalletStorage;
use ton::ton_core::{cell::TonCell, traits::tlb::TLB};
#[test]
fn test_known_malformed_unknown_and_fixed_layout_suffix() -> anyhow::Result<()> {
    let mut b = TonCell::builder();
    b.write_num(&0xcc1a97ae_u32, 32)?;
    assert!(crate::contracts::ston_minter::msgs::StonksStonMinterLegacyMessage::from_cell(&b.build()?).is_err());
    let mut b = TonCell::builder();
    b.write_num(&0x12345678_u32, 32)?;
    b.write_bit(true)?;
    let unknown = b.build()?;
    assert_eq!(
        crate::contracts::ston_minter::msgs::StonksStonMinterLegacyMessage::from_cell(&unknown)?.to_cell()?,
        unknown
    );
    let fixture = TonCell::from_boc(include_bytes!("fixtures/virtual_minter-1.boc").to_vec())?;
    let mut b = TonCell::builder();
    b.write_cell(&fixture)?;
    b.write_bit(false)?;
    assert_eq!(StonksStonMinterBuyFromPresaleMsg::from_cell(&b.build()?)?.to_cell()?, fixture);
    Ok(())
}
#[test]
fn test_bonding_stack_generation_lengths() -> anyhow::Result<()> {
    use ton::block_tlb::{FromTVMStack, TVMStack};
    let mut stack = TVMStack::new(vec![]);
    for n in [0, 2000, 1000, 10000, 0, 100, 0, 1] {
        stack.push_tiny_int(n);
    }
    crate::contracts::ston_minter::method_results::StonksStonMinterGetBondingDataResult::from_stack(
        &mut stack.clone(),
    )?;
    assert!(
        crate::contracts::dedust_minter_v12::method_results::StonksDedustMinterV12GetBondingDataResult::from_stack(
            &mut stack.clone()
        )
        .is_err()
    );
    for n in [100, 140, 5000] {
        stack.push_tiny_int(n);
    }
    stack.push_cell_slice(TonCell::from_boc_hex(
        "b5ee9c7201010101002400004380000000000000000000000000000000000000000000000000000000000000000010",
    )?);
    let parsed =
        crate::contracts::dedust_minter_v12::method_results::StonksDedustMinterV12GetBondingDataResult::from_stack(
            &mut stack.clone(),
        )?;
    assert_eq!(parsed.cpmm_fee_bps, 100);
    assert_eq!(parsed.presale_tax_bps, 140);
    assert!(
        crate::contracts::ston_minter::method_results::StonksStonMinterGetBondingDataResult::from_stack(&mut stack)
            .is_err()
    );
    Ok(())
}
#[test]
fn test_current_ston_null_recipient_is_a_known_invalid_body() -> anyhow::Result<()> {
    use ton::ton_core::types::{
        TonAddress,
        tlb_core::{MsgAddress, TLBCoins},
    };
    let mut fields = StonksStonMinterSellNotificationMsg {
        query_id: 1,
        jetton_amount: TLBCoins::ONE,
        sell_initiator: tonabis_core::adapters::nullable_internal_address::NullableInternalAddress::from_address(
            MsgAddress::Int(TonAddress::ZERO.to_msg_address_int()),
        )?,
        tons_recipient: None,
        min_ton_out: TLBCoins::ZERO,
        payload: None,
    };
    assert!(StonksStonMinterCurrentSellNotificationMsg::from_cell(&fields.to_cell()?).is_err());
    fields.tons_recipient = tonabis_core::adapters::nullable_internal_address::NullableInternalAddress::from_address(
        MsgAddress::Int(TonAddress::ZERO.to_msg_address_int()),
    )?;
    let valid = StonksStonMinterCurrentSellNotificationMsg::new(fields.clone())?;
    assert_eq!(
        StonksStonMinterCurrentSellNotificationMsg::from_cell(&valid.to_cell()?)?.fields(),
        &fields
    );
    Ok(())
}

#[test]
fn test_authored_current_storage_and_variant_layouts() -> anyhow::Result<()> {
    let cell = TonCell::from_boc(include_bytes!("fixtures/synthetic-current-storage.boc").to_vec())?;
    let storage = StonksVirtualMinterStorage::from_cell(&cell)?;
    assert_eq!(storage.total_supply.to_u128(), 100000000000000000);
    let state = storage.bonding_state.ok_or_else(|| anyhow::anyhow!("bonding"))?;
    let config = crate::contracts::dedust_minter_v12::storage::StonksDedustMinterV12MigrationConfig::from_cell(
        &state.migration_config,
    )?;
    assert_eq!(config.creator_presale_bps, 100);
    assert_eq!(config.creator_fee_bps, 5000);
    assert_eq!(
        crate::contracts::dedust_minter_v7::storage::StonksDedustMinterV7MigrationConfig::from_cell(
            &state.migration_config
        )?
        .creator_fee_bps,
        5000
    );
    let launch =
        crate::contracts::dedust_minter_v7::storage::StonksDedustMinterV7LaunchState::from_cell(&state.launch_config)?;
    assert!(launch.active);
    assert!(!launch.completed);
    assert_eq!(launch.query_id, 44);
    assert_eq!(launch.ton_amount.to_u128(), 2000000000000);
    let legacy = TonCell::from_boc(include_bytes!("fixtures/synthetic-legacy-config.boc").to_vec())?;
    assert_eq!(
        crate::contracts::dedust_minter_v7::storage::StonksDedustMinterV7MigrationConfig::from_cell(&legacy)?
            .creator_fee_bps,
        4000
    );
    assert!(
        crate::contracts::dedust_minter_v12::storage::StonksDedustMinterV12MigrationConfig::from_cell(&legacy).is_err()
    );
    let create = TonCell::from_boc(include_bytes!("fixtures/synthetic-current-create.boc").to_vec())?;
    assert_eq!(
        crate::contracts::dedust_factory::msgs::StonksDedustFactoryCreateVirtualLiquidityJettonMsg::from_cell(&create)?
            .creator_presale_bps,
        100
    );
    assert_eq!(
        crate::contracts::ston_factory::msgs::StonksStonFactoryCreateVirtualLiquidityJettonMsg::from_cell(&create)?
            .max_buy_percent,
        100
    );
    Ok(())
}
#[test]
fn test_authored_current_migration_and_forwarding() -> anyhow::Result<()> {
    let request = StonksDedustMinterV7MigrationRequestMsg::from_boc(
        include_bytes!("fixtures/synthetic-migration-request.boc").to_vec(),
    )?;
    assert_eq!(
        request
            .custom_payload
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("parameters"))?
            .creator_fee_bps,
        5000
    );
    let callback = StonksDedustMinterV7MigrationCallbackMsg::from_boc(
        include_bytes!("fixtures/synthetic-migration-callback.boc").to_vec(),
    )?;
    assert_eq!(request.query_id, 44);
    assert_eq!(request.custom_payload, callback.custom_payload);
    let parameters = request.custom_payload.ok_or_else(|| anyhow::anyhow!("parameters"))?;
    assert_eq!(parameters.token_amount.to_u128(), 98274214144569575);
    assert_eq!(parameters.ton_amount.to_u128(), 2000000000000);
    let withdraw =
        StonksStonMinterWithdrawTonMsg::from_boc(include_bytes!("fixtures/synthetic-withdraw.boc").to_vec())?;
    assert_eq!(withdraw.amount.to_u128(), 123);
    StonksStonMinterRetryMigrationMsg::from_boc(include_bytes!("fixtures/synthetic-retry.boc").to_vec())?;
    let body = TonCell::from_boc(include_bytes!("fixtures/forwarded-buy.boc").to_vec())?;
    let buy = StonksStonMinterBuyMsg::from_cell(&body)?;
    assert!(buy.recipient.is_some());
    assert_eq!(buy.query_id, 45);
    assert_eq!(
        buy.payload
            .ok_or_else(|| anyhow::anyhow!("payload"))?
            .value
            .parser()
            .read_num::<u32>(32)?,
        0x12345678
    );
    let transfer = crate::contracts::dedust_wallet::msgs::StonksDedustWalletInternalTransferMsg::from_boc(
        include_bytes!("fixtures/synthetic-forward-transfer.boc").to_vec(),
    )?;
    assert_eq!(transfer.amount.to_u128(), 72364193515215);
    assert_eq!(transfer.forward_payload.parser().read_num::<u32>(32)?, 0x12345678);
    Ok(())
}
#[test]
fn test_controller_messages_and_bounce_validation() -> anyhow::Result<()> {
    let init =
        StonksStonMinterControllerInitMsg::from_boc(include_bytes!("fixtures/synthetic-controller-init.boc").to_vec())?;
    assert!(matches!(
        crate::contracts::ston_minter::msgs::StonksStonMinterLegacyMessage::from_boc(
            include_bytes!("fixtures/synthetic-controller-init.boc").to_vec()
        )?,
        crate::contracts::ston_minter::msgs::StonksStonMinterLegacyMessage::ControllerInit(_)
    ));
    assert_eq!(init.migration_ton_target.to_u128(), 2000000000000);
    assert_eq!(init.additional_jetton_reserve.to_u128(), 49700598802325000);
    let status = StonksStonMinterWalletStatusRequestMsg::from_boc(
        include_bytes!("fixtures/synthetic-wallet-status.boc").to_vec(),
    )?;
    assert!(matches!(
        StonksStonMinterCurrentMessage::from_boc(include_bytes!("fixtures/synthetic-wallet-status.boc").to_vec())?,
        StonksStonMinterCurrentMessage::WalletStatus(_)
    ));
    assert_eq!(status.query_id, 44);
    let flag = StonksDedustWalletSetFlagMsg::from_boc(include_bytes!("fixtures/synthetic-set-flag.boc").to_vec())?;
    assert!(!flag.flag);
    let mut wrong = TonCell::builder();
    wrong.write_num(&0xffffffff_u32, 32)?;
    wrong.write_num(&0x12345678_u32, 32)?;
    assert!(crate::contracts::custom_wallet::msgs::StonksCustomWalletMessage::from_cell(&wrong.build()?).is_err());
    Ok(())
}
#[test]
fn test_tax_wallet_empty_body_and_unknown_short_body() -> anyhow::Result<()> {
    let empty = TonCell::empty().clone();
    assert!(matches!(
        crate::contracts::custom_wallet::msgs::StonksCustomWalletMessage::from_cell(&empty)?,
        crate::contracts::custom_wallet::msgs::StonksCustomWalletMessage::Empty(_)
    ));
    let mut bits = TonCell::builder();
    bits.write_bit(true)?;
    let short = bits.build()?;
    assert!(matches!(
        crate::contracts::custom_wallet::msgs::StonksCustomWalletMessage::from_cell(&short)?,
        crate::contracts::custom_wallet::msgs::StonksCustomWalletMessage::Unknown(_)
    ));
    Ok(())
}
#[test]
fn test_authored_ston_and_dedust_wallet_contexts_are_distinct() -> anyhow::Result<()> {
    let ston = TonCell::from_boc(include_bytes!("fixtures/synthetic-ston-wallet.boc").to_vec())?;
    let dedust = TonCell::from_boc(include_bytes!("fixtures/synthetic-dedust-wallet.boc").to_vec())?;
    let ston_wallet = StonksStonWalletStorage::from_cell(&ston)?;
    assert!(ston_wallet.context.flag);
    let dedust_wallet = StonksDedustWalletStorage::from_cell(&dedust)?;
    assert!(!dedust_wallet.context.flag);
    assert!(StonksStonWalletStorage::from_cell(&dedust).is_err());
    // Target selection is explicit: a shorter schema may consume a longer context prefix.
    StonksDedustWalletStorage::from_cell(&ston)?;
    let launch = crate::contracts::ston_minter::storage::StonksStonMinterLaunchState::from_boc(
        include_bytes!("fixtures/synthetic-ston-launch.boc").to_vec(),
    )?;
    assert!(!launch.active);
    assert!(launch.completed);
    assert_eq!(launch.token_amount.to_u128(), 99999999962500000);
    let config = crate::contracts::ston_minter::storage::StonksStonMinterMigrationConfig::from_boc(
        include_bytes!("fixtures/synthetic-ston-config.boc").to_vec(),
    )?;
    assert_eq!(config.ton_amount.to_u128(), 123);
    let sell =
        StonksStonMinterSellWithPayloadMsg::from_boc(include_bytes!("fixtures/synthetic-sell-payload.boc").to_vec())?;
    assert_eq!(sell.jetton_amount.to_u128(), 123);
    assert_eq!(sell.query_id, 44);
    Ok(())
}
#[test]
fn test_current_transfer_context_and_single_wallet_stack() -> anyhow::Result<()> {
    use ton::{
        block_tlb::{FromTVMStack, TVMStack},
        ton_core::types::{TonAddress, tlb_core::MsgAddress},
    };
    let ston = TonCell::from_boc(include_bytes!("fixtures/synthetic-ston-transfer.boc").to_vec())?;
    let dedust = TonCell::from_boc(include_bytes!("fixtures/synthetic-forward-transfer.boc").to_vec())?;
    assert_eq!(
        crate::contracts::ston_wallet::msgs::StonksStonWalletInternalTransferMsg::from_cell(&ston)?
            .amount
            .to_u128(),
        74812293551680
    );
    assert!(crate::contracts::ston_wallet::msgs::StonksStonWalletInternalTransferMsg::from_cell(&dedust).is_err());
    crate::contracts::dedust_wallet::msgs::StonksDedustWalletInternalTransferMsg::from_cell(&ston)?;
    let zero = MsgAddress::Int(TonAddress::ZERO.to_msg_address_int());
    let mut stack = TVMStack::new(vec![]);
    stack.push_cell_slice(zero.to_cell()?);
    assert_eq!(MsgAddress::from_stack(&mut stack.clone())?, zero);
    stack.push_tiny_int(7);
    assert!(MsgAddress::from_stack(&mut stack).is_err());
    Ok(())
}
#[test]
fn test_presale_bounce_and_sell_suffix_dispatch() -> anyhow::Result<()> {
    let bounce = TonCell::from_boc(include_bytes!("fixtures/synthetic-presale-bounce.boc").to_vec())?;
    let decoded = StonksDedustWalletBouncedTransferMsg::from_cell(&bounce)?;
    assert_eq!(decoded.original_opcode, 0xcc1a97b0);
    assert_eq!(decoded.amount.to_u128(), 123);
    assert_eq!(
        crate::contracts::dedust_wallet::msgs::StonksDedustWalletMessage::from_cell(&bounce)?.to_cell()?,
        bounce
    );
    let sell = TonCell::from_boc(include_bytes!("fixtures/synthetic-sell-payload.boc").to_vec())?;
    assert!(StonksStonMinterSellMsg::from_cell(&sell)?.payload.is_some());
    assert_eq!(
        crate::contracts::dedust_wallet::msgs::StonksDedustWalletMessage::from_cell(&sell)?.to_cell()?,
        sell
    );
    let mut wrong = TonCell::builder();
    wrong.write_num(&0xffffffff_u32, 32)?;
    wrong.write_num(&0x12345678_u32, 32)?;
    assert!(crate::contracts::dedust_wallet::msgs::StonksDedustWalletMessage::from_cell(&wrong.build()?).is_err());
    Ok(())
}
#[test]
fn test_missing_forward_selector_is_malformed_known_message() -> anyhow::Result<()> {
    let valid = TonCell::from_boc(include_bytes!("fixtures/schema-custom_wallet-JettonTransfer.boc").to_vec())?;
    let mut fields = StonksCustomWalletJettonTransferMsg::from_cell(&valid)?;
    fields.forward_payload = TonCell::empty().clone();
    let missing = fields.to_cell()?;
    assert!(crate::contracts::custom_wallet::msgs::StonksCustomWalletMessage::from_cell(&missing).is_err());
    assert!(crate::contracts::dedust_wallet::msgs::StonksDedustWalletMessage::from_cell(&missing).is_err());
    assert!(crate::contracts::ston_wallet::msgs::StonksStonWalletMessage::from_cell(&missing).is_err());
    Ok(())
}

#[test]
fn test_legacy_dedust_continuation_and_dispatch() -> anyhow::Result<()> {
    let cell = TonCell::from_boc(include_bytes!("fixtures/synthetic-legacy-request.boc").to_vec())?;
    let request = StonksDedustMinterV7MigrationRequestMsg::from_cell(&cell)?;
    assert_eq!(
        request
            .custom_payload
            .ok_or_else(|| anyhow::anyhow!("parameters"))?
            .creator_fee_bps,
        4000
    );
    let cell = TonCell::from_boc(include_bytes!("fixtures/synthetic-legacy-callback.boc").to_vec())?;
    let callback = crate::contracts::dedust_minter_v7::msgs::StonksDedustMinterV7Message::from_cell(&cell)?;
    assert!(matches!(
        callback,
        crate::contracts::dedust_minter_v7::msgs::StonksDedustMinterV7Message::MigrationCallback(_)
    ));
    assert_eq!(callback.to_cell()?.hash()?, cell.hash()?);
    let mut malformed = TonCell::builder();
    malformed.write_num(&0xce185bd7_u32, 32)?;
    assert!(
        crate::contracts::dedust_minter_v7::msgs::StonksDedustMinterV7Message::from_cell(&malformed.build()?).is_err()
    );
    let mut unknown = TonCell::builder();
    unknown.write_num(&0x12345678_u32, 32)?;
    unknown.write_bit(true)?;
    let cell = unknown.build()?;
    assert_eq!(
        crate::contracts::dedust_minter_v7::msgs::StonksDedustMinterV7Message::from_cell(&cell)?.to_cell()?,
        cell
    );
    Ok(())
}

#[test]
fn test_presale_internal_transfer_rejects_false_payload_selector() -> anyhow::Result<()> {
    let ston = TonCell::from_boc(include_bytes!("fixtures/synthetic-ston-empty-transfer.boc").to_vec())?;
    let dedust = TonCell::from_boc(include_bytes!("fixtures/synthetic-dedust-empty-transfer.boc").to_vec())?;
    assert!(crate::contracts::ston_wallet::msgs::StonksStonWalletInternalTransferMsg::from_cell(&ston).is_err());
    assert!(crate::contracts::dedust_wallet::msgs::StonksDedustWalletInternalTransferMsg::from_cell(&dedust).is_err());
    assert!(crate::contracts::ston_wallet::msgs::StonksStonWalletMessage::from_cell(&ston).is_err());
    assert!(crate::contracts::dedust_wallet::msgs::StonksDedustWalletMessage::from_cell(&dedust).is_err());
    Ok(())
}

#[test]
fn test_custom_wallet_initial_context_matches_independent_wire() -> anyhow::Result<()> {
    use ton::ton_core::types::{TonAddress, tlb_core::MsgAddress};
    let _zero = MsgAddress::Int(TonAddress::ZERO.to_msg_address_int());
    let state = StonksCustomWalletContract::initial_state(
        TonCell::empty().clone(),
        Some(TonAddress::ZERO),
        Some(TonAddress::ZERO),
    )?;
    let expected = TonCell::from_boc(include_bytes!("fixtures/synthetic-custom-initial-data.boc").to_vec())?;
    assert_eq!(state.data.as_deref(), Some(&expected));
    Ok(())
}

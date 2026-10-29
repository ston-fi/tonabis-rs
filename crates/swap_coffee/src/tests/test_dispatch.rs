//! Role-specific known/malformed/unknown dispatch from independent schema vectors.

use super::test_schema_vectors::cell_for;
use crate::contracts::cross_dex::msgs::CoffeeCrossDexIncomingMessage;
use crate::contracts::cross_dex::msgs::CoffeeCrossDexOutgoingMessage;
use crate::contracts::cross_dex_bidask::msgs::CoffeeCrossDexBidaskIncomingMessage;
use crate::contracts::cross_dex_bidask::msgs::CoffeeCrossDexBidaskOutgoingMessage;
use crate::contracts::cross_dex_legacy::msgs::CoffeeCrossDexLegacyIncomingMessage;
use crate::contracts::cross_dex_legacy::msgs::CoffeeCrossDexLegacyOutgoingMessage;
use crate::contracts::factory::msgs::CoffeeFactoryIncomingMessage;
use crate::contracts::factory::msgs::CoffeeFactoryOutgoingMessage;
use crate::contracts::init::msgs::CoffeeInitIncomingMessage;
use crate::contracts::liquidity_depository::msgs::CoffeeLiquidityDepositoryIncomingMessage;
use crate::contracts::liquidity_depository::msgs::CoffeeLiquidityDepositoryOutgoingMessage;
use crate::contracts::lp_wallet::msgs::CoffeeLpWalletIncomingMessage;
use crate::contracts::lp_wallet::msgs::CoffeeLpWalletOutgoingMessage;
use crate::contracts::mev_protector::msgs::CoffeeMevProtectorIncomingMessage;
use crate::contracts::mev_protector::msgs::CoffeeMevProtectorOutgoingMessage;
use crate::contracts::mev_protector_legacy::msgs::CoffeeMevProtectorLegacyIncomingMessage;
use crate::contracts::mev_protector_legacy::msgs::CoffeeMevProtectorLegacyOutgoingMessage;
use crate::contracts::pool_constant_product::msgs::CoffeeConstantProductPoolIncomingMessage;
use crate::contracts::pool_constant_product::msgs::CoffeeConstantProductPoolOutgoingMessage;
use crate::contracts::pool_creator::msgs::CoffeePoolCreatorIncomingMessage;
use crate::contracts::pool_creator::msgs::CoffeePoolCreatorOutgoingMessage;
use crate::contracts::pool_curve_fi_stable::msgs::CoffeeCurveFiStablePoolIncomingMessage;
use crate::contracts::pool_curve_fi_stable::msgs::CoffeeCurveFiStablePoolOutgoingMessage;
use crate::contracts::staking_item::msgs::CoffeeStakingItemIncomingMessage;
use crate::contracts::staking_item::msgs::CoffeeStakingItemOutgoingMessage;
use crate::contracts::staking_master::msgs::CoffeeStakingMasterIncomingMessage;
use crate::contracts::staking_master::msgs::CoffeeStakingMasterOutgoingMessage;
use crate::contracts::staking_vault::msgs::CoffeeStakingVaultIncomingMessage;
use crate::contracts::staking_vault::msgs::CoffeeStakingVaultOutgoingMessage;
use crate::contracts::vault_extra::msgs::CoffeeVaultExtraIncomingMessage;
use crate::contracts::vault_extra::msgs::CoffeeVaultExtraOutgoingMessage;
use crate::contracts::vault_jetton::msgs::CoffeeVaultJettonIncomingMessage;
use crate::contracts::vault_jetton::msgs::CoffeeVaultJettonOutgoingMessage;
use crate::contracts::vault_native::msgs::CoffeeVaultNativeIncomingMessage;
use crate::contracts::vault_native::msgs::CoffeeVaultNativeOutgoingMessage;

use ton::ton_core::cell::TonCell;

use ton::ton_core::traits::tlb::TLB;
fn opcode_only(op: u32) -> anyhow::Result<TonCell> {
    let mut b = TonCell::builder();
    b.write_num(&op, 32)?;
    Ok(b.build()?)
}
fn unknown_body() -> anyhow::Result<TonCell> {
    let mut b = TonCell::builder();
    b.write_num(&0xdeadbeefu32, 32)?;
    b.write_num(&5u8, 3)?;
    b.write_ref(opcode_only(0xaabbccdd)?)?;
    Ok(b.build()?)
}
#[test]
fn test_cross_dex_role_dispatch() -> anyhow::Result<()> {
    let raw = cell_for("cross_dex/CoffeeCrossDexJettonTransferNotification/0")?;
    assert!(matches!(
        CoffeeCrossDexIncomingMessage::from_cell(&raw)?,
        CoffeeCrossDexIncomingMessage::JettonTransferNotification(_)
    ));
    assert!(CoffeeCrossDexIncomingMessage::from_cell(&opcode_only(0x7362d09c)?).is_err());
    let raw = cell_for("cross_dex/CoffeeCrossDexDedustPayout/0")?;
    assert!(matches!(
        CoffeeCrossDexIncomingMessage::from_cell(&raw)?,
        CoffeeCrossDexIncomingMessage::DedustPayout(_)
    ));
    assert!(CoffeeCrossDexIncomingMessage::from_cell(&opcode_only(0x474f86cf)?).is_err());
    let raw = cell_for("cross_dex/CoffeeCrossDexPtonTonTransfer/0")?;
    assert!(matches!(
        CoffeeCrossDexIncomingMessage::from_cell(&raw)?,
        CoffeeCrossDexIncomingMessage::PtonTonTransfer(_)
    ));
    assert!(CoffeeCrossDexIncomingMessage::from_cell(&opcode_only(0x01f3835d)?).is_err());
    let raw = cell_for("cross_dex/CoffeeCrossDexCoffeeNotification/0")?;
    assert!(matches!(
        CoffeeCrossDexIncomingMessage::from_cell(&raw)?,
        CoffeeCrossDexIncomingMessage::CoffeeNotification(_)
    ));
    assert!(CoffeeCrossDexIncomingMessage::from_cell(&opcode_only(0xc0ffee36)?).is_err());
    let raw = cell_for("cross_dex/CoffeeCrossDexBidaskNativeTransferNotification/0")?;
    assert!(matches!(
        CoffeeCrossDexIncomingMessage::from_cell(&raw)?,
        CoffeeCrossDexIncomingMessage::BidaskNativeTransferNotification(_)
    ));
    assert!(CoffeeCrossDexIncomingMessage::from_cell(&opcode_only(0x6edd65f0)?).is_err());
    let raw = cell_for("cross_dex/CoffeeCrossDexWrappedPayload/0")?;
    assert!(matches!(
        CoffeeCrossDexIncomingMessage::from_cell(&raw)?,
        CoffeeCrossDexIncomingMessage::WrappedPayload(_)
    ));
    assert!(CoffeeCrossDexIncomingMessage::from_cell(&opcode_only(0x9d142643)?).is_err());
    let raw = cell_for("cross_dex/CoffeeCrossDexResend/0")?;
    assert!(matches!(
        CoffeeCrossDexIncomingMessage::from_cell(&raw)?,
        CoffeeCrossDexIncomingMessage::Resend(_)
    ));
    assert!(CoffeeCrossDexIncomingMessage::from_cell(&opcode_only(0x200f9086)?).is_err());
    let raw = cell_for("cross_dex/CoffeeCrossDexSetAdmin/0")?;
    assert!(matches!(
        CoffeeCrossDexIncomingMessage::from_cell(&raw)?,
        CoffeeCrossDexIncomingMessage::SetAdmin(_)
    ));
    assert!(CoffeeCrossDexIncomingMessage::from_cell(&opcode_only(0x57861aa7)?).is_err());
    let raw = cell_for("cross_dex/CoffeeCrossDexUpdateContract/0")?;
    assert!(matches!(
        CoffeeCrossDexIncomingMessage::from_cell(&raw)?,
        CoffeeCrossDexIncomingMessage::UpdateContract(_)
    ));
    assert!(CoffeeCrossDexIncomingMessage::from_cell(&opcode_only(0x6f80872c)?).is_err());
    let raw = cell_for("cross_dex/CoffeeCrossDexNativeRescue/0")?;
    assert!(matches!(
        CoffeeCrossDexIncomingMessage::from_cell(&raw)?,
        CoffeeCrossDexIncomingMessage::NativeRescue(_)
    ));
    assert!(CoffeeCrossDexIncomingMessage::from_cell(&opcode_only(0x2bf71be6)?).is_err());
    let raw = cell_for("cross_dex/CoffeeCrossDexJettonRescue/0")?;
    assert!(matches!(
        CoffeeCrossDexIncomingMessage::from_cell(&raw)?,
        CoffeeCrossDexIncomingMessage::JettonRescue(_)
    ));
    assert!(CoffeeCrossDexIncomingMessage::from_cell(&opcode_only(0x7b4b82fe)?).is_err());
    let raw = cell_for("cross_dex/CoffeeCrossDexExcesses/0")?;
    assert!(matches!(
        CoffeeCrossDexIncomingMessage::from_cell(&raw)?,
        CoffeeCrossDexIncomingMessage::Excesses(_)
    ));
    assert!(CoffeeCrossDexIncomingMessage::from_cell(&opcode_only(0xd53276db)?).is_err());
    let raw = unknown_body()?;
    let decoded = CoffeeCrossDexIncomingMessage::from_cell(&raw)?;
    assert!(matches!(decoded, CoffeeCrossDexIncomingMessage::Unknown(_)));
    assert_eq!(decoded.to_cell()?, raw);
    let raw = cell_for("cross_dex/CoffeeCrossDexJettonTransfer/0")?;
    assert!(matches!(
        CoffeeCrossDexOutgoingMessage::from_cell(&raw)?,
        CoffeeCrossDexOutgoingMessage::JettonTransfer(_)
    ));
    assert!(CoffeeCrossDexOutgoingMessage::from_cell(&opcode_only(0x0f8a7ea5)?).is_err());
    let raw = cell_for("cross_dex/CoffeeCrossDexExcesses/0")?;
    assert!(matches!(
        CoffeeCrossDexOutgoingMessage::from_cell(&raw)?,
        CoffeeCrossDexOutgoingMessage::Excesses(_)
    ));
    assert!(CoffeeCrossDexOutgoingMessage::from_cell(&opcode_only(0xd53276db)?).is_err());
    let raw = unknown_body()?;
    let decoded = CoffeeCrossDexOutgoingMessage::from_cell(&raw)?;
    assert!(matches!(decoded, CoffeeCrossDexOutgoingMessage::Unknown(_)));
    assert_eq!(decoded.to_cell()?, raw);
    Ok(())
}
#[test]
fn test_cross_dex_bidask_role_dispatch() -> anyhow::Result<()> {
    let raw = cell_for("cross_dex_bidask/CoffeeCrossDexBidaskJettonTransferNotification/0")?;
    assert!(matches!(
        CoffeeCrossDexBidaskIncomingMessage::from_cell(&raw)?,
        CoffeeCrossDexBidaskIncomingMessage::JettonTransferNotification(_)
    ));
    assert!(CoffeeCrossDexBidaskIncomingMessage::from_cell(&opcode_only(0x7362d09c)?).is_err());
    let raw = cell_for("cross_dex_bidask/CoffeeCrossDexBidaskDedustPayout/0")?;
    assert!(matches!(
        CoffeeCrossDexBidaskIncomingMessage::from_cell(&raw)?,
        CoffeeCrossDexBidaskIncomingMessage::DedustPayout(_)
    ));
    assert!(CoffeeCrossDexBidaskIncomingMessage::from_cell(&opcode_only(0x474f86cf)?).is_err());
    let raw = cell_for("cross_dex_bidask/CoffeeCrossDexBidaskPtonTonTransfer/0")?;
    assert!(matches!(
        CoffeeCrossDexBidaskIncomingMessage::from_cell(&raw)?,
        CoffeeCrossDexBidaskIncomingMessage::PtonTonTransfer(_)
    ));
    assert!(CoffeeCrossDexBidaskIncomingMessage::from_cell(&opcode_only(0x01f3835d)?).is_err());
    let raw = cell_for("cross_dex_bidask/CoffeeCrossDexBidaskCoffeeNotification/0")?;
    assert!(matches!(
        CoffeeCrossDexBidaskIncomingMessage::from_cell(&raw)?,
        CoffeeCrossDexBidaskIncomingMessage::CoffeeNotification(_)
    ));
    assert!(CoffeeCrossDexBidaskIncomingMessage::from_cell(&opcode_only(0xc0ffee36)?).is_err());
    let raw = cell_for("cross_dex_bidask/CoffeeCrossDexBidaskBidaskNativeTransferNotification/0")?;
    assert!(matches!(
        CoffeeCrossDexBidaskIncomingMessage::from_cell(&raw)?,
        CoffeeCrossDexBidaskIncomingMessage::BidaskNativeTransferNotification(_)
    ));
    assert!(CoffeeCrossDexBidaskIncomingMessage::from_cell(&opcode_only(0x6edd65f0)?).is_err());
    let raw = cell_for("cross_dex_bidask/CoffeeCrossDexBidaskResend/0")?;
    assert!(matches!(
        CoffeeCrossDexBidaskIncomingMessage::from_cell(&raw)?,
        CoffeeCrossDexBidaskIncomingMessage::Resend(_)
    ));
    assert!(CoffeeCrossDexBidaskIncomingMessage::from_cell(&opcode_only(0x200f9086)?).is_err());
    let raw = cell_for("cross_dex_bidask/CoffeeCrossDexBidaskSetAdmin/0")?;
    assert!(matches!(
        CoffeeCrossDexBidaskIncomingMessage::from_cell(&raw)?,
        CoffeeCrossDexBidaskIncomingMessage::SetAdmin(_)
    ));
    assert!(CoffeeCrossDexBidaskIncomingMessage::from_cell(&opcode_only(0x57861aa7)?).is_err());
    let raw = cell_for("cross_dex_bidask/CoffeeCrossDexBidaskUpdateContract/0")?;
    assert!(matches!(
        CoffeeCrossDexBidaskIncomingMessage::from_cell(&raw)?,
        CoffeeCrossDexBidaskIncomingMessage::UpdateContract(_)
    ));
    assert!(CoffeeCrossDexBidaskIncomingMessage::from_cell(&opcode_only(0x6f80872c)?).is_err());
    let raw = cell_for("cross_dex_bidask/CoffeeCrossDexBidaskWithdrawTon/0")?;
    assert!(matches!(
        CoffeeCrossDexBidaskIncomingMessage::from_cell(&raw)?,
        CoffeeCrossDexBidaskIncomingMessage::WithdrawTon(_)
    ));
    assert!(CoffeeCrossDexBidaskIncomingMessage::from_cell(&opcode_only(0x2bf71be6)?).is_err());
    let raw = cell_for("cross_dex_bidask/CoffeeCrossDexBidaskWithdrawJettons/0")?;
    assert!(matches!(
        CoffeeCrossDexBidaskIncomingMessage::from_cell(&raw)?,
        CoffeeCrossDexBidaskIncomingMessage::WithdrawJettons(_)
    ));
    assert!(CoffeeCrossDexBidaskIncomingMessage::from_cell(&opcode_only(0x7b4b82fe)?).is_err());
    let raw = cell_for("cross_dex_bidask/CoffeeCrossDexBidaskExcesses/0")?;
    assert!(matches!(
        CoffeeCrossDexBidaskIncomingMessage::from_cell(&raw)?,
        CoffeeCrossDexBidaskIncomingMessage::Excesses(_)
    ));
    assert!(CoffeeCrossDexBidaskIncomingMessage::from_cell(&opcode_only(0xd53276db)?).is_err());
    let raw = unknown_body()?;
    let decoded = CoffeeCrossDexBidaskIncomingMessage::from_cell(&raw)?;
    assert!(matches!(decoded, CoffeeCrossDexBidaskIncomingMessage::Unknown(_)));
    assert_eq!(decoded.to_cell()?, raw);
    let raw = cell_for("cross_dex_bidask/CoffeeCrossDexBidaskJettonTransfer/0")?;
    assert!(matches!(
        CoffeeCrossDexBidaskOutgoingMessage::from_cell(&raw)?,
        CoffeeCrossDexBidaskOutgoingMessage::JettonTransfer(_)
    ));
    assert!(CoffeeCrossDexBidaskOutgoingMessage::from_cell(&opcode_only(0x0f8a7ea5)?).is_err());
    let raw = cell_for("cross_dex_bidask/CoffeeCrossDexBidaskExcesses/0")?;
    assert!(matches!(
        CoffeeCrossDexBidaskOutgoingMessage::from_cell(&raw)?,
        CoffeeCrossDexBidaskOutgoingMessage::Excesses(_)
    ));
    assert!(CoffeeCrossDexBidaskOutgoingMessage::from_cell(&opcode_only(0xd53276db)?).is_err());
    let raw = unknown_body()?;
    let decoded = CoffeeCrossDexBidaskOutgoingMessage::from_cell(&raw)?;
    assert!(matches!(decoded, CoffeeCrossDexBidaskOutgoingMessage::Unknown(_)));
    assert_eq!(decoded.to_cell()?, raw);
    Ok(())
}
#[test]
fn test_cross_dex_legacy_role_dispatch() -> anyhow::Result<()> {
    let raw = cell_for("cross_dex_legacy/CoffeeCrossDexLegacyJettonTransferNotification/0")?;
    assert!(matches!(
        CoffeeCrossDexLegacyIncomingMessage::from_cell(&raw)?,
        CoffeeCrossDexLegacyIncomingMessage::JettonTransferNotification(_)
    ));
    assert!(CoffeeCrossDexLegacyIncomingMessage::from_cell(&opcode_only(0x7362d09c)?).is_err());
    let raw = cell_for("cross_dex_legacy/CoffeeCrossDexLegacyDedustPayout/0")?;
    assert!(matches!(
        CoffeeCrossDexLegacyIncomingMessage::from_cell(&raw)?,
        CoffeeCrossDexLegacyIncomingMessage::DedustPayout(_)
    ));
    assert!(CoffeeCrossDexLegacyIncomingMessage::from_cell(&opcode_only(0x474f86cf)?).is_err());
    let raw = cell_for("cross_dex_legacy/CoffeeCrossDexLegacyPtonTonTransfer/0")?;
    assert!(matches!(
        CoffeeCrossDexLegacyIncomingMessage::from_cell(&raw)?,
        CoffeeCrossDexLegacyIncomingMessage::PtonTonTransfer(_)
    ));
    assert!(CoffeeCrossDexLegacyIncomingMessage::from_cell(&opcode_only(0x01f3835d)?).is_err());
    let raw = cell_for("cross_dex_legacy/CoffeeCrossDexLegacySetAdmin/0")?;
    assert!(matches!(
        CoffeeCrossDexLegacyIncomingMessage::from_cell(&raw)?,
        CoffeeCrossDexLegacyIncomingMessage::SetAdmin(_)
    ));
    assert!(CoffeeCrossDexLegacyIncomingMessage::from_cell(&opcode_only(0x57861aa7)?).is_err());
    let raw = cell_for("cross_dex_legacy/CoffeeCrossDexLegacyUpdateContract/0")?;
    assert!(matches!(
        CoffeeCrossDexLegacyIncomingMessage::from_cell(&raw)?,
        CoffeeCrossDexLegacyIncomingMessage::UpdateContract(_)
    ));
    assert!(CoffeeCrossDexLegacyIncomingMessage::from_cell(&opcode_only(0x6f80872c)?).is_err());
    let raw = cell_for("cross_dex_legacy/CoffeeCrossDexLegacyWithdrawTon/0")?;
    assert!(matches!(
        CoffeeCrossDexLegacyIncomingMessage::from_cell(&raw)?,
        CoffeeCrossDexLegacyIncomingMessage::WithdrawTon(_)
    ));
    assert!(CoffeeCrossDexLegacyIncomingMessage::from_cell(&opcode_only(0x2bf71be6)?).is_err());
    let raw = cell_for("cross_dex_legacy/CoffeeCrossDexLegacyWithdrawJettons/0")?;
    assert!(matches!(
        CoffeeCrossDexLegacyIncomingMessage::from_cell(&raw)?,
        CoffeeCrossDexLegacyIncomingMessage::WithdrawJettons(_)
    ));
    assert!(CoffeeCrossDexLegacyIncomingMessage::from_cell(&opcode_only(0x7b4b82fe)?).is_err());
    let raw = cell_for("cross_dex_legacy/CoffeeCrossDexLegacyExcesses/0")?;
    assert!(matches!(
        CoffeeCrossDexLegacyIncomingMessage::from_cell(&raw)?,
        CoffeeCrossDexLegacyIncomingMessage::Excesses(_)
    ));
    assert!(CoffeeCrossDexLegacyIncomingMessage::from_cell(&opcode_only(0xd53276db)?).is_err());
    let raw = unknown_body()?;
    let decoded = CoffeeCrossDexLegacyIncomingMessage::from_cell(&raw)?;
    assert!(matches!(decoded, CoffeeCrossDexLegacyIncomingMessage::Unknown(_)));
    assert_eq!(decoded.to_cell()?, raw);
    let raw = cell_for("cross_dex_legacy/CoffeeCrossDexLegacyJettonTransfer/0")?;
    assert!(matches!(
        CoffeeCrossDexLegacyOutgoingMessage::from_cell(&raw)?,
        CoffeeCrossDexLegacyOutgoingMessage::JettonTransfer(_)
    ));
    assert!(CoffeeCrossDexLegacyOutgoingMessage::from_cell(&opcode_only(0x0f8a7ea5)?).is_err());
    let raw = cell_for("cross_dex_legacy/CoffeeCrossDexLegacyExcesses/0")?;
    assert!(matches!(
        CoffeeCrossDexLegacyOutgoingMessage::from_cell(&raw)?,
        CoffeeCrossDexLegacyOutgoingMessage::Excesses(_)
    ));
    assert!(CoffeeCrossDexLegacyOutgoingMessage::from_cell(&opcode_only(0xd53276db)?).is_err());
    let raw = unknown_body()?;
    let decoded = CoffeeCrossDexLegacyOutgoingMessage::from_cell(&raw)?;
    assert!(matches!(decoded, CoffeeCrossDexLegacyOutgoingMessage::Unknown(_)));
    assert_eq!(decoded.to_cell()?, raw);
    Ok(())
}
#[test]
fn test_factory_role_dispatch() -> anyhow::Result<()> {
    let raw = cell_for("factory/CoffeeFactoryCreatePoolCreatorRequest/0")?;
    assert!(matches!(
        CoffeeFactoryIncomingMessage::from_cell(&raw)?,
        CoffeeFactoryIncomingMessage::CreatePoolCreatorRequest(_)
    ));
    assert!(CoffeeFactoryIncomingMessage::from_cell(&opcode_only(0xc0ffee23)?).is_err());
    let raw = cell_for("factory/CoffeeFactoryCreateLiquidityDepositoryRequest/0")?;
    assert!(matches!(
        CoffeeFactoryIncomingMessage::from_cell(&raw)?,
        CoffeeFactoryIncomingMessage::CreateLiquidityDepositoryRequest(_)
    ));
    assert!(CoffeeFactoryIncomingMessage::from_cell(&opcode_only(0xc0ffee27)?).is_err());
    let raw = cell_for("factory/CoffeeFactoryCreatePoolRequest/0")?;
    assert!(matches!(
        CoffeeFactoryIncomingMessage::from_cell(&raw)?,
        CoffeeFactoryIncomingMessage::CreatePoolRequest(_)
    ));
    assert!(CoffeeFactoryIncomingMessage::from_cell(&opcode_only(0xc0ffee25)?).is_err());
    let raw = cell_for("factory/CoffeeFactoryCreateVault/0")?;
    assert!(matches!(
        CoffeeFactoryIncomingMessage::from_cell(&raw)?,
        CoffeeFactoryIncomingMessage::CreateVault(_)
    ));
    assert!(CoffeeFactoryIncomingMessage::from_cell(&opcode_only(0xc0ffee06)?).is_err());
    let raw = cell_for("factory/CoffeeFactoryUpdateAdmin/0")?;
    assert!(matches!(
        CoffeeFactoryIncomingMessage::from_cell(&raw)?,
        CoffeeFactoryIncomingMessage::UpdateAdmin(_)
    ));
    assert!(CoffeeFactoryIncomingMessage::from_cell(&opcode_only(0xc0ffee40)?).is_err());
    let raw = cell_for("factory/CoffeeFactoryUpdateWithdrawer/0")?;
    assert!(matches!(
        CoffeeFactoryIncomingMessage::from_cell(&raw)?,
        CoffeeFactoryIncomingMessage::UpdateWithdrawer(_)
    ));
    assert!(CoffeeFactoryIncomingMessage::from_cell(&opcode_only(0xc0ffee46)?).is_err());
    let raw = cell_for("factory/CoffeeFactoryUpdateCodeCells/0")?;
    assert!(matches!(
        CoffeeFactoryIncomingMessage::from_cell(&raw)?,
        CoffeeFactoryIncomingMessage::UpdateCodeCells(_)
    ));
    assert!(CoffeeFactoryIncomingMessage::from_cell(&opcode_only(0xc0ffee44)?).is_err());
    let raw = cell_for("factory/CoffeeFactoryUpdateContract/0")?;
    assert!(matches!(
        CoffeeFactoryIncomingMessage::from_cell(&raw)?,
        CoffeeFactoryIncomingMessage::UpdateContract(_)
    ));
    assert!(CoffeeFactoryIncomingMessage::from_cell(&opcode_only(0xc0ffee45)?).is_err());
    let raw = cell_for("factory/CoffeeFactoryUpdatePool/0")?;
    assert!(matches!(
        CoffeeFactoryIncomingMessage::from_cell(&raw)?,
        CoffeeFactoryIncomingMessage::UpdatePool(_)
    ));
    assert!(CoffeeFactoryIncomingMessage::from_cell(&opcode_only(0xc0ffee41)?).is_err());
    let raw = cell_for("factory/CoffeeFactoryActivateVault/0")?;
    assert!(matches!(
        CoffeeFactoryIncomingMessage::from_cell(&raw)?,
        CoffeeFactoryIncomingMessage::ActivateVault(_)
    ));
    assert!(CoffeeFactoryIncomingMessage::from_cell(&opcode_only(0xc0ffee42)?).is_err());
    let raw = cell_for("factory/CoffeeFactoryWithdraw/0")?;
    assert!(matches!(
        CoffeeFactoryIncomingMessage::from_cell(&raw)?,
        CoffeeFactoryIncomingMessage::Withdraw(_)
    ));
    assert!(CoffeeFactoryIncomingMessage::from_cell(&opcode_only(0xc0ffee43)?).is_err());
    let raw = unknown_body()?;
    let decoded = CoffeeFactoryIncomingMessage::from_cell(&raw)?;
    assert!(matches!(decoded, CoffeeFactoryIncomingMessage::Unknown(_)));
    assert_eq!(decoded.to_cell()?, raw);
    let raw = cell_for("factory/CoffeeFactoryDeploy/0")?;
    assert!(matches!(
        CoffeeFactoryOutgoingMessage::from_cell(&raw)?,
        CoffeeFactoryOutgoingMessage::Deploy(_)
    ));
    assert!(CoffeeFactoryOutgoingMessage::from_cell(&opcode_only(0xc0ffee22)?).is_err());
    let raw = cell_for("factory/CoffeeFactoryCreatePoolCreatorInternal/0")?;
    assert!(matches!(
        CoffeeFactoryOutgoingMessage::from_cell(&raw)?,
        CoffeeFactoryOutgoingMessage::CreatePoolCreatorInternal(_)
    ));
    assert!(CoffeeFactoryOutgoingMessage::from_cell(&opcode_only(0xc0ffee24)?).is_err());
    let raw = cell_for("factory/CoffeeFactoryCreateLiquidityDepositoryInternal/0")?;
    assert!(matches!(
        CoffeeFactoryOutgoingMessage::from_cell(&raw)?,
        CoffeeFactoryOutgoingMessage::CreateLiquidityDepositoryInternal(_)
    ));
    assert!(CoffeeFactoryOutgoingMessage::from_cell(&opcode_only(0xc0ffee28)?).is_err());
    let raw = cell_for("factory/CoffeeFactoryCreatePoolInternal/0")?;
    assert!(matches!(
        CoffeeFactoryOutgoingMessage::from_cell(&raw)?,
        CoffeeFactoryOutgoingMessage::CreatePoolInternal(_)
    ));
    assert!(CoffeeFactoryOutgoingMessage::from_cell(&opcode_only(0xc0ffee26)?).is_err());
    let raw = cell_for("factory/CoffeeFactoryCreateVaultInternal/0")?;
    assert!(matches!(
        CoffeeFactoryOutgoingMessage::from_cell(&raw)?,
        CoffeeFactoryOutgoingMessage::CreateVaultInternal(_)
    ));
    assert!(CoffeeFactoryOutgoingMessage::from_cell(&opcode_only(0xc0ffee2a)?).is_err());
    let raw = cell_for("factory/CoffeeFactoryUpdatePoolInternal/0")?;
    assert!(matches!(
        CoffeeFactoryOutgoingMessage::from_cell(&raw)?,
        CoffeeFactoryOutgoingMessage::UpdatePoolInternal(_)
    ));
    assert!(CoffeeFactoryOutgoingMessage::from_cell(&opcode_only(0xc0ffee2b)?).is_err());
    let raw = cell_for("factory/CoffeeFactoryActivateVaultInternal/0")?;
    assert!(matches!(
        CoffeeFactoryOutgoingMessage::from_cell(&raw)?,
        CoffeeFactoryOutgoingMessage::ActivateVaultInternal(_)
    ));
    assert!(CoffeeFactoryOutgoingMessage::from_cell(&opcode_only(0xc0ffee2c)?).is_err());
    let raw = cell_for("factory/CoffeeFactoryWithdrawInternal/0")?;
    assert!(matches!(
        CoffeeFactoryOutgoingMessage::from_cell(&raw)?,
        CoffeeFactoryOutgoingMessage::WithdrawInternal(_)
    ));
    assert!(CoffeeFactoryOutgoingMessage::from_cell(&opcode_only(0xc0ffee2d)?).is_err());
    let raw = cell_for("factory/CoffeeFactoryUpdateContractInternal/0")?;
    assert!(matches!(
        CoffeeFactoryOutgoingMessage::from_cell(&raw)?,
        CoffeeFactoryOutgoingMessage::UpdateContractInternal(_)
    ));
    assert!(CoffeeFactoryOutgoingMessage::from_cell(&opcode_only(0xc0ffee2e)?).is_err());
    let raw = cell_for("factory/CoffeeFactoryExcesses/0")?;
    assert!(matches!(
        CoffeeFactoryOutgoingMessage::from_cell(&raw)?,
        CoffeeFactoryOutgoingMessage::Excesses(_)
    ));
    assert!(CoffeeFactoryOutgoingMessage::from_cell(&opcode_only(0xd53276db)?).is_err());
    let raw = unknown_body()?;
    let decoded = CoffeeFactoryOutgoingMessage::from_cell(&raw)?;
    assert!(matches!(decoded, CoffeeFactoryOutgoingMessage::Unknown(_)));
    assert_eq!(decoded.to_cell()?, raw);
    Ok(())
}
#[test]
fn test_init_role_dispatch() -> anyhow::Result<()> {
    let raw = cell_for("init/CoffeeInitDeploy/0")?;
    assert!(matches!(
        CoffeeInitIncomingMessage::from_cell(&raw)?,
        CoffeeInitIncomingMessage::Deploy(_)
    ));
    assert!(CoffeeInitIncomingMessage::from_cell(&opcode_only(0xc0ffee22)?).is_err());
    let raw = unknown_body()?;
    let decoded = CoffeeInitIncomingMessage::from_cell(&raw)?;
    assert!(matches!(decoded, CoffeeInitIncomingMessage::Unknown(_)));
    assert_eq!(decoded.to_cell()?, raw);
    Ok(())
}
#[test]
fn test_liquidity_depository_role_dispatch() -> anyhow::Result<()> {
    let raw = cell_for("liquidity_depository/CoffeeLiquidityDepositoryCreateLiquidityDepositoryInternal/0")?;
    assert!(matches!(
        CoffeeLiquidityDepositoryIncomingMessage::from_cell(&raw)?,
        CoffeeLiquidityDepositoryIncomingMessage::CreateLiquidityDepositoryInternal(_)
    ));
    assert!(CoffeeLiquidityDepositoryIncomingMessage::from_cell(&opcode_only(0xc0ffee28)?).is_err());
    let raw = cell_for("liquidity_depository/CoffeeLiquidityDepositoryDeploy/0")?;
    assert!(matches!(
        CoffeeLiquidityDepositoryIncomingMessage::from_cell(&raw)?,
        CoffeeLiquidityDepositoryIncomingMessage::Deploy(_)
    ));
    assert!(CoffeeLiquidityDepositoryIncomingMessage::from_cell(&opcode_only(0xc0ffee22)?).is_err());
    let raw = cell_for("liquidity_depository/CoffeeLiquidityDepositoryWithdrawDeposit/0")?;
    assert!(matches!(
        CoffeeLiquidityDepositoryIncomingMessage::from_cell(&raw)?,
        CoffeeLiquidityDepositoryIncomingMessage::WithdrawDeposit(_)
    ));
    assert!(CoffeeLiquidityDepositoryIncomingMessage::from_cell(&opcode_only(0xc0ffee07)?).is_err());
    let raw = cell_for("liquidity_depository/CoffeeLiquidityDepositoryUpdateContractInternal/0")?;
    assert!(matches!(
        CoffeeLiquidityDepositoryIncomingMessage::from_cell(&raw)?,
        CoffeeLiquidityDepositoryIncomingMessage::UpdateContractInternal(_)
    ));
    assert!(CoffeeLiquidityDepositoryIncomingMessage::from_cell(&opcode_only(0xc0ffee2e)?).is_err());
    let raw = unknown_body()?;
    let decoded = CoffeeLiquidityDepositoryIncomingMessage::from_cell(&raw)?;
    assert!(matches!(decoded, CoffeeLiquidityDepositoryIncomingMessage::Unknown(_)));
    assert_eq!(decoded.to_cell()?, raw);
    let raw = cell_for("liquidity_depository/CoffeeLiquidityDepositoryDepositLiquidityInternal/0")?;
    assert!(matches!(
        CoffeeLiquidityDepositoryOutgoingMessage::from_cell(&raw)?,
        CoffeeLiquidityDepositoryOutgoingMessage::DepositLiquidityInternal(_)
    ));
    assert!(CoffeeLiquidityDepositoryOutgoingMessage::from_cell(&opcode_only(0xc0ffee29)?).is_err());
    let raw = cell_for("liquidity_depository/CoffeeLiquidityDepositoryPayoutInternal/0")?;
    assert!(matches!(
        CoffeeLiquidityDepositoryOutgoingMessage::from_cell(&raw)?,
        CoffeeLiquidityDepositoryOutgoingMessage::PayoutInternal(_)
    ));
    assert!(CoffeeLiquidityDepositoryOutgoingMessage::from_cell(&opcode_only(0xc0ffee21)?).is_err());
    let raw = cell_for("liquidity_depository/CoffeeLiquidityDepositoryExcesses/0")?;
    assert!(matches!(
        CoffeeLiquidityDepositoryOutgoingMessage::from_cell(&raw)?,
        CoffeeLiquidityDepositoryOutgoingMessage::Excesses(_)
    ));
    assert!(CoffeeLiquidityDepositoryOutgoingMessage::from_cell(&opcode_only(0xd53276db)?).is_err());
    let raw = unknown_body()?;
    let decoded = CoffeeLiquidityDepositoryOutgoingMessage::from_cell(&raw)?;
    assert!(matches!(decoded, CoffeeLiquidityDepositoryOutgoingMessage::Unknown(_)));
    assert_eq!(decoded.to_cell()?, raw);
    Ok(())
}
#[test]
fn test_lp_wallet_role_dispatch() -> anyhow::Result<()> {
    let raw = cell_for("lp_wallet/JettonWalletCoffeeLpTransfer/0")?;
    assert!(matches!(
        CoffeeLpWalletIncomingMessage::from_cell(&raw)?,
        CoffeeLpWalletIncomingMessage::Transfer(_)
    ));
    assert!(CoffeeLpWalletIncomingMessage::from_cell(&opcode_only(0x0f8a7ea5)?).is_err());
    let raw = cell_for("lp_wallet/JettonWalletCoffeeLpInternalTransfer/0")?;
    assert!(matches!(
        CoffeeLpWalletIncomingMessage::from_cell(&raw)?,
        CoffeeLpWalletIncomingMessage::InternalTransfer(_)
    ));
    assert!(CoffeeLpWalletIncomingMessage::from_cell(&opcode_only(0x178d4519)?).is_err());
    let raw = cell_for("lp_wallet/JettonWalletCoffeeLpBurn/0")?;
    assert!(matches!(
        CoffeeLpWalletIncomingMessage::from_cell(&raw)?,
        CoffeeLpWalletIncomingMessage::Burn(_)
    ));
    assert!(CoffeeLpWalletIncomingMessage::from_cell(&opcode_only(0x595f07bc)?).is_err());
    let raw = unknown_body()?;
    let decoded = CoffeeLpWalletIncomingMessage::from_cell(&raw)?;
    assert!(matches!(decoded, CoffeeLpWalletIncomingMessage::Unknown(_)));
    assert_eq!(decoded.to_cell()?, raw);
    let raw = cell_for("lp_wallet/JettonWalletCoffeeLpInternalTransfer/0")?;
    assert!(matches!(
        CoffeeLpWalletOutgoingMessage::from_cell(&raw)?,
        CoffeeLpWalletOutgoingMessage::InternalTransfer(_)
    ));
    assert!(CoffeeLpWalletOutgoingMessage::from_cell(&opcode_only(0x178d4519)?).is_err());
    let raw = cell_for("lp_wallet/JettonWalletCoffeeLpTransferNotification/0")?;
    assert!(matches!(
        CoffeeLpWalletOutgoingMessage::from_cell(&raw)?,
        CoffeeLpWalletOutgoingMessage::TransferNotification(_)
    ));
    assert!(CoffeeLpWalletOutgoingMessage::from_cell(&opcode_only(0x7362d09c)?).is_err());
    let raw = cell_for("lp_wallet/JettonWalletCoffeeLpExcesses/0")?;
    assert!(matches!(
        CoffeeLpWalletOutgoingMessage::from_cell(&raw)?,
        CoffeeLpWalletOutgoingMessage::Excesses(_)
    ));
    assert!(CoffeeLpWalletOutgoingMessage::from_cell(&opcode_only(0xd53276db)?).is_err());
    let raw = cell_for("lp_wallet/JettonWalletCoffeeLpBurnNotification/0")?;
    assert!(matches!(
        CoffeeLpWalletOutgoingMessage::from_cell(&raw)?,
        CoffeeLpWalletOutgoingMessage::BurnNotification(_)
    ));
    assert!(CoffeeLpWalletOutgoingMessage::from_cell(&opcode_only(0x7bdd97de)?).is_err());
    let raw = unknown_body()?;
    let decoded = CoffeeLpWalletOutgoingMessage::from_cell(&raw)?;
    assert!(matches!(decoded, CoffeeLpWalletOutgoingMessage::Unknown(_)));
    assert_eq!(decoded.to_cell()?, raw);
    Ok(())
}
#[test]
fn test_mev_protector_role_dispatch() -> anyhow::Result<()> {
    let raw = cell_for("mev_protector/CoffeeMevProtectorJettonTransferNotification/0")?;
    assert!(matches!(
        CoffeeMevProtectorIncomingMessage::from_cell(&raw)?,
        CoffeeMevProtectorIncomingMessage::JettonTransferNotification(_)
    ));
    assert!(CoffeeMevProtectorIncomingMessage::from_cell(&opcode_only(0x7362d09c)?).is_err());
    let raw = cell_for("mev_protector/CoffeeMevProtectHoldFunds/0")?;
    assert!(matches!(
        CoffeeMevProtectorIncomingMessage::from_cell(&raw)?,
        CoffeeMevProtectorIncomingMessage::CoffeeMevProtectHoldFunds(_)
    ));
    assert!(CoffeeMevProtectorIncomingMessage::from_cell(&opcode_only(0x6bc79e7e)?).is_err());
    let raw = cell_for("mev_protector/CoffeeMevProtectorDedustPayout/0")?;
    assert!(matches!(
        CoffeeMevProtectorIncomingMessage::from_cell(&raw)?,
        CoffeeMevProtectorIncomingMessage::DedustPayout(_)
    ));
    assert!(CoffeeMevProtectorIncomingMessage::from_cell(&opcode_only(0x474f86cf)?).is_err());
    let raw = cell_for("mev_protector/CoffeeMevProtectorCoffeeNotification/0")?;
    assert!(matches!(
        CoffeeMevProtectorIncomingMessage::from_cell(&raw)?,
        CoffeeMevProtectorIncomingMessage::CoffeeNotification(_)
    ));
    assert!(CoffeeMevProtectorIncomingMessage::from_cell(&opcode_only(0xc0ffee36)?).is_err());
    let raw = cell_for("mev_protector/CoffeeMevProtectorSetAdmin/0")?;
    assert!(matches!(
        CoffeeMevProtectorIncomingMessage::from_cell(&raw)?,
        CoffeeMevProtectorIncomingMessage::SetAdmin(_)
    ));
    assert!(CoffeeMevProtectorIncomingMessage::from_cell(&opcode_only(0x57861aa7)?).is_err());
    let raw = cell_for("mev_protector/CoffeeMevProtectorSetPublicKey/0")?;
    assert!(matches!(
        CoffeeMevProtectorIncomingMessage::from_cell(&raw)?,
        CoffeeMevProtectorIncomingMessage::SetPublicKey(_)
    ));
    assert!(CoffeeMevProtectorIncomingMessage::from_cell(&opcode_only(0xb19e2647)?).is_err());
    let raw = cell_for("mev_protector/CoffeeMevProtectorUpdateContract/0")?;
    assert!(matches!(
        CoffeeMevProtectorIncomingMessage::from_cell(&raw)?,
        CoffeeMevProtectorIncomingMessage::UpdateContract(_)
    ));
    assert!(CoffeeMevProtectorIncomingMessage::from_cell(&opcode_only(0x6f80872c)?).is_err());
    let raw = cell_for("mev_protector/CoffeeMevProtectorNativeRescue/0")?;
    assert!(matches!(
        CoffeeMevProtectorIncomingMessage::from_cell(&raw)?,
        CoffeeMevProtectorIncomingMessage::NativeRescue(_)
    ));
    assert!(CoffeeMevProtectorIncomingMessage::from_cell(&opcode_only(0x2bf71be6)?).is_err());
    let raw = cell_for("mev_protector/CoffeeMevProtectorJettonRescue/0")?;
    assert!(matches!(
        CoffeeMevProtectorIncomingMessage::from_cell(&raw)?,
        CoffeeMevProtectorIncomingMessage::JettonRescue(_)
    ));
    assert!(CoffeeMevProtectorIncomingMessage::from_cell(&opcode_only(0x7b4b82fe)?).is_err());
    let raw = cell_for("mev_protector/CoffeeMevProtectorExcesses/0")?;
    assert!(matches!(
        CoffeeMevProtectorIncomingMessage::from_cell(&raw)?,
        CoffeeMevProtectorIncomingMessage::Excesses(_)
    ));
    assert!(CoffeeMevProtectorIncomingMessage::from_cell(&opcode_only(0xd53276db)?).is_err());
    let raw = unknown_body()?;
    let decoded = CoffeeMevProtectorIncomingMessage::from_cell(&raw)?;
    assert!(matches!(decoded, CoffeeMevProtectorIncomingMessage::Unknown(_)));
    assert_eq!(decoded.to_cell()?, raw);
    let raw = cell_for("mev_protector/CoffeeMevProtectorJettonTransfer/0")?;
    assert!(matches!(
        CoffeeMevProtectorOutgoingMessage::from_cell(&raw)?,
        CoffeeMevProtectorOutgoingMessage::JettonTransfer(_)
    ));
    assert!(CoffeeMevProtectorOutgoingMessage::from_cell(&opcode_only(0x0f8a7ea5)?).is_err());
    let raw = cell_for("mev_protector/CoffeeMevProtectorExcesses/0")?;
    assert!(matches!(
        CoffeeMevProtectorOutgoingMessage::from_cell(&raw)?,
        CoffeeMevProtectorOutgoingMessage::Excesses(_)
    ));
    assert!(CoffeeMevProtectorOutgoingMessage::from_cell(&opcode_only(0xd53276db)?).is_err());
    let raw = unknown_body()?;
    let decoded = CoffeeMevProtectorOutgoingMessage::from_cell(&raw)?;
    assert!(matches!(decoded, CoffeeMevProtectorOutgoingMessage::Unknown(_)));
    assert_eq!(decoded.to_cell()?, raw);
    Ok(())
}
#[test]
fn test_mev_protector_legacy_role_dispatch() -> anyhow::Result<()> {
    let raw = cell_for("mev_protector_legacy/CoffeeMevProtectorLegacyJettonTransferNotification/0")?;
    assert!(matches!(
        CoffeeMevProtectorLegacyIncomingMessage::from_cell(&raw)?,
        CoffeeMevProtectorLegacyIncomingMessage::JettonTransferNotification(_)
    ));
    assert!(CoffeeMevProtectorLegacyIncomingMessage::from_cell(&opcode_only(0x7362d09c)?).is_err());
    let raw = cell_for("mev_protector_legacy/CoffeeMevProtectorLegacyHoldFunds/0")?;
    assert!(matches!(
        CoffeeMevProtectorLegacyIncomingMessage::from_cell(&raw)?,
        CoffeeMevProtectorLegacyIncomingMessage::HoldFunds(_)
    ));
    assert!(CoffeeMevProtectorLegacyIncomingMessage::from_cell(&opcode_only(0x6bc79e7e)?).is_err());
    let raw = cell_for("mev_protector_legacy/CoffeeMevProtectorLegacyDedustPayout/0")?;
    assert!(matches!(
        CoffeeMevProtectorLegacyIncomingMessage::from_cell(&raw)?,
        CoffeeMevProtectorLegacyIncomingMessage::DedustPayout(_)
    ));
    assert!(CoffeeMevProtectorLegacyIncomingMessage::from_cell(&opcode_only(0x474f86cf)?).is_err());
    let raw = cell_for("mev_protector_legacy/CoffeeMevProtectorLegacySetAdmin/0")?;
    assert!(matches!(
        CoffeeMevProtectorLegacyIncomingMessage::from_cell(&raw)?,
        CoffeeMevProtectorLegacyIncomingMessage::SetAdmin(_)
    ));
    assert!(CoffeeMevProtectorLegacyIncomingMessage::from_cell(&opcode_only(0x57861aa7)?).is_err());
    let raw = cell_for("mev_protector_legacy/CoffeeMevProtectorLegacySetPublicKey/0")?;
    assert!(matches!(
        CoffeeMevProtectorLegacyIncomingMessage::from_cell(&raw)?,
        CoffeeMevProtectorLegacyIncomingMessage::SetPublicKey(_)
    ));
    assert!(CoffeeMevProtectorLegacyIncomingMessage::from_cell(&opcode_only(0xb19e2647)?).is_err());
    let raw = cell_for("mev_protector_legacy/CoffeeMevProtectorLegacyUpdateContract/0")?;
    assert!(matches!(
        CoffeeMevProtectorLegacyIncomingMessage::from_cell(&raw)?,
        CoffeeMevProtectorLegacyIncomingMessage::UpdateContract(_)
    ));
    assert!(CoffeeMevProtectorLegacyIncomingMessage::from_cell(&opcode_only(0x6f80872c)?).is_err());
    let raw = cell_for("mev_protector_legacy/CoffeeMevProtectorLegacyAdmin2bf71be6/0")?;
    assert!(matches!(
        CoffeeMevProtectorLegacyIncomingMessage::from_cell(&raw)?,
        CoffeeMevProtectorLegacyIncomingMessage::Admin2bf71be6(_)
    ));
    assert!(CoffeeMevProtectorLegacyIncomingMessage::from_cell(&opcode_only(0x2bf71be6)?).is_err());
    let raw = cell_for("mev_protector_legacy/CoffeeMevProtectorLegacyAdmin7b4b82fe/0")?;
    assert!(matches!(
        CoffeeMevProtectorLegacyIncomingMessage::from_cell(&raw)?,
        CoffeeMevProtectorLegacyIncomingMessage::Admin7b4b82fe(_)
    ));
    assert!(CoffeeMevProtectorLegacyIncomingMessage::from_cell(&opcode_only(0x7b4b82fe)?).is_err());
    let raw = cell_for("mev_protector_legacy/CoffeeMevProtectorLegacyExcesses/0")?;
    assert!(matches!(
        CoffeeMevProtectorLegacyIncomingMessage::from_cell(&raw)?,
        CoffeeMevProtectorLegacyIncomingMessage::Excesses(_)
    ));
    assert!(CoffeeMevProtectorLegacyIncomingMessage::from_cell(&opcode_only(0xd53276db)?).is_err());
    let raw = unknown_body()?;
    let decoded = CoffeeMevProtectorLegacyIncomingMessage::from_cell(&raw)?;
    assert!(matches!(decoded, CoffeeMevProtectorLegacyIncomingMessage::Unknown(_)));
    assert_eq!(decoded.to_cell()?, raw);
    let raw = cell_for("mev_protector_legacy/CoffeeMevProtectorLegacyJettonTransfer/0")?;
    assert!(matches!(
        CoffeeMevProtectorLegacyOutgoingMessage::from_cell(&raw)?,
        CoffeeMevProtectorLegacyOutgoingMessage::JettonTransfer(_)
    ));
    assert!(CoffeeMevProtectorLegacyOutgoingMessage::from_cell(&opcode_only(0x0f8a7ea5)?).is_err());
    let raw = cell_for("mev_protector_legacy/CoffeeMevProtectorLegacyExcesses/0")?;
    assert!(matches!(
        CoffeeMevProtectorLegacyOutgoingMessage::from_cell(&raw)?,
        CoffeeMevProtectorLegacyOutgoingMessage::Excesses(_)
    ));
    assert!(CoffeeMevProtectorLegacyOutgoingMessage::from_cell(&opcode_only(0xd53276db)?).is_err());
    let raw = unknown_body()?;
    let decoded = CoffeeMevProtectorLegacyOutgoingMessage::from_cell(&raw)?;
    assert!(matches!(decoded, CoffeeMevProtectorLegacyOutgoingMessage::Unknown(_)));
    assert_eq!(decoded.to_cell()?, raw);
    Ok(())
}
#[test]
fn test_pool_constant_product_role_dispatch() -> anyhow::Result<()> {
    let raw = cell_for("pool_constant_product/CoffeePoolConstantProductSwapInternal/0")?;
    assert!(matches!(
        CoffeeConstantProductPoolIncomingMessage::from_cell(&raw)?,
        CoffeeConstantProductPoolIncomingMessage::SwapInternal(_)
    ));
    assert!(CoffeeConstantProductPoolIncomingMessage::from_cell(&opcode_only(0xc0ffee20)?).is_err());
    let raw = cell_for("pool_constant_product/CoffeePoolConstantProductDepositLiquidityInternal/0")?;
    assert!(matches!(
        CoffeeConstantProductPoolIncomingMessage::from_cell(&raw)?,
        CoffeeConstantProductPoolIncomingMessage::DepositLiquidityInternal(_)
    ));
    assert!(CoffeeConstantProductPoolIncomingMessage::from_cell(&opcode_only(0xc0ffee29)?).is_err());
    let raw = cell_for("pool_constant_product/CoffeePoolConstantProductJettonBurnNotification/0")?;
    assert!(matches!(
        CoffeeConstantProductPoolIncomingMessage::from_cell(&raw)?,
        CoffeeConstantProductPoolIncomingMessage::JettonBurnNotification(_)
    ));
    assert!(CoffeeConstantProductPoolIncomingMessage::from_cell(&opcode_only(0x7bdd97de)?).is_err());
    let raw = cell_for("pool_constant_product/CoffeePoolConstantProductProvideWalletAddress/0")?;
    assert!(matches!(
        CoffeeConstantProductPoolIncomingMessage::from_cell(&raw)?,
        CoffeeConstantProductPoolIncomingMessage::ProvideWalletAddress(_)
    ));
    assert!(CoffeeConstantProductPoolIncomingMessage::from_cell(&opcode_only(0x2c76b973)?).is_err());
    let raw = cell_for("pool_constant_product/CoffeePoolConstantProductWithdrawInternal/0")?;
    assert!(matches!(
        CoffeeConstantProductPoolIncomingMessage::from_cell(&raw)?,
        CoffeeConstantProductPoolIncomingMessage::WithdrawInternal(_)
    ));
    assert!(CoffeeConstantProductPoolIncomingMessage::from_cell(&opcode_only(0xc0ffee2d)?).is_err());
    let raw = cell_for("pool_constant_product/CoffeePoolConstantProductUpdatePoolInternal/0")?;
    assert!(matches!(
        CoffeeConstantProductPoolIncomingMessage::from_cell(&raw)?,
        CoffeeConstantProductPoolIncomingMessage::UpdatePoolInternal(_)
    ));
    assert!(CoffeeConstantProductPoolIncomingMessage::from_cell(&opcode_only(0xc0ffee2b)?).is_err());
    let raw = cell_for("pool_constant_product/CoffeePoolConstantProductUpdateContractInternal/0")?;
    assert!(matches!(
        CoffeeConstantProductPoolIncomingMessage::from_cell(&raw)?,
        CoffeeConstantProductPoolIncomingMessage::UpdateContractInternal(_)
    ));
    assert!(CoffeeConstantProductPoolIncomingMessage::from_cell(&opcode_only(0xc0ffee2e)?).is_err());
    let raw = cell_for("pool_constant_product/CoffeePoolConstantProductCreatePoolInternal/0")?;
    assert!(matches!(
        CoffeeConstantProductPoolIncomingMessage::from_cell(&raw)?,
        CoffeeConstantProductPoolIncomingMessage::CreatePoolInternal(_)
    ));
    assert!(CoffeeConstantProductPoolIncomingMessage::from_cell(&opcode_only(0xc0ffee26)?).is_err());
    let raw = cell_for("pool_constant_product/CoffeePoolConstantProductDeploy/0")?;
    assert!(matches!(
        CoffeeConstantProductPoolIncomingMessage::from_cell(&raw)?,
        CoffeeConstantProductPoolIncomingMessage::Deploy(_)
    ));
    assert!(CoffeeConstantProductPoolIncomingMessage::from_cell(&opcode_only(0xc0ffee22)?).is_err());
    let raw = unknown_body()?;
    let decoded = CoffeeConstantProductPoolIncomingMessage::from_cell(&raw)?;
    assert!(matches!(decoded, CoffeeConstantProductPoolIncomingMessage::Unknown(_)));
    assert_eq!(decoded.to_cell()?, raw);
    let raw = cell_for("pool_constant_product/CoffeePoolConstantProductPayoutInternal/0")?;
    assert!(matches!(
        CoffeeConstantProductPoolOutgoingMessage::from_cell(&raw)?,
        CoffeeConstantProductPoolOutgoingMessage::PayoutInternal(_)
    ));
    assert!(CoffeeConstantProductPoolOutgoingMessage::from_cell(&opcode_only(0xc0ffee21)?).is_err());
    let raw = cell_for("pool_constant_product/CoffeePoolConstantProductSwapInternalOut/0")?;
    assert!(matches!(
        CoffeeConstantProductPoolOutgoingMessage::from_cell(&raw)?,
        CoffeeConstantProductPoolOutgoingMessage::SwapInternalOut(_)
    ));
    assert!(CoffeeConstantProductPoolOutgoingMessage::from_cell(&opcode_only(0xc0ffee20)?).is_err());
    let raw = cell_for("pool_constant_product/CoffeePoolConstantProductNotification/0")?;
    assert!(matches!(
        CoffeeConstantProductPoolOutgoingMessage::from_cell(&raw)?,
        CoffeeConstantProductPoolOutgoingMessage::Notification(_)
    ));
    assert!(CoffeeConstantProductPoolOutgoingMessage::from_cell(&opcode_only(0xc0ffee36)?).is_err());
    let raw = cell_for("pool_constant_product/CoffeePoolConstantProductSwapSucceeded/0")?;
    assert!(matches!(
        CoffeeConstantProductPoolOutgoingMessage::from_cell(&raw)?,
        CoffeeConstantProductPoolOutgoingMessage::SwapSucceeded(_)
    ));
    assert!(CoffeeConstantProductPoolOutgoingMessage::from_cell(&opcode_only(0xc0ffee30)?).is_err());
    let raw = cell_for("pool_constant_product/CoffeePoolConstantProductSwapFailed/0")?;
    assert!(matches!(
        CoffeeConstantProductPoolOutgoingMessage::from_cell(&raw)?,
        CoffeeConstantProductPoolOutgoingMessage::SwapFailed(_)
    ));
    assert!(CoffeeConstantProductPoolOutgoingMessage::from_cell(&opcode_only(0xc0ffee31)?).is_err());
    let raw = cell_for("pool_constant_product/CoffeePoolConstantProductDepositLiquiditySucceeded/0")?;
    assert!(matches!(
        CoffeeConstantProductPoolOutgoingMessage::from_cell(&raw)?,
        CoffeeConstantProductPoolOutgoingMessage::DepositLiquiditySucceeded(_)
    ));
    assert!(CoffeeConstantProductPoolOutgoingMessage::from_cell(&opcode_only(0xc0ffee33)?).is_err());
    let raw = cell_for("pool_constant_product/CoffeePoolConstantProductDepositLiquidityFailed/0")?;
    assert!(matches!(
        CoffeeConstantProductPoolOutgoingMessage::from_cell(&raw)?,
        CoffeeConstantProductPoolOutgoingMessage::DepositLiquidityFailed(_)
    ));
    assert!(CoffeeConstantProductPoolOutgoingMessage::from_cell(&opcode_only(0xc0ffee34)?).is_err());
    let raw = cell_for("pool_constant_product/CoffeePoolConstantProductWithdrawLiquiditySucceeded/0")?;
    assert!(matches!(
        CoffeeConstantProductPoolOutgoingMessage::from_cell(&raw)?,
        CoffeeConstantProductPoolOutgoingMessage::WithdrawLiquiditySucceeded(_)
    ));
    assert!(CoffeeConstantProductPoolOutgoingMessage::from_cell(&opcode_only(0xc0ffee35)?).is_err());
    let raw = cell_for("pool_constant_product/CoffeePoolConstantProductTakeWalletAddress/0")?;
    assert!(matches!(
        CoffeeConstantProductPoolOutgoingMessage::from_cell(&raw)?,
        CoffeeConstantProductPoolOutgoingMessage::TakeWalletAddress(_)
    ));
    assert!(CoffeeConstantProductPoolOutgoingMessage::from_cell(&opcode_only(0xd1735400)?).is_err());
    let raw = cell_for("pool_constant_product/CoffeePoolConstantProductExcesses/0")?;
    assert!(matches!(
        CoffeeConstantProductPoolOutgoingMessage::from_cell(&raw)?,
        CoffeeConstantProductPoolOutgoingMessage::Excesses(_)
    ));
    assert!(CoffeeConstantProductPoolOutgoingMessage::from_cell(&opcode_only(0xd53276db)?).is_err());
    let raw = cell_for("pool_constant_product/CoffeePoolConstantProductLpWalletInternalTransfer/0")?;
    assert!(matches!(
        CoffeeConstantProductPoolOutgoingMessage::from_cell(&raw)?,
        CoffeeConstantProductPoolOutgoingMessage::LpWalletInternalTransfer(_)
    ));
    assert!(CoffeeConstantProductPoolOutgoingMessage::from_cell(&opcode_only(0x178d4519)?).is_err());
    let raw = unknown_body()?;
    let decoded = CoffeeConstantProductPoolOutgoingMessage::from_cell(&raw)?;
    assert!(matches!(decoded, CoffeeConstantProductPoolOutgoingMessage::Unknown(_)));
    assert_eq!(decoded.to_cell()?, raw);
    Ok(())
}
#[test]
fn test_pool_creator_role_dispatch() -> anyhow::Result<()> {
    let raw = cell_for("pool_creator/CoffeePoolCreatorCreatePoolCreatorInternal/0")?;
    assert!(matches!(
        CoffeePoolCreatorIncomingMessage::from_cell(&raw)?,
        CoffeePoolCreatorIncomingMessage::CreatePoolCreatorInternal(_)
    ));
    assert!(CoffeePoolCreatorIncomingMessage::from_cell(&opcode_only(0xc0ffee24)?).is_err());
    let raw = cell_for("pool_creator/CoffeePoolCreatorDeploy/0")?;
    assert!(matches!(
        CoffeePoolCreatorIncomingMessage::from_cell(&raw)?,
        CoffeePoolCreatorIncomingMessage::Deploy(_)
    ));
    assert!(CoffeePoolCreatorIncomingMessage::from_cell(&opcode_only(0xc0ffee22)?).is_err());
    let raw = cell_for("pool_creator/CoffeePoolCreatorWithdrawDeposit/0")?;
    assert!(matches!(
        CoffeePoolCreatorIncomingMessage::from_cell(&raw)?,
        CoffeePoolCreatorIncomingMessage::WithdrawDeposit(_)
    ));
    assert!(CoffeePoolCreatorIncomingMessage::from_cell(&opcode_only(0xc0ffee07)?).is_err());
    let raw = cell_for("pool_creator/CoffeePoolCreatorUpdateContractInternal/0")?;
    assert!(matches!(
        CoffeePoolCreatorIncomingMessage::from_cell(&raw)?,
        CoffeePoolCreatorIncomingMessage::UpdateContractInternal(_)
    ));
    assert!(CoffeePoolCreatorIncomingMessage::from_cell(&opcode_only(0xc0ffee2e)?).is_err());
    let raw = unknown_body()?;
    let decoded = CoffeePoolCreatorIncomingMessage::from_cell(&raw)?;
    assert!(matches!(decoded, CoffeePoolCreatorIncomingMessage::Unknown(_)));
    assert_eq!(decoded.to_cell()?, raw);
    let raw = cell_for("pool_creator/CoffeePoolCreatorCreatePoolRequest/0")?;
    assert!(matches!(
        CoffeePoolCreatorOutgoingMessage::from_cell(&raw)?,
        CoffeePoolCreatorOutgoingMessage::CreatePoolRequest(_)
    ));
    assert!(CoffeePoolCreatorOutgoingMessage::from_cell(&opcode_only(0xc0ffee25)?).is_err());
    let raw = cell_for("pool_creator/CoffeePoolCreatorPayoutInternal/0")?;
    assert!(matches!(
        CoffeePoolCreatorOutgoingMessage::from_cell(&raw)?,
        CoffeePoolCreatorOutgoingMessage::PayoutInternal(_)
    ));
    assert!(CoffeePoolCreatorOutgoingMessage::from_cell(&opcode_only(0xc0ffee21)?).is_err());
    let raw = cell_for("pool_creator/CoffeePoolCreatorExcesses/0")?;
    assert!(matches!(
        CoffeePoolCreatorOutgoingMessage::from_cell(&raw)?,
        CoffeePoolCreatorOutgoingMessage::Excesses(_)
    ));
    assert!(CoffeePoolCreatorOutgoingMessage::from_cell(&opcode_only(0xd53276db)?).is_err());
    let raw = unknown_body()?;
    let decoded = CoffeePoolCreatorOutgoingMessage::from_cell(&raw)?;
    assert!(matches!(decoded, CoffeePoolCreatorOutgoingMessage::Unknown(_)));
    assert_eq!(decoded.to_cell()?, raw);
    Ok(())
}
#[test]
fn test_pool_curve_fi_stable_role_dispatch() -> anyhow::Result<()> {
    let raw = cell_for("pool_curve_fi_stable/CoffeePoolCurveFiStableSwapInternal/0")?;
    assert!(matches!(
        CoffeeCurveFiStablePoolIncomingMessage::from_cell(&raw)?,
        CoffeeCurveFiStablePoolIncomingMessage::SwapInternal(_)
    ));
    assert!(CoffeeCurveFiStablePoolIncomingMessage::from_cell(&opcode_only(0xc0ffee20)?).is_err());
    let raw = cell_for("pool_curve_fi_stable/CoffeePoolCurveFiStableDepositLiquidityInternal/0")?;
    assert!(matches!(
        CoffeeCurveFiStablePoolIncomingMessage::from_cell(&raw)?,
        CoffeeCurveFiStablePoolIncomingMessage::DepositLiquidityInternal(_)
    ));
    assert!(CoffeeCurveFiStablePoolIncomingMessage::from_cell(&opcode_only(0xc0ffee29)?).is_err());
    let raw = cell_for("pool_curve_fi_stable/CoffeePoolCurveFiStableJettonBurnNotification/0")?;
    assert!(matches!(
        CoffeeCurveFiStablePoolIncomingMessage::from_cell(&raw)?,
        CoffeeCurveFiStablePoolIncomingMessage::JettonBurnNotification(_)
    ));
    assert!(CoffeeCurveFiStablePoolIncomingMessage::from_cell(&opcode_only(0x7bdd97de)?).is_err());
    let raw = cell_for("pool_curve_fi_stable/CoffeePoolCurveFiStableProvideWalletAddress/0")?;
    assert!(matches!(
        CoffeeCurveFiStablePoolIncomingMessage::from_cell(&raw)?,
        CoffeeCurveFiStablePoolIncomingMessage::ProvideWalletAddress(_)
    ));
    assert!(CoffeeCurveFiStablePoolIncomingMessage::from_cell(&opcode_only(0x2c76b973)?).is_err());
    let raw = cell_for("pool_curve_fi_stable/CoffeePoolCurveFiStableWithdrawInternal/0")?;
    assert!(matches!(
        CoffeeCurveFiStablePoolIncomingMessage::from_cell(&raw)?,
        CoffeeCurveFiStablePoolIncomingMessage::WithdrawInternal(_)
    ));
    assert!(CoffeeCurveFiStablePoolIncomingMessage::from_cell(&opcode_only(0xc0ffee2d)?).is_err());
    let raw = cell_for("pool_curve_fi_stable/CoffeePoolCurveFiStableUpdatePoolInternal/0")?;
    assert!(matches!(
        CoffeeCurveFiStablePoolIncomingMessage::from_cell(&raw)?,
        CoffeeCurveFiStablePoolIncomingMessage::UpdatePoolInternal(_)
    ));
    assert!(CoffeeCurveFiStablePoolIncomingMessage::from_cell(&opcode_only(0xc0ffee2b)?).is_err());
    let raw = cell_for("pool_curve_fi_stable/CoffeePoolCurveFiStableUpdateContractInternal/0")?;
    assert!(matches!(
        CoffeeCurveFiStablePoolIncomingMessage::from_cell(&raw)?,
        CoffeeCurveFiStablePoolIncomingMessage::UpdateContractInternal(_)
    ));
    assert!(CoffeeCurveFiStablePoolIncomingMessage::from_cell(&opcode_only(0xc0ffee2e)?).is_err());
    let raw = cell_for("pool_curve_fi_stable/CoffeePoolCurveFiStableCreatePoolInternal/0")?;
    assert!(matches!(
        CoffeeCurveFiStablePoolIncomingMessage::from_cell(&raw)?,
        CoffeeCurveFiStablePoolIncomingMessage::CreatePoolInternal(_)
    ));
    assert!(CoffeeCurveFiStablePoolIncomingMessage::from_cell(&opcode_only(0xc0ffee26)?).is_err());
    let raw = cell_for("pool_curve_fi_stable/CoffeePoolCurveFiStableDeploy/0")?;
    assert!(matches!(
        CoffeeCurveFiStablePoolIncomingMessage::from_cell(&raw)?,
        CoffeeCurveFiStablePoolIncomingMessage::Deploy(_)
    ));
    assert!(CoffeeCurveFiStablePoolIncomingMessage::from_cell(&opcode_only(0xc0ffee22)?).is_err());
    let raw = unknown_body()?;
    let decoded = CoffeeCurveFiStablePoolIncomingMessage::from_cell(&raw)?;
    assert!(matches!(decoded, CoffeeCurveFiStablePoolIncomingMessage::Unknown(_)));
    assert_eq!(decoded.to_cell()?, raw);
    let raw = cell_for("pool_curve_fi_stable/CoffeePoolCurveFiStablePayoutInternal/0")?;
    assert!(matches!(
        CoffeeCurveFiStablePoolOutgoingMessage::from_cell(&raw)?,
        CoffeeCurveFiStablePoolOutgoingMessage::PayoutInternal(_)
    ));
    assert!(CoffeeCurveFiStablePoolOutgoingMessage::from_cell(&opcode_only(0xc0ffee21)?).is_err());
    let raw = cell_for("pool_curve_fi_stable/CoffeePoolCurveFiStableSwapInternalOut/0")?;
    assert!(matches!(
        CoffeeCurveFiStablePoolOutgoingMessage::from_cell(&raw)?,
        CoffeeCurveFiStablePoolOutgoingMessage::SwapInternalOut(_)
    ));
    assert!(CoffeeCurveFiStablePoolOutgoingMessage::from_cell(&opcode_only(0xc0ffee20)?).is_err());
    let raw = cell_for("pool_curve_fi_stable/CoffeePoolCurveFiStableNotification/0")?;
    assert!(matches!(
        CoffeeCurveFiStablePoolOutgoingMessage::from_cell(&raw)?,
        CoffeeCurveFiStablePoolOutgoingMessage::Notification(_)
    ));
    assert!(CoffeeCurveFiStablePoolOutgoingMessage::from_cell(&opcode_only(0xc0ffee36)?).is_err());
    let raw = cell_for("pool_curve_fi_stable/CoffeePoolCurveFiStableSwapSucceeded/0")?;
    assert!(matches!(
        CoffeeCurveFiStablePoolOutgoingMessage::from_cell(&raw)?,
        CoffeeCurveFiStablePoolOutgoingMessage::SwapSucceeded(_)
    ));
    assert!(CoffeeCurveFiStablePoolOutgoingMessage::from_cell(&opcode_only(0xc0ffee30)?).is_err());
    let raw = cell_for("pool_curve_fi_stable/CoffeePoolCurveFiStableSwapFailed/0")?;
    assert!(matches!(
        CoffeeCurveFiStablePoolOutgoingMessage::from_cell(&raw)?,
        CoffeeCurveFiStablePoolOutgoingMessage::SwapFailed(_)
    ));
    assert!(CoffeeCurveFiStablePoolOutgoingMessage::from_cell(&opcode_only(0xc0ffee31)?).is_err());
    let raw = cell_for("pool_curve_fi_stable/CoffeePoolCurveFiStableDepositLiquiditySucceeded/0")?;
    assert!(matches!(
        CoffeeCurveFiStablePoolOutgoingMessage::from_cell(&raw)?,
        CoffeeCurveFiStablePoolOutgoingMessage::DepositLiquiditySucceeded(_)
    ));
    assert!(CoffeeCurveFiStablePoolOutgoingMessage::from_cell(&opcode_only(0xc0ffee33)?).is_err());
    let raw = cell_for("pool_curve_fi_stable/CoffeePoolCurveFiStableDepositLiquidityFailed/0")?;
    assert!(matches!(
        CoffeeCurveFiStablePoolOutgoingMessage::from_cell(&raw)?,
        CoffeeCurveFiStablePoolOutgoingMessage::DepositLiquidityFailed(_)
    ));
    assert!(CoffeeCurveFiStablePoolOutgoingMessage::from_cell(&opcode_only(0xc0ffee34)?).is_err());
    let raw = cell_for("pool_curve_fi_stable/CoffeePoolCurveFiStableWithdrawLiquiditySucceeded/0")?;
    assert!(matches!(
        CoffeeCurveFiStablePoolOutgoingMessage::from_cell(&raw)?,
        CoffeeCurveFiStablePoolOutgoingMessage::WithdrawLiquiditySucceeded(_)
    ));
    assert!(CoffeeCurveFiStablePoolOutgoingMessage::from_cell(&opcode_only(0xc0ffee35)?).is_err());
    let raw = cell_for("pool_curve_fi_stable/CoffeePoolCurveFiStableTakeWalletAddress/0")?;
    assert!(matches!(
        CoffeeCurveFiStablePoolOutgoingMessage::from_cell(&raw)?,
        CoffeeCurveFiStablePoolOutgoingMessage::TakeWalletAddress(_)
    ));
    assert!(CoffeeCurveFiStablePoolOutgoingMessage::from_cell(&opcode_only(0xd1735400)?).is_err());
    let raw = cell_for("pool_curve_fi_stable/CoffeePoolCurveFiStableExcesses/0")?;
    assert!(matches!(
        CoffeeCurveFiStablePoolOutgoingMessage::from_cell(&raw)?,
        CoffeeCurveFiStablePoolOutgoingMessage::Excesses(_)
    ));
    assert!(CoffeeCurveFiStablePoolOutgoingMessage::from_cell(&opcode_only(0xd53276db)?).is_err());
    let raw = cell_for("pool_curve_fi_stable/CoffeePoolCurveFiStableLpWalletInternalTransfer/0")?;
    assert!(matches!(
        CoffeeCurveFiStablePoolOutgoingMessage::from_cell(&raw)?,
        CoffeeCurveFiStablePoolOutgoingMessage::LpWalletInternalTransfer(_)
    ));
    assert!(CoffeeCurveFiStablePoolOutgoingMessage::from_cell(&opcode_only(0x178d4519)?).is_err());
    let raw = unknown_body()?;
    let decoded = CoffeeCurveFiStablePoolOutgoingMessage::from_cell(&raw)?;
    assert!(matches!(decoded, CoffeeCurveFiStablePoolOutgoingMessage::Unknown(_)));
    assert_eq!(decoded.to_cell()?, raw);
    Ok(())
}
#[test]
fn test_staking_item_role_dispatch() -> anyhow::Result<()> {
    let raw = cell_for("staking_item/CoffeeStakingItemInit/0")?;
    assert!(matches!(
        CoffeeStakingItemIncomingMessage::from_cell(&raw)?,
        CoffeeStakingItemIncomingMessage::Init(_)
    ));
    assert!(CoffeeStakingItemIncomingMessage::from_cell(&opcode_only(0xbe5a7595)?).is_err());
    let raw = cell_for("staking_item/CoffeeStakingItemUnlock/0")?;
    assert!(matches!(
        CoffeeStakingItemIncomingMessage::from_cell(&raw)?,
        CoffeeStakingItemIncomingMessage::Unlock(_)
    ));
    assert!(CoffeeStakingItemIncomingMessage::from_cell(&opcode_only(0xcb03bfaf)?).is_err());
    let raw = cell_for("staking_item/CoffeeStakingItemExtend/0")?;
    assert!(matches!(
        CoffeeStakingItemIncomingMessage::from_cell(&raw)?,
        CoffeeStakingItemIncomingMessage::Extend(_)
    ));
    assert!(CoffeeStakingItemIncomingMessage::from_cell(&opcode_only(0xeb666ce0)?).is_err());
    let raw = unknown_body()?;
    let decoded = CoffeeStakingItemIncomingMessage::from_cell(&raw)?;
    assert!(matches!(decoded, CoffeeStakingItemIncomingMessage::Unknown(_)));
    assert_eq!(decoded.to_cell()?, raw);
    let raw = cell_for("staking_item/CoffeeStakingItemUnlockNotification/0")?;
    assert!(matches!(
        CoffeeStakingItemOutgoingMessage::from_cell(&raw)?,
        CoffeeStakingItemOutgoingMessage::UnlockNotification(_)
    ));
    assert!(CoffeeStakingItemOutgoingMessage::from_cell(&opcode_only(0xcb03bfaf)?).is_err());
    let raw = cell_for("staking_item/CoffeeStakingItemExtendNotification/0")?;
    assert!(matches!(
        CoffeeStakingItemOutgoingMessage::from_cell(&raw)?,
        CoffeeStakingItemOutgoingMessage::ExtendNotification(_)
    ));
    assert!(CoffeeStakingItemOutgoingMessage::from_cell(&opcode_only(0xeb666ce0)?).is_err());
    let raw = unknown_body()?;
    let decoded = CoffeeStakingItemOutgoingMessage::from_cell(&raw)?;
    assert!(matches!(decoded, CoffeeStakingItemOutgoingMessage::Unknown(_)));
    assert_eq!(decoded.to_cell()?, raw);
    Ok(())
}
#[test]
fn test_staking_master_role_dispatch() -> anyhow::Result<()> {
    let raw = cell_for("staking_master/CoffeeStakingMasterInit/0")?;
    assert!(matches!(
        CoffeeStakingMasterIncomingMessage::from_cell(&raw)?,
        CoffeeStakingMasterIncomingMessage::Init(_)
    ));
    assert!(CoffeeStakingMasterIncomingMessage::from_cell(&opcode_only(0xbe5a7595)?).is_err());
    let raw = cell_for("staking_master/CoffeeStakingUpdateRewards/0")?;
    assert!(matches!(
        CoffeeStakingMasterIncomingMessage::from_cell(&raw)?,
        CoffeeStakingMasterIncomingMessage::CoffeeStakingUpdateRewards(_)
    ));
    assert!(CoffeeStakingMasterIncomingMessage::from_cell(&opcode_only(0x0a9577f0)?).is_err());
    let raw = cell_for("staking_master/CoffeeStakingDeposit/0")?;
    assert!(matches!(
        CoffeeStakingMasterIncomingMessage::from_cell(&raw)?,
        CoffeeStakingMasterIncomingMessage::CoffeeStakingDeposit(_)
    ));
    assert!(CoffeeStakingMasterIncomingMessage::from_cell(&opcode_only(0xf9471134)?).is_err());
    let raw = cell_for("staking_master/CoffeeStakingClaimRewards/0")?;
    assert!(matches!(
        CoffeeStakingMasterIncomingMessage::from_cell(&raw)?,
        CoffeeStakingMasterIncomingMessage::CoffeeStakingClaimRewards(_)
    ));
    assert!(CoffeeStakingMasterIncomingMessage::from_cell(&opcode_only(0xb30c7310)?).is_err());
    let raw = cell_for("staking_master/CoffeeStakingSetDistributor/0")?;
    assert!(matches!(
        CoffeeStakingMasterIncomingMessage::from_cell(&raw)?,
        CoffeeStakingMasterIncomingMessage::CoffeeStakingSetDistributor(_)
    ));
    assert!(CoffeeStakingMasterIncomingMessage::from_cell(&opcode_only(0x27ee4c7d)?).is_err());
    let raw = cell_for("staking_master/CoffeeStakingSetOwner/0")?;
    assert!(matches!(
        CoffeeStakingMasterIncomingMessage::from_cell(&raw)?,
        CoffeeStakingMasterIncomingMessage::CoffeeStakingSetOwner(_)
    ));
    assert!(CoffeeStakingMasterIncomingMessage::from_cell(&opcode_only(0xf1eef33c)?).is_err());
    let raw = cell_for("staking_master/CoffeeStakingSetJettons/0")?;
    assert!(matches!(
        CoffeeStakingMasterIncomingMessage::from_cell(&raw)?,
        CoffeeStakingMasterIncomingMessage::CoffeeStakingSetJettons(_)
    ));
    assert!(CoffeeStakingMasterIncomingMessage::from_cell(&opcode_only(0xeec15c2a)?).is_err());
    let raw = cell_for("staking_master/CoffeeStakingSetPeriods/0")?;
    assert!(matches!(
        CoffeeStakingMasterIncomingMessage::from_cell(&raw)?,
        CoffeeStakingMasterIncomingMessage::CoffeeStakingSetPeriods(_)
    ));
    assert!(CoffeeStakingMasterIncomingMessage::from_cell(&opcode_only(0xfab30dc2)?).is_err());
    let raw = cell_for("staking_master/CoffeeStakingUpdateContract/0")?;
    assert!(matches!(
        CoffeeStakingMasterIncomingMessage::from_cell(&raw)?,
        CoffeeStakingMasterIncomingMessage::CoffeeStakingUpdateContract(_)
    ));
    assert!(CoffeeStakingMasterIncomingMessage::from_cell(&opcode_only(0xce4e2066)?).is_err());
    let raw = cell_for("staking_master/CoffeeStakingItemExtended/0")?;
    assert!(matches!(
        CoffeeStakingMasterIncomingMessage::from_cell(&raw)?,
        CoffeeStakingMasterIncomingMessage::CoffeeStakingItemExtended(_)
    ));
    assert!(CoffeeStakingMasterIncomingMessage::from_cell(&opcode_only(0xeb666ce0)?).is_err());
    let raw = cell_for("staking_master/CoffeeStakingItemUnlocked/0")?;
    assert!(matches!(
        CoffeeStakingMasterIncomingMessage::from_cell(&raw)?,
        CoffeeStakingMasterIncomingMessage::CoffeeStakingItemUnlocked(_)
    ));
    assert!(CoffeeStakingMasterIncomingMessage::from_cell(&opcode_only(0xcb03bfaf)?).is_err());
    let raw = unknown_body()?;
    let decoded = CoffeeStakingMasterIncomingMessage::from_cell(&raw)?;
    assert!(matches!(decoded, CoffeeStakingMasterIncomingMessage::Unknown(_)));
    assert_eq!(decoded.to_cell()?, raw);
    let raw = cell_for("staking_master/CoffeeStakingMasterInitialized/0")?;
    assert!(matches!(
        CoffeeStakingMasterOutgoingMessage::from_cell(&raw)?,
        CoffeeStakingMasterOutgoingMessage::Initialized(_)
    ));
    assert!(CoffeeStakingMasterOutgoingMessage::from_cell(&opcode_only(0xbe5a7595)?).is_err());
    let raw = cell_for("staking_master/CoffeeStakingRewardsUpdated/0")?;
    assert!(matches!(
        CoffeeStakingMasterOutgoingMessage::from_cell(&raw)?,
        CoffeeStakingMasterOutgoingMessage::CoffeeStakingRewardsUpdated(_)
    ));
    assert!(CoffeeStakingMasterOutgoingMessage::from_cell(&opcode_only(0x0a9577f0)?).is_err());
    let raw = cell_for("staking_master/CoffeeStakingRewardsClaimed/0")?;
    assert!(matches!(
        CoffeeStakingMasterOutgoingMessage::from_cell(&raw)?,
        CoffeeStakingMasterOutgoingMessage::CoffeeStakingRewardsClaimed(_)
    ));
    assert!(CoffeeStakingMasterOutgoingMessage::from_cell(&opcode_only(0xb30c7310)?).is_err());
    let raw = cell_for("staking_master/CoffeeStakingDistributorUpdated/0")?;
    assert!(matches!(
        CoffeeStakingMasterOutgoingMessage::from_cell(&raw)?,
        CoffeeStakingMasterOutgoingMessage::CoffeeStakingDistributorUpdated(_)
    ));
    assert!(CoffeeStakingMasterOutgoingMessage::from_cell(&opcode_only(0x27ee4c7d)?).is_err());
    let raw = cell_for("staking_master/CoffeeStakingOwnerUpdated/0")?;
    assert!(matches!(
        CoffeeStakingMasterOutgoingMessage::from_cell(&raw)?,
        CoffeeStakingMasterOutgoingMessage::CoffeeStakingOwnerUpdated(_)
    ));
    assert!(CoffeeStakingMasterOutgoingMessage::from_cell(&opcode_only(0xf1eef33c)?).is_err());
    let raw = cell_for("staking_master/CoffeeStakingJettonsUpdated/0")?;
    assert!(matches!(
        CoffeeStakingMasterOutgoingMessage::from_cell(&raw)?,
        CoffeeStakingMasterOutgoingMessage::CoffeeStakingJettonsUpdated(_)
    ));
    assert!(CoffeeStakingMasterOutgoingMessage::from_cell(&opcode_only(0xeec15c2a)?).is_err());
    let raw = cell_for("staking_master/CoffeeStakingPeriodsUpdated/0")?;
    assert!(matches!(
        CoffeeStakingMasterOutgoingMessage::from_cell(&raw)?,
        CoffeeStakingMasterOutgoingMessage::CoffeeStakingPeriodsUpdated(_)
    ));
    assert!(CoffeeStakingMasterOutgoingMessage::from_cell(&opcode_only(0xfab30dc2)?).is_err());
    let raw = cell_for("staking_master/CoffeeStakingItemExtended/0")?;
    assert!(matches!(
        CoffeeStakingMasterOutgoingMessage::from_cell(&raw)?,
        CoffeeStakingMasterOutgoingMessage::CoffeeStakingItemExtended(_)
    ));
    assert!(CoffeeStakingMasterOutgoingMessage::from_cell(&opcode_only(0xeb666ce0)?).is_err());
    let raw = cell_for("staking_master/CoffeeStakingItemUnlocked/0")?;
    assert!(matches!(
        CoffeeStakingMasterOutgoingMessage::from_cell(&raw)?,
        CoffeeStakingMasterOutgoingMessage::CoffeeStakingItemUnlocked(_)
    ));
    assert!(CoffeeStakingMasterOutgoingMessage::from_cell(&opcode_only(0xcb03bfaf)?).is_err());
    let raw = unknown_body()?;
    let decoded = CoffeeStakingMasterOutgoingMessage::from_cell(&raw)?;
    assert!(matches!(decoded, CoffeeStakingMasterOutgoingMessage::Unknown(_)));
    assert_eq!(decoded.to_cell()?, raw);
    Ok(())
}
#[test]
fn test_staking_vault_role_dispatch() -> anyhow::Result<()> {
    let raw = cell_for("staking_vault/CoffeeStakingVaultInit/0")?;
    assert!(matches!(
        CoffeeStakingVaultIncomingMessage::from_cell(&raw)?,
        CoffeeStakingVaultIncomingMessage::Init(_)
    ));
    assert!(CoffeeStakingVaultIncomingMessage::from_cell(&opcode_only(0xbe5a7595)?).is_err());
    let raw = cell_for("staking_vault/CoffeeStakingVaultTransferNotification/0")?;
    assert!(matches!(
        CoffeeStakingVaultIncomingMessage::from_cell(&raw)?,
        CoffeeStakingVaultIncomingMessage::TransferNotification(_)
    ));
    assert!(CoffeeStakingVaultIncomingMessage::from_cell(&opcode_only(0x7362d09c)?).is_err());
    let raw = cell_for("staking_vault/CoffeeStakingVaultSetOwner/0")?;
    assert!(matches!(
        CoffeeStakingVaultIncomingMessage::from_cell(&raw)?,
        CoffeeStakingVaultIncomingMessage::SetOwner(_)
    ));
    assert!(CoffeeStakingVaultIncomingMessage::from_cell(&opcode_only(0xf1eef33c)?).is_err());
    let raw = cell_for("staking_vault/CoffeeStakingVaultSetJettons/0")?;
    assert!(matches!(
        CoffeeStakingVaultIncomingMessage::from_cell(&raw)?,
        CoffeeStakingVaultIncomingMessage::SetJettons(_)
    ));
    assert!(CoffeeStakingVaultIncomingMessage::from_cell(&opcode_only(0xeec15c2a)?).is_err());
    let raw = cell_for("staking_vault/CoffeeStakingVaultSetPeriods/0")?;
    assert!(matches!(
        CoffeeStakingVaultIncomingMessage::from_cell(&raw)?,
        CoffeeStakingVaultIncomingMessage::SetPeriods(_)
    ));
    assert!(CoffeeStakingVaultIncomingMessage::from_cell(&opcode_only(0xfab30dc2)?).is_err());
    let raw = cell_for("staking_vault/CoffeeStakingVaultUpdateContract/0")?;
    assert!(matches!(
        CoffeeStakingVaultIncomingMessage::from_cell(&raw)?,
        CoffeeStakingVaultIncomingMessage::UpdateContract(_)
    ));
    assert!(CoffeeStakingVaultIncomingMessage::from_cell(&opcode_only(0xce4e2066)?).is_err());
    let raw = unknown_body()?;
    let decoded = CoffeeStakingVaultIncomingMessage::from_cell(&raw)?;
    assert!(matches!(decoded, CoffeeStakingVaultIncomingMessage::Unknown(_)));
    assert_eq!(decoded.to_cell()?, raw);
    let raw = cell_for("staking_vault/CoffeeStakingVaultDepositToMaster/0")?;
    assert!(matches!(
        CoffeeStakingVaultOutgoingMessage::from_cell(&raw)?,
        CoffeeStakingVaultOutgoingMessage::DepositToMaster(_)
    ));
    assert!(CoffeeStakingVaultOutgoingMessage::from_cell(&opcode_only(0xf9471134)?).is_err());
    let raw = cell_for("staking_vault/CoffeeStakingVaultJettonRefund/0")?;
    assert!(matches!(
        CoffeeStakingVaultOutgoingMessage::from_cell(&raw)?,
        CoffeeStakingVaultOutgoingMessage::JettonRefund(_)
    ));
    assert!(CoffeeStakingVaultOutgoingMessage::from_cell(&opcode_only(0x0f8a7ea5)?).is_err());
    let raw = unknown_body()?;
    let decoded = CoffeeStakingVaultOutgoingMessage::from_cell(&raw)?;
    assert!(matches!(decoded, CoffeeStakingVaultOutgoingMessage::Unknown(_)));
    assert_eq!(decoded.to_cell()?, raw);
    Ok(())
}
#[test]
fn test_vault_extra_role_dispatch() -> anyhow::Result<()> {
    let raw = cell_for("vault_extra/CoffeePayoutInternal/0")?;
    assert!(matches!(
        CoffeeVaultExtraIncomingMessage::from_cell(&raw)?,
        CoffeeVaultExtraIncomingMessage::CoffeePayoutInternal(_)
    ));
    assert!(CoffeeVaultExtraIncomingMessage::from_cell(&opcode_only(0xc0ffee21)?).is_err());
    let raw = cell_for("vault_extra/CoffeeSwapExtra/0")?;
    assert!(matches!(
        CoffeeVaultExtraIncomingMessage::from_cell(&raw)?,
        CoffeeVaultExtraIncomingMessage::CoffeeSwapExtra(_)
    ));
    assert!(CoffeeVaultExtraIncomingMessage::from_cell(&opcode_only(0xc0ffee01)?).is_err());
    let raw = cell_for("vault_extra/CoffeeDepositLiquidityExtra/0")?;
    assert!(matches!(
        CoffeeVaultExtraIncomingMessage::from_cell(&raw)?,
        CoffeeVaultExtraIncomingMessage::CoffeeDepositLiquidityExtra(_)
    ));
    assert!(CoffeeVaultExtraIncomingMessage::from_cell(&opcode_only(0xc0ffee05)?).is_err());
    let raw = cell_for("vault_extra/CoffeeCreatePoolExtra/0")?;
    assert!(matches!(
        CoffeeVaultExtraIncomingMessage::from_cell(&raw)?,
        CoffeeVaultExtraIncomingMessage::CoffeeCreatePoolExtra(_)
    ));
    assert!(CoffeeVaultExtraIncomingMessage::from_cell(&opcode_only(0xc0ffee03)?).is_err());
    let raw = cell_for("vault_extra/CoffeeUpdateContractInternal/0")?;
    assert!(matches!(
        CoffeeVaultExtraIncomingMessage::from_cell(&raw)?,
        CoffeeVaultExtraIncomingMessage::CoffeeUpdateContractInternal(_)
    ));
    assert!(CoffeeVaultExtraIncomingMessage::from_cell(&opcode_only(0xc0ffee2e)?).is_err());
    let raw = unknown_body()?;
    let decoded = CoffeeVaultExtraIncomingMessage::from_cell(&raw)?;
    assert!(matches!(decoded, CoffeeVaultExtraIncomingMessage::Unknown(_)));
    assert_eq!(decoded.to_cell()?, raw);
    let raw = cell_for("vault_extra/CoffeeNotification/0")?;
    assert!(matches!(
        CoffeeVaultExtraOutgoingMessage::from_cell(&raw)?,
        CoffeeVaultExtraOutgoingMessage::CoffeeNotification(_)
    ));
    assert!(CoffeeVaultExtraOutgoingMessage::from_cell(&opcode_only(0xc0ffee36)?).is_err());
    let raw = cell_for("vault_extra/CoffeePayout/0")?;
    assert!(matches!(
        CoffeeVaultExtraOutgoingMessage::from_cell(&raw)?,
        CoffeeVaultExtraOutgoingMessage::CoffeePayout(_)
    ));
    assert!(CoffeeVaultExtraOutgoingMessage::from_cell(&opcode_only(0xc0ffee32)?).is_err());
    let raw = cell_for("vault_extra/CoffeeSwapInternal/0")?;
    assert!(matches!(
        CoffeeVaultExtraOutgoingMessage::from_cell(&raw)?,
        CoffeeVaultExtraOutgoingMessage::CoffeeSwapInternal(_)
    ));
    assert!(CoffeeVaultExtraOutgoingMessage::from_cell(&opcode_only(0xc0ffee20)?).is_err());
    let raw = cell_for("vault_extra/CoffeeCreateLiquidityDepositoryRequest/0")?;
    assert!(matches!(
        CoffeeVaultExtraOutgoingMessage::from_cell(&raw)?,
        CoffeeVaultExtraOutgoingMessage::CoffeeCreateLiquidityDepositoryRequest(_)
    ));
    assert!(CoffeeVaultExtraOutgoingMessage::from_cell(&opcode_only(0xc0ffee27)?).is_err());
    let raw = cell_for("vault_extra/CoffeeCreatePoolCreatorRequest/0")?;
    assert!(matches!(
        CoffeeVaultExtraOutgoingMessage::from_cell(&raw)?,
        CoffeeVaultExtraOutgoingMessage::CoffeeCreatePoolCreatorRequest(_)
    ));
    assert!(CoffeeVaultExtraOutgoingMessage::from_cell(&opcode_only(0xc0ffee23)?).is_err());
    let raw = cell_for("vault_extra/CoffeeExcesses/0")?;
    assert!(matches!(
        CoffeeVaultExtraOutgoingMessage::from_cell(&raw)?,
        CoffeeVaultExtraOutgoingMessage::CoffeeExcesses(_)
    ));
    assert!(CoffeeVaultExtraOutgoingMessage::from_cell(&opcode_only(0xd53276db)?).is_err());
    let raw = unknown_body()?;
    let decoded = CoffeeVaultExtraOutgoingMessage::from_cell(&raw)?;
    assert!(matches!(decoded, CoffeeVaultExtraOutgoingMessage::Unknown(_)));
    assert_eq!(decoded.to_cell()?, raw);
    Ok(())
}
#[test]
fn test_vault_jetton_role_dispatch() -> anyhow::Result<()> {
    let raw = cell_for("vault_jetton/CoffeeTransferNotification/0")?;
    assert!(matches!(
        CoffeeVaultJettonIncomingMessage::from_cell(&raw)?,
        CoffeeVaultJettonIncomingMessage::CoffeeTransferNotification(_)
    ));
    assert!(CoffeeVaultJettonIncomingMessage::from_cell(&opcode_only(0x7362d09c)?).is_err());
    let raw = cell_for("vault_jetton/CoffeePayoutInternal/0")?;
    assert!(matches!(
        CoffeeVaultJettonIncomingMessage::from_cell(&raw)?,
        CoffeeVaultJettonIncomingMessage::CoffeePayoutInternal(_)
    ));
    assert!(CoffeeVaultJettonIncomingMessage::from_cell(&opcode_only(0xc0ffee21)?).is_err());
    let raw = cell_for("vault_jetton/CoffeeCreateVaultInternal/0")?;
    assert!(matches!(
        CoffeeVaultJettonIncomingMessage::from_cell(&raw)?,
        CoffeeVaultJettonIncomingMessage::CoffeeCreateVaultInternal(_)
    ));
    assert!(CoffeeVaultJettonIncomingMessage::from_cell(&opcode_only(0xc0ffee2a)?).is_err());
    let raw = cell_for("vault_jetton/CoffeeTakeWalletAddress/0")?;
    assert!(matches!(
        CoffeeVaultJettonIncomingMessage::from_cell(&raw)?,
        CoffeeVaultJettonIncomingMessage::CoffeeTakeWalletAddress(_)
    ));
    assert!(CoffeeVaultJettonIncomingMessage::from_cell(&opcode_only(0xd1735400)?).is_err());
    let raw = cell_for("vault_jetton/CoffeeActivateVaultInternal/0")?;
    assert!(matches!(
        CoffeeVaultJettonIncomingMessage::from_cell(&raw)?,
        CoffeeVaultJettonIncomingMessage::CoffeeActivateVaultInternal(_)
    ));
    assert!(CoffeeVaultJettonIncomingMessage::from_cell(&opcode_only(0xc0ffee2c)?).is_err());
    let raw = cell_for("vault_jetton/CoffeeUpdateContractInternal/0")?;
    assert!(matches!(
        CoffeeVaultJettonIncomingMessage::from_cell(&raw)?,
        CoffeeVaultJettonIncomingMessage::CoffeeUpdateContractInternal(_)
    ));
    assert!(CoffeeVaultJettonIncomingMessage::from_cell(&opcode_only(0xc0ffee2e)?).is_err());
    let raw = unknown_body()?;
    let decoded = CoffeeVaultJettonIncomingMessage::from_cell(&raw)?;
    assert!(matches!(decoded, CoffeeVaultJettonIncomingMessage::Unknown(_)));
    assert_eq!(decoded.to_cell()?, raw);
    let raw = cell_for("vault_jetton/CoffeeNotification/0")?;
    assert!(matches!(
        CoffeeVaultJettonOutgoingMessage::from_cell(&raw)?,
        CoffeeVaultJettonOutgoingMessage::CoffeeNotification(_)
    ));
    assert!(CoffeeVaultJettonOutgoingMessage::from_cell(&opcode_only(0xc0ffee36)?).is_err());
    let raw = cell_for("vault_jetton/CoffeeSwapInternal/0")?;
    assert!(matches!(
        CoffeeVaultJettonOutgoingMessage::from_cell(&raw)?,
        CoffeeVaultJettonOutgoingMessage::CoffeeSwapInternal(_)
    ));
    assert!(CoffeeVaultJettonOutgoingMessage::from_cell(&opcode_only(0xc0ffee20)?).is_err());
    let raw = cell_for("vault_jetton/CoffeeCreateLiquidityDepositoryRequest/0")?;
    assert!(matches!(
        CoffeeVaultJettonOutgoingMessage::from_cell(&raw)?,
        CoffeeVaultJettonOutgoingMessage::CoffeeCreateLiquidityDepositoryRequest(_)
    ));
    assert!(CoffeeVaultJettonOutgoingMessage::from_cell(&opcode_only(0xc0ffee27)?).is_err());
    let raw = cell_for("vault_jetton/CoffeeCreatePoolCreatorRequest/0")?;
    assert!(matches!(
        CoffeeVaultJettonOutgoingMessage::from_cell(&raw)?,
        CoffeeVaultJettonOutgoingMessage::CoffeeCreatePoolCreatorRequest(_)
    ));
    assert!(CoffeeVaultJettonOutgoingMessage::from_cell(&opcode_only(0xc0ffee23)?).is_err());
    let raw = cell_for("vault_jetton/CoffeeJettonTransfer/0")?;
    assert!(matches!(
        CoffeeVaultJettonOutgoingMessage::from_cell(&raw)?,
        CoffeeVaultJettonOutgoingMessage::CoffeeJettonTransfer(_)
    ));
    assert!(CoffeeVaultJettonOutgoingMessage::from_cell(&opcode_only(0x0f8a7ea5)?).is_err());
    let raw = cell_for("vault_jetton/CoffeeProvideWalletAddress/0")?;
    assert!(matches!(
        CoffeeVaultJettonOutgoingMessage::from_cell(&raw)?,
        CoffeeVaultJettonOutgoingMessage::CoffeeProvideWalletAddress(_)
    ));
    assert!(CoffeeVaultJettonOutgoingMessage::from_cell(&opcode_only(0x2c76b973)?).is_err());
    let raw = cell_for("vault_jetton/CoffeeExcesses/0")?;
    assert!(matches!(
        CoffeeVaultJettonOutgoingMessage::from_cell(&raw)?,
        CoffeeVaultJettonOutgoingMessage::CoffeeExcesses(_)
    ));
    assert!(CoffeeVaultJettonOutgoingMessage::from_cell(&opcode_only(0xd53276db)?).is_err());
    let raw = unknown_body()?;
    let decoded = CoffeeVaultJettonOutgoingMessage::from_cell(&raw)?;
    assert!(matches!(decoded, CoffeeVaultJettonOutgoingMessage::Unknown(_)));
    assert_eq!(decoded.to_cell()?, raw);
    Ok(())
}
#[test]
fn test_vault_native_role_dispatch() -> anyhow::Result<()> {
    let raw = cell_for("vault_native/CoffeePayoutInternal/0")?;
    assert!(matches!(
        CoffeeVaultNativeIncomingMessage::from_cell(&raw)?,
        CoffeeVaultNativeIncomingMessage::CoffeePayoutInternal(_)
    ));
    assert!(CoffeeVaultNativeIncomingMessage::from_cell(&opcode_only(0xc0ffee21)?).is_err());
    let raw = cell_for("vault_native/CoffeeSwapNative/0")?;
    assert!(matches!(
        CoffeeVaultNativeIncomingMessage::from_cell(&raw)?,
        CoffeeVaultNativeIncomingMessage::CoffeeSwapNative(_)
    ));
    assert!(CoffeeVaultNativeIncomingMessage::from_cell(&opcode_only(0xc0ffee00)?).is_err());
    let raw = cell_for("vault_native/CoffeeDepositLiquidityNative/0")?;
    assert!(matches!(
        CoffeeVaultNativeIncomingMessage::from_cell(&raw)?,
        CoffeeVaultNativeIncomingMessage::CoffeeDepositLiquidityNative(_)
    ));
    assert!(CoffeeVaultNativeIncomingMessage::from_cell(&opcode_only(0xc0ffee04)?).is_err());
    let raw = cell_for("vault_native/CoffeeCreatePoolNative/0")?;
    assert!(matches!(
        CoffeeVaultNativeIncomingMessage::from_cell(&raw)?,
        CoffeeVaultNativeIncomingMessage::CoffeeCreatePoolNative(_)
    ));
    assert!(CoffeeVaultNativeIncomingMessage::from_cell(&opcode_only(0xc0ffee02)?).is_err());
    let raw = cell_for("vault_native/CoffeeUpdateContractInternal/0")?;
    assert!(matches!(
        CoffeeVaultNativeIncomingMessage::from_cell(&raw)?,
        CoffeeVaultNativeIncomingMessage::CoffeeUpdateContractInternal(_)
    ));
    assert!(CoffeeVaultNativeIncomingMessage::from_cell(&opcode_only(0xc0ffee2e)?).is_err());
    let raw = unknown_body()?;
    let decoded = CoffeeVaultNativeIncomingMessage::from_cell(&raw)?;
    assert!(matches!(decoded, CoffeeVaultNativeIncomingMessage::Unknown(_)));
    assert_eq!(decoded.to_cell()?, raw);
    let raw = cell_for("vault_native/CoffeeNotification/0")?;
    assert!(matches!(
        CoffeeVaultNativeOutgoingMessage::from_cell(&raw)?,
        CoffeeVaultNativeOutgoingMessage::CoffeeNotification(_)
    ));
    assert!(CoffeeVaultNativeOutgoingMessage::from_cell(&opcode_only(0xc0ffee36)?).is_err());
    let raw = cell_for("vault_native/CoffeePayout/0")?;
    assert!(matches!(
        CoffeeVaultNativeOutgoingMessage::from_cell(&raw)?,
        CoffeeVaultNativeOutgoingMessage::CoffeePayout(_)
    ));
    assert!(CoffeeVaultNativeOutgoingMessage::from_cell(&opcode_only(0xc0ffee32)?).is_err());
    let raw = cell_for("vault_native/CoffeeSwapInternal/0")?;
    assert!(matches!(
        CoffeeVaultNativeOutgoingMessage::from_cell(&raw)?,
        CoffeeVaultNativeOutgoingMessage::CoffeeSwapInternal(_)
    ));
    assert!(CoffeeVaultNativeOutgoingMessage::from_cell(&opcode_only(0xc0ffee20)?).is_err());
    let raw = cell_for("vault_native/CoffeeCreateLiquidityDepositoryRequest/0")?;
    assert!(matches!(
        CoffeeVaultNativeOutgoingMessage::from_cell(&raw)?,
        CoffeeVaultNativeOutgoingMessage::CoffeeCreateLiquidityDepositoryRequest(_)
    ));
    assert!(CoffeeVaultNativeOutgoingMessage::from_cell(&opcode_only(0xc0ffee27)?).is_err());
    let raw = cell_for("vault_native/CoffeeCreatePoolCreatorRequest/0")?;
    assert!(matches!(
        CoffeeVaultNativeOutgoingMessage::from_cell(&raw)?,
        CoffeeVaultNativeOutgoingMessage::CoffeeCreatePoolCreatorRequest(_)
    ));
    assert!(CoffeeVaultNativeOutgoingMessage::from_cell(&opcode_only(0xc0ffee23)?).is_err());
    let raw = cell_for("vault_native/CoffeeExcesses/0")?;
    assert!(matches!(
        CoffeeVaultNativeOutgoingMessage::from_cell(&raw)?,
        CoffeeVaultNativeOutgoingMessage::CoffeeExcesses(_)
    ));
    assert!(CoffeeVaultNativeOutgoingMessage::from_cell(&opcode_only(0xd53276db)?).is_err());
    let raw = unknown_body()?;
    let decoded = CoffeeVaultNativeOutgoingMessage::from_cell(&raw)?;
    assert!(matches!(decoded, CoffeeVaultNativeOutgoingMessage::Unknown(_)));
    assert_eq!(decoded.to_cell()?, raw);
    Ok(())
}

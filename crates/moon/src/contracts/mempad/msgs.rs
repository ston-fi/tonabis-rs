//! Generated Moon wire records. See tools/reference/moon/README.md.
use crate::contracts::address::MoonReferencedAddress;
use crate::contracts::mempad::curve::storage::MoonMempadEconState;
use ton::ton_core::TLB;
use ton::ton_core::cell::TonCell;
use ton::ton_core::errors::TonCoreResult;
use ton::ton_core::traits::tlb::TLB;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::TLBCoins;
use ton::ton_core::types::tlb_core::TLBRef;
use ton::ton_core::types::tlb_core::adapters::InternalAddress;
use ton::ton_core::types::tlb_core::adapters::NullableAddress;

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x595f07bc, bits_len = 32)]
pub struct MoonMempadAskToBurnMsg {
    pub query_id: u64,
    pub jetton_amount: TLBCoins,
    #[tlb(adapter = "NullableAddress")]
    pub send_excesses_to: Option<TonAddress>,
    pub custom_payload: Option<TLBRef<TonCell>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x0f8a7ea5, bits_len = 32)]
pub struct MoonMempadAskToTransferMsg {
    pub query_id: u64,
    pub jetton_amount: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub transfer_recipient: TonAddress,
    #[tlb(adapter = "NullableAddress")]
    pub send_excesses_to: Option<TonAddress>,
    pub custom_payload: Option<TLBRef<TonCell>>,
    pub forward_ton_amount: TLBCoins,
    pub forward_payload: TonCell,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x7bdd97de, bits_len = 32)]
pub struct MoonMempadBurnNotificationForMinterMsg {
    pub query_id: u64,
    pub jetton_amount: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub burn_initiator: TonAddress,
    #[tlb(adapter = "NullableAddress")]
    pub send_excesses_to: Option<TonAddress>,
    pub forward_payload: TLBRef<TonCell>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x6501f354, bits_len = 32)]
pub struct MoonMempadChangeMinterAdminMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub new_admin_address: TonAddress,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xcb862902, bits_len = 32)]
pub struct MoonMempadChangeMinterMetadataMsg {
    pub query_id: u64,
    pub new_metadata: TLBRef<TonCell>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xfb88e119, bits_len = 32)]
pub struct MoonMempadClaimMinterAdminMsg {
    pub query_id: u64,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x7431f221, bits_len = 32)]
pub struct MoonMempadDropMinterAdminMsg {
    pub query_id: u64,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x178d4519, bits_len = 32)]
pub struct MoonMempadInternalTransferStepMsg {
    pub query_id: u64,
    pub jetton_amount: TLBCoins,
    #[tlb(adapter = "NullableAddress")]
    pub transfer_initiator: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub send_excesses_to: Option<TonAddress>,
    pub forward_ton_amount: TLBCoins,
    pub forward_payload: TonCell,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x90af0b63, bits_len = 32)]
pub struct MoonMempadJettonDepositLiquidityPayloadMsg {
    pub min_lp_out: TLBCoins,
    pub locked: bool,
    pub split: Option<TLBRef<MoonMempadLpSplit>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonMempadLaunchContent {
    pub name: TLBRef<TonCell>,
    pub symbol: TLBRef<TonCell>,
    pub image: TLBRef<TonCell>,
    pub extra: Option<TLBRef<MoonMempadLaunchExtra>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonMempadLaunchExtra {
    pub description: TLBRef<TonCell>,
    pub links: Option<TLBRef<MoonMempadLaunchLinks>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonMempadLaunchLinks {
    pub website: Option<TLBRef<TonCell>>,
    pub twitter: Option<TLBRef<TonCell>>,
    pub telegram: Option<TLBRef<TonCell>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonMempadLpSplit {
    #[tlb(adapter = "InternalAddress")]
    pub a_recipient: TonAddress,
    pub a_bps: u16,
    #[tlb(adapter = "InternalAddress")]
    pub b_recipient: TonAddress,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x642b7d07, bits_len = 32)]
pub struct MoonMempadMintNewJettonsMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub mint_recipient: TonAddress,
    pub ton_amount: TLBCoins,
    pub internal_transfer_msg: TLBRef<MoonMempadInternalTransferStepMsg>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x29d1f1c1, bits_len = 32)]
pub struct MoonMempadBuyMsg {
    pub query_id: u64,
    pub min_out: TLBCoins,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x7e838abb, bits_len = 32)]
pub struct MoonMempadCreateLaunchMsg {
    pub query_id: u64,
    pub salt: u64,
    pub fee_bps: u8,
    pub initial_buy: TLBCoins,
    pub content: TLBRef<MoonMempadLaunchContent>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xec725c4f, bits_len = 32)]
pub struct MoonMempadCreatePoolMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub x_master: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub y_master: Option<TonAddress>,
    pub fee_bps: u8,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x6ec846e7, bits_len = 32)]
pub struct MoonMempadCreatorClaimFeesMsg {
    pub query_id: u64,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x90af0b63, bits_len = 32)]
pub struct MoonMempadDepositLiquidityMsg {
    pub query_id: u64,
    pub min_lp_out: TLBCoins,
    pub locked: bool,
    pub split: Option<TLBRef<MoonMempadLpSplit>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x329068a6, bits_len = 32)]
pub struct MoonMempadFactoryWithdrawMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub recipient: TonAddress,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x61c9641a, bits_len = 32)]
pub struct MoonMempadInitLaunchMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub cpmm_factory: TonAddress,
    #[tlb(adapter = "InternalAddress")]
    pub protocol_recipient: TonAddress,
    pub initial_buy: TLBCoins,
    pub econ: TLBRef<MoonMempadEconState>,
    pub jw_code: TLBRef<TonCell>,
    pub metadata: TLBRef<TonCell>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x24293ccc, bits_len = 32)]
pub struct MoonMempadInternalUpgradeCurveMsg {
    pub query_id: u64,
    pub new_code: Option<TLBRef<TonCell>>,
    pub new_data: Option<TLBRef<TonCell>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x1d42af37, bits_len = 32)]
pub struct MoonMempadProvidePoolAddressMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub x_master: Option<TonAddress>,
    #[tlb(adapter = "InternalAddress")]
    pub y_master: TonAddress,
    pub fee_bps: u8,
    #[tlb(adapter = "InternalAddress")]
    pub initiator: TonAddress,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x44daf70f, bits_len = 32)]
pub struct MoonMempadProvidePoolReadyMsg {
    pub query_id: u64,
    pub attempt: u8,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x81a58422, bits_len = 32)]
pub struct MoonMempadTakePoolAddressMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub pool: TonAddress,
    #[tlb(adapter = "NullableAddress")]
    pub x_master: Option<TonAddress>,
    #[tlb(adapter = "InternalAddress")]
    pub y_master: TonAddress,
    pub fee_bps: u8,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x67071f0d, bits_len = 32)]
pub struct MoonMempadTakePoolReadyMsg {
    pub query_id: u64,
    pub attempt: u8,
    pub is_ready: bool,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xdee7a887, bits_len = 32)]
pub struct MoonMempadTokenSalePayoutMsg {
    pub query_id: u64,
    pub gram_out: TLBCoins,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x9057f56b, bits_len = 32)]
pub struct MoonMempadUpgradeCurveMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub curve: TonAddress,
    pub new_code: Option<TLBRef<TonCell>>,
    pub new_data: Option<TLBRef<TonCell>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x0eb678fe, bits_len = 32)]
pub struct MoonMempadUpgradeFactoryMsg {
    pub query_id: u64,
    pub new_code: Option<TLBRef<TonCell>>,
    pub new_data: Option<TLBRef<TonCell>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x2c76b973, bits_len = 32)]
pub struct MoonMempadRequestWalletAddressMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub owner_address: TonAddress,
    pub include_owner_address: bool,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xd1735400, bits_len = 32)]
pub struct MoonMempadResponseWalletAddressMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub jetton_wallet_address: Option<TonAddress>,
    pub owner_address: Option<TLBRef<MoonReferencedAddress>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xd53276db, bits_len = 32)]
pub struct MoonMempadReturnExcessesBackMsg {
    pub query_id: u64,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonMempadTokenSalePayload {
    pub op: u32,
    pub min_out: TLBCoins,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xd372158c, bits_len = 32)]
pub struct MoonMempadTopUpTonsMsg {}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x7362d09c, bits_len = 32)]
pub struct MoonMempadTransferNotificationForRecipientMsg {
    pub query_id: u64,
    pub jetton_amount: TLBCoins,
    #[tlb(adapter = "NullableAddress")]
    pub transfer_initiator: Option<TonAddress>,
    pub forward_payload: TonCell,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x2508d66a, bits_len = 32)]
pub struct MoonMempadUpgradeMinterCodeMsg {
    pub query_id: u64,
    pub new_data: TLBRef<TonCell>,
    pub new_code: TLBRef<TonCell>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xfce532d8, bits_len = 32)]
pub struct MoonMempadFactoryPayoutResponseMsg {
    pub query_id: u64,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x0b8cca52, bits_len = 32)]
pub struct MoonMempadCurveResponseMsg {
    pub query_id: u64,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x46f6a58e, bits_len = 32)]
pub struct MoonMempadPoolDeploymentResponseMsg {
    pub query_id: u64,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xca6c5778, bits_len = 32)]
pub struct MoonMempadCurvePayoutResponseMsg {
    pub query_id: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MoonMempadMsg {
    MoonMempadAskToBurnMsg(Box<MoonMempadAskToBurnMsg>),
    MoonMempadAskToTransferMsg(Box<MoonMempadAskToTransferMsg>),
    MoonMempadBurnNotificationForMinterMsg(Box<MoonMempadBurnNotificationForMinterMsg>),
    MoonMempadChangeMinterAdminMsg(Box<MoonMempadChangeMinterAdminMsg>),
    MoonMempadChangeMinterMetadataMsg(Box<MoonMempadChangeMinterMetadataMsg>),
    MoonMempadClaimMinterAdminMsg(Box<MoonMempadClaimMinterAdminMsg>),
    MoonMempadDropMinterAdminMsg(Box<MoonMempadDropMinterAdminMsg>),
    MoonMempadInternalTransferStepMsg(Box<MoonMempadInternalTransferStepMsg>),
    MoonMempadMintNewJettonsMsg(Box<MoonMempadMintNewJettonsMsg>),
    MoonMempadBuyMsg(Box<MoonMempadBuyMsg>),
    MoonMempadCreateLaunchMsg(Box<MoonMempadCreateLaunchMsg>),
    MoonMempadCreatePoolMsg(Box<MoonMempadCreatePoolMsg>),
    MoonMempadCreatorClaimFeesMsg(Box<MoonMempadCreatorClaimFeesMsg>),
    MoonMempadDepositLiquidityMsg(Box<MoonMempadDepositLiquidityMsg>),
    MoonMempadFactoryWithdrawMsg(Box<MoonMempadFactoryWithdrawMsg>),
    MoonMempadInitLaunchMsg(Box<MoonMempadInitLaunchMsg>),
    MoonMempadInternalUpgradeCurveMsg(Box<MoonMempadInternalUpgradeCurveMsg>),
    MoonMempadProvidePoolAddressMsg(Box<MoonMempadProvidePoolAddressMsg>),
    MoonMempadProvidePoolReadyMsg(Box<MoonMempadProvidePoolReadyMsg>),
    MoonMempadTakePoolAddressMsg(Box<MoonMempadTakePoolAddressMsg>),
    MoonMempadTakePoolReadyMsg(Box<MoonMempadTakePoolReadyMsg>),
    MoonMempadTokenSalePayoutMsg(Box<MoonMempadTokenSalePayoutMsg>),
    MoonMempadUpgradeCurveMsg(Box<MoonMempadUpgradeCurveMsg>),
    MoonMempadUpgradeFactoryMsg(Box<MoonMempadUpgradeFactoryMsg>),
    MoonMempadRequestWalletAddressMsg(Box<MoonMempadRequestWalletAddressMsg>),
    MoonMempadResponseWalletAddressMsg(Box<MoonMempadResponseWalletAddressMsg>),
    MoonMempadReturnExcessesBackMsg(Box<MoonMempadReturnExcessesBackMsg>),
    MoonMempadTopUpTonsMsg(Box<MoonMempadTopUpTonsMsg>),
    MoonMempadTransferNotificationForRecipientMsg(Box<MoonMempadTransferNotificationForRecipientMsg>),
    MoonMempadUpgradeMinterCodeMsg(Box<MoonMempadUpgradeMinterCodeMsg>),
    MoonMempadFactoryPayoutResponseMsg(Box<MoonMempadFactoryPayoutResponseMsg>),
    MoonMempadCurveResponseMsg(Box<MoonMempadCurveResponseMsg>),
    MoonMempadPoolDeploymentResponseMsg(Box<MoonMempadPoolDeploymentResponseMsg>),
    MoonMempadCurvePayoutResponseMsg(Box<MoonMempadCurvePayoutResponseMsg>),
    Unknown(TonCell),
}
impl MoonMempadMsg {
    pub fn from_cell(cell: &TonCell) -> TonCoreResult<Self> {
        let mut parser = cell.parser();
        if parser.data_bits_left()? < 32 {
            return Ok(Self::Unknown(cell.clone()));
        }
        match parser.read_num::<u32>(32)? {
            0x595f07bc => Ok(Self::MoonMempadAskToBurnMsg(Box::new(MoonMempadAskToBurnMsg::from_cell(cell)?))),
            0x0f8a7ea5 => Ok(Self::MoonMempadAskToTransferMsg(Box::new(
                MoonMempadAskToTransferMsg::from_cell(cell)?,
            ))),
            0x7bdd97de => Ok(Self::MoonMempadBurnNotificationForMinterMsg(Box::new(
                MoonMempadBurnNotificationForMinterMsg::from_cell(cell)?,
            ))),
            0x6501f354 => Ok(Self::MoonMempadChangeMinterAdminMsg(Box::new(
                MoonMempadChangeMinterAdminMsg::from_cell(cell)?,
            ))),
            0xcb862902 => Ok(Self::MoonMempadChangeMinterMetadataMsg(Box::new(
                MoonMempadChangeMinterMetadataMsg::from_cell(cell)?,
            ))),
            0xfb88e119 => Ok(Self::MoonMempadClaimMinterAdminMsg(Box::new(
                MoonMempadClaimMinterAdminMsg::from_cell(cell)?,
            ))),
            0x7431f221 => Ok(Self::MoonMempadDropMinterAdminMsg(Box::new(
                MoonMempadDropMinterAdminMsg::from_cell(cell)?,
            ))),
            0x178d4519 => Ok(Self::MoonMempadInternalTransferStepMsg(Box::new(
                MoonMempadInternalTransferStepMsg::from_cell(cell)?,
            ))),
            0x642b7d07 => Ok(Self::MoonMempadMintNewJettonsMsg(Box::new(
                MoonMempadMintNewJettonsMsg::from_cell(cell)?,
            ))),
            0x29d1f1c1 => Ok(Self::MoonMempadBuyMsg(Box::new(MoonMempadBuyMsg::from_cell(cell)?))),
            0x7e838abb => Ok(Self::MoonMempadCreateLaunchMsg(Box::new(MoonMempadCreateLaunchMsg::from_cell(
                cell,
            )?))),
            0xec725c4f => Ok(Self::MoonMempadCreatePoolMsg(Box::new(MoonMempadCreatePoolMsg::from_cell(
                cell,
            )?))),
            0x6ec846e7 => Ok(Self::MoonMempadCreatorClaimFeesMsg(Box::new(
                MoonMempadCreatorClaimFeesMsg::from_cell(cell)?,
            ))),
            0x90af0b63 => Ok(Self::MoonMempadDepositLiquidityMsg(Box::new(
                MoonMempadDepositLiquidityMsg::from_cell(cell)?,
            ))),
            0x329068a6 => Ok(Self::MoonMempadFactoryWithdrawMsg(Box::new(
                MoonMempadFactoryWithdrawMsg::from_cell(cell)?,
            ))),
            0x61c9641a => Ok(Self::MoonMempadInitLaunchMsg(Box::new(MoonMempadInitLaunchMsg::from_cell(
                cell,
            )?))),
            0x24293ccc => Ok(Self::MoonMempadInternalUpgradeCurveMsg(Box::new(
                MoonMempadInternalUpgradeCurveMsg::from_cell(cell)?,
            ))),
            0x1d42af37 => Ok(Self::MoonMempadProvidePoolAddressMsg(Box::new(
                MoonMempadProvidePoolAddressMsg::from_cell(cell)?,
            ))),
            0x44daf70f => Ok(Self::MoonMempadProvidePoolReadyMsg(Box::new(
                MoonMempadProvidePoolReadyMsg::from_cell(cell)?,
            ))),
            0x81a58422 => Ok(Self::MoonMempadTakePoolAddressMsg(Box::new(
                MoonMempadTakePoolAddressMsg::from_cell(cell)?,
            ))),
            0x67071f0d => Ok(Self::MoonMempadTakePoolReadyMsg(Box::new(
                MoonMempadTakePoolReadyMsg::from_cell(cell)?,
            ))),
            0xdee7a887 => Ok(Self::MoonMempadTokenSalePayoutMsg(Box::new(
                MoonMempadTokenSalePayoutMsg::from_cell(cell)?,
            ))),
            0x9057f56b => Ok(Self::MoonMempadUpgradeCurveMsg(Box::new(MoonMempadUpgradeCurveMsg::from_cell(
                cell,
            )?))),
            0x0eb678fe => Ok(Self::MoonMempadUpgradeFactoryMsg(Box::new(
                MoonMempadUpgradeFactoryMsg::from_cell(cell)?,
            ))),
            0x2c76b973 => Ok(Self::MoonMempadRequestWalletAddressMsg(Box::new(
                MoonMempadRequestWalletAddressMsg::from_cell(cell)?,
            ))),
            0xd1735400 => Ok(Self::MoonMempadResponseWalletAddressMsg(Box::new(
                MoonMempadResponseWalletAddressMsg::from_cell(cell)?,
            ))),
            0xd53276db => Ok(Self::MoonMempadReturnExcessesBackMsg(Box::new(
                MoonMempadReturnExcessesBackMsg::from_cell(cell)?,
            ))),
            0xd372158c => Ok(Self::MoonMempadTopUpTonsMsg(Box::new(MoonMempadTopUpTonsMsg::from_cell(cell)?))),
            0x7362d09c => Ok(Self::MoonMempadTransferNotificationForRecipientMsg(Box::new(
                MoonMempadTransferNotificationForRecipientMsg::from_cell(cell)?,
            ))),
            0x2508d66a => Ok(Self::MoonMempadUpgradeMinterCodeMsg(Box::new(
                MoonMempadUpgradeMinterCodeMsg::from_cell(cell)?,
            ))),
            0xfce532d8 => Ok(Self::MoonMempadFactoryPayoutResponseMsg(Box::new(
                MoonMempadFactoryPayoutResponseMsg::from_cell(cell)?,
            ))),
            0x0b8cca52 => Ok(Self::MoonMempadCurveResponseMsg(Box::new(
                MoonMempadCurveResponseMsg::from_cell(cell)?,
            ))),
            0x46f6a58e => Ok(Self::MoonMempadPoolDeploymentResponseMsg(Box::new(
                MoonMempadPoolDeploymentResponseMsg::from_cell(cell)?,
            ))),
            0xca6c5778 => Ok(Self::MoonMempadCurvePayoutResponseMsg(Box::new(
                MoonMempadCurvePayoutResponseMsg::from_cell(cell)?,
            ))),
            _ => Ok(Self::Unknown(cell.clone())),
        }
    }
    pub fn to_cell(&self) -> TonCoreResult<TonCell> {
        match self {
            Self::MoonMempadAskToBurnMsg(value) => value.to_cell(),
            Self::MoonMempadAskToTransferMsg(value) => value.to_cell(),
            Self::MoonMempadBurnNotificationForMinterMsg(value) => value.to_cell(),
            Self::MoonMempadChangeMinterAdminMsg(value) => value.to_cell(),
            Self::MoonMempadChangeMinterMetadataMsg(value) => value.to_cell(),
            Self::MoonMempadClaimMinterAdminMsg(value) => value.to_cell(),
            Self::MoonMempadDropMinterAdminMsg(value) => value.to_cell(),
            Self::MoonMempadInternalTransferStepMsg(value) => value.to_cell(),
            Self::MoonMempadMintNewJettonsMsg(value) => value.to_cell(),
            Self::MoonMempadBuyMsg(value) => value.to_cell(),
            Self::MoonMempadCreateLaunchMsg(value) => value.to_cell(),
            Self::MoonMempadCreatePoolMsg(value) => value.to_cell(),
            Self::MoonMempadCreatorClaimFeesMsg(value) => value.to_cell(),
            Self::MoonMempadDepositLiquidityMsg(value) => value.to_cell(),
            Self::MoonMempadFactoryWithdrawMsg(value) => value.to_cell(),
            Self::MoonMempadInitLaunchMsg(value) => value.to_cell(),
            Self::MoonMempadInternalUpgradeCurveMsg(value) => value.to_cell(),
            Self::MoonMempadProvidePoolAddressMsg(value) => value.to_cell(),
            Self::MoonMempadProvidePoolReadyMsg(value) => value.to_cell(),
            Self::MoonMempadTakePoolAddressMsg(value) => value.to_cell(),
            Self::MoonMempadTakePoolReadyMsg(value) => value.to_cell(),
            Self::MoonMempadTokenSalePayoutMsg(value) => value.to_cell(),
            Self::MoonMempadUpgradeCurveMsg(value) => value.to_cell(),
            Self::MoonMempadUpgradeFactoryMsg(value) => value.to_cell(),
            Self::MoonMempadRequestWalletAddressMsg(value) => value.to_cell(),
            Self::MoonMempadResponseWalletAddressMsg(value) => value.to_cell(),
            Self::MoonMempadReturnExcessesBackMsg(value) => value.to_cell(),
            Self::MoonMempadTopUpTonsMsg(value) => value.to_cell(),
            Self::MoonMempadTransferNotificationForRecipientMsg(value) => value.to_cell(),
            Self::MoonMempadUpgradeMinterCodeMsg(value) => value.to_cell(),
            Self::MoonMempadFactoryPayoutResponseMsg(value) => value.to_cell(),
            Self::MoonMempadCurveResponseMsg(value) => value.to_cell(),
            Self::MoonMempadPoolDeploymentResponseMsg(value) => value.to_cell(),
            Self::MoonMempadCurvePayoutResponseMsg(value) => value.to_cell(),
            Self::Unknown(value) => Ok(value.clone()),
        }
    }
}

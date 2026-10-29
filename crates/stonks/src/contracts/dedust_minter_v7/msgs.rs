//! dedust_minter_v7 msgs records.
use crate::contracts::ston_minter::msgs::StonksStonMinterAskToPresaleSellMsg;
use crate::contracts::ston_minter::msgs::StonksStonMinterBuyFromPresaleMsg;
use crate::contracts::ston_minter::msgs::StonksStonMinterChangeAdminMsg;
use crate::contracts::ston_minter::msgs::StonksStonMinterChangeContentMsg;
use crate::contracts::ston_minter::msgs::StonksStonMinterControllerInitMsg;
use crate::contracts::ston_minter::msgs::StonksStonMinterJettonBurnNotificationMsg;
use crate::contracts::ston_minter::msgs::StonksStonMinterJettonExcessesMsg;
use crate::contracts::ston_minter::msgs::StonksStonMinterJettonInternalTransferMsg;
use crate::contracts::ston_minter::msgs::StonksStonMinterJettonMintMsg;
use crate::contracts::ston_minter::msgs::StonksStonMinterPresaleSellNotificationForMinterMsg;
use crate::contracts::ston_minter::msgs::StonksStonMinterPresaleTradeEventMsg;
use crate::contracts::ston_minter::msgs::StonksStonMinterProvideWalletAddressMsg;
use crate::contracts::ston_minter::msgs::StonksStonMinterRetryMigrationMsg;
use crate::contracts::ston_minter::msgs::StonksStonMinterTakeWalletAddressMsg;
use crate::contracts::ston_minter::msgs::StonksStonMinterVirtualMinterInitMsg;
use crate::contracts::ston_minter::msgs::StonksStonMinterWalletStatusRequestMsg;
use crate::contracts::ston_minter::msgs::StonksStonMinterWithdrawTonMsg;
use ton::ton_core::TLB;
use ton::ton_core::cell::CellBuilder;
use ton::ton_core::cell::CellParser;
use ton::ton_core::cell::TonCell;
use ton::ton_core::errors::TonCoreError;
use ton::ton_core::errors::TonCoreResult;
use ton::ton_core::traits::tlb::TLB as _;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::TLBCoins;
use ton::ton_core::types::tlb_core::TLBRef;
use ton::ton_core::types::tlb_core::adapters::NullableAddress;
/// Migration parameters returned through the destination wallet callback.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xcc1a97b7, bits_len = 32)]
pub struct StonksDedustMinterV7MigrationParametersMsg {
    /// Pending query id.
    pub query_id: u64,
    /// Destination/launch wallet.
    #[tlb(adapter = "NullableAddress")]
    pub wallet: Option<TonAddress>,
    /// Pending TON seed allocation.
    pub ton_amount: TLBCoins,
    /// Pending token seed allocation.
    pub token_amount: TLBCoins,
    /// Destination creator fee share in basis points (5000 in captured v12, 4000 in v7).
    pub creator_fee_bps: u16,
}

/// Destination wallet request carrying the exact typed migration parameters.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xde8402ce, bits_len = 32)]
pub struct StonksDedustMinterV7MigrationRequestMsg {
    /// Correlation id.
    pub query_id: u64,
    /// Optional referenced migration continuation.
    #[tlb(adapter = "MigrationPayload")]
    pub custom_payload: Option<TLBRef<StonksDedustMinterV7MigrationParametersMsg>>,
}

/// Authenticated wallet callback; success alone does not prove settlement.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xce185bd7, bits_len = 32)]
pub struct StonksDedustMinterV7MigrationCallbackMsg {
    /// Pending query id.
    pub query_id: u64,
    /// Signed destination exit code.
    pub exit_code: i32,
    /// Original migration parameters.
    #[tlb(adapter = "MigrationPayload")]
    pub custom_payload: Option<TLBRef<StonksDedustMinterV7MigrationParametersMsg>>,
}

/// Legacy DeDust v7 minter dispatch. Its fixed buy/sell layouts are distinct
/// from current forwarding; the destination continuation is shared with v12.
#[derive(Debug, Clone, PartialEq, ton::ton_core::TLB)]
pub enum StonksDedustMinterV7Message {
    MigrationCallback(StonksDedustMinterV7MigrationCallbackMsg),
    JettonMint(StonksStonMinterJettonMintMsg),
    JettonBurnNotification(StonksStonMinterJettonBurnNotificationMsg),
    ProvideWalletAddress(StonksStonMinterProvideWalletAddressMsg),
    ChangeAdmin(StonksStonMinterChangeAdminMsg),
    ChangeContent(StonksStonMinterChangeContentMsg),
    VirtualMinterInit(StonksStonMinterVirtualMinterInitMsg),
    BuyFromPresale(StonksStonMinterBuyFromPresaleMsg),
    PresaleSellNotificationForMinter(StonksStonMinterPresaleSellNotificationForMinterMsg),
    AskToPresaleSell(StonksStonMinterAskToPresaleSellMsg),
    JettonInternalTransfer(StonksStonMinterJettonInternalTransferMsg),
    JettonExcesses(StonksStonMinterJettonExcessesMsg),
    TakeWalletAddress(StonksStonMinterTakeWalletAddressMsg),
    PresaleTradeEvent(StonksStonMinterPresaleTradeEventMsg),
    ControllerInit(StonksStonMinterControllerInitMsg),
    WalletStatus(StonksStonMinterWalletStatusRequestMsg),
    Withdraw(StonksStonMinterWithdrawTonMsg),
    Retry(StonksStonMinterRetryMigrationMsg),
    Unknown(TonCell),
}

/// A bad nested continuation opcode is malformed data inside a known message,
/// not a top-level variant mismatch that native enum dispatch may skip.
struct MigrationPayload;
impl MigrationPayload {
    fn read(
        &self,
        parser: &mut CellParser,
    ) -> TonCoreResult<Option<TLBRef<StonksDedustMinterV7MigrationParametersMsg>>> {
        Option::<TLBRef<StonksDedustMinterV7MigrationParametersMsg>>::read(parser).map_err(|error| match error {
            TonCoreError::TLBWrongPrefix { .. } => {
                TonCoreError::data(module_path!(), "invalid migration continuation opcode")
            },
            error => error,
        })
    }
    fn write(
        &self,
        builder: &mut CellBuilder,
        value: &Option<TLBRef<StonksDedustMinterV7MigrationParametersMsg>>,
    ) -> TonCoreResult<()> {
        value.write(builder)
    }
}

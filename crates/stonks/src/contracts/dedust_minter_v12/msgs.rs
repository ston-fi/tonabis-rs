//! dedust_minter_v12 msgs records.
use crate::contracts::dedust_minter_v7::msgs::StonksDedustMinterV7MigrationCallbackMsg;
use crate::contracts::ston_minter::msgs::StonksStonMinterAskToPresaleSellMsg;
use crate::contracts::ston_minter::msgs::StonksStonMinterBuyMsg;
use crate::contracts::ston_minter::msgs::StonksStonMinterChangeAdminMsg;
use crate::contracts::ston_minter::msgs::StonksStonMinterChangeContentMsg;
use crate::contracts::ston_minter::msgs::StonksStonMinterControllerInitMsg;
use crate::contracts::ston_minter::msgs::StonksStonMinterJettonBurnNotificationMsg;
use crate::contracts::ston_minter::msgs::StonksStonMinterJettonExcessesMsg;
use crate::contracts::ston_minter::msgs::StonksStonMinterJettonInternalTransferMsg;
use crate::contracts::ston_minter::msgs::StonksStonMinterJettonMintMsg;
use crate::contracts::ston_minter::msgs::StonksStonMinterPresaleTradeEventMsg;
use crate::contracts::ston_minter::msgs::StonksStonMinterProvideWalletAddressMsg;
use crate::contracts::ston_minter::msgs::StonksStonMinterRetryMigrationMsg;
use crate::contracts::ston_minter::msgs::StonksStonMinterSellNotificationMsg;
use crate::contracts::ston_minter::msgs::StonksStonMinterTakeWalletAddressMsg;
use crate::contracts::ston_minter::msgs::StonksStonMinterVirtualMinterInitMsg;
use crate::contracts::ston_minter::msgs::StonksStonMinterWalletStatusRequestMsg;
use crate::contracts::ston_minter::msgs::StonksStonMinterWithdrawTonMsg;
use ton::ton_core::cell::TonCell;
/// Current DeDust minter dispatch, including source-defined extensions and control.
#[derive(Debug, Clone, PartialEq, ton::ton_core::TLB)]
pub enum StonksDedustMinterV12Message {
    MigrationCallback(StonksDedustMinterV7MigrationCallbackMsg),
    Buy(StonksStonMinterBuyMsg),
    Sell(StonksStonMinterSellNotificationMsg),
    JettonMint(StonksStonMinterJettonMintMsg),
    JettonBurnNotification(StonksStonMinterJettonBurnNotificationMsg),
    ProvideWalletAddress(StonksStonMinterProvideWalletAddressMsg),
    ChangeAdmin(StonksStonMinterChangeAdminMsg),
    ChangeContent(StonksStonMinterChangeContentMsg),
    VirtualMinterInit(StonksStonMinterVirtualMinterInitMsg),
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

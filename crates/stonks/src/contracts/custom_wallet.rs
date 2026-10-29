//! StonksCustomWalletContract API. Providers and transaction context are caller-owned.
use crate::contracts::custom_wallet::method_results::StonksCustomWalletGetTaxResult;
use crate::contracts::custom_wallet::method_results::StonksCustomWalletGetWalletDataResult;
use crate::contracts::custom_wallet::storage::StonksCustomWalletJettonTransferContext;
use crate::contracts::custom_wallet::storage::StonksCustomWalletStorage;
use ton::block_tlb::StateInit;
use ton::ton_core::cell::TonCell;
use ton::ton_core::errors::TonCoreResult;
use ton::ton_core::ton_methods;
use ton::ton_core::traits::tlb::TLB as _;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::TLBCoins;
use ton::ton_core::types::tlb_core::TLBRef;
use ton::{contracts::TonContract, errors::TonResult};
ton::ton_contract!(StonksCustomWalletContract<StonksCustomWalletStorage>);
#[ton_methods]
impl StonksCustomWalletContract {
    /// Exact peer-wallet initialization emitted by the captured sTONks tax wallet.
    /// Both balances start at zero; context contains two literal standard zero
    /// addresses. The executable code is stored in both StateInit and wallet data.
    pub fn initial_state(
        code: TonCell,
        owner: Option<TonAddress>,
        master: Option<TonAddress>,
    ) -> TonCoreResult<StateInit> {
        let zero = Some(TonAddress::ZERO);
        let data = StonksCustomWalletStorage {
            jetton_balance: TLBCoins::ZERO,
            accounting_balance: TLBCoins::ZERO,
            transfer_context: TLBRef::new(StonksCustomWalletJettonTransferContext {
                context_address0: zero,
                context_address1: zero,
            }),
            owner_address: owner,
            jetton_master_address: master,
            jetton_wallet_code: TLBRef::new(code.clone()),
        }
        .to_cell()?;
        Ok(StateInit::new(code, data))
    }
    /// Read buy and sell taxes in per-mille units.
    pub async fn get_tax(&self) -> TonResult<StonksCustomWalletGetTaxResult>;
    /// Read the complete four-field wallet data.
    pub async fn get_wallet_data(&self) -> TonResult<StonksCustomWalletGetWalletDataResult>;
}
pub mod method_results;
pub mod msgs;
pub mod storage;

//! StonksDedustWalletContract API. Providers and transaction context are caller-owned.
use crate::contracts::dedust_wallet::method_results::StonksDedustWalletGetExtendedDataResult;
use crate::contracts::dedust_wallet::method_results::StonksDedustWalletGetWalletDataResult;
use crate::contracts::dedust_wallet::storage::StonksDedustWalletContext;
use crate::contracts::dedust_wallet::storage::StonksDedustWalletStorage;
use ton::block_tlb::StateInit;
use ton::ton_core::cell::TonCell;
use ton::ton_core::errors::TonCoreResult;
use ton::ton_core::ton_methods;
use ton::ton_core::traits::tlb::TLB as _;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::TLBCoins;
use ton::ton_core::types::tlb_core::TLBRef;
use ton::{contracts::TonContract, errors::TonResult};
ton::ton_contract!(StonksDedustWalletContract<StonksDedustWalletStorage>);
#[ton_methods]
impl StonksDedustWalletContract {
    /// Exact empty-wallet initial data used by captured DeDust v7/v12 get_wallet_address.
    /// Caller supplies the pinned executable code and selects workchain explicitly.
    pub fn initial_state(
        code: TonCell,
        owner: Option<TonAddress>,
        master: Option<TonAddress>,
    ) -> TonCoreResult<StateInit> {
        let context = StonksDedustWalletContext {
            authority: Some(TonAddress::ZERO),
            flag: false,
        };
        let data = StonksDedustWalletStorage {
            balance: TLBCoins::ZERO,
            owner,
            master,
            context: TLBRef::new(context),
        }
        .to_cell()?;
        Ok(StateInit::new(code, data))
    }
    /// Read the complete four-field wallet data.
    pub async fn get_wallet_data(&self) -> TonResult<StonksDedustWalletGetWalletDataResult>;
    /// Read the seven-field DeDust presale wallet context at numeric selector 84724.
    #[ton_method(id = 84724)]
    pub async fn get_extended_data(&self) -> TonResult<StonksDedustWalletGetExtendedDataResult>;
}
pub mod method_results;
pub mod msgs;
pub mod storage;

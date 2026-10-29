//! StonksStonWalletContract API. Providers and transaction context are caller-owned.
use crate::contracts::dedust_wallet::method_results::StonksDedustWalletGetWalletDataResult;
use crate::contracts::ston_wallet::method_results::StonksStonWalletGetExtendedDataResult;
use crate::contracts::ston_wallet::storage::StonksStonWalletContext;
use crate::contracts::ston_wallet::storage::StonksStonWalletStorage;
use ton::block_tlb::StateInit;
use ton::ton_core::cell::TonCell;
use ton::ton_core::errors::TonCoreResult;
use ton::ton_core::ton_methods;
use ton::ton_core::traits::tlb::TLB as _;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::TLBCoins;
use ton::ton_core::types::tlb_core::TLBRef;
use ton::{contracts::TonContract, errors::TonResult};
ton::ton_contract!(StonksStonWalletContract<StonksStonWalletStorage>);
#[ton_methods]
impl StonksStonWalletContract {
    /// Derive the captured STON wallet state with its distinct initial context.
    pub fn initial_state(
        code: TonCell,
        owner: Option<TonAddress>,
        master: Option<TonAddress>,
    ) -> TonCoreResult<StateInit> {
        let zero = Some(TonAddress::ZERO);
        let context = StonksStonWalletContext {
            address0: zero,
            address1: zero,
            flag: true,
        };
        Ok(StateInit::new(
            code,
            StonksStonWalletStorage {
                balance: TLBCoins::ZERO,
                owner,
                master,
                context: TLBRef::new(context),
            }
            .to_cell()?,
        ))
    }
    /// Read the complete four-field wallet data.
    pub async fn get_wallet_data(&self) -> TonResult<StonksDedustWalletGetWalletDataResult>;
    /// Read the seven-field STON presale wallet context at numeric selector 74038.
    #[ton_method(id = 74038)]
    pub async fn get_extended_data(&self) -> TonResult<StonksStonWalletGetExtendedDataResult>;
}
pub mod method_results;
pub mod msgs;
pub mod storage;

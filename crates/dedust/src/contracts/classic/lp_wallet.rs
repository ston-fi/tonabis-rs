//! Classic LP wallet surface, reconciled with the embedded V1 wallet code and
//! SDK0.8.7. Caller-owned forwarding tails preserve the exact source STSLICE
//! boundary, including forms the wallet forwards without decoding.
use self::{method_results::DedustClassicLpWalletWalletData, storage::DedustClassicLpWalletStorage};
use ton::ton_core::ton_methods;
use ton::ton_core::types::TonAddress;
use ton::{
    block_tlb::StateInit,
    contracts::TonContract,
    errors::TonResult,
    ton_contract,
    ton_core::{
        cell::TonCell,
        errors::TonCoreError,
        traits::tlb::TLB,
        types::tlb_core::{TLBCoins, TLBRef},
    },
};

ton_contract!(DedustClassicLpWalletContract<DedustClassicLpWalletStorage>);

#[ton_methods]
impl DedustClassicLpWalletContract {
    pub async fn get_wallet_data(&self) -> TonResult<DedustClassicLpWalletWalletData>;
}
/// Pool getters commit the same wallet code in data and StateInit. Callers must
/// supply the revision's captured code; no historical code is silently chosen.
pub fn state_init(code: TonCell, pool: TonAddress, owner: TonAddress) -> Result<StateInit, TonCoreError> {
    let data = DedustClassicLpWalletStorage {
        balance: TLBCoins::new(0),
        owner,
        pool,
        wallet_code: TLBRef::new(code.clone()),
    };
    Ok(StateInit::new(code, data.to_cell()?))
}

pub mod method_results;
pub mod msgs;

pub mod storage;

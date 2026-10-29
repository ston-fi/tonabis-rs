//! Moon order_new contract API; storage and structured results live in child modules.
use crate::contracts::order_new::method_results::MoonOrderNewGetVestingDataResult;
use crate::contracts::order_new::storage::MoonOrderNewStorage;
use fastnum::I512;
use ton::contracts::TonContract;
use ton::errors::TonResult;
use ton::ton_core::cell::TonCell;
use ton::ton_core::ton_methods;
ton::ton_contract!(MoonOrderNewContract<MoonOrderNewStorage>);
#[ton_methods]
impl MoonOrderNewContract {
    pub async fn get_status(&self) -> TonResult<u8>;
    pub async fn get_order_amount(&self) -> TonResult<I512>;
    pub async fn get_order(&self) -> TonResult<TonCell>;
    pub async fn get_wallets(&self) -> TonResult<TonCell>;
    pub async fn get_vesting_data(&self) -> TonResult<MoonOrderNewGetVestingDataResult>;
    pub async fn get_fill_out(&self, amount_in: I512) -> TonResult<I512>;
    pub async fn get_assets(&self) -> TonResult<TonCell>;

    pub async fn get_collected_fees(&self) -> TonResult<I512>;
}

pub mod method_results;
pub mod msgs;
pub mod storage;

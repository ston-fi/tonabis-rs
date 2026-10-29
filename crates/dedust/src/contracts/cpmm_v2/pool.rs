//! DedustV2Cpmm contract API. Data records live in [`storage`].
use self::{
    method_results::{DedustV2CpmmPoolGetLiquidityAmountsResult, DedustV2CpmmPoolGetPoolDataResult},
    storage::DedustV2CpmmStorage,
};
use fastnum::I512;
use ton::{
    block_tlb::TVMStack,
    contracts::TonContract,
    errors::TonResult,
    ton_contract,
    ton_core::{ton_methods, types::TonAddress},
};

pub mod method_results;
pub mod storage;
ton_contract!(DedustV2CpmmContract<DedustV2CpmmStorage>);

pub mod msgs;

#[ton_methods]
impl DedustV2CpmmContract {
    pub async fn get_position_address(&self, owner_address: TonAddress) -> TonResult<TonAddress> {
        let mut stack = TVMStack::default();
        stack.push_cell_slice(tonabis_core::address::internal_address_cell(&owner_address)?);
        self.emulate_get_method(72157i32, &stack).await
    }

    pub async fn get_pool_data(&self) -> TonResult<DedustV2CpmmPoolGetPoolDataResult>;
    /// Captured nominal liquidity valuation. Zero pool liquidity returns zeroes; signed inputs and amounts above total are arithmetic getter values, not executable withdrawal authorization.
    #[ton_method(id = 112421)]
    pub async fn get_liquidity_amounts(&self, liquidity: I512) -> TonResult<DedustV2CpmmPoolGetLiquidityAmountsResult>;
}

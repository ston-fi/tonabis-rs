//! V2 position-NFT API, referenced position state, deposit callbacks and rich bounce records.
use crate::contracts::v2::position_nft::method_results::ToncoPositionNftV2GetNftDataResult;
use crate::contracts::v2::position_nft::method_results::ToncoPositionNftV2GetPositionInfoResult;
use crate::contracts::v2::position_nft::storage::ToncoPositionNftV2Storage;
use ton::block_tlb::TVMStack;
use ton::ton_core::types::TonAddress;
use tonabis_core::adapters::nullable_internal_address::NullableInternalAddress;
pub mod method_results;
pub mod msgs;
pub mod storage;

use ton::ton_core::ton_methods;
use ton::ton_core::types::tlb_core::MsgAddress;
use ton::{contracts::TonContract, errors::TonResult, ton_contract};

ton_contract!(ToncoPositionNftV2Contract<ToncoPositionNftV2Storage>);

#[ton_methods]
impl ToncoPositionNftV2Contract {
    pub async fn get_pool_address(&self) -> TonResult<Option<TonAddress>> {
        let stack = TVMStack::default();
        let value: MsgAddress = self.emulate_get_method("getPoolAddress", &stack).await?;
        Ok(NullableInternalAddress::from_address(value)?)
    }
    pub async fn get_user_address(&self) -> TonResult<Option<TonAddress>> {
        let stack = TVMStack::default();
        let value: MsgAddress = self.emulate_get_method("getUserAddress", &stack).await?;
        Ok(NullableInternalAddress::from_address(value)?)
    }
    #[ton_method(name = "getPositionInfo")]
    pub async fn get_position_info(&self) -> TonResult<ToncoPositionNftV2GetPositionInfoResult>;

    pub async fn get_nft_data(&self) -> TonResult<ToncoPositionNftV2GetNftDataResult>;
}

//! V1 position-NFT API and incompatible V1 initialization/burn bodies.
use crate::contracts::v1::position_nft::method_results::ToncoPositionNftV1GetNftDataResult;
use crate::contracts::v1::position_nft::method_results::ToncoPositionNftV1GetPositionInfoResult;
use crate::contracts::v1::position_nft::storage::ToncoPositionNftV1Storage;
use ton::block_tlb::TVMStack;
use ton::ton_core::types::TonAddress;
use tonabis_core::adapters::nullable_internal_address::NullableInternalAddress;
pub mod method_results;
pub mod msgs;
pub mod storage;

use ton::ton_core::ton_methods;
use ton::ton_core::types::tlb_core::MsgAddress;
use ton::{contracts::TonContract, errors::TonResult, ton_contract};

// Adapted from ton-blockchain/abis 7dc57067, MIT; see reference provenance.

ton_contract!(ToncoPositionNftV1Contract<ToncoPositionNftV1Storage>);

#[ton_methods]
impl ToncoPositionNftV1Contract {
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
    pub async fn get_position_info(&self) -> TonResult<ToncoPositionNftV1GetPositionInfoResult>;

    pub async fn get_nft_data(&self) -> TonResult<ToncoPositionNftV1GetNftDataResult>;
}

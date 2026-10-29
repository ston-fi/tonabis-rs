//! StonksDedustMinterV12Contract API. Providers and transaction context are caller-owned.
use crate::contracts::dedust_minter_v12::method_results::StonksDedustMinterV12GetBondingDataResult;
use crate::contracts::dedust_minter_v12::method_results::StonksDedustMinterV12GetExtendedDataResult;
use crate::contracts::dedust_minter_v12::method_results::StonksDedustMinterV12GetFeeDataResult;
use crate::contracts::ston_minter::method_results::StonksStonMinterGetJettonDataResult;
use crate::contracts::ston_minter::storage::StonksVirtualMinterStorage;
use ton::block_tlb::TVMStack;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::adapters::NullableAddress;
use ton::ton_core::{ton_methods, types::tlb_core::MsgAddress};
use ton::{contracts::TonContract, errors::TonResult};
use tonabis_core::adapters::nullable_internal_address::NullableInternalAddress;
ton::ton_contract!(StonksDedustMinterV12Contract<StonksVirtualMinterStorage>);
#[ton_methods]
impl StonksDedustMinterV12Contract {
    /// Read the complete generation-specific bonding stack.
    pub async fn get_bonding_data(&self) -> TonResult<StonksDedustMinterV12GetBondingDataResult>;
    /// Read standard five-field Jetton master data.
    pub async fn get_jetton_data(&self) -> TonResult<StonksStonMinterGetJettonDataResult>;
    /// Read the native version value; callers select and authenticate the generation.
    pub async fn get_version(&self) -> TonResult<u32>;
    /// Derive the owner wallet through the minter, packing the wire owner as a Slice.
    pub async fn get_wallet_address(&self, owner_address: Option<TonAddress>) -> TonResult<Option<TonAddress>> {
        let mut builder = TonCell::builder();
        NullableAddress.write(&mut builder, &owner_address)?;
        let mut stack = TVMStack::default();
        stack.push_cell_slice(builder.build()?);
        let result: MsgAddress = self.emulate_get_method("get_wallet_address", &stack).await?;
        Ok(NullableInternalAddress::from_address(result)?)
    }
    /// Read the complete generation-specific extended stack at numeric selector 82057.
    #[ton_method(id = 82057)]
    pub async fn get_extended_data(&self) -> TonResult<StonksDedustMinterV12GetExtendedDataResult>;
    /// Read CPMM fee, total presale tax, creator share and recipient in that order.
    #[ton_method(id = 71772)]
    pub async fn get_fee_data(&self) -> TonResult<StonksDedustMinterV12GetFeeDataResult>;
}
pub mod method_results;
pub mod msgs;
pub mod storage;

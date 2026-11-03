//! Jetton-quoted v12 minter. Its undocumented persistent storage remains opaque.
use crate::contracts::anypair_minter_v12::method_results::StonksAnypairMinterV12GetBondingDataResult;
use crate::contracts::ston_minter::method_results::StonksStonMinterGetJettonDataResult;
use ton::block_tlb::TVMStack;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::adapters::NullableAddress;
use ton::ton_core::{ton_methods, types::tlb_core::MsgAddress};
use ton::{contracts::TonContract, errors::TonResult};
use tonabis_core::adapters::nullable_internal_address::NullableInternalAddress;

ton::ton_contract!(StonksAnypairMinterV12Contract<TonCell>);

#[ton_methods]
impl StonksAnypairMinterV12Contract {
    /// Read all thirteen fields, including the quote jetton master.
    pub async fn get_bonding_data(&self) -> TonResult<StonksAnypairMinterV12GetBondingDataResult>;
    /// Read standard five-field Jetton master data.
    pub async fn get_jetton_data(&self) -> TonResult<StonksStonMinterGetJettonDataResult>;
    /// Read the version; v12 alone does not distinguish TON and jetton quote assets.
    pub async fn get_version(&self) -> TonResult<u32>;
    /// Resolve the launched-token wallet, preserving an absent versus internal-zero owner.
    pub async fn get_wallet_address(&self, owner_address: Option<TonAddress>) -> TonResult<Option<TonAddress>> {
        let mut builder = TonCell::builder();
        NullableAddress.write(&mut builder, &owner_address)?;
        let mut stack = TVMStack::default();
        stack.push_cell_slice(builder.build()?);
        let result: MsgAddress = self.emulate_get_method("get_wallet_address", &stack).await?;
        Ok(NullableInternalAddress::from_address(result)?)
    }
}

pub mod method_results;

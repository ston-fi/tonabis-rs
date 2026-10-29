//! Getter result records and protocol-specific stack decoding.
use ton::ton_core::FromTVMStack;
use ton::ton_core::types::Coins;
use ton::ton_core::{cell::TonCell, types::TonAddress};
use tonabis_core::adapters::nullable_internal_address::NullableInternalAddress;

#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct DedustUranusMemeV2GetMemeDataResult {
    pub initialized: bool,
    pub migrated: bool,
    pub controller_address: TonAddress,
    pub creator_address: TonAddress,
    pub creator_fee: Coins,
    pub seed: u128,
    pub is_graduated: bool,
    pub alpha: Coins,
    pub beta: Coins,
    pub on_sell_supply: Coins,
    pub trade_fee_bps: u16,
    pub raised_funds: Coins,
    pub current_supply: Coins,
}

#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct DedustUranusMemeV2GetJettonDataResult {
    pub total_supply: Coins,
    pub mintable: bool,
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub admin_address: Option<TonAddress>,
    pub jetton_content: TonCell,
    pub jetton_wallet_code: TonCell,
}

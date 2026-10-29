//! Persistent TL-B records and local caller-code StateInit construction. No deployment is performed.
use std::collections::HashMap;
use ton::block_tlb::StateInit;
use ton::tlb_adapters::DictKeyAdapterInt;
use ton::tlb_adapters::DictValAdapterTLB;
use ton::tlb_adapters::TLBHashMapE;
use ton::ton_core::errors::TonCoreResult;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::adapters::NullableAddress;

use fastnum::U256;
use ton::ton_core::TLB;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::tlb_core::adapters::ConstLen;
use ton::ton_core::types::tlb_core::{TLBCoins, TLBRef};

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoPoolV1Storage {
    #[tlb(adapter = "NullableAddress")]
    pub router_address: Option<TonAddress>,
    pub lp_fee_base: u16,
    pub protocol_fee: u16,
    pub lp_fee_current: u16,
    #[tlb(adapter = "NullableAddress")]
    pub jetton0_wallet: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub jetton1_wallet: Option<TonAddress>,
    #[tlb(bits_len = 24)]
    pub tick_spacing: u32,
    pub seqno: u64,
    pub fees_and_reserves: TLBRef<ToncoPoolV1FeesAndReserves>,
    pub state: TLBRef<ToncoPoolV1State>,
    pub ticks: TLBRef<ToncoPoolV1Ticks>,
    pub child_contracts: TLBRef<ToncoPoolV1ChildContracts>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoPoolV1FeesAndReserves {
    pub fee_growth_global0_x128: U256,
    pub fee_growth_global1_x128: U256,
    pub collected_protocol_fee0: u128,
    pub collected_protocol_fee1: u128,
    pub reserve0: TLBCoins,
    pub reserve1: TLBCoins,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoPoolV1State {
    pub pool_active: bool,
    #[tlb(bits_len = 24)]
    pub tick: i32,
    #[tlb(bits_len = 160)]
    pub price_sqrt: U256,
    pub liquidity: u128,
    #[tlb(bits_len = 24)]
    pub occupied_ticks: u32,
    pub nftv3item_counter: u64,
    pub nftv3items_active: u64,
    #[tlb(adapter = "NullableAddress")]
    pub admin_address: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub controller_address: Option<TonAddress>,
    pub minters: TLBRef<ToncoPoolV1Minters>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoPoolV1Minters {
    #[tlb(adapter = "NullableAddress")]
    pub jetton0_minter: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub jetton1_minter: Option<TonAddress>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoPoolV1Ticks {
    #[tlb(adapter = "TLBHashMapE::<DictKeyAdapterInt<24, i32>, DictValAdapterTLB<_>>::new(24)")]
    pub ticks: HashMap<i32, ToncoPoolV1TickInfo>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoPoolV1TickInfo {
    pub liquidity_gross: U256,
    pub liquidity_net: i128,
    pub outer_fee_growth0_token: U256,
    pub outer_fee_growth1_token: U256,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoPoolV1ChildContracts {
    pub accountv3_code: TLBRef<TonCell>,
    pub position_nftv3_code: TLBRef<TonCell>,
    pub nftv3_content: TLBRef<TonCell>,
    pub nftv3item_content: TLBRef<TonCell>,
}

impl ToncoPoolV1Storage {
    /// Encode this exact revision's data with caller-supplied code. Does not deploy.
    pub fn state_init(&self, code: TonCell) -> TonCoreResult<StateInit> {
        Ok(StateInit::new(code, ton::ton_core::traits::tlb::TLB::to_cell(self)?))
    }
    /// Hash the standard state-init; caller owns code identity and workchain policy.
    pub fn derive_address(&self, code: TonCell, workchain: i32) -> TonCoreResult<TonAddress> {
        self.state_init(code)?.derive_address(workchain)
    }
}

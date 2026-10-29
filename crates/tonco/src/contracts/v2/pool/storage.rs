//! Persistent TL-B records and local caller-code StateInit construction. No deployment is performed.
use std::collections::HashMap;
use ton::block_tlb::StateInit;
use ton::tlb_adapters::DictKeyAdapterInt;
use ton::tlb_adapters::DictValAdapterTLB;
use ton::tlb_adapters::TLBHashMapE;
use ton::ton_core::cell::CellBuilder;
use ton::ton_core::cell::CellParser;
use ton::ton_core::errors::TonCoreResult;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::adapters::NullableAddress;

use fastnum::{I128, I256, U256};
use ton::ton_core::TLB;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::tlb_core::adapters::ConstLen;
use ton::ton_core::types::tlb_core::{TLBCoins, TLBRef};

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoPoolV2TickInfo {
    #[tlb(bits_len = 160)]
    pub price_cache: U256,
    pub liquidity_gross: U256,
    pub liquidity_net: I128,
    #[tlb(bits_len = 224)]
    pub outer_fee_growth0: I256,
    #[tlb(bits_len = 224)]
    pub outer_fee_growth1: I256,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoPoolV2Accounting {
    pub fee_growth_global0: U256,
    pub fee_growth_global1: U256,
    pub protocol_fees0: u128,
    pub protocol_fees1: u128,
    pub reserve0: TLBCoins,
    pub reserve1: TLBCoins,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoPoolV2Minters {
    #[tlb(adapter = "NullableAddress")]
    pub token0: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub token1: Option<TonAddress>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoPoolV2Roles {
    #[tlb(adapter = "NullableAddress")]
    pub arbiter: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub alm: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub creator: Option<TonAddress>,
    pub minters: TLBRef<ToncoPoolV2Minters>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoPoolV2Oracle {
    #[tlb(adapter = "NullableAddress")]
    pub address: Option<TonAddress>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoPoolV2State {
    #[tlb(bits_len = 24)]
    pub tick: i32,
    #[tlb(bits_len = 160)]
    pub price: U256,
    pub liquidity: u128,
    #[tlb(bits_len = 24)]
    pub occupied_ticks: u32,
    #[tlb(adapter = "TLBHashMapE::<DictKeyAdapterInt<24,i32>, DictValAdapterTLB<_>>::new(24)")]
    pub ticks: HashMap<i32, ToncoPoolV2TickInfo>,
    #[tlb(adapter = "NullableAddress")]
    pub admin: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub controller: Option<TonAddress>,
    pub roles: TLBRef<ToncoPoolV2Roles>,
    pub oracle: TLBRef<ToncoPoolV2Oracle>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoPoolV2ChildContracts {
    pub account_code: TLBRef<TonCell>,
    pub position_code: TLBRef<TonCell>,
    pub collection_content: TLBRef<TonCell>,
    pub item_content: TLBRef<TonCell>,
}

/// Runtime pending updates are untagged after the initial delay.
#[derive(Debug, Clone, PartialEq)]
pub struct ToncoPoolV2Timelock {
    pub delay: u64,
    pub pending: Option<ToncoPoolV2PendingUpdates>,
}

impl ton::ton_core::traits::tlb::TLB for ToncoPoolV2Timelock {
    fn read_definition(p: &mut CellParser) -> TonCoreResult<Self> {
        let delay = u64::read(p)?;
        let pending = if p.data_bits_left()? == 0 {
            None
        } else {
            Some(ToncoPoolV2PendingUpdates::read(p)?)
        };
        p.ensure_empty()?;
        Ok(Self { delay, pending })
    }
    fn write_definition(&self, b: &mut CellBuilder) -> TonCoreResult<()> {
        self.delay.write(b)?;
        if let Some(v) = &self.pending {
            v.write(b)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoPoolV2PendingUpdates {
    pub code_time: u64,
    pub code: Option<TLBRef<TonCell>>,
    pub alm_time: u64,
    #[tlb(adapter = "NullableAddress")]
    pub alm: Option<TonAddress>,
    pub arbiter_time: u64,
    #[tlb(adapter = "NullableAddress")]
    pub arbiter: Option<TonAddress>,
    pub flags_time: u64,
    pub flags: u64,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoPoolV2Helper {
    pub nft_counter: u64,
    pub nft_active: u64,
    pub children: TLBRef<ToncoPoolV2ChildContracts>,
    pub timelock: TLBRef<ToncoPoolV2Timelock>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoPoolV2Storage {
    #[tlb(adapter = "NullableAddress")]
    pub router: Option<TonAddress>,
    pub active: bool,
    pub base_fee: u16,
    pub protocol_fee: u16,
    pub current_fee: u16,
    #[tlb(adapter = "NullableAddress")]
    pub wallet0: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub wallet1: Option<TonAddress>,
    #[tlb(bits_len = 24)]
    pub spacing: u32,
    pub seqno: u64,
    pub flags: u64,
    pub accounting: TLBRef<ToncoPoolV2Accounting>,
    pub state: TLBRef<ToncoPoolV2State>,
    pub helper: TLBRef<ToncoPoolV2Helper>,
}

impl ToncoPoolV2Storage {
    /// Encode this exact revision's data with caller-supplied code. Does not deploy.
    pub fn state_init(&self, code: TonCell) -> TonCoreResult<StateInit> {
        Ok(StateInit::new(code, ton::ton_core::traits::tlb::TLB::to_cell(self)?))
    }
    /// Hash the standard state-init; caller owns code identity and workchain policy.
    pub fn derive_address(&self, code: TonCell, workchain: i32) -> TonCoreResult<TonAddress> {
        self.state_init(code)?.derive_address(workchain)
    }
}

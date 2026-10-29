//! Moon wire records. See tools/reference/moon/README.md.

use crate::contracts::metadata::MoonPrefixedString;
use fastnum::I512;
use std::collections::HashMap;
use ton::{
    tlb_adapters::{DictKeyAdapterTonHash, DictValAdapterTLB, TLBHashMapE},
    ton_core::{
        TLB,
        cell::{CellBuilder, CellParser, TonCell, TonHash},
        errors::{TonCoreError, TonCoreResult},
        traits::tlb::TLB,
        types::{
            TonAddress,
            tlb_core::{
                TLBCoins, TLBRef,
                adapters::{ConstLen, InternalAddress, NullableAddress},
            },
        },
    },
};
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonMempadCurveInitData {
    #[tlb(adapter = "InternalAddress")]
    pub factory: TonAddress,
    #[tlb(adapter = "InternalAddress")]
    pub creator: TonAddress,
    pub salt: u64,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonMempadCurveStorage {
    #[tlb(adapter = "InternalAddress")]
    pub factory: TonAddress,
    #[tlb(adapter = "InternalAddress")]
    pub creator: TonAddress,
    pub salt: u64,
    pub state: MoonMempadCurveState,
    pub params: TLBRef<MoonMempadParams>,
    pub trading: TLBRef<MoonMempadTradingState>,
    pub grad: TLBRef<MoonMempadGradState>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonMempadEconState {
    pub tokens_for_sale: TLBCoins,
    pub tokens_for_lp: TLBCoins,
    pub virtual_gram: TLBCoins,
    pub virtual_tokens: TLBCoins,
    pub target_gram: TLBCoins,
    pub trade_fee_bps: u16,
    pub creator_fee_bps: u16,
    pub pool_fee_bps: u8,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonMempadGradState {
    #[tlb(adapter = "InternalAddress")]
    pub cpmm_factory: TonAddress,
    #[tlb(adapter = "InternalAddress")]
    pub protocol_recipient: TonAddress,
    #[tlb(adapter = "NullableAddress")]
    pub pool: Option<TonAddress>,
    pub state: MoonMempadGraduation,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonMempadJettonDataReply {
    #[tlb(bits_len = 257)]
    pub total_supply: I512,
    pub mintable: bool,
    #[tlb(adapter = "NullableAddress")]
    pub admin_address: Option<TonAddress>,
    pub jetton_content: TLBRef<MoonMempadOnchainMetadataReply>,
    pub jetton_wallet_code: TLBRef<TonCell>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonMempadMinterStorage {
    pub total_supply: TLBCoins,
    #[tlb(adapter = "NullableAddress")]
    pub admin_address: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub next_admin_address: Option<TonAddress>,
    pub metadata: TLBRef<TonCell>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x00, bits_len = 8)]
pub struct MoonMempadOnchainMetadataReply {
    #[tlb(adapter = "TLBHashMapE::<DictKeyAdapterTonHash, DictValAdapterTLB<TLBRef<MoonPrefixedString>>>::new(256)")]
    pub content_dict: HashMap<TonHash, TLBRef<MoonPrefixedString>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonMempadParams {
    pub econ: TLBRef<MoonMempadEconState>,
    pub jw_code: TLBRef<TonCell>,
    pub metadata: TLBRef<TonCell>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct MoonMempadTradingState {
    pub gram_reserve: TLBCoins,
    pub supply: TLBCoins,
    pub creator_fees: TLBCoins,
    pub platform_fees: TLBCoins,
}

// Launch lifecycle values with their protocol-defined widths and valid tags.
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct MoonMempadCurveState(u8);
impl MoonMempadCurveState {
    pub fn value(self) -> u8 {
        self.0
    }
    pub fn try_new(value: u8) -> TonCoreResult<Self> {
        if matches!(value, 1..=3) {
            Ok(Self(value))
        } else {
            ton::ton_core::bail_ton_core_data!("invalid CurveState")
        }
    }
}
impl TLB for MoonMempadCurveState {
    fn read_definition(parser: &mut CellParser) -> TonCoreResult<Self> {
        Self::try_new(parser.read_num(2)?)
    }
    fn write_definition(&self, builder: &mut CellBuilder) -> TonCoreResult<()> {
        builder.write_num(&self.0, 2)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct MoonMempadGraduation(u8);
impl MoonMempadGraduation {
    pub fn value(self) -> u8 {
        self.0
    }
    pub fn try_new(value: u8) -> TonCoreResult<Self> {
        if matches!(value, 0..=2 | 4) {
            Ok(Self(value))
        } else {
            ton::ton_core::bail_ton_core_data!("invalid Graduation")
        }
    }
}
impl TLB for MoonMempadGraduation {
    fn read_definition(parser: &mut CellParser) -> TonCoreResult<Self> {
        Self::try_new(parser.read_num(3)?)
    }
    fn write_definition(&self, builder: &mut CellBuilder) -> TonCoreResult<()> {
        builder.write_num(&self.0, 3)
    }
}

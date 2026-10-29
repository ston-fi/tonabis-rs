//! Persistent TL-B records and local caller-code StateInit construction. No deployment is performed.
use ton::block_tlb::StateInit;
use ton::ton_core::errors::TonCoreResult;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::adapters::NullableAddress;

use ton::ton_core::TLB;
use ton::ton_core::cell::{CellBuilder, CellParser, TonCell};
use ton::ton_core::types::tlb_core::TLBRef;

/// Some initializers omit a trailing uint64 that runtime state may contain.
/// `None` encodes zero bits, without a Maybe tag. Its role is contract-specific.
#[derive(Debug, Clone, PartialEq)]
pub struct ToncoRouterV2OptionalU64(pub Option<u64>);

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoRouterV2ChildContracts {
    #[tlb(adapter = "NullableAddress")]
    pub pool_admin: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub factory: Option<TonAddress>,
    pub prototype_code: TLBRef<TonCell>,
    pub pool_code: TLBRef<TonCell>,
    pub account_code: TLBRef<TonCell>,
    pub position_code: TLBRef<TonCell>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoRouterV2PendingUpdates {
    pub admin_time: u64,
    #[tlb(adapter = "NullableAddress")]
    pub admin: Option<TonAddress>,
    pub proxy_time: u64,
    #[tlb(adapter = "NullableAddress")]
    pub proxy: Option<TonAddress>,
    pub flags_time: u64,
    pub flags: u64,
    pub code_time: u64,
    pub code: Option<TLBRef<TonCell>>,
    pub pool_code_time: u64,
    pub pool_code: Option<TLBRef<TonCell>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoRouterV2Storage {
    #[tlb(adapter = "NullableAddress")]
    pub admin: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub proxy_wallet: Option<TonAddress>,
    pub flags: u64,
    pub seqno: u64,
    pub children: TLBRef<ToncoRouterV2ChildContracts>,
    pub timelock: TLBRef<ToncoRouterV2Timelock>,
    pub nonce: ToncoRouterV2OptionalU64,
}

/// Initial timelock cell has only delay; runtime pending updates are an untagged suffix.
#[derive(Debug, Clone, PartialEq)]
pub struct ToncoRouterV2Timelock {
    pub delay: u64,
    pub pending: Option<ToncoRouterV2PendingUpdates>,
}

impl ton::ton_core::traits::tlb::TLB for ToncoRouterV2Timelock {
    fn read_definition(p: &mut CellParser) -> TonCoreResult<Self> {
        let delay = u64::read(p)?;
        let pending = if p.data_bits_left()? == 0 {
            None
        } else {
            Some(ToncoRouterV2PendingUpdates::read(p)?)
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

impl ToncoRouterV2Storage {
    /// Encode this exact revision's data with caller-supplied code. Does not deploy.
    pub fn state_init(&self, code: TonCell) -> TonCoreResult<StateInit> {
        Ok(StateInit::new(code, ton::ton_core::traits::tlb::TLB::to_cell(self)?))
    }
    /// Hash the standard state-init; caller owns code identity and workchain policy.
    pub fn derive_address(&self, code: TonCell, workchain: i32) -> TonCoreResult<TonAddress> {
        self.state_init(code)?.derive_address(workchain)
    }
}

impl ton::ton_core::traits::tlb::TLB for ToncoRouterV2OptionalU64 {
    fn read_definition(p: &mut CellParser) -> TonCoreResult<Self> {
        let value = if p.data_bits_left()? == 0 {
            None
        } else {
            Some(u64::read(p)?)
        };
        p.ensure_empty()?;
        Ok(Self(value))
    }
    fn write_definition(&self, b: &mut CellBuilder) -> TonCoreResult<()> {
        if let Some(v) = self.0 {
            v.write(b)?;
        }
        Ok(())
    }
}

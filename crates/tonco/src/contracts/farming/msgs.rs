//! Wire records and dispatch for this family. Unknown cells survive; see the crate README for dispatch rules.
use crate::contracts::native::wallet::msgs::ToncoNativeWalletTransferMsg;
use fastnum::U256;
use ton::ton_core::TLB;
use ton::ton_core::cell::CellBuilder;
use ton::ton_core::cell::CellParser;
use ton::ton_core::cell::TonCell;
use ton::ton_core::errors::TonCoreResult;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::adapters::NullableAddress;
use ton::ton_core::types::tlb_core::{TLBCoins, TLBRef};

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoFarmingEntry {
    pub farming_id: u32,
    pub position_id: u128,
    #[tlb(adapter = "NullableAddress")]
    pub wallet: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub recipient: Option<TonAddress>,
    pub amount: TLBCoins,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x923b38fa, bits_len = 32)]
pub struct ToncoFarmingDeployMsg {
    pub query_id: u64,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xf0febe64, bits_len = 32)]
pub struct ToncoFarmingUpdateMerkleRootMsg {
    pub query_id: u64,
    pub farming_id: u32,
    pub root: U256,
}

#[derive(Debug, Clone, PartialEq, TLB)]
/// External claim body: query ID, proof ref and optional legacy inline index.
pub struct ToncoFarmingClaimMsg {
    pub query_id: u64,
    pub proof: TLBRef<TonCell>,
    pub legacy_index: ToncoFarmingLegacyIndex,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub enum ToncoFarmingIncomingMessage {
    ClaimCallback(Box<ToncoFarmingClaimCallbackMsg>),
    Deploy(Box<ToncoFarmingDeployMsg>),
    UpdateMerkleRoot(Box<ToncoFarmingUpdateMerkleRootMsg>),
    Unknown(TonCell),
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xa769de27, bits_len = 32)]
pub struct ToncoFarmingClaimCallbackMsg {
    pub query_id: u64,
    pub farming_id: u32,
    pub position_id: u128,
    pub proof: TLBRef<TonCell>,
    pub index: U256,
}

/// The executable external method reads query ID and proof ref. Older SDKs also
/// supplied an inline index; it is not used to authenticate the claim.
#[derive(Debug, Clone, PartialEq)]
pub struct ToncoFarmingLegacyIndex(pub Option<U256>);

impl ton::ton_core::traits::tlb::TLB for ToncoFarmingLegacyIndex {
    fn read_definition(p: &mut CellParser) -> TonCoreResult<Self> {
        let v = if p.data_bits_left()? == 0 {
            None
        } else {
            Some(U256::read(p)?)
        };
        p.ensure_empty()?;
        Ok(Self(v))
    }
    fn write_definition(&self, b: &mut CellBuilder) -> TonCoreResult<()> {
        if let Some(v) = self.0 {
            v.write(b)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub enum ToncoFarmingOutgoingMessage {
    ClaimCallback(Box<ToncoFarmingClaimCallbackMsg>),
    Transfer(Box<ToncoNativeWalletTransferMsg>),
    Unknown(TonCell),
}

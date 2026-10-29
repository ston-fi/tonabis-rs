//! Wire records and dispatch for this family. Unknown cells survive; see the crate README for dispatch rules.
use crate::contracts::native::wallet::msgs::ToncoNativeWalletExcessesMsg;
use crate::contracts::v1::pool::msgs::ToncoPoolV1BurnMsg;
use fastnum::U256;
use ton::ton_core::TLB;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::TLBEither;
use ton::ton_core::types::tlb_core::adapters::ConstLen;
use ton::ton_core::types::tlb_core::adapters::NullableAddress;
use ton::ton_core::types::tlb_core::{TLBCoins, TLBRef};

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoPositionNftV1PositionInitOldFeeCell {
    pub fee_growth_inside0_last_x128: U256,
    pub fee_growth_inside1_last_x128: U256,
    pub nft_index: u64,
    pub jetton0_amount: TLBCoins,
    pub jetton1_amount: TLBCoins,
    #[tlb(bits_len = 24)]
    pub tick: i32,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xd5ecca2a, bits_len = 32)]
pub struct ToncoPositionNftV1InitMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub user_address: Option<TonAddress>,
    pub liquidity: u128,
    #[tlb(bits_len = 24)]
    pub tick_lower: i32,
    #[tlb(bits_len = 24)]
    pub tick_upper: i32,
    pub pool_seqno: u64,
    pub old_fee_cell: TLBRef<ToncoPositionNftV1PositionInitOldFeeCell>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoPositionNftV1PositionBurnOldFeeCell {
    pub fee_growth_inside0_last_x128: U256,
    pub fee_growth_inside1_last_x128: U256,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x46ca335a, bits_len = 32)]
pub struct ToncoPositionNftV1BurnMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub nft_owner: Option<TonAddress>,
    pub liquidity2_burn: u128,
    #[tlb(bits_len = 24)]
    pub tick_lower: i32,
    #[tlb(bits_len = 24)]
    pub tick_upper: i32,
    pub old_fee_cell: TLBRef<ToncoPositionNftV1PositionBurnOldFeeCell>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x5fcc3d14, bits_len = 32)]
pub struct ToncoPositionNftV1TransferMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub new_owner_address: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub response_destination: Option<TonAddress>,
    pub custom_payload: Option<TLBRef<TonCell>>,
    pub forward_ton_amount: TLBCoins,
    pub forward_payload: TLBEither<TonCell, TLBRef<TonCell>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x05138d91, bits_len = 32)]
pub struct ToncoPositionNftV1OwnershipAssignedMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub previous_owner_address: Option<TonAddress>,
    pub forward_payload: TLBEither<TonCell, TLBRef<TonCell>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub enum ToncoPositionNftV1IncomingMessage {
    PositionInit(Box<ToncoPositionNftV1InitMsg>),
    PositionBurn(Box<ToncoPositionNftV1BurnMsg>),
    Transfer(Box<ToncoPositionNftV1TransferMsg>),
    Unknown(TonCell),
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub enum ToncoPositionNftV1OutgoingMessage {
    OwnershipNotify(Box<ToncoPositionNftV1OwnershipNotifyMsg>),
    PoolBurn(Box<ToncoPoolV1BurnMsg>),
    OwnershipAssigned(Box<ToncoPositionNftV1OwnershipAssignedMsg>),
    Excesses(Box<ToncoNativeWalletExcessesMsg>),
    Unknown(TonCell),
}

/// Collection/indexer notification emitted after an ownership transfer.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xf58760ef, bits_len = 32)]
pub struct ToncoPositionNftV1OwnershipNotifyMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub new_owner: Option<TonAddress>,
    pub index: u64,
}

//! Wire records and dispatch for this family. Unknown cells survive; see the crate README for dispatch rules.
use crate::contracts::native::wallet::msgs::ToncoNativeWalletExcessesMsg;
use crate::contracts::v2::factory::storage::ToncoFactoryV2WhitelistEntry;
use crate::contracts::v2::router::msgs::ToncoRouterV2CreatePoolMsg;
use fastnum::U256;
use ton::ton_core::TLB;
use ton::ton_core::cell::CellBuilder;
use ton::ton_core::cell::CellParser;
use ton::ton_core::cell::TonCell;
use ton::ton_core::errors::TonCoreResult;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::TLBRef;
use ton::ton_core::types::tlb_core::adapters::ConstLen;
use ton::ton_core::types::tlb_core::adapters::NullableAddress;

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoFactoryV2Wallets {
    #[tlb(adapter = "NullableAddress")]
    pub token0: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub token1: Option<TonAddress>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoFactoryV2CreateSettings {
    pub fee: u16,
    #[tlb(bits_len = 24)]
    pub spacing: u32,
    pub active: bool,
    pub content: Option<TLBRef<TonCell>>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x9e9a8f7f, bits_len = 32)]
pub struct ToncoFactoryV2CreatePoolMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub minter0: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub minter1: Option<TonAddress>,
    #[tlb(bits_len = 160)]
    pub price: U256,
    pub settings: u16,
    pub wallets: TLBRef<ToncoFactoryV2Wallets>,
    pub extra: ToncoFactoryV2ExtraSettingsSuffix,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x8d7721f0, bits_len = 32)]
pub struct ToncoFactoryV2AddWhitelistMsg {
    pub query_id: u64,
    pub entry: ToncoFactoryV2WhitelistEntry,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x256eb11c, bits_len = 32)]
pub struct ToncoFactoryV2DeleteWhitelistMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub address: Option<TonAddress>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub enum ToncoFactoryV2IncomingMessage {
    ChangeAdmin(Box<ToncoFactoryV2ChangeAdminMsg>),
    Excesses(Box<ToncoNativeWalletExcessesMsg>),
    ChangeParams(Box<ToncoFactoryV2ChangeParamsMsg>),
    ResetGas(Box<ToncoFactoryV2ResetGasMsg>),
    ExecuteOrder(Box<ToncoFactoryV2ExecuteOrderMsg>),
    CreatePool(Box<crate::contracts::v2::factory::msgs::ToncoFactoryV2CreatePoolMsg>),
    AddWhitelist(Box<ToncoFactoryV2AddWhitelistMsg>),
    DeleteWhitelist(Box<ToncoFactoryV2DeleteWhitelistMsg>),
    Unknown(TonCell),
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xaa0db3d7, bits_len = 32)]
pub struct ToncoFactoryV2ChangeParamsMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub router: Option<TonAddress>,
    pub collection: TLBRef<TonCell>,
    pub item: TLBRef<TonCell>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x42a0fb43, bits_len = 32)]
pub struct ToncoFactoryV2ResetGasMsg {
    pub query_id: u64,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xd4b33b0a, bits_len = 32)]
pub struct ToncoFactoryV2ExecuteOrderMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub minter0: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub minter1: Option<TonAddress>,
    #[tlb(bits_len = 160)]
    pub price: U256,
    pub settings: u16,
    pub wallets: TLBRef<ToncoFactoryV2Wallets>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoFactoryV2Minters {
    #[tlb(adapter = "NullableAddress")]
    pub token0: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub token1: Option<TonAddress>,
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x8b85de63, bits_len = 32)]
pub struct ToncoFactoryV2InitOrderMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub router: Option<TonAddress>,
    #[tlb(bits_len = 160)]
    pub price: U256,
    pub settings: u16,
    pub minters: TLBRef<ToncoFactoryV2Minters>,
}

/// Executable factory admin transfer, absent from the SDK's opcode declarations.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x246189c4, bits_len = 32)]
pub struct ToncoFactoryV2ChangeAdminMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub admin: Option<TonAddress>,
}

/// Source selects extra settings by remaining refs. One SDK builder omits the
/// tag; another writes a Maybe bit that the executable does not inspect.
#[derive(Debug, Clone, PartialEq)]
pub struct ToncoFactoryV2ExtraSettingsSuffix {
    pub sdk_tag: Option<bool>,
    pub settings: Option<TLBRef<ToncoFactoryV2CreateSettings>>,
}

impl ton::ton_core::traits::tlb::TLB for ToncoFactoryV2ExtraSettingsSuffix {
    fn read_definition(p: &mut CellParser) -> TonCoreResult<Self> {
        let sdk_tag = if p.data_bits_left()? == 0 {
            None
        } else {
            Some(bool::read(p)?)
        };
        let settings = if p.ensure_empty().is_ok() {
            None
        } else {
            Some(TLBRef::<ToncoFactoryV2CreateSettings>::read(p)?)
        };
        p.ensure_empty()?;
        Ok(Self { sdk_tag, settings })
    }
    fn write_definition(&self, b: &mut CellBuilder) -> TonCoreResult<()> {
        if let Some(tag) = self.sdk_tag {
            tag.write(b)?;
        }
        if let Some(value) = &self.settings {
            value.write(b)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, TLB)]
pub enum ToncoFactoryV2OutgoingMessage {
    InitOrder(Box<ToncoFactoryV2InitOrderMsg>),
    CreatePool(Box<ToncoRouterV2CreatePoolMsg>),
    Unknown(TonCell),
}

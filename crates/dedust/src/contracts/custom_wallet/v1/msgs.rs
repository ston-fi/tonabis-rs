//! Fixed V1 wallet dispatch and delegated-signature records, reconciled from
//! code43d447... (entrypoints0/524287). Construction never signs or authorizes.
use crate::contracts::signature_ref::DedustSignatureRef;
use fastnum::{U256, U512};
use ton::{
    block_tlb::Msg,
    ton_core::{
        TLB,
        cell::{CellBuilder, CellParser, TonCell},
        errors::TonCoreError,
        traits::tlb::TLB as Tlb,
        types::tlb_core::{MsgAddress, TLBCoins, TLBRef},
    },
};

/// These complete signed records reject unmodeled fields to avoid silently
/// changing the caller's signature commitment during reserialization.
/// TON Connect cell-signature envelope. The wallet checks its known schema hash
/// and signature itself. Preserve the signed domain cell exactly; its byte graph
/// is signature-bound and is skipped by the captured V1 loader.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x75569022, bits_len = 32, ensure_empty = true)]
pub struct DedustX1000WalletV1DelegationEnvelopeMsg {
    pub schema_hash: u32,
    pub timestamp: u64,
    pub user_address: MsgAddress,
    pub app_domain_cell: TLBRef<TonCell>,
    pub payload: TLBRef<DedustX1000WalletV1DelegatedKey>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(ensure_empty = true)]
pub struct DedustX1000WalletV1DelegatedKey {
    pub valid_until: u64,
    pub public_key: U256,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(ensure_empty = true)]
pub struct DedustX1000WalletV1Authorization {
    pub signature: U512,
    pub message: TLBRef<DedustX1000WalletV1DelegationEnvelopeMsg>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(ensure_empty = true)]
pub struct DedustX1000WalletV1SignedCommandMsg {
    pub network_id: i32,
    pub sequence: u32,
    pub valid_until: u64,
    pub actions: DedustSignatureRef<DedustX1000WalletV1SendActions>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xcac1d607, bits_len = 32)]
pub struct DedustX1000WalletV1ExecuteMsg {
    pub authorization: TLBRef<DedustX1000WalletV1Authorization>,
    pub signature: U512,
    pub command: TLBRef<DedustX1000WalletV1SignedCommandMsg>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x5beb0c69, bits_len = 32)]
pub struct DedustX1000WalletV1RevokeKeyMsg {
    pub signature: U512,
    pub message: TLBRef<DedustX1000WalletV1DelegationEnvelopeMsg>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc5e0ac83, bits_len = 32)]
pub struct DedustX1000WalletV1PruneExpiredKeysMsg;

/// A source-restricted OutList: at most255 send-message actions, oldest first.
/// Every nonfinal action must set mode bit1, as checked by captured V1 code.
#[derive(Debug, Clone, PartialEq)]
pub struct DedustX1000WalletV1SendActions {
    pub actions: Vec<DedustX1000WalletV1SendAction>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct DedustX1000WalletV1SendAction {
    pub mode: u8,
    pub message: Msg,
}
impl Tlb for DedustX1000WalletV1SendActions {
    fn read_definition(p: &mut CellParser) -> Result<Self, TonCoreError> {
        let mut cell = p.read_remaining()?;
        let mut actions = Vec::new();
        actions
            .try_reserve_exact(255)
            .map_err(|_| TonCoreError::Custom("V1 action-list allocation failed".into()))?;
        loop {
            if cell.data_len_bits() == 0 {
                cell.parser().ensure_empty()?;
                break;
            }
            if actions.len() == 255 {
                return Err(TonCoreError::Custom("V1 action list exceeds255".into()));
            }
            let mut current = cell.parser();
            let previous = current.read_next_ref()?.clone();
            let opcode = current.read_num::<u32>(32)?;
            if opcode != 0x0ec3c86d {
                return Err(crate::contracts::utils::invalid_tag("V1 send action", u64::from(opcode)));
            }
            let mode = current.read_num::<u8>(8)?;
            if !actions.is_empty() && mode & 2 == 0 {
                return Err(TonCoreError::Custom("V1 nonfinal action requires mode bit1".into()));
            }
            let message = <Msg as Tlb>::from_cell(current.read_next_ref()?)?;
            current.ensure_empty()?;
            actions.push(DedustX1000WalletV1SendAction { mode, message });
            cell = previous;
        }
        actions.reverse();
        Ok(Self { actions })
    }
    fn write_definition(&self, b: &mut CellBuilder) -> Result<(), TonCoreError> {
        if self.actions.len() > 255 {
            return Err(TonCoreError::Custom("V1 action list exceeds255".into()));
        }
        let mut previous = TonCell::empty().clone();
        for (index, action) in self.actions.iter().enumerate() {
            if index + 1 < self.actions.len() && action.mode & 2 == 0 {
                return Err(TonCoreError::Custom("V1 nonfinal action requires mode bit1".into()));
            }
            let mut next = TonCell::builder();
            next.write_ref(previous)?;
            next.write_num(&0x0ec3c86du32, 32)?;
            next.write_num(&action.mode, 8)?;
            next.write_ref(action.message.to_cell()?)?;
            previous = next.build()?;
        }
        b.write_cell(&previous)
    }
}

#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x01f3835d, bits_len = 32)]
pub struct DedustX1000WalletV1Callback01f3835dMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    pub address: MsgAddress,
    pub payload: Option<TLBRef<DedustX1000WalletV1Callback476adc0cMsg>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x6edd65f0, bits_len = 32)]
pub struct DedustX1000WalletV1Callback6edd65f0Msg {
    pub query_id: u64,
    pub amount: TLBCoins,
    pub payload: Option<TLBRef<DedustX1000WalletV1Callback476adc0cMsg>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x474f86cf, bits_len = 32)]
pub struct DedustX1000WalletV1Callback474f86cfMsg {
    pub query_id: u64,
    pub payload: Option<TLBRef<DedustX1000WalletV1Callback476adc0cMsg>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee36, bits_len = 32)]
pub struct DedustX1000WalletV1CallbackC0ffee36Msg {
    pub query_id: u64,
    pub payload: TLBRef<DedustX1000WalletV1Callback476adc0cMsg>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xb39f717e, bits_len = 32)]
pub struct DedustX1000WalletV1CallbackB39f717eMsg {
    pub query_id: u64,
    pub payload: Option<TLBRef<DedustX1000WalletV1Callback476adc0cMsg>>,
}
/// Fixed payout continuation loaded with tag476adc0c, query64, two
/// message addresses and two uint16 shares; the V1 loader ends this cell.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x476adc0c, bits_len = 32)]
pub struct DedustX1000WalletV1Callback476adc0cMsg {
    pub query_id: u64,
    pub address0: MsgAddress,
    pub word0: u16,
    pub address1: MsgAddress,
    pub word1: u16,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DedustX1000WalletV1IncomingMsg {
    Execute(Box<DedustX1000WalletV1ExecuteMsg>),
    RevokeKey(Box<DedustX1000WalletV1RevokeKeyMsg>),
    PruneExpiredKeys(DedustX1000WalletV1PruneExpiredKeysMsg),
    Callback01f3835d(DedustX1000WalletV1Callback01f3835dMsg),
    Callback6edd65f0(DedustX1000WalletV1Callback6edd65f0Msg),
    Callback474f86cf(DedustX1000WalletV1Callback474f86cfMsg),
    CallbackC0ffee36(DedustX1000WalletV1CallbackC0ffee36Msg),
    CallbackB39f717e(DedustX1000WalletV1CallbackB39f717eMsg),
    Callback476adc0c(DedustX1000WalletV1Callback476adc0cMsg),
    Unknown(TonCell),
}
// Keep opcode dispatch: native derive can swallow a bad nested continuation
// prefix as a top-level mismatch and incorrectly return Unknown.
impl DedustX1000WalletV1IncomingMsg {
    pub fn from_cell(c: &TonCell) -> Result<Self, TonCoreError> {
        if c.data_len_bits() < 32 {
            return Ok(Self::Unknown(c.clone()));
        }
        Ok(match c.parser().read_num::<u32>(32)? {
            0xcac1d607 => Self::Execute(Box::new(Tlb::from_cell(c)?)),
            0x5beb0c69 => Self::RevokeKey(Box::new(Tlb::from_cell(c)?)),
            0xc5e0ac83 => Self::PruneExpiredKeys(Tlb::from_cell(c)?),
            0x01f3835d => Self::Callback01f3835d(Tlb::from_cell(c)?),
            0x6edd65f0 => Self::Callback6edd65f0(Tlb::from_cell(c)?),
            0x474f86cf => Self::Callback474f86cf(Tlb::from_cell(c)?),
            0xc0ffee36 => Self::CallbackC0ffee36(Tlb::from_cell(c)?),
            0xb39f717e => Self::CallbackB39f717e(Tlb::from_cell(c)?),
            0x476adc0c => Self::Callback476adc0c(Tlb::from_cell(c)?),
            _ => Self::Unknown(c.clone()),
        })
    }
    pub fn to_cell(&self) -> Result<TonCell, TonCoreError> {
        match self {
            Self::Execute(v) => v.to_cell(),
            Self::RevokeKey(v) => v.to_cell(),
            Self::PruneExpiredKeys(v) => v.to_cell(),
            Self::Callback01f3835d(v) => v.to_cell(),
            Self::Callback6edd65f0(v) => v.to_cell(),
            Self::Callback474f86cf(v) => v.to_cell(),
            Self::CallbackC0ffee36(v) => v.to_cell(),
            Self::CallbackB39f717e(v) => v.to_cell(),
            Self::Callback476adc0c(v) => v.to_cell(),
            Self::Unknown(v) => Ok(v.clone()),
        }
    }
}

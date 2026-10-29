//! Exact-arity structured getter results; integer units/widths follow the getter, not storage.
use fastnum::I512;
use ton::block_tlb::FromTVMStack;
use ton::block_tlb::TVMStack;
use ton::errors::TonResult;
use ton::ton_core::FromTVMStack;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::MsgAddress;
use tonabis_core::adapters::nullable_internal_address::NullableInternalAddress;

#[derive(Debug, Clone, PartialEq)]
pub struct ToncoRouterV2GetRouterStateResult {
    pub admin: Option<TonAddress>,
    pub pool_admin: Option<TonAddress>,
    pub factory: Option<TonAddress>,
    /// Outer None is TVM NULL; Some(None) is addr_none; Some(Some(value)) is internal.
    pub proxy: Option<Option<TonAddress>>,
    pub flags: I512,
    pub seqno: I512,
    pub version: I512,
}
impl FromTVMStack for ToncoRouterV2GetRouterStateResult {
    fn from_stack(stack: &mut TVMStack) -> TonResult<Self> {
        let version = <I512 as FromTVMStack>::from_stack(stack)?;
        let seqno = <I512 as FromTVMStack>::from_stack(stack)?;
        let flags = <I512 as FromTVMStack>::from_stack(stack)?;
        let proxy = Option::<MsgAddress>::from_stack(stack)?
            .map(NullableInternalAddress::from_address)
            .transpose()?;
        let factory = NullableInternalAddress.read(&mut TonCell::from_stack(stack)?.parser())?;
        let pool_admin = NullableInternalAddress.read(&mut TonCell::from_stack(stack)?.parser())?;
        let admin = NullableInternalAddress.read(&mut TonCell::from_stack(stack)?.parser())?;
        stack.ensure_empty()?;
        Ok(Self {
            admin,
            pool_admin,
            factory,
            proxy,
            flags,
            seqno,
            version,
        })
    }
}

#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct ToncoRouterV2GetChildContractsResult {
    pub pool: TonCell,
    pub account: TonCell,
    pub position: TonCell,
    pub prototype: TonCell,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ToncoRouterV2GetPendingTimelocksResult {
    pub delay: I512,
    pub admin_time: I512,
    /// Outer None is TVM NULL; Some(None) is addr_none; Some(Some(value)) is internal.
    pub admin: Option<Option<TonAddress>>,
    pub proxy_time: I512,
    /// Outer None is TVM NULL; Some(None) is addr_none; Some(Some(value)) is internal.
    pub proxy: Option<Option<TonAddress>>,
    pub flags_time: I512,
    pub flags: I512,
    pub code_time: I512,
    pub code: Option<TonCell>,
    pub pool_code_time: I512,
    pub pool_code: Option<TonCell>,
}
impl FromTVMStack for ToncoRouterV2GetPendingTimelocksResult {
    fn from_stack(stack: &mut TVMStack) -> TonResult<Self> {
        let pool_code = <Option<TonCell> as FromTVMStack>::from_stack(stack)?;
        let pool_code_time = <I512 as FromTVMStack>::from_stack(stack)?;
        let code = <Option<TonCell> as FromTVMStack>::from_stack(stack)?;
        let code_time = <I512 as FromTVMStack>::from_stack(stack)?;
        let flags = <I512 as FromTVMStack>::from_stack(stack)?;
        let flags_time = <I512 as FromTVMStack>::from_stack(stack)?;
        let proxy = Option::<MsgAddress>::from_stack(stack)?
            .map(NullableInternalAddress::from_address)
            .transpose()?;
        let proxy_time = <I512 as FromTVMStack>::from_stack(stack)?;
        let admin = Option::<MsgAddress>::from_stack(stack)?
            .map(NullableInternalAddress::from_address)
            .transpose()?;
        let admin_time = <I512 as FromTVMStack>::from_stack(stack)?;
        let delay = <I512 as FromTVMStack>::from_stack(stack)?;
        stack.ensure_empty()?;
        Ok(Self {
            delay,
            admin_time,
            admin,
            proxy_time,
            proxy,
            flags_time,
            flags,
            code_time,
            code,
            pool_code_time,
            pool_code,
        })
    }
}

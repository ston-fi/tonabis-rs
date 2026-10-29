//! Structured getter results with exact TVM stack arity.
use fastnum::I512;
use ton::block_tlb::FromTVMStack;
use ton::block_tlb::TVMStack;
use ton::errors::TonError;
use ton::errors::TonResult;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::MsgAddress;
use tonabis_core::adapters::nullable_internal_address::NullableInternalAddress;
#[derive(Debug, Clone, PartialEq)]
pub struct MoonMempadCurveJettonDataResult {
    pub total_supply: I512,
    pub mintable: bool,
    /// Outer None is TVM NULL; Some(None) is addr_none; Some(Some(value)) is internal.
    pub admin_address: Option<Option<TonAddress>>,
    pub content: TonCell,
    pub wallet_code: TonCell,
}
impl FromTVMStack for MoonMempadCurveJettonDataResult {
    fn from_stack(stack: &mut TVMStack) -> TonResult<Self> {
        let wallet_code = <TonCell as FromTVMStack>::from_stack(stack)?;
        let content = <TonCell as FromTVMStack>::from_stack(stack)?;
        let admin_address = Option::<MsgAddress>::from_stack(stack)?
            .map(NullableInternalAddress::from_address)
            .transpose()?;
        let mintable = <bool as FromTVMStack>::from_stack(stack)?;
        let total_supply = <I512 as FromTVMStack>::from_stack(stack)?;
        stack.ensure_empty()?;
        Ok(Self {
            total_supply,
            mintable,
            admin_address,
            content,
            wallet_code,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MoonMempadCurveNullResult;
impl FromTVMStack for MoonMempadCurveNullResult {
    fn from_stack(stack: &mut TVMStack) -> TonResult<Self> {
        let value = stack.pop_checked()?;
        if value.as_null().is_none() {
            return Err(TonError::UnexpectedValue {
                expected: "one null stack value".into(),
                actual: format!("{value:?}"),
            });
        }
        stack.ensure_empty()?;
        Ok(Self)
    }
}

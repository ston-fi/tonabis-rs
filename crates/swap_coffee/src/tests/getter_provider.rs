//! Test-only provider that verifies Coffee's requests without network or state lookup.

use fastnum::U256;
use std::future::Future;
use ton::errors::TonError;
use ton::errors::TonResult;
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::tlb_core::MsgAddressInt;

use std::pin::Pin;

use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use std::time::Duration;

use ton::block_tlb::TVMStack;
use ton::block_tlb::TVMStackValue;

use ton::contracts::ContractClient;

use ton::ton_core::errors::TonCoreError;
use ton::ton_core::errors::TonCoreResult;

use ton::ton_core::traits::emulation_provider::EmulationProvider;
use ton::ton_core::traits::emulation_provider::EmulatorContractState;
use ton::ton_core::traits::emulation_provider::EmulatorGetMethodRequest;
use ton::ton_core::traits::emulation_provider::EmulatorGetMethodSuccess;

use ton::ton_core::traits::state_provider::ContractState;
use ton::ton_core::traits::state_provider::StateProvider;

use ton::ton_core::traits::tlb::TLB;

use ton::ton_core::types::TonAddress;
use ton::ton_core::types::TxLTHash;

type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = TonCoreResult<T>> + Send + 'a>>;
struct NoState;
impl StateProvider for NoState {
    fn last_mc_seqno<'a, 'b>(&'a self) -> BoxFuture<'b, u32>
    where
        'a: 'b,
        Self: 'b,
    {
        Box::pin(async { Err(TonCoreError::Custom("unexpected state lookup".into())) })
    }
    fn load_state<'a, 'b>(&'a self, _: TonAddress, _: Option<TxLTHash>) -> BoxFuture<'b, ContractState>
    where
        'a: 'b,
        Self: 'b,
    {
        Box::pin(async { Err(TonCoreError::Custom("unexpected state lookup".into())) })
    }
    fn load_latest_tx_per_address<'a, 'b>(&'a self, _: u32) -> BoxFuture<'b, Vec<(TonAddress, TxLTHash)>>
    where
        'a: 'b,
        Self: 'b,
    {
        Box::pin(async { Err(TonCoreError::Custom("unexpected state lookup".into())) })
    }
}
struct Replay {
    method_id: i32,
    input: TVMStack,
    output: TVMStack,
    failure: bool,
    calls: Arc<AtomicUsize>,
}
impl EmulationProvider for Replay {
    fn emulate_get_method<'a, 'b>(
        &'a self,
        request: EmulatorGetMethodRequest,
        timeout: Option<Duration>,
    ) -> BoxFuture<'b, EmulatorGetMethodSuccess>
    where
        'a: 'b,
        Self: 'b,
    {
        Box::pin(async move {
            self.calls.fetch_add(1, Ordering::SeqCst);
            assert_eq!(request.method_id, self.method_id);
            assert_eq!(TVMStack::from_boc(request.stack_boc)?, self.input);
            assert!(timeout.is_some());
            assert!(matches!(
                request.contract_state,
                EmulatorContractState::Address {
                    address: TonAddress::ZERO,
                    tx_id: None
                }
            ));
            if self.failure {
                return Err(TonCoreError::data("coffee-test", "provider failed"));
            }
            Ok(EmulatorGetMethodSuccess::new(0, self.output.to_boc()?))
        })
    }
}
pub(super) fn client(
    method_id: i32,
    input: TVMStack,
    output: TVMStack,
) -> anyhow::Result<(ContractClient, Arc<AtomicUsize>)> {
    let calls = Arc::new(AtomicUsize::new(0));
    Ok((
        ContractClient::builder(
            NoState,
            Replay {
                method_id,
                input,
                output,
                failure: false,
                calls: calls.clone(),
            },
        )?
        .build()?,
        calls,
    ))
}
pub(super) fn address_cell(value: u8) -> anyhow::Result<TonCell> {
    let mut b = TonCell::builder();
    b.write_num(&4u8, 3)?;
    b.write_num(&-1i8, 8)?;
    b.write_num(&U256::from(value), 256)?;
    Ok(b.build()?)
}
pub(super) fn opaque_cell() -> anyhow::Result<TonCell> {
    let mut b = TonCell::builder();
    b.write_num(&5u8, 3)?;
    Ok(b.build()?)
}
pub(super) fn extra_stack(stack: &TVMStack) -> TVMStack {
    let mut extra = TVMStack::default();
    extra.push_tiny_int(777);
    extra.extend(stack.iter().cloned());
    extra
}
pub(super) fn wrong_type(value: &TVMStackValue) -> anyhow::Result<TVMStackValue> {
    let mut stack = TVMStack::default();
    if matches!(value, TVMStackValue::Int(_) | TVMStackValue::TinyInt(_)) {
        stack.push_cell(opaque_cell()?);
    } else {
        stack.push_tiny_int(1);
    }
    Ok(stack.pop_checked()?)
}

pub(super) fn none_address_cell() -> anyhow::Result<TonCell> {
    let mut b = TonCell::builder();
    b.write_num(&0u8, 2)?;
    Ok(b.build()?)
}
pub(super) fn nullable_address(stack: &mut TVMStack) -> TonResult<Option<TonAddress>> {
    use ton::block_tlb::FromTVMStack;
    use ton::ton_core::types::tlb_core::{MsgAddress, MsgAddressExt};
    match MsgAddress::from_stack(stack)? {
        MsgAddress::Int(MsgAddressInt::Std(address)) if address.anycast.is_none() => {
            Ok(Some(TonAddress::new(i32::from(address.workchain), address.address)))
        },
        MsgAddress::Ext(MsgAddressExt::None(_)) => Ok(None),
        value => Err(TonError::UnexpectedValue {
            expected: "nullable internal address".into(),
            actual: format!("{value:?}"),
        }),
    }
}

pub(super) fn failure_client() -> anyhow::Result<(ContractClient, Arc<AtomicUsize>)> {
    let calls = Arc::new(AtomicUsize::new(0));
    let client = ContractClient::builder(
        NoState,
        Replay {
            method_id: 113142,
            input: TVMStack::default(),
            output: TVMStack::default(),
            failure: true,
            calls: calls.clone(),
        },
    )?
    .build()?;
    Ok((client, calls))
}

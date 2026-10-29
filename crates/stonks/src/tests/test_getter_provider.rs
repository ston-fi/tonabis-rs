//! Recording provider for STONks getter boundary tests.
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
    time::Duration,
};
use ton::ton_core::cell::TonCell;
use ton::ton_core::types::tlb_core::MsgAddress;

use ton::__async_trait as async_trait;
use ton::{
    block_tlb::{TVMInt, TVMStack, TVMStackValue},
    contracts::ContractClient,
    ton_core::{
        errors::{TonCoreError, TonCoreResult},
        traits::{
            emulation_provider::{EmulationProvider, EmulatorGetMethodRequest, EmulatorGetMethodSuccess},
            state_provider::{ContractState, StateProvider},
            tlb::TLB,
        },
        types::{TonAddress, TxLTHash},
    },
};

type RecordedCall = (i32, TVMStack, Option<TVMStack>);

#[derive(Clone)]
struct RecordingProvider(Arc<Mutex<VecDeque<RecordedCall>>>);
#[async_trait]
impl EmulationProvider for RecordingProvider {
    async fn emulate_get_method(
        &self,
        request: EmulatorGetMethodRequest,
        _timeout: Option<Duration>,
    ) -> TonCoreResult<EmulatorGetMethodSuccess> {
        let (selector, mut expected, response) = self
            .0
            .lock()
            .map_err(|_| TonCoreError::Custom("recording lock".into()))?
            .pop_front()
            .ok_or_else(|| TonCoreError::Custom("unexpected getter call".into()))?;
        assert_eq!(request.method_id, selector);
        let mut actual = TVMStack::from_boc(request.stack_boc)?;
        // Int and TinyInt encode the same TVM number. Native bool packing uses
        // TinyInt; keep values/order and Cell-versus-Slice arguments exact.
        for stack in [&mut actual, &mut expected] {
            for value in stack.iter_mut() {
                if let TVMStackValue::TinyInt(number) = value {
                    *value = TVMInt {
                        value: number.value.into(),
                    }
                    .into();
                }
            }
        }
        assert_eq!(actual.to_boc()?, expected.to_boc()?);
        let response = response.ok_or_else(|| TonCoreError::Custom("stonks provider sentinel".into()))?;
        Ok(EmulatorGetMethodSuccess::new(0, response.to_boc()?))
    }
}
#[async_trait]
impl StateProvider for RecordingProvider {
    async fn last_mc_seqno(&self) -> TonCoreResult<u32> {
        Err(TonCoreError::Custom("unexpected state query".into()))
    }
    async fn load_state(&self, _address: TonAddress, _tx: Option<TxLTHash>) -> TonCoreResult<ContractState> {
        Err(TonCoreError::Custom("unexpected state load".into()))
    }
    async fn load_latest_tx_per_address(&self, _seqno: u32) -> TonCoreResult<Vec<(TonAddress, TxLTHash)>> {
        Err(TonCoreError::Custom("unexpected state refresh".into()))
    }
}

pub(super) fn recording_client(
    selector: i32,
    arguments: TVMStack,
    reply: Option<TVMStack>,
) -> anyhow::Result<ContractClient> {
    let provider = RecordingProvider(Arc::new(Mutex::new(VecDeque::from([(selector, arguments, reply)]))));
    Ok(ContractClient::builder(provider.clone(), provider)?.build()?)
}

pub(super) fn fixture_stack(id: usize) -> anyhow::Result<TVMStack> {
    let rows: Vec<serde_json::Value> = serde_json::from_str(include_str!("fixtures/getter-stacks.json"))?;
    let mut stack = TVMStack::EMPTY;
    for value in rows[id]["stack"].as_array().ok_or_else(|| anyhow::anyhow!("stack"))? {
        let data = value["value"].as_str().ok_or_else(|| anyhow::anyhow!("value"))?;
        match value["type"].as_str() {
            Some("int") => stack.push_tiny_int(data.parse()?),
            Some("slice") => stack.push_cell_slice(TonCell::from_boc_hex(data)?),
            Some("cell") => stack.push_cell(TonCell::from_boc_hex(data)?),
            _ => anyhow::bail!("fixture stack kind"),
        };
    }
    Ok(stack)
}
pub(super) fn bonding_stack(v12: bool) -> anyhow::Result<TVMStack> {
    let mut stack = TVMStack::EMPTY;
    for n in [1, 2000, 1000, 10000, 20, 100, 30, 1] {
        stack.push_tiny_int(n);
    }
    if v12 {
        for n in [100, 140, 5000] {
            stack.push_tiny_int(n);
        }
        let zero = MsgAddress::Int(TonAddress::ZERO.to_msg_address_int());
        stack.push_cell_slice(zero.to_cell()?);
    }
    Ok(stack)
}

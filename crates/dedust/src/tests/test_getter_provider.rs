//! Shared recording provider for public getter boundary tests.
use fastnum::I512;
use fastnum::U256;
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex, OnceLock},
    time::Duration,
};

use ton::__async_trait as async_trait;
use ton::{
    block_tlb::{TVMInt, TVMNull, TVMStack, TVMStackValue},
    contracts::ContractClient,
    ton_core::{
        cell::TonCell,
        errors::{TonCoreError, TonCoreResult},
        traits::{
            emulation_provider::{EmulationProvider, EmulatorGetMethodRequest, EmulatorGetMethodSuccess},
            state_provider::{ContractState, StateProvider},
            tlb::TLB,
        },
        types::{TonAddress, TxLTHash},
    },
};

pub(super) fn fixture_values(contract: &str, selector: u32) -> anyhow::Result<&'static [serde_json::Value]> {
    static FIXTURE: OnceLock<Result<serde_json::Value, serde_json::Error>> = OnceLock::new();
    let fixture = FIXTURE
        .get_or_init(|| serde_json::from_str(include_str!("fixtures/getter-stack-vectors.json")))
        .as_ref()
        .map_err(|e| anyhow::anyhow!("{e}"))?;
    fixture["cases"]
        .as_array()
        .and_then(|cases| {
            cases
                .iter()
                .find(|case| case["contract"] == contract && case["selector"] == selector)
        })
        .and_then(|case| case["values"].as_array())
        .map(Vec::as_slice)
        .ok_or_else(|| anyhow::anyhow!("missing getter fixture {contract}/{selector}"))
}

#[derive(Clone)]
pub(super) struct RecordingProvider(pub(super) Arc<Mutex<VecDeque<(i32, TVMStack, TVMStack)>>>);
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

pub(super) fn recording_client(selector: i32, arguments: TVMStack, reply: TVMStack) -> anyhow::Result<ContractClient> {
    let provider = RecordingProvider(Arc::new(Mutex::new(VecDeque::from([(selector, arguments, reply)]))));
    Ok(ContractClient::builder(provider.clone(), provider)?.build()?)
}

pub(super) fn cell(v: &serde_json::Value) -> anyhow::Result<TonCell> {
    Ok(TonCell::from_boc_hex(
        v["boc_hex"].as_str().ok_or_else(|| anyhow::anyhow!("cell"))?,
    )?)
}
pub(super) fn integer(v: &serde_json::Value) -> anyhow::Result<I512> {
    Ok(I512::from_str(v["value"].as_str().ok_or_else(|| anyhow::anyhow!("integer"))?)?)
}
pub(super) fn fixture_stack(values: &[serde_json::Value]) -> anyhow::Result<TVMStack> {
    let mut stack = TVMStack::EMPTY;
    for v in values {
        match v["kind"].as_str() {
            Some("null") => stack.push(TVMStackValue::Null(TVMNull)),
            Some("int") => stack.push_int(integer(v)?),
            Some("cell") => stack.push_cell(cell(v)?),
            Some("slice") => stack.push_cell_slice(cell(v)?),
            _ => anyhow::bail!("unknown fixture kind"),
        };
    }
    Ok(stack)
}
/// Independent representative DedustClassicAsset/address arguments; no production serializer
/// computes the expected input stack.
pub(super) fn fixture_arguments(values: &[&str]) -> anyhow::Result<TVMStack> {
    let mut stack = TVMStack::EMPTY;
    for v in values {
        match *v {
            "native" => {
                let mut b = TonCell::builder();
                b.write_num(&0u8, 4)?;
                stack.push_cell_slice(b.build()?);
            },
            "jetton" => {
                let mut b = TonCell::builder();
                b.write_num(&1u8, 4)?;
                b.write_num(&0i8, 8)?;
                b.write_num(&U256::from_u64(12), 256)?;
                stack.push_cell_slice(b.build()?);
            },
            "owner" => {
                let mut b = TonCell::builder();
                b.write_num(&4u8, 3)?;
                b.write_num(&0i8, 8)?;
                b.write_num(&U256::ZERO, 256)?;
                stack.push_cell_slice(b.build()?);
            },
            n => {
                stack.push_int(I512::from_str(n)?);
            },
        }
    }
    Ok(stack)
}

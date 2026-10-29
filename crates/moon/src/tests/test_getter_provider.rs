//! Shared recording provider for public getter boundary tests.
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
    time::Duration,
};

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

//! Recording provider and loader for independent structural fixtures.
use crate::contracts::native::wallet::ToncoNativeWalletV1Contract;
use anyhow::{Context, bail};
use fastnum::I512;
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
    time::Duration,
};
use ton::{
    block_tlb::{TVMNull, TVMTuple},
    ton_core::cell::TonCell,
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

type ExpectedCall = (i32, TVMStack, Option<TVMStack>);

#[derive(Clone)]
pub(super) struct RecordingProvider(pub(super) Arc<Mutex<VecDeque<ExpectedCall>>>);
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
        Ok(EmulatorGetMethodSuccess::new(
            0,
            response
                .ok_or_else(|| TonCoreError::data("tonco-test-provider", "injected failure"))?
                .to_boc()?,
        ))
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
    reply: TVMStack,
) -> anyhow::Result<(ContractClient, RecordingProvider)> {
    let provider = RecordingProvider(Arc::new(Mutex::new(VecDeque::from([(selector, arguments, Some(reply))]))));
    Ok((ContractClient::builder(provider.clone(), provider.clone())?.build()?, provider))
}

pub(super) fn stack(values: &serde_json::Value) -> anyhow::Result<TVMStack> {
    let mut out = TVMStack::default();
    for v in values.as_array().context("stack array")? {
        if v.is_null() {
            out.push(TVMStackValue::Null(TVMNull));
        } else if v.is_array() {
            let mut t = TVMTuple::default();
            for item in stack(v)?.iter() {
                t.push(item.clone());
            }
            out.push_tuple(t);
        } else if let Some(n) = v.as_str() {
            out.push_int(I512::from_str(n)?);
        } else {
            let c = TonCell::from_boc_hex(v["boc"].as_str().context("cell BOC")?)?;
            match v["kind"].as_str() {
                Some("C") => out.push_cell(c),
                Some("CS") => out.push_cell_slice(c),
                _ => bail!("unknown cell type"),
            }
        }
    }
    Ok(out)
}

#[test]
fn test_provider_failure_is_typed_and_getter_does_not_retry() -> anyhow::Result<()> {
    use ton::{contracts::TonContract, errors::TonError};
    let provider = RecordingProvider(Arc::new(Mutex::new(VecDeque::from([(97026, TVMStack::default(), None)]))));
    let client = ContractClient::builder(provider.clone(), provider.clone())?.build()?;
    let address = TonAddress::ZERO;
    let contract = ToncoNativeWalletV1Contract::new(&client, &address, None);
    assert!(provider.0.lock().map_err(|_| anyhow::anyhow!("recording lock"))?.len() == 1);
    let result = tokio_test::block_on(contract.get_wallet_data());
    match result {
        Err(TonError::TLCoreError(TonCoreError::DataError { producer, msg })) => {
            assert_eq!(producer, "tonco-test-provider");
            assert_eq!(msg, "injected failure");
        },
        other => anyhow::bail!("provider failure was changed: {other:?}"),
    }
    assert!(provider.0.lock().map_err(|_| anyhow::anyhow!("recording lock"))?.is_empty());
    Ok(())
}

impl RecordingProvider {
    pub(super) fn ensure_consumed(&self) -> anyhow::Result<()> {
        anyhow::ensure!(
            self.0.lock().map_err(|_| anyhow::anyhow!("recording lock"))?.is_empty(),
            "getter did not issue its expected request"
        );
        Ok(())
    }
}

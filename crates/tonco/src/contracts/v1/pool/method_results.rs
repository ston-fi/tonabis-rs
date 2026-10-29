//! Exact-arity structured getter results; integer units/widths follow the getter, not storage.
use ton::block_tlb::FromTVMStack;
use ton::block_tlb::{TVMStack, TVMStackValue};
use ton::errors::TonError;
use ton::ton_core::types::TonAddress;
use tonabis_core::adapters::nullable_internal_address::NullableInternalAddress;

use fastnum::I512;
use ton::errors::TonResult;
use ton::ton_core::FromTVMStack;
use ton::ton_core::cell::TonCell;

#[derive(Debug, Clone, PartialEq)]
pub struct ToncoPoolTickRecord {
    pub index: i32,
    pub liquidity_gross: I512,
    pub liquidity_net: I512,
    pub outer_fees: Option<(I512, I512)>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ToncoPoolTickPage {
    pub entries: Vec<ToncoPoolTickRecord>,
}

impl FromTVMStack for ToncoPoolTickPage {
    fn from_stack(stack: &mut TVMStack) -> TonResult<Self> {
        let outer = stack.pop_checked()?;
        stack.ensure_empty()?;
        let groups = match outer {
            TVMStackValue::Null(_) => return Ok(Self { entries: vec![] }),
            TVMStackValue::Tuple(t) => t,
            v => return Err(wrong(&v)),
        };
        let mut entries = Vec::new();
        for group in groups.iter() {
            let TVMStackValue::Tuple(ticks) = group else {
                return Err(wrong(group));
            };
            for tick in ticks.iter() {
                let TVMStackValue::Tuple(fields) = tick else {
                    return Err(wrong(tick));
                };
                if fields.len() != 3 && fields.len() != 5 {
                    return Err(TonError::UnexpectedValue {
                        expected: "3 or 5 tick fields".into(),
                        actual: fields.len().to_string(),
                    });
                }
                let mut fields = TVMStack::new(fields.to_vec());
                let outer_fees = if fields.len() == 5 {
                    let f1 = I512::from_stack(&mut fields)?;
                    let f0 = I512::from_stack(&mut fields)?;
                    Some((f0, f1))
                } else {
                    None
                };
                let liquidity_net = I512::from_stack(&mut fields)?;
                let liquidity_gross = I512::from_stack(&mut fields)?;
                let index = i32::from_stack(&mut fields)?;
                fields.ensure_empty()?;
                if !(-887272..=887272).contains(&index) {
                    return Err(TonError::UnexpectedValue {
                        expected: "TONCO tick domain".into(),
                        actual: index.to_string(),
                    });
                }
                entries.push(ToncoPoolTickRecord {
                    index,
                    liquidity_gross,
                    liquidity_net,
                    outer_fees,
                });
            }
        }
        Ok(Self { entries })
    }
}

#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct ToncoPoolV1GetPoolStateAndConfigurationResult {
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub router_address: Option<TonAddress>,
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub admin_address: Option<TonAddress>,
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub controller_address: Option<TonAddress>,
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub jetton_wallet0: Option<TonAddress>,
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub jetton_wallet1: Option<TonAddress>,
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub jetton_minter0: Option<TonAddress>,
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub jetton_minter1: Option<TonAddress>,
    pub is_active: bool,
    pub tick_spacing: I512,
    pub base_fee: I512,
    pub protocol_fee: I512,
    pub active_fee: I512,
    pub current_tick: I512,
    pub current_price: I512,
    pub current_liquidity: I512,
    pub fee_grows_global0: I512,
    pub fee_grows_global1: I512,
    pub collected_token0_protocol_fee: I512,
    pub collected_token1_protocol_fee: I512,
    pub minted_nft_positions: I512,
    pub reserve0: I512,
    pub reserve1: I512,
    pub active_nft_positions: I512,
    pub occupied_ticks: I512,
    pub operations_since_deploy: I512,
}

#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct ToncoPoolV1GetChildContractsResult {
    pub account_code: TonCell,
    pub position_nft_code: TonCell,
    pub nft_collection_metadata: TonCell,
    pub nft_item_metadata: TonCell,
}

#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct ToncoPoolV1GetCollectedFeesResult {
    pub amount0: I512,
    pub amount1: I512,
}

#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct ToncoPoolV1GetMintEstimateResult {
    pub amount0: I512,
    pub amount1: I512,
    pub mint_errors: I512,
}

#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct ToncoPoolV1GetSwapEstimateResult {
    pub amount0: I512,
    pub amount1: I512,
}

#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct ToncoPoolV1GetFeeGrowthInsideResult {
    pub fee_growth_inside0_x128: I512,
    pub fee_growth_inside1_x128: I512,
}

#[derive(Debug, Clone, PartialEq, FromTVMStack)]
#[from_tvm_stack(ensure_empty = true)]
pub struct ToncoPoolV1GetCollectionDataResult {
    pub next_item_index: I512,
    pub collection_content: TonCell,
    #[from_tvm_stack(adapter = NullableInternalAddress)]
    pub owner_address: Option<TonAddress>,
}

fn wrong(actual: &TVMStackValue) -> TonError {
    TonError::UnexpectedValue {
        expected: "nested TONCO tick tuple".into(),
        actual: format!("{actual:?}"),
    }
}

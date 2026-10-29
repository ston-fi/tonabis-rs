use crate::contracts::staking_item::method_results::CoffeeStakingItemGetNftDataResult;
use crate::contracts::staking_item::storage::CoffeeStakingItemInitialData;
use crate::contracts::staking_master::storage::CoffeeStakingMasterInitialData;
use fastnum::U256;
use ton::block_tlb::FromTVMStack;
use ton::block_tlb::TVMNull;
use ton::block_tlb::TVMStack;
use ton::ton_core::cell::TonCell;
use ton::ton_core::cell::TonHash;
use ton::ton_core::traits::tlb::TLB;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::TLBRef;
fn address(id: u16) -> anyhow::Result<TonAddress> {
    let mut bytes = [0; 32];
    bytes[30..].copy_from_slice(&id.to_be_bytes());
    Ok(TonAddress::new(0, TonHash::from_slice(&bytes)?))
}
fn none() -> anyhow::Result<TonCell> {
    let mut b = TonCell::builder();
    b.write_num(&0u8, 2)?;
    Ok(b.build()?)
}
#[test]
fn test_initial_data_hashes_and_nullable_getter_records() -> anyhow::Result<()> {
    let data: serde_json::Value = serde_json::from_str(include_str!("fixtures/staking-initial-getters.json"))?;
    let master_address = address(300)?;
    let vault = address(302)?;
    let initial = CoffeeStakingItemInitialData {
        nft_id: 0,
        master: master_address,
    };
    assert_eq!(
        format!("{}", initial.to_cell()?.hash()?).to_lowercase(),
        data["nft_data_hash_hex"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("missing NFT hash"))?
    );
    let initial = CoffeeStakingMasterInitialData {
        vault,
        item_code: TLBRef::new(TonCell::empty().clone()),
        alternate_item_code: TLBRef::new(TonCell::empty().clone()),
    };
    assert_eq!(
        format!("{}", initial.to_cell()?.hash()?).to_lowercase(),
        data["master_data_hash_hex"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("missing master hash"))?
    );
    assert_eq!(CoffeeStakingMasterInitialData::from_cell(&initial.to_cell()?)?, initial);
    let mut state = TonCell::builder();
    state.write_num(&0u8, 4)?;
    state.write_num(&0u64, 64)?;
    let state = state.build()?;
    let mut output = TVMStack::default();
    output.push_tiny_int(0);
    output.push_cell_slice(none()?);
    output.push_cell_slice(none()?);
    output.push_cell_slice(vault.to_cell()?);
    output.push_cell(state);
    for _ in 0..3 {
        output.push(TVMNull {}.into());
    }
    let reply = crate::contracts::staking_master::method_results::CoffeeStakingMasterGetStoredDataResult::from_stack(
        &mut output,
    )?;
    assert!(!reply.initialized);
    assert!(reply.owner.is_none());
    assert!(reply.distributor.is_none());
    assert!(reply.rewards.is_empty());
    // Initial NFT asset uses absent wallet + zero Coins, not an internal zero.
    let mut asset = TonCell::builder();
    asset.write_num(&0u8, 6)?;
    let asset = asset.build()?;
    let mut info = TonCell::builder();
    info.write_num(&U256::ZERO, 168)?;
    let info = info.build()?;
    let mut output = TVMStack::default();
    output.push_tiny_int(0);
    output.push_tiny_int(0);
    output.push_cell_slice(master_address.to_cell()?);
    output.push_cell_slice(none()?);
    output.push_cell(asset);
    output.push_cell(info);
    output.push(TVMNull {}.into());
    let reply =
        crate::contracts::staking_item::method_results::CoffeeStakingItemGetStoredDataResult::from_stack(&mut output)?;
    assert!(reply.owner.is_none());
    assert!(reply.asset.wallet.is_none());
    assert_eq!(reply.asset.amount.to_u128(), 0);
    for owner in [none()?, address(0)?.to_cell()?] {
        let present = owner.data_len_bits() == 267;
        let mut output = TVMStack::default();
        output.push_tiny_int(0);
        output.push_tiny_int(0);
        output.push_cell_slice(master_address.to_cell()?);
        output.push_cell_slice(owner);
        output.push_cell(TonCell::empty().clone());
        let reply = CoffeeStakingItemGetNftDataResult::from_stack(&mut output)?;
        assert_eq!(reply.owner_address.is_some(), present);
    }
    Ok(())
}

#[test]
fn test_initial_phase_decoders_reject_initialized_owned_and_nonzero_master_state() -> anyhow::Result<()> {
    use crate::contracts::staking::CoffeeStakingPeriod;
    use crate::contracts::staking_master::single_item::storage::{
        CoffeeStakingMasterSingleItemInitialData, CoffeeStakingMasterSingleItemStorage,
    };
    use crate::contracts::staking_master::storage::CoffeeStakingMasterInitialData;
    use crate::contracts::staking_master::storage::{CoffeeStakingMasterState, CoffeeStakingMasterStateRefs};
    use ton::ton_core::types::tlb_core::TLBCoins;
    // Runtime storage is an independent way to encode data that violates only
    // one initial-phase constraint. Presence of internal zero is still ownership.
    for invalid in 0..7 {
        let mut state = CoffeeStakingMasterStateRefs {
            state: TLBRef::new(CoffeeStakingMasterState {
                total_points: TLBCoins::new(if invalid == 3 { 1 } else { 0 }),
                next_item_index: u64::from(invalid == 4),
            }),
            rewards: Default::default(),
            jettons: Default::default(),
            periods: Default::default(),
        };
        if invalid == 5 {
            state.periods.insert(
                7,
                TLBRef::new(CoffeeStakingPeriod {
                    duration: 1,
                    percentage: 0,
                }),
            );
        }
        let storage = CoffeeStakingMasterSingleItemStorage {
            initialized: invalid == 0,
            owner: if invalid == 1 { Some(address(0)?) } else { None },
            distributor: if invalid == 2 { Some(address(0)?) } else { None },
            vault: address(302)?,
            state_refs: TLBRef::new(state),
            item_code: TLBRef::new(TonCell::empty().clone()),
        };
        let single = storage.to_cell()?;
        // Case 6 is valid; append exactly the third code reference for the
        // catalog master. The two decoders never infer another generation.
        let mut catalog = TonCell::builder();
        catalog.write_cell(&single)?;
        catalog.write_ref(TonCell::empty().clone())?;
        assert_eq!(
            CoffeeStakingMasterSingleItemInitialData::from_cell(&single).is_ok(),
            invalid == 6
        );
        assert_eq!(
            CoffeeStakingMasterInitialData::from_cell(&catalog.build()?).is_ok(),
            invalid == 6
        );
    }
    Ok(())
}

#[test]
fn test_initial_nft_decoder_rejects_nonzero_and_owned_fields() -> anyhow::Result<()> {
    use crate::contracts::staking_item::storage::CoffeeStakingItemInitialData;
    use ton::ton_core::types::tlb_core::TLBCoins;
    // Build the initial wire layout directly: false, id, master, absent owner,
    // referenced asset/position and an empty periods dictionary.
    for invalid in 0..7 {
        let mut data = TonCell::builder();
        (invalid == 0).write(&mut data)?;
        17u64.write(&mut data)?;
        address(300)?.write(&mut data)?;
        if invalid == 1 {
            address(0)?.to_msg_address_int().write(&mut data)?;
        } else {
            data.write_num(&0u8, 2)?;
        }
        let mut asset = TonCell::builder();
        if invalid == 2 {
            address(0)?.to_msg_address_int().write(&mut asset)?;
        } else {
            asset.write_num(&0u8, 2)?;
        }
        TLBCoins::new(u128::from(invalid == 3)).write(&mut asset)?;
        data.write_ref(asset.build()?)?;
        let mut position = TonCell::builder();
        TLBCoins::new(u128::from(invalid == 4)).write(&mut position)?;
        TLBCoins::new(0).write(&mut position)?;
        0u64.write(&mut position)?;
        0u64.write(&mut position)?;
        u32::from(invalid == 5).write(&mut position)?;
        data.write_ref(position.build()?)?;
        false.write(&mut data)?;
        assert_eq!(CoffeeStakingItemInitialData::from_cell(&data.build()?).is_ok(), invalid == 6);
    }
    Ok(())
}

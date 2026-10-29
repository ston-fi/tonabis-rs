//! Factory derivation matches actual pinned TVM getters on original synthetic
//! state. Synthetic32-bit code cells are commitments, never third-party programs.
use crate::contracts::classic::asset::{DedustClassicAsset, DedustClassicJettonAsset, DedustClassicNativeAsset};
use crate::contracts::classic::common as factory_common;
use std::str::FromStr;
use ton::{
    block_tlb::{FromTVMStack, TVMStack},
    ton_core::{
        cell::{TonCell, TonHash},
        traits::tlb::TLB,
        types::{TonAddress, tlb_core::MsgAddressInt},
    },
};

fn owner() -> anyhow::Result<TonAddress> {
    Ok(TonAddress::from_str(
        "0:0000000000000000000000000000000000000000000000000000000000000001",
    )?)
}

#[test]
fn test_classic_factory_v1_captured_getter_derivation_and_typed_dictionary() -> anyhow::Result<()> {
    use crate::contracts::classic::factory_v1::{DedustFactoryV1Contract, storage::DedustFactoryV1Storage};

    let data = TonCell::from_boc(include_bytes!("fixtures/factory-1-data.boc").to_vec())?;
    let state: DedustFactoryV1Storage = TLB::from_cell(&data)?;
    assert_eq!(state.config.vault_code_by_asset_type.len(), 2);
    assert_eq!(
        state
            .config
            .vault_code_by_asset_type
            .get(&1)
            .ok_or_else(|| anyhow::anyhow!("missing vault implementation"))?
            .version,
        24
    );
    assert_eq!(
        state
            .config
            .vault_code_by_asset_type
            .get(&1)
            .ok_or_else(|| anyhow::anyhow!("missing implementation"))?
            .code
            .parser()
            .read_num::<u32>(32)?,
        7
    );
    let expected = TonCell::from_boc(include_bytes!("fixtures/factory-1-addresses.boc").to_vec())?;
    let mut p = expected.parser();
    let factory = TonAddress::from_msg_address(MsgAddressInt::read(&mut p)?)?;
    let vault = MsgAddressInt::read(&mut p)?;
    let pool = MsgAddressInt::read(&mut p)?;
    let deposit: MsgAddressInt = TLB::from_cell(p.read_next_ref()?)?;
    p.ensure_empty()?;
    let native = DedustClassicAsset::Native(DedustClassicNativeAsset {});
    let jetton = DedustClassicAsset::Jetton(DedustClassicJettonAsset {
        workchain_id: -1,
        address: TonHash::from_str("0000000000000000000000000000000000000000000000000000000000000011")?,
    });
    let v = DedustFactoryV1Contract::vault_state_init(&factory, (*state.blank_code).clone(), &native)?;
    assert_eq!(v.derive_address(0)?.to_msg_address_int(), vault);
    let p = DedustFactoryV1Contract::pool_state_init(&factory, (*state.blank_code).clone(), true, &native, &jetton)?;
    assert_eq!(p.derive_address(0)?.to_msg_address_int(), pool);
    let reversed =
        DedustFactoryV1Contract::pool_state_init(&factory, (*state.blank_code).clone(), true, &jetton, &native)?;
    assert_eq!(p, reversed);
    assert!(
        DedustFactoryV1Contract::pool_state_init(&factory, (*state.blank_code).clone(), false, &native, &native)
            .is_err()
    );
    let d = DedustFactoryV1Contract::liquidity_deposit_state_init(
        &factory,
        (*state.blank_code).clone(),
        &owner()?,
        true,
        &jetton,
        &native,
    )?;
    assert_eq!(d.derive_address(0)?.to_msg_address_int(), deposit);
    let expected_jetton_vault: MsgAddressInt = TLB::from_cell(&TonCell::from_boc(
        include_bytes!("fixtures/factory-1-jetton-vault-address.boc").to_vec(),
    )?)?;
    let jetton_init = DedustFactoryV1Contract::vault_state_init(&factory, (*state.blank_code).clone(), &jetton)?;
    assert_eq!(jetton_init.derive_address(0)?.to_msg_address_int(), expected_jetton_vault);
    assert_eq!(data.hash()?, state.to_cell()?.hash()?);
    Ok(())
}

#[test]
fn test_classic_factory_v2_captured_getter_derivation_and_typed_dictionary() -> anyhow::Result<()> {
    use crate::contracts::classic::factory_v2::{DedustFactoryV2Contract, storage::DedustFactoryV2Storage};

    let data = TonCell::from_boc(include_bytes!("fixtures/factory-2-data.boc").to_vec())?;
    let state: DedustFactoryV2Storage = TLB::from_cell(&data)?;
    assert_eq!(state.config.vault_code_by_asset_type.len(), 2);
    assert_eq!(
        state
            .config
            .vault_code_by_asset_type
            .get(&1)
            .ok_or_else(|| anyhow::anyhow!("missing vault implementation"))?
            .version,
        24
    );
    assert_eq!(
        state
            .config
            .vault_code_by_asset_type
            .get(&1)
            .ok_or_else(|| anyhow::anyhow!("missing implementation"))?
            .code
            .parser()
            .read_num::<u32>(32)?,
        7
    );
    assert_eq!(
        state
            .operator
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing operator"))?
            .code
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing operator program"))?
            .parser()
            .read_num::<u32>(32)?,
        6
    );
    let expected = TonCell::from_boc(include_bytes!("fixtures/factory-2-addresses.boc").to_vec())?;
    let mut p = expected.parser();
    let factory = TonAddress::from_msg_address(MsgAddressInt::read(&mut p)?)?;
    let vault = MsgAddressInt::read(&mut p)?;
    let pool = MsgAddressInt::read(&mut p)?;
    let deposit: MsgAddressInt = TLB::from_cell(p.read_next_ref()?)?;
    p.ensure_empty()?;
    let native = DedustClassicAsset::Native(DedustClassicNativeAsset {});
    let jetton = DedustClassicAsset::Jetton(DedustClassicJettonAsset {
        workchain_id: -1,
        address: TonHash::from_str("0000000000000000000000000000000000000000000000000000000000000011")?,
    });
    let v = DedustFactoryV2Contract::vault_state_init(&factory, (*state.blank_code).clone(), &native)?;
    assert_eq!(v.derive_address(0)?.to_msg_address_int(), vault);
    let p = DedustFactoryV2Contract::pool_state_init(&factory, (*state.blank_code).clone(), true, &native, &jetton)?;
    assert_eq!(p.derive_address(0)?.to_msg_address_int(), pool);
    let reversed =
        DedustFactoryV2Contract::pool_state_init(&factory, (*state.blank_code).clone(), true, &jetton, &native)?;
    assert_eq!(p, reversed);
    assert!(
        DedustFactoryV2Contract::pool_state_init(&factory, (*state.blank_code).clone(), false, &native, &native)
            .is_err()
    );
    let d = DedustFactoryV2Contract::liquidity_deposit_state_init(
        &factory,
        (*state.blank_code).clone(),
        &owner()?,
        true,
        &jetton,
        &native,
    )?;
    assert_eq!(d.derive_address(0)?.to_msg_address_int(), deposit);
    let expected_jetton_vault: MsgAddressInt = TLB::from_cell(&TonCell::from_boc(
        include_bytes!("fixtures/factory-2-jetton-vault-address.boc").to_vec(),
    )?)?;
    let jetton_init = DedustFactoryV2Contract::vault_state_init(&factory, (*state.blank_code).clone(), &jetton)?;
    assert_eq!(jetton_init.derive_address(0)?.to_msg_address_int(), expected_jetton_vault);
    assert_eq!(data.hash()?, state.to_cell()?.hash()?);

    assert_eq!(
        state
            .operator
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("operator missing"))?
            .version,
        29
    );
    let expected: MsgAddressInt =
        TLB::from_cell(&TonCell::from_boc(include_bytes!("fixtures/factory-2-operator.boc").to_vec())?)?;
    assert_eq!(
        DedustFactoryV2Contract::operator_state_init(&factory, (*state.blank_code).clone(), 255)?
            .derive_address(0)?
            .to_msg_address_int(),
        expected
    );
    Ok(())
}

#[test]
fn test_factory_versioned_code_stack_code_and_arity() -> anyhow::Result<()> {
    let mut b = TonCell::builder();
    b.write_num(&2u32, 32)?;
    let code = b.build()?;
    let mut stack = TVMStack::default();
    stack.push_tiny_int(17);
    stack.push_cell(code.clone());
    let parsed = factory_common::method_results::DedustClassicVersionedCode::from_stack(&mut stack)?;
    assert_eq!((parsed.version, parsed.code.into_inner()), (17, code.clone()));
    let mut extra = TVMStack::default();
    extra.push_tiny_int(99);
    extra.push_tiny_int(17);
    extra.push_cell(code);
    assert!(factory_common::method_results::DedustClassicVersionedCode::from_stack(&mut extra).is_err());
    Ok(())
}

#[test]
fn test_factory_v2_absent_legacy_operator_extension_is_not_zero_version() -> anyhow::Result<()> {
    use crate::contracts::classic::factory_v2::storage::DedustFactoryV2Storage;

    let raw = TonCell::from_boc(include_bytes!("fixtures/factory-2-legacy-data.boc").to_vec())?;
    let data: DedustFactoryV2Storage = TLB::from_cell(&raw)?;
    assert!(data.operator.is_none());
    assert_eq!(data.to_cell()?.hash()?, raw.hash()?);
    let current: DedustFactoryV2Storage =
        TLB::from_cell(&TonCell::from_boc(include_bytes!("fixtures/factory-2-data.boc").to_vec())?)?;
    assert!(current.operator.is_some());
    Ok(())
}

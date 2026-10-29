//! Original wire encoders and captured initialization-hook observations. Program
//! fields in candidate fixtures are synthetic32-bit markers, never third-party code.
use crate::contracts::classic::common::{DedustClassicJettonVaultParameters, msgs::DedustClassicInstallMsg};
use crate::contracts::classic::factory_v2::DedustFactoryV2Contract;
use crate::contracts::classic::{
    common as installer, liquidity_deposit_v1, liquidity_deposit_v2, pool_v1, pool_v2, vault_jetton_v1,
    vault_jetton_v2, vault_native_v1, vault_native_v2,
};
use crate::contracts::operator::v2::storage::DedustOperatorV2Storage;
use std::str::FromStr;
use ton::ton_core::types::TonAddress;
use ton::ton_core::{cell::TonCell, traits::tlb::TLB};

fn cell(bytes: &[u8]) -> anyhow::Result<TonCell> {
    Ok(TonCell::from_boc(bytes.to_vec())?)
}
#[test]
fn test_typed_installer_envelope_selects_known_fixed_program_arguments() -> anyhow::Result<()> {
    let original = cell(include_bytes!("fixtures/installer-pool.boc"))?;
    let pool: DedustClassicInstallMsg<installer::DedustClassicPoolParameters> = TLB::from_cell(&original)?;
    assert_eq!(
        (
            pool.query_id,
            pool.code_version,
            pool.parameters.trade_fee_numerator,
            pool.parameters.asset0_decimals,
            pool.parameters.asset1_decimals
        ),
        (99, 5, 30, 9, 6)
    );
    assert_eq!(pool.code.parser().read_num::<u32>(32)?, 0x22222222);
    assert_eq!(pool.parameters.lp_wallet_code.parser().read_num::<u32>(32)?, 0x33333333);
    assert_eq!(pool.to_cell()?.hash()?, original.hash()?);
    let deposit: DedustClassicInstallMsg<installer::DedustClassicDepositParameters> =
        TLB::from_cell(&cell(include_bytes!("fixtures/installer-deposit.boc"))?)?;
    assert_eq!(
        (
            *deposit.parameters.min_lp_amount,
            *deposit.parameters.asset0_target_balance,
            *deposit.parameters.asset1_target_balance
        ),
        (17, 100, 200)
    );
    let derive: DedustClassicInstallMsg<installer::DedustClassicJettonVaultParameters> =
        TLB::from_cell(&cell(include_bytes!("fixtures/installer-jetton-derive.boc"))?)?;
    assert_eq!(derive.parameters.minter_override, None);
    let literal_zero = cell(include_bytes!("fixtures/installer-jetton-zero.boc"))?;
    let explicit: DedustClassicInstallMsg<installer::DedustClassicJettonVaultParameters> =
        TLB::from_cell(&literal_zero)?;
    assert_eq!(explicit.parameters.minter_override, Some(Some(TonAddress::ZERO)));
    assert_eq!(explicit.parameters.ignored_suffix.data_len_bits(), 3);
    assert_eq!(explicit.to_cell()?.hash()?, literal_zero.hash()?);
    let native: DedustClassicInstallMsg<installer::DedustClassicNativeVaultParameters> =
        TLB::from_cell(&cell(include_bytes!("fixtures/installer-native.boc"))?)?;
    assert_eq!(native.parameters.ignored_arguments.data_len_bits(), 3);
    assert_eq!(native.parameters.ignored_arguments.refs().len(), 1);
    // A typed caller cannot accidentally treat the short native ignored tail as
    // the fixed Pool initializer, despite sharing the envelope opcode.
    assert!(
        <DedustClassicInstallMsg<installer::DedustClassicPoolParameters> as TLB>::from_cell(&cell(include_bytes!(
            "fixtures/installer-native.boc"
        ))?)
        .is_err()
    );
    Ok(())
}
#[test]
fn test_classic_pool_initialized_storage_and_normalization_match_source_hooks() -> anyhow::Result<()> {
    let v1: pool_v1::storage::DedustPoolV1Storage =
        TLB::from_cell(&cell(include_bytes!("fixtures/pool-v1-initializer-post-synthetic.boc"))?)?;
    let v2: pool_v2::storage::DedustPoolV2Storage =
        TLB::from_cell(&cell(include_bytes!("fixtures/pool-v2-initializer-post-synthetic.boc"))?)?;
    assert_eq!((v1.version, v1.trade_fee_numerator, *v1.total_supply), (5, 30, 0));
    assert_eq!((v2.version, v2.trade_fee_numerator, *v2.total_supply), (5, 30, 0));
    assert_eq!((v1.config.asset0_decimals, v1.config.asset1_decimals), (9, 6));
    assert_eq!(
        (v2.normalization_factor0, v2.normalization_factor1),
        (1000000000000000000000000000, 1000000000000000000000000000000)
    );
    assert_eq!(v1.proof_code.data_len_bits(), 32);
    assert_eq!(v2.proof_code.data_len_bits(), 32);
    Ok(())
}
#[test]
fn test_deposit_initialized_details_and_ignored_initializer_suffix_match_hooks() -> anyhow::Result<()> {
    let v1: liquidity_deposit_v1::storage::DedustLiquidityDepositV1Storage =
        TLB::from_cell(&cell(include_bytes!("fixtures/deposit-v1-initializer-post-synthetic.boc"))?)?;
    let v2: liquidity_deposit_v2::storage::DedustLiquidityDepositV2Storage =
        TLB::from_cell(&cell(include_bytes!("fixtures/deposit-v2-initializer-post-synthetic.boc"))?)?;
    assert_eq!((*v1.asset0_balance, *v1.asset1_balance, *v1.min_lp_amount), (0, 0, 17));
    assert_eq!((*v2.asset0_balance, *v2.asset1_balance, *v2.min_lp_amount), (0, 0, 17));
    assert_eq!(
        (*v1.details.asset0_target_balance, *v1.details.asset1_target_balance),
        (100, 200)
    );
    assert_eq!(
        (*v2.details.asset0_target_balance, *v2.details.asset1_target_balance),
        (100, 200)
    );
    assert!(!v1.is_processing && !v2.is_processing);
    let parameters: installer::DedustClassicDepositParameters =
        TLB::from_cell(&cell(include_bytes!("fixtures/deposit-v2-initializer-params.boc"))?)?;
    assert_eq!(parameters.ignored_suffix.data_len_bits(), 3);
    assert_eq!(parameters.ignored_suffix.refs().len(), 1);
    Ok(())
}
#[test]
fn test_native_and_jetton_initialized_storage_match_both_generation_hooks() -> anyhow::Result<()> {
    let n1: vault_native_v1::storage::DedustVaultNativeV1Storage =
        TLB::from_cell(&cell(include_bytes!("fixtures/native-v1-initializer-post-synthetic.boc"))?)?;
    let n2: vault_native_v2::storage::DedustVaultNativeV2Storage =
        TLB::from_cell(&cell(include_bytes!("fixtures/native-v2-initializer-post-synthetic.boc"))?)?;
    assert_eq!((n1.version, n2.version, *n2.native_balance), (5, 5, 0));
    let j1: vault_jetton_v1::storage::DedustVaultJettonV1Storage =
        TLB::from_cell(&cell(include_bytes!("fixtures/jetton-v1-initializer-post-synthetic.boc"))?)?;
    let j2: vault_jetton_v2::storage::DedustVaultJettonV2Storage =
        TLB::from_cell(&cell(include_bytes!("fixtures/jetton-v2-initializer-post-synthetic.boc"))?)?;
    assert_eq!((j1.version, j2.version), (5, 5));
    assert!(!j1.is_ready && !j2.is_ready);
    assert_eq!(j1.jetton_wallet_addr, None);
    assert_eq!(j2.jetton_wallet_addr, None);
    Ok(())
}

#[test]
fn test_operator_initializer_preserves_role_literal_zero_user_and_timer() -> anyhow::Result<()> {
    let storage: DedustOperatorV2Storage =
        TLB::from_cell(&cell(include_bytes!("fixtures/operator-v2-initializer-post-synthetic.boc"))?)?;
    assert_eq!((storage.version, storage.role, storage.can_execute_after), (5, 10, 0));
    assert!(storage.user == TonAddress::ZERO);
    Ok(())
}
#[test]
fn test_both_managed_pool_initializers_store_explicit_zero_activation() -> anyhow::Result<()> {
    use crate::contracts::classic::operator_pool;

    for bytes in [
        include_bytes!("fixtures/pool-managed-a-initializer-post-synthetic.boc").as_slice(),
        include_bytes!("fixtures/pool-managed-b-initializer-post-synthetic.boc").as_slice(),
    ] {
        let cell = cell(bytes)?;
        let state = operator_pool::storage::DedustClassicOperatorPoolStorage::from_cell(&cell)?;
        assert_eq!(state.swap_active_since, Some(0));
        assert_eq!(
            (state.pool.version, state.pool.trade_fee_numerator, *state.pool.total_supply),
            (5, 30, 0)
        );
        assert_eq!(state.pool.normalization_factor0, 1000000000000000000000000000u128);
        assert_eq!(state.pool.normalization_factor1, 1000000000000000000000000000000u128);
        assert_eq!(state.to_cell()?.hash()?, cell.hash()?);
        assert_eq!(pool_v2::storage::DedustPoolV2Storage::from_cell(&cell)?, state.pool);
    }
    Ok(())
}

#[test]
fn test_blank_operator_proof_fixed_kind_and_factory_commitment() -> anyhow::Result<()> {
    use crate::contracts::classic::common::DedustClassicOperatorProof;

    let original = cell(include_bytes!("fixtures/blank-operator-proof.boc"))?;
    let proof = DedustClassicOperatorProof::from_cell(&original)?;
    assert_eq!(proof.role, 10);
    assert!(
        proof.factory == TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000001")?
    );
    let init = DedustFactoryV2Contract::operator_state_init(&proof.factory, TonCell::empty().clone(), proof.role)?;
    assert_eq!(
        init.data.as_ref().ok_or_else(|| anyhow::anyhow!("data"))?.hash()?,
        original.hash()?
    );
    let mut wrong = TonCell::builder();
    proof.factory.write(&mut wrong)?;
    wrong.write_num(&3u8, 8)?;
    wrong.write_num(&10u8, 8)?;
    assert!(DedustClassicOperatorProof::from_cell(&wrong.build()?).is_err());
    let mut suffix = TonCell::builder();
    suffix.write_cell(&original)?;
    suffix.write_bit(false)?;
    assert_eq!(DedustClassicOperatorProof::from_cell(&suffix.build()?)?, proof);
    Ok(())
}

#[test]
fn test_jetton_minter_override_distinguishes_omission_from_encoded_none() -> anyhow::Result<()> {
    let mut omitted = TonCell::builder();
    omitted.write_ref(TonCell::empty().clone())?;
    let omitted = omitted.build()?;
    let parsed = DedustClassicJettonVaultParameters::from_cell(&omitted)?;
    assert_eq!(parsed.minter_override, None);
    assert_eq!(parsed.to_cell()?, omitted);

    let mut explicit = TonCell::builder();
    explicit.write_num(&0u8, 2)?; // addr_none, with an ordinary ignored suffix
    explicit.write_num(&7u8, 3)?;
    explicit.write_ref(TonCell::empty().clone())?;
    let explicit = explicit.build()?;
    let parsed = DedustClassicJettonVaultParameters::from_cell(&explicit)?;
    assert_eq!(parsed.minter_override, Some(None));
    assert_eq!(parsed.to_cell()?, explicit);
    assert!(
        DedustClassicJettonVaultParameters {
            minter_override: None,
            ignored_suffix: parsed.ignored_suffix,
        }
        .to_cell()
        .is_err()
    );
    Ok(())
}

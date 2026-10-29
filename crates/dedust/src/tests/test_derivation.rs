use crate::contracts::cpmm_v2::deployment;
use fastnum::U256;
use std::str::FromStr;
use ton::ton_core::{
    cell::TonCell,
    traits::tlb::TLB,
    types::{TonAddress, tlb_core::MsgAddressInt},
};

#[test]
fn test_cpmm_position_address_matches_captured_getter() -> anyhow::Result<()> {
    // Literal owner-zero case from catalog PoolV1 revision test, independent
    // of the implementation's StateInit/hash calculations.
    let pool = TonAddress::from_str("EQAEGspJNRpLvMDXeLrD8QcUNSNAP61jwgFQIJlOEPDCtwwi")?.to_msg_address_int();
    let mut owner = TonCell::builder();
    owner.write_num(&0b100u8, 3)?;
    owner.write_num(&0i8, 8)?;
    owner.write_num(&U256::ZERO, 256)?;
    let owner: MsgAddressInt = TLB::from_cell(&owner.build()?)?;
    let (init, address) =
        deployment::position_state_init(TonAddress::from_msg_address(pool)?, TonAddress::from_msg_address(owner)?)?;
    let expected = TonAddress::from_str("0:0404f5b7b5485cf99f712f6f2d51032d6f93814eb0601103b59f428bfbd32d59")?
        .to_msg_address_int();
    assert_eq!(address, ton::ton_core::types::TonAddress::from_msg_address(expected)?);
    assert_eq!(init.split_depth, Some(8));
    Ok(())
}

#[test]
fn test_pool_library_identity_is_revision_specific() -> anyhow::Result<()> {
    use crate::contracts::cpmm_v2::deployment::DedustV2CpmmRevision;

    let v1 = deployment::pool_code(DedustV2CpmmRevision::PoolV1)?;
    let v2 = deployment::pool_code(DedustV2CpmmRevision::PoolV2)?;
    // These are root exotic-cell hashes from catalog captured code, not library hashes.
    assert_eq!(
        v1.hash()?.to_string().to_lowercase(),
        "3997a5c1ee8923e93cf3a6a98ea3ef9482c64dd8aea2ba8365689e14d99c760d"
    );
    assert_ne!(v1, v2);
    Ok(())
}

#[test]
fn test_uranus_legacy_wallet_address_matches_captured_getter() -> anyhow::Result<()> {
    let meme = TonAddress::from_str("EQAO_1qemostkDtMfhp7XU0nOsAkQ_tLUO5YJeNMrj3dRAaH")?.to_msg_address_int();
    let owner = TonAddress::from_str("0:D4A0253D0688EE4B1292BF17B51FBFFBC8F0F2B0000BFECF9E47590A5E83790D")?
        .to_msg_address_int();
    let (_, address) = crate::contracts::uranus::deployment::wallet_state_init(
        ton::ton_core::types::TonAddress::from_msg_address(meme)?,
        ton::ton_core::types::TonAddress::from_msg_address(owner)?,
        crate::contracts::uranus::deployment::DedustUranusRevision::V2,
    )?;
    let expected = TonAddress::from_str("EQDUR-Q4u6FxL7SagNQDT7B-CuWpTHLNrTy20PMHoRuO6ofc")?.to_msg_address_int();
    assert_eq!(address, ton::ton_core::types::TonAddress::from_msg_address(expected)?);
    Ok(())
}

#[test]
fn test_cpmm_deposit_address_matches_both_pinned_pool_child_checks() -> anyhow::Result<()> {
    use crate::contracts::cpmm_v2::deposit::storage::DedustV2CpmmDepositStorage;

    for (bytes, expected) in [
        (
            include_bytes!("fixtures/deposit-initial-v1-data.boc").as_slice(),
            "63e4561c799162c3d752e6058253953bed8376f2a98203957250103aa3ec7618",
        ),
        (
            include_bytes!("fixtures/deposit-initial-v2-data.boc").as_slice(),
            "53ef8ab2a154328d8b75b230eaa445d89aaa4aa7b8548c243b1d9816f7aa8b38",
        ),
    ] {
        let cell = TonCell::from_boc(bytes.to_vec())?;
        let storage: DedustV2CpmmDepositStorage = TLB::from_cell(&cell)?;
        let (init, address) =
            deployment::deposit_state_init((*storage.config).clone(), storage.pool, storage.initiator)?;
        assert_eq!(init.split_depth, Some(8));
        assert_eq!(
            address,
            ton::ton_core::types::TonAddress::from_msg_address(
                TonAddress::from_str(&format!("0:{expected}"))?.to_msg_address_int()
            )?
        );
    }
    Ok(())
}
#[test]
fn test_uranus_v3_wallet_and_created_cpmm_pool_initial_state_match_execution() -> anyhow::Result<()> {
    use crate::contracts::{
        cpmm_v2::{deployment, pool},
        uranus,
    };
    use ton::ton_core::{
        cell::TonCell,
        traits::tlb::TLB,
        types::tlb_core::{MsgAddressInt, MsgAddressIntStd},
    };

    let addresses = TonCell::from_boc(include_bytes!("fixtures/uranus-v3-wallet-addresses.boc").to_vec())?;
    let mut p = addresses.parser();
    let meme = MsgAddressInt::read(&mut p)?;
    let owner = MsgAddressInt::read(&mut p)?;
    let expected = MsgAddressInt::read(&mut p)?;
    p.ensure_empty()?;
    let (init, actual) = uranus::deployment::wallet_state_init(
        ton::ton_core::types::TonAddress::from_msg_address(meme)?,
        ton::ton_core::types::TonAddress::from_msg_address(owner)?,
        uranus::deployment::DedustUranusRevision::V3,
    )?;
    assert_eq!(actual, ton::ton_core::types::TonAddress::from_msg_address(expected)?);
    assert_eq!(init.split_depth, Some(8));
    let data = TonCell::from_boc(include_bytes!("fixtures/uranus-created-pool-data.boc").to_vec())?;
    let storage = pool::storage::DedustV2CpmmStorage::from_cell(&data)?;
    let init = deployment::pool_state_init(
        (*storage.config).clone(),
        storage.extra.controller,
        deployment::DedustV2CpmmRevision::PoolV2,
    )?;
    assert_eq!(init.split_depth, None);
    assert_eq!(init.data.as_ref().ok_or_else(|| anyhow::anyhow!("data"))?.hash()?, data.hash()?);
    let actual = MsgAddressInt::Std(MsgAddressIntStd {
        anycast: None,
        workchain: 0,
        address: init.cell_hash()?,
    });
    let expected = MsgAddressInt::from_cell(&TonCell::from_boc(
        include_bytes!("fixtures/uranus-created-pool-address.boc").to_vec(),
    )?)?;
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn test_uranus_v2_created_cpmm_pool_v1_initial_state_matches_execution() -> anyhow::Result<()> {
    use crate::contracts::cpmm_v2::{
        deployment::{self, DedustV2CpmmRevision},
        pool,
    };
    use ton::ton_core::{
        cell::TonCell,
        traits::tlb::TLB,
        types::tlb_core::{MsgAddressInt, MsgAddressIntStd},
    };

    let data = TonCell::from_boc(include_bytes!("fixtures/uranus-v2-created-pool-data.boc").to_vec())?;
    let storage = pool::storage::DedustV2CpmmStorage::from_cell(&data)?;
    let init = deployment::pool_state_init(
        (*storage.config).clone(),
        storage.extra.controller,
        DedustV2CpmmRevision::PoolV1,
    )?;
    assert_eq!(init.split_depth, None);
    assert_eq!(init.data.as_ref().ok_or_else(|| anyhow::anyhow!("data"))?.hash()?, data.hash()?);
    let actual = MsgAddressInt::Std(MsgAddressIntStd {
        anycast: None,
        workchain: 0,
        address: init.cell_hash()?,
    });
    let expected = MsgAddressInt::from_cell(&TonCell::from_boc(
        include_bytes!("fixtures/uranus-v2-created-pool-address.boc").to_vec(),
    )?)?;
    assert_eq!(actual, expected);
    Ok(())
}

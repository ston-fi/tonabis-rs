//! Roots use canonical TON StateInit with explicit caller program/state. These
//! original synthetic program commitments qualify storage/address encoding,
//! not executable identity or a historical factory's deployment policy.
use crate::contracts::{
    classic::{factory_v1, factory_v2},
    custom_wallet::{v1, v2},
    uranus::{factory_v3, meme_v2},
};
use ton::{
    block_tlb::StateInit,
    ton_core::{
        cell::TonCell,
        traits::tlb::TLB,
        types::tlb_core::{MsgAddressInt, MsgAddressIntStd},
    },
};

fn check<T: TLB>(init: &[u8], address: &[u8], id: u32) -> anyhow::Result<()> {
    let original = StateInit::from_cell(&TonCell::from_boc(init.to_vec())?)?;
    let data = original.data.as_ref().ok_or_else(|| anyhow::anyhow!("data"))?;
    let typed = T::from_cell(data)?;
    let code = original.code.ok_or_else(|| anyhow::anyhow!("code"))?;
    assert_eq!(code.data_len_bits(), 32);
    assert_eq!(code.parser().read_num::<u32>(32)?, 0x11220000 + id);
    let actual = StateInit::new(code, typed.to_cell()?);
    let actual = MsgAddressInt::Std(MsgAddressIntStd {
        anycast: None,
        workchain: 0,
        address: actual.cell_hash()?,
    });
    let expected = MsgAddressInt::from_cell(&TonCell::from_boc(address.to_vec())?)?;
    assert_eq!(actual, expected);
    Ok(())
}
#[test]
fn test_typed_root_storage_preserves_explicit_caller_program_address_commitment() -> anyhow::Result<()> {
    check::<factory_v1::storage::DedustFactoryV1Storage>(
        include_bytes!("fixtures/root-init-factory-v1.boc"),
        include_bytes!("fixtures/root-address-factory-v1.boc"),
        1,
    )?;
    check::<factory_v2::storage::DedustFactoryV2Storage>(
        include_bytes!("fixtures/root-init-factory-v2.boc"),
        include_bytes!("fixtures/root-address-factory-v2.boc"),
        2,
    )?;
    check::<v1::storage::DedustX1000WalletV1Storage>(
        include_bytes!("fixtures/root-init-x1000-v1.boc"),
        include_bytes!("fixtures/root-address-x1000-v1.boc"),
        3,
    )?;
    check::<v2::storage::DedustX1000WalletV2Storage>(
        include_bytes!("fixtures/root-init-x1000-v2.boc"),
        include_bytes!("fixtures/root-address-x1000-v2.boc"),
        4,
    )?;
    check::<factory_v3::storage::DedustUranusFactoryV3Storage>(
        include_bytes!("fixtures/root-init-uranus-factory-v3.boc"),
        include_bytes!("fixtures/root-address-uranus-factory-v3.boc"),
        5,
    )?;
    check::<meme_v2::storage::DedustUranusMemeV2Storage>(
        include_bytes!("fixtures/root-init-uranus-meme-v2.boc"),
        include_bytes!("fixtures/root-address-uranus-meme-v2.boc"),
        6,
    )?;
    Ok(())
}

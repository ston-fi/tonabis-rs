use crate::contracts::{cpmm::msgs::MoonCpmmMsg, mempad::msgs::MoonMempadMsg};
use fastnum::U256;
use ton::ton_core::{cell::TonCell, traits::tlb::TLB};

#[test]
fn test_metadata_preserves_empty_continuations_and_rejects_partial_bytes() -> anyhow::Result<()> {
    use crate::contracts::metadata::MoonPrefixedString;
    let mut bytes = TonCell::builder();
    bytes.write_num(&0xffu8, 8)?;
    bytes.write_ref(TonCell::empty().clone())?;
    let mut empty = TonCell::builder();
    empty.write_ref(bytes.build()?)?;
    let mut root = TonCell::builder();
    root.write_num(&0u8, 8)?;
    root.write_ref(empty.build()?)?;
    let cell = root.build()?;
    let parsed = MoonPrefixedString::from_cell(&cell)?;
    assert_eq!(parsed.chunks, vec![vec![], vec![], vec![0xff], vec![]]);
    assert_eq!(parsed.to_cell()?, cell);

    let mut partial = TonCell::builder();
    partial.write_bit(true)?;
    let mut root = TonCell::builder();
    root.write_num(&0u8, 8)?;
    root.write_ref(partial.build()?)?;
    assert!(MoonPrefixedString::from_cell(&root.build()?).is_err());
    Ok(())
}

#[test]
fn test_unknown_short_bodies_retain_bits_and_refs() -> anyhow::Result<()> {
    for bits in [0, 1, 31, 32] {
        let mut builder = TonCell::builder();
        builder.write_num(&0u32, bits)?;
        builder.write_ref(TonCell::empty().clone())?;
        let cell = builder.build()?;
        assert!(matches!(MoonCpmmMsg::from_cell(&cell)?, MoonCpmmMsg::Unknown(_)));
        assert_eq!(MoonCpmmMsg::from_cell(&cell)?.to_cell()?, cell);
        assert!(matches!(MoonMempadMsg::from_cell(&cell)?, MoonMempadMsg::Unknown(_)));
        assert_eq!(MoonMempadMsg::from_cell(&cell)?.to_cell()?, cell);
    }
    Ok(())
}

#[test]
fn test_known_mint_with_wrong_nested_opcode_is_an_error() -> anyhow::Result<()> {
    // Full known outer body, deliberately wrong referenced internal-transfer opcode.
    let mut nested = TonCell::builder();
    nested.write_num(&0xdeadbeefu32, 32)?;
    let mut outer = TonCell::builder();
    outer.write_num(&0x642b7d07u32, 32)?;
    outer.write_num(&1u64, 64)?;
    outer.write_num(&4u8, 3)?; // addr_std without anycast
    outer.write_num(&0i8, 8)?;
    outer.write_num(&U256::ZERO, 256)?;
    outer.write_num(&0u8, 4)?; // zero coins
    outer.write_ref(nested.build()?)?;
    assert!(MoonMempadMsg::from_cell(&outer.build()?).is_err());
    Ok(())
}

#[test]
fn test_special_transfer_literals_fail_on_read_and_write() -> anyhow::Result<()> {
    use crate::contracts::cpmm::msgs::MoonCpmmJettonTransferMsg;
    use ton::ton_core::types::{TonAddress, tlb_core::TLBCoins};
    let mut transfer = MoonCpmmJettonTransferMsg {
        query_id: 1,
        amount: TLBCoins::ONE,
        destination: TonAddress::ZERO,
        response_destination: TonAddress::ZERO,
        custom_payload: None,
        forward_ton_amount: TLBCoins::ZERO,
        have_payload: false,
    };
    let cell = transfer.to_cell()?;
    let mut parser = cell.parser();
    let mut bad = TonCell::builder();
    bad.write_bits(&parser.read_bits(cell.data_len_bits() - 1)?, cell.data_len_bits() - 1)?;
    bad.write_bit(true)?;
    assert!(MoonCpmmJettonTransferMsg::from_cell(&bad.build()?).is_err());
    transfer.have_payload = true;
    assert!(transfer.to_cell().is_err());
    Ok(())
}

#[test]
fn test_required_and_nullable_internal_zero_addresses() -> anyhow::Result<()> {
    use crate::contracts::cpmm::pool::storage::MoonCpmmPoolMasters;
    use ton::ton_core::types::TonAddress;
    let absent = MoonCpmmPoolMasters {
        x_master: None,
        y_master: TonAddress::ZERO,
        initiator: TonAddress::ZERO,
    };
    let present = MoonCpmmPoolMasters {
        x_master: Some(TonAddress::ZERO),
        ..absent.clone()
    };
    assert_ne!(absent.cell_hash()?, present.cell_hash()?);
    assert_eq!(MoonCpmmPoolMasters::from_cell(&absent.to_cell()?)?.x_master, None);
    assert_eq!(
        MoonCpmmPoolMasters::from_cell(&present.to_cell()?)?.x_master,
        Some(TonAddress::ZERO)
    );
    let mut malformed = TonCell::builder();
    malformed.write_num(&0u8, 2)?; // nullable x_master
    malformed.write_num(&0u8, 2)?; // required y_master cannot be addr_none
    malformed.write_num(&0u8, 2)?;
    assert!(MoonCpmmPoolMasters::from_cell(&malformed.build()?).is_err());
    Ok(())
}

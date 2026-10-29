use crate::adapters::{
    maybe_address::MaybeAddress, maybe_ref_address::MaybeRefAddress,
    maybe_ref_nullable_address::MaybeRefNullableAddress,
};
use ton_core::{cell::TonCell, types::TonAddress};

#[test]
fn test_maybe_addresses_preserve_tag_reference_and_zero() -> anyhow::Result<()> {
    let mut raw = TonCell::builder();
    raw.write_num(&4u8, 3)?;
    raw.write_num(&0i8, 8)?;
    raw.write_bits([0u8; 32], 256)?;
    let zero = raw.build()?;
    for present in [false, true] {
        let expected = present.then_some(TonAddress::ZERO);
        let mut inline = TonCell::builder();
        inline.write_bit(present)?;
        if present {
            inline.write_cell(&zero)?;
        }
        let inline = inline.build()?;
        assert_eq!(MaybeAddress.read(&mut inline.parser())?, expected);
        let mut encoded = TonCell::builder();
        MaybeAddress.write(&mut encoded, &expected)?;
        assert_eq!(encoded.build()?, inline);

        let mut referenced = TonCell::builder();
        referenced.write_bit(present)?;
        if present {
            referenced.write_ref(zero.clone())?;
        }
        let referenced = referenced.build()?;
        assert_eq!(MaybeRefAddress.read(&mut referenced.parser())?, expected);
        let mut encoded = TonCell::builder();
        MaybeRefAddress.write(&mut encoded, &expected)?;
        assert_eq!(encoded.build()?, referenced);
    }
    let mut missing = TonCell::builder();
    missing.write_bit(true)?;
    assert!(MaybeRefAddress.read(&mut missing.build()?.parser()).is_err());
    let mut none_child = TonCell::builder();
    none_child.write_num(&0u8, 2)?;
    let mut invalid = TonCell::builder();
    invalid.write_bit(true)?;
    invalid.write_ref(none_child.build()?)?;
    assert!(MaybeRefAddress.read(&mut invalid.build()?.parser()).is_err());
    let invalid = Some(TonAddress::new(128, [0u8; 32].into()));
    assert!(MaybeAddress.write(&mut TonCell::builder(), &invalid).is_err());
    assert!(MaybeRefAddress.write(&mut TonCell::builder(), &invalid).is_err());
    Ok(())
}

#[test]
fn test_referenced_nullable_address_keeps_all_three_states_and_tails() -> anyhow::Result<()> {
    for address in [None, Some(None), Some(Some(TonAddress::ZERO))] {
        let mut expected = TonCell::builder();
        expected.write_bit(address.is_some())?;
        if let Some(inner) = address {
            let mut child = TonCell::builder();
            if inner.is_some() {
                child.write_num(&4u8, 3)?;
                child.write_num(&0i8, 8)?;
                child.write_bits([0u8; 32], 256)?;
            } else {
                child.write_num(&0u8, 2)?;
            }
            expected.write_ref(child.build()?)?;
        }
        let expected = expected.build()?;
        assert_eq!(MaybeRefNullableAddress.read(&mut expected.parser())?, address);
        let mut encoded = TonCell::builder();
        MaybeRefNullableAddress.write(&mut encoded, &address)?;
        assert_eq!(encoded.build()?, expected);
    }
    let mut child = TonCell::builder();
    child.write_num(&0u8, 2)?;
    child.write_num(&53u8, 8)?;
    child.write_ref(TonCell::empty().clone())?;
    let mut body = TonCell::builder();
    body.write_bit(true)?;
    body.write_ref(child.build()?)?;
    body.write_num(&7u8, 8)?;
    let body = body.build()?;
    let mut parser = body.parser();
    assert_eq!(MaybeRefNullableAddress.read(&mut parser)?, Some(None));
    assert_eq!(parser.read_num::<u8>(8)?, 7);
    Ok(())
}

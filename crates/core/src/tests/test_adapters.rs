use crate::adapters::nullable_internal_address::NullableInternalAddress;
use ton_core::{
    TLB,
    cell::TonCell,
    traits::tlb::TLB,
    types::{
        TonAddress,
        tlb_core::{MsgAddress, MsgAddressIntVar},
    },
};

#[derive(Debug, PartialEq, TLB)]
struct NullableRecord {
    #[tlb(adapter = "NullableInternalAddress")]
    address: Option<TonAddress>,
    suffix: u8,
}

#[test]
fn test_nullable_internal_address_preserves_none_zero_and_field_boundary() -> anyhow::Result<()> {
    // Independently write addr_none, literal internal zero, and a nonstandard address.
    let mut zero = TonCell::builder();
    zero.write_num(&4u8, 3)?;
    zero.write_num(&0i8, 8)?;
    zero.write_bits([0u8; 32], 256)?;
    let zero = zero.build()?;
    let mut variable = TonCell::builder();
    variable.write_num(&6u8, 3)?;
    variable.write_num(&8u16, 9)?;
    variable.write_num(&256i32, 32)?;
    variable.write_num(&42u8, 8)?;
    let variable = variable.build()?;
    let mut anycast = TonCell::builder();
    anycast.write_num(&5u8, 3)?;
    anycast.write_num(&1u8, 5)?;
    anycast.write_bit(true)?;
    anycast.write_num(&0i8, 8)?;
    anycast.write_bits([0u8; 32], 256)?;
    let anycast = anycast.build()?;
    for address_cell in [MsgAddress::NONE.to_cell()?, zero] {
        let mut builder = TonCell::builder();
        builder.write_cell(&address_cell)?;
        builder.write_num(&53u8, 8)?;
        let cell = builder.build()?;
        let decoded = NullableRecord::from_cell(&cell)?;
        assert_eq!(decoded.suffix, 53);
        assert_eq!(decoded.to_cell()?, cell);
    }
    let raw = MsgAddressIntVar {
        anycast: None,
        addr_bits_len: 8,
        workchain: 256,
        address: vec![42],
    };
    assert!(NullableInternalAddress::from_address(MsgAddress::Int(raw.into())).is_err());
    for invalid in [variable, anycast] {
        assert!(NullableInternalAddress.read(&mut invalid.parser()).is_err());
        assert!(NullableInternalAddress::from_address(MsgAddress::from_cell(&invalid)?).is_err());
    }
    // addr_extern$01 with an eight-bit payload is outside this adapter's domain.
    let mut external = TonCell::builder();
    external.write_num(&1u8, 2)?;
    external.write_num(&8u16, 9)?;
    external.write_num(&42u8, 8)?;
    assert!(NullableInternalAddress.read(&mut external.build()?.parser()).is_err());
    Ok(())
}

//! Untagged suffixes and nested prefix errors keep their protocol meaning.
use crate::contracts::{
    dedust_minter_v7, dedust_minter_v12,
    ston_minter::msgs::{self, StonksStonMinterBuyMsg, StonksStonMinterSellMsg, StonksStonMinterSellNotificationMsg},
};
use ton::ton_core::{
    cell::TonCell,
    traits::tlb::TLB,
    types::{
        TonAddress,
        tlb_core::{TLBCoins, TLBEitherRef},
    },
};

#[test]
fn test_untagged_suffix_shapes_and_nested_dispatch() -> anyhow::Result<()> {
    let mut buy = StonksStonMinterBuyMsg {
        query_id: 7,
        ton_amount: TLBCoins::ONE,
        min_jettons_out: TLBCoins::ZERO,
        recipient: None,
        payload: None,
    };
    let omitted = buy.to_cell()?;
    assert_eq!(StonksStonMinterBuyMsg::from_cell(&omitted)?, buy);
    buy.payload = Some(TLBEitherRef::new(TonCell::empty().clone()));
    let present = buy.to_cell()?;
    assert_ne!(omitted, present);
    assert!(StonksStonMinterBuyMsg::from_cell(&present)?.payload.is_some());
    buy.recipient = Some(TonAddress::new(128, [0; 32].into()));
    assert!(buy.to_cell().is_err());
    buy.recipient = Some(TonAddress::ZERO);
    assert_eq!(StonksStonMinterBuyMsg::from_cell(&buy.to_cell()?)?, buy);

    let sell = StonksStonMinterSellMsg {
        query_id: 8,
        jetton_amount: TLBCoins::ONE,
        tons_recipient: None,
        min_ton_out: TLBCoins::ZERO,
        payload: None,
    };
    let cell = sell.to_cell()?;
    let mut missing_ref = TonCell::builder();
    missing_ref.write_cell(&cell)?;
    missing_ref.write_bit(true)?;
    assert!(StonksStonMinterSellMsg::from_cell(&missing_ref.build()?).is_err());
    let notify = StonksStonMinterSellNotificationMsg {
        query_id: 9,
        jetton_amount: TLBCoins::ONE,
        sell_initiator: None,
        tons_recipient: None,
        min_ton_out: TLBCoins::ZERO,
        payload: None,
    };
    let cell = notify.to_cell()?;
    assert!(msgs::StonksStonMinterCurrentMessage::from_cell(&cell).is_err());
    assert!(matches!(
        dedust_minter_v12::msgs::StonksDedustMinterV12Message::from_cell(&cell)?,
        dedust_minter_v12::msgs::StonksDedustMinterV12Message::Sell(_)
    ));
    let mut missing_ref = TonCell::builder();
    missing_ref.write_cell(&cell)?;
    missing_ref.write_bit(true)?;
    assert!(dedust_minter_v12::msgs::StonksDedustMinterV12Message::from_cell(&missing_ref.build()?).is_err());

    let mut child = TonCell::builder();
    child.write_num(&0x12345678_u32, 32)?;
    let mut callback = TonCell::builder();
    callback.write_num(&0xce185bd7_u32, 32)?;
    callback.write_num(&1_u64, 64)?;
    callback.write_num(&0_i32, 32)?;
    callback.write_bit(true)?;
    callback.write_ref(child.build()?)?;
    let callback = callback.build()?;
    assert!(dedust_minter_v7::msgs::StonksDedustMinterV7Message::from_cell(&callback).is_err());
    assert!(dedust_minter_v12::msgs::StonksDedustMinterV12Message::from_cell(&callback).is_err());

    let mut raw = TonCell::builder();
    raw.write_num(&0x12345678_u32, 32)?;
    raw.write_bit(true)?;
    raw.write_ref(TonCell::empty().clone())?;
    let raw = raw.build()?;
    assert_eq!(msgs::StonksStonMinterCurrentMessage::from_cell(&raw)?.to_cell()?, raw);
    assert_eq!(
        dedust_minter_v12::msgs::StonksDedustMinterV12Message::from_cell(&raw)?.to_cell()?,
        raw
    );
    Ok(())
}

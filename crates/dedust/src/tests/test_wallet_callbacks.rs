use crate::contracts::classic::lp_wallet::msgs::DedustLpWalletOutgoingMsg;
use crate::contracts::custom_wallet::v2::{self, msgs as callbacks};
use ton::ton_core::{
    cell::TonCell,
    traits::tlb::TLB,
    types::{TonAddress, tlb_core::EitherRefLayout},
};

fn wire(name: &str) -> anyhow::Result<TonCell> {
    let rows: serde_json::Value = serde_json::from_str(include_str!("fixtures/x1000-callback-vectors.json"))?;
    Ok(TonCell::from_boc_base64(
        rows[name]["boc_base64"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("missing callback {name}"))?,
    )?)
}
#[test]
fn test_fixed_wallet_callback_dispatch_and_literal_zero_sender_preservation() -> anyhow::Result<()> {
    let none: callbacks::DedustX1000WalletV2JettonNotificationMsg = TLB::from_cell(&wire("callback-jetton-null")?)?;
    let zero: callbacks::DedustX1000WalletV2JettonNotificationMsg = TLB::from_cell(&wire("callback-jetton-zero")?)?;
    assert_eq!((none.query_id, *none.amount), (1, 77));
    assert_eq!((zero.query_id, *zero.amount), (2, 88));
    assert_eq!(none.sender, None);
    assert_eq!(zero.sender, Some(TonAddress::ZERO));
    match DedustLpWalletOutgoingMsg::from_cell(&wire("callback-jetton-zero")?)? {
        DedustLpWalletOutgoingMsg::TransferNotification(v) => {
            assert_eq!(v.sender, zero.sender);
            assert_eq!(v.to_cell()?.hash()?, wire("callback-jetton-zero")?.hash()?);
        },
        _ => anyhow::bail!("LP notification sender form lost"),
    }
    assert_ne!(none.sender, zero.sender);
    assert_eq!(tonabis_core::address::nullable_address_cell(&none.sender)?.data_len_bits(), 2);
    assert_eq!(tonabis_core::address::nullable_address_cell(&zero.sender)?.data_len_bits(), 267);
    assert_eq!(none.forward_payload.layout, EitherRefLayout::ToCell);
    assert_eq!(zero.forward_payload.layout, EitherRefLayout::ToRef);
    assert_eq!(none.forward_payload.value.parser().read_num::<u32>(32)?, 0x11223344);
    assert_eq!(zero.forward_payload.value.parser().read_num::<u8>(8)?, 0x55);
    for name in [
        "callback-jetton-null",
        "callback-jetton-zero",
        "callback-excess",
        "callback-classic",
        "callback-coffee",
        "callback-pton-inline",
        "callback-pton-reference",
        "callback-bidask-legacy",
        "callback-bidask-flagged",
        "callback-clmm-result",
        "callback-clmm-refund",
        "callback-moon-success",
        "callback-moon-failure",
    ] {
        let cell = wire(name)?;
        let parsed = v2::msgs::DedustX1000WalletV2IncomingMsg::from_cell(&cell)?;
        assert!(!matches!(parsed, v2::msgs::DedustX1000WalletV2IncomingMsg::Unknown(_)));
        assert_eq!(parsed.to_cell()?.hash()?, cell.hash()?);
        let mut short = TonCell::builder();
        short.write_num(&cell.parser().read_num::<u32>(32)?, 32)?;
        assert!(v2::msgs::DedustX1000WalletV2IncomingMsg::from_cell(&short.build()?).is_err());
    }
    let mut unknown = TonCell::builder();
    unknown.write_num(&0xaabbccddu32, 32)?;
    unknown.write_num(&3u8, 3)?;
    let unknown = unknown.build()?;
    match v2::msgs::DedustX1000WalletV2IncomingMsg::from_cell(&unknown)? {
        v2::msgs::DedustX1000WalletV2IncomingMsg::Unknown(raw) => {
            assert_eq!(raw.hash()?, unknown.hash()?)
        },
        _ => anyhow::bail!("unknown callback bytes not preserved"),
    }
    Ok(())
}
#[test]
fn test_callback_generation_flags_reference_forms_and_peer_nested_layouts() -> anyhow::Result<()> {
    let legacy: callbacks::DedustX1000WalletV2BidaskNotificationMsg = TLB::from_cell(&wire("callback-bidask-legacy")?)?;
    let flagged: callbacks::DedustX1000WalletV2BidaskNotificationMsg =
        TLB::from_cell(&wire("callback-bidask-flagged")?)?;
    match legacy {
        callbacks::DedustX1000WalletV2BidaskNotificationMsg::Legacy(v) => {
            assert_eq!((v.query_id, *v.amount), (8, 1234));
            assert!(v.payload.is_none());
        },
        _ => anyhow::bail!("legacy notification gained a flag"),
    }
    match flagged {
        callbacks::DedustX1000WalletV2BidaskNotificationMsg::Flagged(v) => {
            assert!(v.notification_bit);
            assert_eq!((v.query_id, *v.amount), (9, 5678));
            assert_eq!(
                v.payload
                    .as_ref()
                    .ok_or_else(|| anyhow::anyhow!("missing peer payload"))?
                    .parser()
                    .read_num::<u16>(16)?,
                0x9900
            );
        },
        _ => anyhow::bail!("notification flag lost"),
    }
    let pton: callbacks::DedustX1000WalletV2PtonTransferMsg = TLB::from_cell(&wire("callback-pton-inline")?)?;
    assert_eq!((pton.query_id, *pton.amount), (6, 99));
    assert_eq!(pton.forward_payload.value.parser().read_num::<u8>(8)?, 0x77);
    let peer: callbacks::DedustX1000WalletV2MoonClmmJettonResultPayloadMsg =
        TLB::from_cell(&wire("callback-clmm-jetton")?)?;
    assert_eq!(
        peer.payload
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing nested peer payload"))?
            .parser()
            .read_num::<u32>(32)?,
        0xaabbccdd
    );
    let native: callbacks::DedustX1000WalletV2MoonClmmNativeResultMsg = TLB::from_cell(&wire("callback-clmm-result")?)?;
    assert_eq!((native.query_id, *native.amount), (10, 44));
    let compact: v2::storage::DedustX1000MoonClmmCallback = TLB::from_cell(&wire("callback-clmm-compact")?)?;
    assert!(matches!(
        compact.payload,
        callbacks::DedustX1000WalletV2CallbackPayloadMsg::Omitted
    ));
    let explicit: v2::storage::DedustX1000MoonClmmCallback = TLB::from_cell(&wire("callback-clmm-explicit-none")?)?;
    assert!(matches!(
        explicit.payload,
        callbacks::DedustX1000WalletV2CallbackPayloadMsg::Explicit(None)
    ));
    assert_ne!(compact.to_cell()?.hash()?, explicit.to_cell()?.hash()?);
    let some: v2::storage::DedustX1000MoonClmmCallback = TLB::from_cell(&wire("callback-clmm-explicit-some")?)?;
    assert!(matches!(
        some.payload,
        callbacks::DedustX1000WalletV2CallbackPayloadMsg::Explicit(Some(_))
    ));
    Ok(())
}

#[test]
fn test_missing_catalog_quote_response_is_known_typed_and_tolerant() -> anyhow::Result<()> {
    use crate::contracts::classic::pool_v2::msgs::DedustPoolV2IncomingMsg as IncomingMessage;
    use crate::contracts::classic::pool_v2::msgs::DedustPoolV2QuoteResponseMsg;
    use ton::ton_core::cell::TonCell;
    use ton::ton_core::traits::tlb::TLB;
    for (id, bytes) in [
        include_bytes!("fixtures/quote-response-0.boc").as_slice(),
        include_bytes!("fixtures/quote-response-1.boc").as_slice(),
    ]
    .into_iter()
    .enumerate()
    {
        let wire = TonCell::from_boc(bytes.to_vec())?;
        let response = DedustPoolV2QuoteResponseMsg::from_cell(&wire)?;
        assert_eq!((response.query_id, response.numerator, response.denominator), (7, 2, 3));
        assert_eq!(response.ignored_context.is_some(), id == 1);
        if let Some(context) = &response.ignored_context {
            assert_eq!(context.parser().read_num::<u8>(8)?, 0x42);
        }
        assert!(matches!(IncomingMessage::from_cell(&wire)?, IncomingMessage::QuoteResponse(_)));
        assert_eq!(response.to_cell()?.hash()?, wire.hash()?);
        let mut bad = TonCell::builder();
        bad.write_cell(&wire)?;
        bad.write_bit(false)?;
        assert_eq!(IncomingMessage::from_cell(&bad.build()?)?.to_cell()?, wire);
    }
    let mut truncated = TonCell::builder();
    truncated.write_num(&0x0a420458u32, 32)?;
    assert!(IncomingMessage::from_cell(&truncated.build()?).is_err());
    Ok(())
}

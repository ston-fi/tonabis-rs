#[test]
fn test_factory_meme_state_init_matches_observed_deployment() -> anyhow::Result<()> {
    use crate::contracts::uranus::{deployment, meme_v3::storage::DedustUranusMemeV3Storage};
    use ton::ton_core::{cell::TonCell, traits::tlb::TLB, types::tlb_core::MsgAddressInt};

    let factory = MsgAddressInt::from_cell(&TonCell::from_boc(
        include_bytes!("fixtures/uranus-factory-address.boc").to_vec(),
    )?)?;
    for (data, address) in [
        (
            include_bytes!("fixtures/factory-preset3-meme-data.boc").as_slice(),
            include_bytes!("fixtures/factory-preset3-meme-address.boc").as_slice(),
        ),
        (
            include_bytes!("fixtures/factory-custom20-meme-data.boc").as_slice(),
            include_bytes!("fixtures/factory-custom20-meme-address.boc").as_slice(),
        ),
    ] {
        let cell = TonCell::from_boc(data.to_vec())?;
        let state = DedustUranusMemeV3Storage::from_cell(&cell)?;
        let expected = MsgAddressInt::from_cell(&TonCell::from_boc(address.to_vec())?)?;
        let (init, actual) =
            deployment::meme_state_init(&state, &ton::ton_core::types::TonAddress::from_msg_address(factory.clone())?)?;
        assert_eq!(actual, ton::ton_core::types::TonAddress::from_msg_address(expected)?);
        assert_eq!(init.split_depth, Some(8));
        assert_eq!(init.data.ok_or_else(|| anyhow::anyhow!("data"))?.hash()?, cell.hash()?);
        assert!(!state.initialized && !state.migrated && !state.curve.graduated);
        assert_eq!(
            (
                *state.curve.current_supply,
                *state.curve.raised_funds,
                *state.creator_fee,
                *state.partner_fee
            ),
            (0, 0, 0, 0)
        );
    }
    Ok(())
}

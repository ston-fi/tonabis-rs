use crate::contracts::*;

use ton::ton_core::cell::TonCell;
use ton::ton_core::traits::tlb::TLB;
#[test]
fn test_observed_empty_funding_is_distinct_from_truncated_opcode_and_reference_only() -> anyhow::Result<()> {
    let data: serde_json::Value = serde_json::from_str(include_str!("fixtures/empty-funding-observations.json"))?;
    assert_eq!(
        format!("{}", TonCell::empty().hash()?).to_lowercase(),
        data["empty_cell_hash_hex"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("missing hash"))?
    );
    let mut short = TonCell::builder();
    short.write_bit(false)?;
    let short = short.build()?;
    let mut reference_only = TonCell::builder();
    reference_only.write_ref(TonCell::empty().clone())?;
    let reference_only = reference_only.build()?;
    macro_rules! check {
        ($module:ident,$name:ident) => {{
            assert!(matches!(
                $module::msgs::$name::from_cell(TonCell::empty())?,
                $module::msgs::$name::Empty(_)
            ));
            assert_eq!(
                $module::msgs::$name::Empty(tonabis_core::msgs::EmptyBodyMsg {}).to_cell()?,
                *TonCell::empty()
            );
            assert!(matches!(
                $module::msgs::$name::from_cell(&short)?,
                $module::msgs::$name::Unknown(_)
            ));
            assert!(matches!(
                $module::msgs::$name::from_cell(&reference_only)?,
                $module::msgs::$name::Unknown(_)
            ));
        }};
    }
    check!(cross_dex, CoffeeCrossDexIncomingMessage);
    check!(cross_dex_legacy, CoffeeCrossDexLegacyIncomingMessage);
    check!(cross_dex_bidask, CoffeeCrossDexBidaskIncomingMessage);
    check!(mev_protector, CoffeeMevProtectorIncomingMessage);
    check!(mev_protector_legacy, CoffeeMevProtectorLegacyIncomingMessage);
    check!(lp_wallet, CoffeeLpWalletIncomingMessage);
    check!(staking_item, CoffeeStakingItemIncomingMessage);
    check!(staking_master, CoffeeStakingMasterIncomingMessage);
    check!(staking_vault, CoffeeStakingVaultIncomingMessage);
    // The Factory and Init have no empty-funding role in these revisions.
    assert!(matches!(
        factory::msgs::CoffeeFactoryIncomingMessage::from_cell(TonCell::empty())?,
        factory::msgs::CoffeeFactoryIncomingMessage::Unknown(_)
    ));
    assert!(matches!(
        init::msgs::CoffeeInitIncomingMessage::from_cell(TonCell::empty())?,
        init::msgs::CoffeeInitIncomingMessage::Unknown(_)
    ));
    Ok(())
}

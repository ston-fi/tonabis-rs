use crate::contracts::cpmm_v2::{
    deposit::storage::DedustV2CpmmDepositStorage, position::storage::DedustV2CpmmPositionStorage,
};
use fastnum::u256;
use ton::ton_core::{cell::TonCell, traits::tlb::TLB};
#[test]
fn test_position_storage_from_independent_tolk_code_replay() -> anyhow::Result<()> {
    let cell = TonCell::from_boc(include_bytes!("fixtures/position-state.boc").to_vec())?;
    let s: DedustV2CpmmPositionStorage = TLB::from_cell(&cell)?;
    assert_eq!((*s.total_liquidity, *s.locked_liquidity), (19, 5));
    assert_eq!((*s.fees.x_pending, *s.fees.y_pending), (7, 11));
    assert_eq!((*s.fees.x_pending_secondary, *s.fees.y_pending_secondary), (13, 17));
    assert_eq!(*s.fees.x_checkpoint, u256!(1329227995784915872903807060280344576));
    assert_eq!(*s.fees.y_checkpoint, u256!(2658455991569831745807614120560689152));
    assert_eq!(s.to_cell()?.hash()?, cell.hash()?);
    Ok(())
}
#[test]
fn test_deposit_storage_from_independent_tolk_code_replay() -> anyhow::Result<()> {
    let cell = TonCell::from_boc(include_bytes!("fixtures/deposit-state.boc").to_vec())?;
    let s: DedustV2CpmmDepositStorage = TLB::from_cell(&cell)?;
    assert_eq!((*s.amount_x, *s.amount_y), (11, 23));
    assert_eq!((*s.config.target_x, *s.config.target_y), (100, 200));
    assert_eq!(*s.config.minimal_liquidity, 17);
    assert_eq!(s.config.locked_liquidity_share, 5000);
    assert_eq!(s.to_cell()?.hash()?, cell.hash()?);
    Ok(())
}

#[test]
fn test_lp_wallet_storage_and_minimal_burn_from_code_replay() -> anyhow::Result<()> {
    use crate::contracts::classic::lp_wallet;
    let cell = TonCell::from_boc(include_bytes!("fixtures/lp-wallet-state.boc").to_vec())?;
    let s: lp_wallet::storage::DedustClassicLpWalletStorage = TLB::from_cell(&cell)?;
    assert_eq!(*s.balance, 12345);
    assert!(s.wallet_code.data_len_bits() == 0);
    let body = TonCell::from_boc(include_bytes!("fixtures/lp-burn-minimal.boc").to_vec())?;
    let burn: lp_wallet::msgs::DedustClassicLpWalletBurnMsg = TLB::from_cell(&body)?;
    assert_eq!(burn.query_id, 1);
    assert_eq!(*burn.amount, 123);
    assert_eq!(burn.suffix.data_len_bits(), 0);
    assert_eq!(burn.to_cell()?, body);
    let mut sdk = TonCell::builder();
    sdk.write_cell(&body)?;
    sdk.write_bit(false)?;
    let sdk = sdk.build()?;
    let with_suffix: lp_wallet::msgs::DedustClassicLpWalletBurnMsg = TLB::from_cell(&sdk)?;
    assert_eq!(with_suffix.suffix.data_len_bits(), 1);
    assert_eq!(with_suffix.to_cell()?, sdk);
    let notification = TonCell::from_boc(include_bytes!("fixtures/lp-burn-notification.boc").to_vec())?;
    match lp_wallet::msgs::DedustLpWalletOutgoingMsg::from_cell(&notification)? {
        lp_wallet::msgs::DedustLpWalletOutgoingMsg::BurnNotification(v) => {
            assert_eq!((v.query_id, *v.amount), (1, 123));
            assert_eq!(v.sender, s.owner);
            assert_eq!(v.response_destination, Some(s.owner));
        },
        _ => anyhow::bail!("LP burn notification lost fixed schema"),
    }

    Ok(())
}

#[test]
fn test_position_emitted_notification_and_state_selection() -> anyhow::Result<()> {
    use crate::contracts::cpmm_v2::position;
    let notification = TonCell::from_boc(include_bytes!("fixtures/position-notification.boc").to_vec())?;
    let notification: position::msgs::DedustV2CpmmPositionDepositNotificationMsg = TLB::from_cell(&notification)?;
    assert_eq!(notification.query_id, 1);
    assert_eq!((*notification.liquidity, *notification.locked_liquidity), (3, 1));
    assert!(notification.payload.is_none());
    let reply = TonCell::from_boc(include_bytes!("fixtures/position-reply.boc").to_vec())?;
    let parsed: position::msgs::DedustV2CpmmPositionStateReplyMsg = TLB::from_cell(&reply)?;
    assert_eq!(parsed.query_id, 777);
    assert_eq!((*parsed.total_liquidity, *parsed.locked_liquidity), (22, 6));
    assert!(parsed.pool.is_some() && parsed.owner.is_some());
    let fees = parsed.fees.as_ref().ok_or_else(|| anyhow::anyhow!("requested fees missing"))?;
    assert_eq!((*fees.x_pending, *fees.y_pending), (45, 68));
    assert!(parsed.rewards.as_ref().is_some_and(|v| v.entries.is_empty()));
    assert_eq!(parsed.to_cell()?, reply);
    let state = TonCell::from_boc(include_bytes!("fixtures/position-credited-state.boc").to_vec())?;
    let state: DedustV2CpmmPositionStorage = TLB::from_cell(&state)?;
    assert_eq!((*state.fees.x_pending_secondary, *state.fees.y_pending_secondary), (13, 17));
    Ok(())
}

#[test]
fn test_uranus_migration_real_cpmm_deposit_event() -> anyhow::Result<()> {
    use crate::contracts::cpmm_v2::pool;
    let event = TonCell::from_boc(include_bytes!("fixtures/migration-deposit-event.boc").to_vec())?;
    let parsed: pool::msgs::DedustV2CpmmDepositEventMsg = TLB::from_cell(&event)?;
    assert_eq!(
        (*parsed.amount_x_in, *parsed.amount_y_in, *parsed.liquidity),
        (11000000, 1000000, 3316624)
    );
    assert_eq!(
        (
            *parsed.next_state.reserve_x,
            *parsed.next_state.reserve_y,
            *parsed.next_state.liquidity
        ),
        (11000000, 1000000, 3316624)
    );
    Ok(())
}

#[test]
fn test_uranus_migration_mint_preserves_minter_sender() -> anyhow::Result<()> {
    use crate::contracts::uranus::wallet_v3;

    let body = TonCell::from_boc(include_bytes!("fixtures/uranus-migration-mint.boc").to_vec())?;
    let mint: wallet_v3::msgs::DedustUranusMemeWalletV3JettonInternalTransferMsg = TLB::from_cell(&body)?;
    assert_eq!(*mint.amount, 1000000);
    assert!(mint.r#from.is_some());
    assert_eq!(mint.to_cell()?, body);
    Ok(())
}

#[test]
fn test_both_uranus_wallet_revisions_accept_independent_null_sender_mint() -> anyhow::Result<()> {
    use crate::contracts::uranus::wallet_v2;
    use crate::contracts::uranus::wallet_v3;

    let legacy = TonCell::from_boc(include_bytes!("fixtures/uranus-v2-mint-null.boc").to_vec())?;
    let modern = TonCell::from_boc(include_bytes!("fixtures/uranus-v3-mint-null.boc").to_vec())?;
    let a: wallet_v2::msgs::DedustUranusMemeWalletV2JettonInternalTransferMsg = TLB::from_cell(&legacy)?;
    let b: wallet_v3::msgs::DedustUranusMemeWalletV3JettonInternalTransferMsg = TLB::from_cell(&modern)?;
    assert_eq!((*a.amount, *b.amount), (17, 17));
    assert_eq!(a.r#from, None);
    assert_eq!(b.r#from, None);
    assert_eq!(a.to_cell()?, legacy);
    assert_eq!(b.to_cell()?, modern);
    Ok(())
}

#[test]
fn test_position_payout_bounce_and_reward_records_from_pinned_transitions() -> anyhow::Result<()> {
    use crate::contracts::cpmm_v2::position;
    fn state(raw: &[u8]) -> anyhow::Result<DedustV2CpmmPositionStorage> {
        Ok(TLB::from_cell(&TonCell::from_boc(raw.to_vec())?)?)
    }
    let claimed = state(include_bytes!("fixtures/position-fees-claimed.boc"))?;
    let restored = state(include_bytes!("fixtures/position-fees-restored.boc"))?;
    assert_eq!((*claimed.fees.x_pending, *claimed.fees.y_pending), (0, 0));
    assert_eq!((*restored.fees.x_pending, *restored.fees.y_pending), (45, 68));
    for bytes in [
        include_bytes!("fixtures/position-withdraw-claim.boc").as_slice(),
        include_bytes!("fixtures/position-withdraw-retain.boc").as_slice(),
    ] {
        let s = state(bytes)?;
        assert_eq!((*s.total_liquidity, *s.locked_liquidity), (16, 5));
        assert_eq!((*s.fees.x_pending_secondary, *s.fees.y_pending_secondary), (13, 17));
    }
    let reward_claimed = state(include_bytes!("fixtures/position-reward-claimed.boc"))?;
    let reward_restored = state(include_bytes!("fixtures/position-reward-restored.boc"))?;
    assert_eq!(
        *reward_claimed
            .rewards
            .get(&0)
            .ok_or_else(|| anyhow::anyhow!("missing reward slot0"))?
            .unclaimed,
        0
    );
    assert_eq!(
        *reward_restored
            .rewards
            .get(&0)
            .ok_or_else(|| anyhow::anyhow!("missing reward slot0"))?
            .unclaimed,
        45
    );
    let bounce = TonCell::from_boc(include_bytes!("fixtures/position-fees-bounce.boc").to_vec())?;
    match position::msgs::DedustV2PositionBounceMsg::from_cell(&bounce)? {
        position::msgs::DedustV2PositionBounceMsg::PositionFees {
            query_id,
            x_fees,
            y_fees,
            ..
        } => assert_eq!((query_id, *x_fees, *y_fees), (7, 45, 68)),
        _ => anyhow::bail!("fee bounce misclassified"),
    }
    Ok(())
}

#[test]
fn test_deposit_partial_credit_fixed_parent_requests_and_rejections() -> anyhow::Result<()> {
    use crate::contracts::cpmm_v2::deposit;
    let partial = TonCell::from_boc(include_bytes!("fixtures/deposit-partial.boc").to_vec())?;
    let partial: DedustV2CpmmDepositStorage = TLB::from_cell(&partial)?;
    assert_eq!((*partial.amount_x, *partial.amount_y), (100, 23));
    let join = TonCell::from_boc(include_bytes!("fixtures/deposit-join.boc").to_vec())?;
    match deposit::msgs::DedustV2DepositOutgoingMsg::from_cell(&join)? {
        deposit::msgs::DedustV2DepositOutgoingMsg::JoinLiquidity(v) => {
            assert_eq!((v.query_id, *v.amount_x, *v.amount_y), (2, 100, 200));
            assert_eq!(
                (
                    *v.config.target_x,
                    *v.config.target_y,
                    *v.config.minimal_liquidity,
                    v.config.locked_liquidity_share
                ),
                (100, 200, 17, 5000)
            );
        },
        _ => anyhow::bail!("deposit join dispatch lost"),
    }
    let refund = TonCell::from_boc(include_bytes!("fixtures/deposit-refund.boc").to_vec())?;
    match deposit::msgs::DedustV2DepositOutgoingMsg::from_cell(&refund)? {
        deposit::msgs::DedustV2DepositOutgoingMsg::RefundDeposit(v) => {
            assert_eq!((v.query_id, *v.amount_x, *v.amount_y), (3, 11, 23))
        },
        _ => anyhow::bail!("deposit refund dispatch lost"),
    }
    for bytes in [
        include_bytes!("fixtures/deposit-rejected-unchanged.boc").as_slice(),
        include_bytes!("fixtures/deposit-bounce-unchanged.boc").as_slice(),
    ] {
        let state: DedustV2CpmmDepositStorage = TLB::from_cell(&TonCell::from_boc(bytes.to_vec())?)?;
        assert_eq!((*state.amount_x, *state.amount_y), (11, 23));
    }
    Ok(())
}

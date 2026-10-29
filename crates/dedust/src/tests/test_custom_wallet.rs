//! Observed fixed wallet records, with original synthetic V1 state.
use crate::contracts::custom_wallet::v1::msgs::DedustX1000WalletV1IncomingMsg;
use crate::contracts::custom_wallet::v1::storage::DedustX1000WalletV1Storage;
use std::str::FromStr;
use ton::ton_core::{
    cell::{TonCell, TonHash},
    traits::tlb::TLB,
};

#[test]
fn test_v1_revocation_records_preserve_expiry_and_bitmap_after_prune() -> anyhow::Result<()> {
    let before = TonCell::from_boc(include_bytes!("fixtures/x1000-v1-state.boc").to_vec())?;
    let after = TonCell::from_boc(include_bytes!("fixtures/x1000-v1-pruned-state.boc").to_vec())?;
    let b: DedustX1000WalletV1Storage = TLB::from_cell(&before)?;
    let a: DedustX1000WalletV1Storage = TLB::from_cell(&after)?;
    let expired = TonHash::from_str("000000000000000000000000000000000000000000000000000000000000007b")?;
    let active = TonHash::from_str("00000000000000000000000000000000000000000000000000000000000001c8")?;
    assert_eq!(b.revoked.get(&expired), Some(&900));
    assert!(!a.revoked.contains_key(&expired));
    assert_eq!(a.revoked.get(&active), Some(&1100));
    assert_eq!(a.seqno_offset, 17);
    assert_eq!(
        a.seqno_bitmap.to_hex(),
        "8000000000000000000000000000000000000000000000000000000000000000"
    );
    assert_eq!(a.public_key, b.public_key);
    assert_eq!(a.to_cell()?.hash()?, after.hash()?);
    let command = TonCell::from_boc(include_bytes!("fixtures/x1000-v1-prune.boc").to_vec())?;
    assert!(matches!(
        DedustX1000WalletV1IncomingMsg::from_cell(&command)?,
        DedustX1000WalletV1IncomingMsg::PruneExpiredKeys(_)
    ));
    let mut malformed = TonCell::builder();
    malformed.write_cell(&command)?;
    malformed.write_bit(false)?;
    assert_eq!(
        DedustX1000WalletV1IncomingMsg::from_cell(&malformed.build()?)?.to_cell()?,
        command
    );
    Ok(())
}

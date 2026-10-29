//! Public DeDust dictionary layouts use native TON label validation and tolerant tails.
use crate::contracts::classic::factory_v1::storage::DedustFactoryV1Config;
use ton::ton_core::{cell::TonCell, traits::tlb::TLB};

#[derive(Clone, Copy)]
enum Label {
    Short,
    Long,
    Same,
}

fn edge(label: Label, remaining: u32, count: u32, leaf: bool, tail: bool) -> anyhow::Result<TonCell> {
    let mut b = TonCell::builder();
    match label {
        Label::Short => {
            b.write_bit(false)?;
            for _ in 0..count {
                b.write_bit(true)?;
            }
            b.write_bit(false)?;
        },
        Label::Long => {
            b.write_num(&2u8, 2)?;
            b.write_num(&count, (u32::BITS - remaining.leading_zeros()) as usize)?;
        },
        Label::Same => {
            b.write_num(&6u8, 3)?;
            b.write_num(&count, (u32::BITS - remaining.leading_zeros()) as usize)?;
        },
    }
    if !matches!(label, Label::Same) && count != 0 {
        b.write_num(&0u32, count as usize)?;
    }
    if leaf {
        b.write_num(&37u16, 16)?;
        let mut code = TonCell::builder();
        code.write_num(&5u32, 32)?;
        b.write_ref(code.build()?)?;
    }
    if tail {
        b.write_bit(true)?;
        b.write_ref(TonCell::empty().clone())?;
    }
    Ok(b.build()?)
}

fn config(root: Option<TonCell>) -> anyhow::Result<TonCell> {
    let mut b = TonCell::builder();
    b.write_num(&1u16, 16)?;
    b.write_ref(TonCell::empty().clone())?;
    b.write_num(&2u16, 16)?;
    b.write_ref(TonCell::empty().clone())?;
    b.write_bit(root.is_some())?;
    if let Some(root) = root {
        b.write_ref(root)?;
    }
    b.write_ref(TonCell::empty().clone())?;
    Ok(b.build()?)
}

#[test]
fn test_public_dictionary_rejects_oversized_short_long_same_labels_and_nested_forks() -> anyhow::Result<()> {
    for label in [Label::Short, Label::Long, Label::Same] {
        assert!(DedustFactoryV1Config::from_cell(&config(Some(edge(label, 4, 5, true, false)?))?).is_err());
        // A one-bit prefix leaves two key bits after the fork, where 3 is invalid.
        let mut fork = TonCell::builder();
        fork.write_num(&2u8, 3)?;
        fork.write_bit(false)?;
        fork.write_ref(edge(label, 2, 3, true, false)?)?;
        fork.write_ref(edge(Label::Same, 2, 2, true, false)?)?;
        assert!(DedustFactoryV1Config::from_cell(&config(Some(fork.build()?))?).is_err());
    }
    Ok(())
}

#[test]
fn test_public_dictionary_accepts_alternate_exact_and_zero_labels_with_leaf_and_fork_tails() -> anyhow::Result<()> {
    for label in [Label::Short, Label::Long, Label::Same] {
        let parsed = DedustFactoryV1Config::from_cell(&config(Some(edge(label, 4, 4, true, true)?))?)?;
        assert_eq!(parsed.vault_code_by_asset_type[&0].version, 37);
        assert_eq!(parsed.vault_code_by_asset_type[&0].code.parser().read_num::<u32>(32)?, 5);
        let mut root = TonCell::builder();
        root.write_cell(&edge(label, 4, 0, false, false)?)?;
        root.write_ref(edge(Label::Same, 3, 3, true, true)?)?;
        root.write_ref(edge(Label::Long, 3, 3, true, false)?)?;
        root.write_bit(true)?;
        root.write_ref(TonCell::empty().clone())?;
        let parsed = DedustFactoryV1Config::from_cell(&config(Some(root.build()?))?)?;
        assert_eq!(parsed.vault_code_by_asset_type.len(), 2);
        assert!(parsed.vault_code_by_asset_type.contains_key(&0));
        assert!(parsed.vault_code_by_asset_type.contains_key(&8));
    }
    // Forks can reduce the remaining key width to zero. Every legal zero label
    // encoding must still decode the leaf rather than shifting or underflowing.
    fn zero_tree(label: Label, remaining: u32) -> anyhow::Result<TonCell> {
        if remaining == 0 {
            return edge(label, 0, 0, true, false);
        }
        let mut b = TonCell::builder();
        b.write_cell(&edge(label, remaining, 0, false, false)?)?;
        let child = zero_tree(label, remaining - 1)?;
        b.write_ref(child.clone())?;
        b.write_ref(child)?;
        Ok(b.build()?)
    }
    for label in [Label::Short, Label::Long, Label::Same] {
        assert_eq!(
            DedustFactoryV1Config::from_cell(&config(Some(zero_tree(label, 4)?))?)?
                .vault_code_by_asset_type
                .len(),
            16
        );
    }
    assert!(
        DedustFactoryV1Config::from_cell(&config(None)?)?
            .vault_code_by_asset_type
            .is_empty()
    );
    Ok(())
}

#[test]
fn test_public_dictionary_rejects_truncated_labels_values_and_missing_branches() -> anyhow::Result<()> {
    let mut truncated = TonCell::builder();
    truncated.write_num(&2u8, 2)?;
    truncated.write_num(&4u8, 3)?;
    for root in [
        truncated.build()?,
        edge(Label::Short, 4, 0, false, false)?,
        edge(Label::Long, 4, 4, false, false)?,
    ] {
        assert!(DedustFactoryV1Config::from_cell(&config(Some(root))?).is_err());
    }
    Ok(())
}

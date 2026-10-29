//! Independently executed, source-selected context prefixes. These values came
//! from Tolk/TVM input-shortening probes, not the Rust writer under test.
use crate::contracts::custom_wallet::v2::storage::DedustX1000WalletV2ContextRecord;
use crate::contracts::custom_wallet::v2::{
    self,
    storage::{DedustX1000WalletV2Context, DedustX1000WalletV2PendingContinuation},
};
use fastnum::U256;
use ton::ton_core::types::tlb_core::TLBCoins;
use ton::ton_core::{cell::TonCell, traits::tlb::TLB};

#[test]
fn test_x1000_all_fixed_context_branches_match_independent_prefix_vectors() -> anyhow::Result<()> {
    let rows: Vec<serde_json::Value> = serde_json::from_str(include_str!("fixtures/x1000-context-vectors.json"))?;
    assert_eq!(rows.len(), 22);
    for row in rows {
        let number = |key: &str| -> anyhow::Result<u8> {
            Ok(u8::try_from(
                row[key].as_u64().ok_or_else(|| anyhow::anyhow!("missing {key}"))?,
            )?)
        };
        let kind = number("kind")?;
        let a = number("field4_0")?;
        let b = number("field4_1")?;
        let cell = TonCell::from_boc_base64(row["boc_base64"].as_str().ok_or_else(|| anyhow::anyhow!("missing BOC"))?)?;
        let parsed = DedustX1000WalletV2Context::from_cell(kind, a, b, &cell)?;
        parsed.validate_header(kind, a, b)?;
        assert!(parsed.validate_header(kind, 0, b).is_err());
        if kind == 3 {
            assert!(parsed.validate_header(kind, a, 1 - b).is_err());
        }
        match &parsed {
            DedustX1000WalletV2Context::Classic(v) | DedustX1000WalletV2Context::StonfiV2(v) => {
                assert_eq!(*v.amount, 42);
                assert_eq!(v.addresses.address0.address.to_hex(), format!("{:064x}", 1));
                assert_eq!(v.second_addresses.address1.address.to_hex(), format!("{:064x}", 4));
            },
            DedustX1000WalletV2Context::ToncoMode0(v)
            | DedustX1000WalletV2Context::StonksSell(v)
            | DedustX1000WalletV2Context::UranusSell(v)
            | DedustX1000WalletV2Context::JettonBurn(v) => {
                assert_eq!(*v.amount, 42);
                assert_eq!(v.addresses.address1.address.to_hex(), format!("{:064x}", 2));
            },
            DedustX1000WalletV2Context::ToncoMode1(v)
            | DedustX1000WalletV2Context::Cpmm(v)
            | DedustX1000WalletV2Context::BidaskDamm(v)
            | DedustX1000WalletV2Context::Moon(v)
            | DedustX1000WalletV2Context::PayloadFa45056b(v)
            | DedustX1000WalletV2Context::BidaskClmm(v) => {
                assert_eq!(*v.amount, 42);
                assert_eq!(v.second_address.address.address.to_hex(), format!("{:064x}", 3));
            },
            DedustX1000WalletV2Context::Coffee(v) => {
                assert_eq!(*v.amount, 42);
                assert_eq!(v.second_address.word, U256::from_u8(17));
            },
            DedustX1000WalletV2Context::MoonClmm(v) => {
                assert_eq!(v.word, U256::from_u8(123));
                assert_eq!(v.second_address.address.address.to_hex(), format!("{:064x}", 3));
            },
            DedustX1000WalletV2Context::IgnoredByFixedBuilder(_) => anyhow::bail!("known active branch became raw"),
        }
        assert_eq!(parsed.to_cell()?.hash()?, cell.hash()?);
        // The source reads complete fixed references: removing them is malformed,
        // never an unknown/opaque extension for an active known branch.
        let missing = if kind == 9 {
            let mut c = TonCell::builder();
            c.write_num(&123u32, 256)?;
            c.build()?
        } else {
            let mut c = TonCell::builder();
            c.write_num(&0x12au16, 12)?;
            c.build()?
        };
        assert!(DedustX1000WalletV2Context::from_cell(kind, a, b, &missing).is_err());
        let mut suffix = TonCell::builder();
        suffix.write_cell(&cell)?;
        suffix.write_num(&5u8, 3)?;
        let suffix = suffix.build()?;
        let with_suffix = DedustX1000WalletV2Context::from_cell(kind, a, b, &suffix)?;
        assert_eq!(with_suffix.to_cell()?.hash()?, suffix.hash()?);
    }
    Ok(())
}
#[test]
fn test_x1000_raw_context_cannot_bypass_a_known_fixed_active_schema() -> anyhow::Result<()> {
    let raw = TonCell::empty().clone();
    let mut node = DedustX1000WalletV2PendingContinuation {
        operation_kind: 4,
        field4_0: 1,
        field4_1: 0,
        context: Some(DedustX1000WalletV2ContextRecord {
            prefix_amount: TLBCoins::new(0),
            body: DedustX1000WalletV2Context::IgnoredByFixedBuilder(raw.clone()),
        }),
        continuation: None,
        fallback: None,
    };
    assert!(node.to_cell().is_err());
    node.field4_0 = 0;
    let encoded = node.to_cell()?;
    let parsed: DedustX1000WalletV2PendingContinuation = TLB::from_cell(&encoded)?;
    assert!(matches!(
        parsed.context,
        Some(DedustX1000WalletV2ContextRecord {
            body: DedustX1000WalletV2Context::IgnoredByFixedBuilder(_),
            ..
        })
    ));
    assert!(matches!(
        DedustX1000WalletV2Context::from_cell(14, 1, 0, &raw)?,
        DedustX1000WalletV2Context::IgnoredByFixedBuilder(_)
    ));
    Ok(())
}

#[test]
fn test_x1000_callback_holds_and_controller_releases_dictionary3_message() -> anyhow::Result<()> {
    use crate::contracts::custom_wallet::v2::storage::DedustX1000WalletV2ExecutionEntry;
    use ton::{block_tlb::Msg, contracts::tep::jetton::jetton_transfer_msg::JettonTransferMsg};

    let held_cell = TonCell::from_boc(include_bytes!("fixtures/x1000-stored-message-state.boc").to_vec())?;
    let held: v2::storage::DedustX1000WalletV2Storage = TLB::from_cell(&held_cell)?;
    assert_eq!(held.runtime_state.pending_operations.len(), 2);
    let next = held
        .runtime_state
        .pending_operations
        .get(&100)
        .ok_or_else(|| anyhow::anyhow!("missing next continuation"))?;
    assert_eq!((next.operation_kind, next.field4_0, next.field4_1), (4, 1, 0));
    let record = next
        .context
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("missing persistent context"))?;
    assert_eq!(*record.prefix_amount, 0);
    match &record.body {
        DedustX1000WalletV2Context::Cpmm(body) => assert_eq!(*body.amount, 42),
        _ => anyhow::bail!("next context lost fixed protocol shape"),
    }
    assert_eq!(
        *held
            .runtime_state
            .reserved_values
            .get(&100)
            .ok_or_else(|| anyhow::anyhow!("missing reserved amount"))?,
        TLBCoins::new(10000)
    );
    assert_eq!(held.runtime_state.request_links.get(&100), Some(&99));
    let message = match held.runtime_state.execution_entries.get(&99) {
        Some(DedustX1000WalletV2ExecutionEntry::StoredMessage(value)) => value,
        _ => anyhow::bail!("dictionary3 marker did not become held message"),
    };
    let transfer: JettonTransferMsg = TLB::from_cell(&message.body.value)?;
    assert_eq!((transfer.query_id, *transfer.amount), (100, 10000));
    assert_eq!(held.to_cell()?.hash()?, held_cell.hash()?);
    let request: Msg = TLB::from_cell(&TonCell::from_boc(
        include_bytes!("fixtures/x1000-controller-request.boc").to_vec(),
    )?)?;
    let requested: v2::msgs::DedustX1000WalletV2ExecutionRequestedMsg = TLB::from_cell(&request.body.value)?;
    assert_eq!(requested.request_id, 99);
    let command = TonCell::from_boc(include_bytes!("fixtures/x1000-controller-execute.boc").to_vec())?;
    let execute: v2::msgs::DedustX1000WalletV2ExecuteStoredMsg = TLB::from_cell(&command)?;
    assert_eq!(execute.request_id, 99);
    assert_eq!(execute.ignored_suffix.data_len_bits(), 3);
    assert_eq!(execute.ignored_suffix.parser().read_num::<u8>(3)?, 5);
    assert_eq!(execute.to_cell()?.hash()?, command.hash()?);
    let post_cell = TonCell::from_boc(include_bytes!("fixtures/x1000-released-message-state.boc").to_vec())?;
    let post: v2::storage::DedustX1000WalletV2Storage = TLB::from_cell(&post_cell)?;
    assert!(!post.runtime_state.execution_entries.contains_key(&99));
    assert_eq!(post.runtime_state.pending_operations, held.runtime_state.pending_operations);
    let outgoing: Msg = TLB::from_cell(&TonCell::from_boc(
        include_bytes!("fixtures/x1000-released-message.boc").to_vec(),
    )?)?;
    assert_eq!(outgoing.body.value.hash()?, message.body.value.hash()?);
    assert_eq!(outgoing.dst(), message.dst());
    Ok(())
}

#[test]
fn test_x1000_all_finite_protocol_builder_outputs_use_real_typed_wire_apis() -> anyhow::Result<()> {
    use crate::contracts::custom_wallet::v2;
    use ton::{block_tlb::Msg, contracts::tep::jetton::jetton_transfer_msg::JettonTransferMsg};

    let rows: Vec<serde_json::Value> = serde_json::from_str(include_str!("fixtures/x1000-context-vectors.json"))?;
    assert_eq!(rows.len(), 22);
    for row in rows {
        let cell = TonCell::from_boc_base64(
            row["message_boc_base64"]
                .as_str()
                .ok_or_else(|| anyhow::anyhow!("missing message"))?,
        )?;
        let msg: Msg = TLB::from_cell(&cell)?;
        assert!(msg.init.is_none());
        let parsed = v2::msgs::DedustX1000WalletV2OutgoingMsg::from_cell(&msg.body.value)?;
        assert!(!matches!(parsed, v2::msgs::DedustX1000WalletV2OutgoingMsg::Unknown(_)));
        assert_eq!(parsed.to_cell()?.hash()?, msg.body.value.hash()?);
        let tag = msg.body.value.parser().read_num::<u32>(32)?;
        assert_eq!(
            u64::from(tag),
            row["outer_opcode"].as_u64().ok_or_else(|| anyhow::anyhow!("missing opcode"))?
        );
        if tag == 0x0f8a7ea5 {
            let transfer: JettonTransferMsg = TLB::from_cell(&msg.body.value)?;
            assert_eq!((transfer.query_id, *transfer.amount), (99, 10000));
            let payload = &transfer.forward_payload.value;
            match row["kind"].as_u64().ok_or_else(|| anyhow::anyhow!("missing kind"))? {
                1 => {
                    let p: v2::msgs::DedustX1000DedustSwapPayloadMsg = TLB::from_cell(payload)
                        .map_err(|e| anyhow::anyhow!("builder kind{} mode{}: {e}", row["kind"], row["field4_1"]))?;
                    assert_eq!(*p.step.limit, 42);
                },
                2 => {
                    let _: v2::msgs::DedustX1000StonfiV2SwapPayloadMsg = TLB::from_cell(payload)
                        .map_err(|e| anyhow::anyhow!("builder kind{} mode{}: {e}", row["kind"], row["field4_1"]))?;
                },
                3 => {
                    let _: v2::msgs::DedustX1000ToncoSwapPayloadMsg = TLB::from_cell(payload)
                        .map_err(|e| anyhow::anyhow!("builder kind{} mode{}: {e}", row["kind"], row["field4_1"]))?;
                },
                4 => {
                    let _: v2::msgs::DedustX1000CpmmPayJettonPayloadMsg = TLB::from_cell(payload)
                        .map_err(|e| anyhow::anyhow!("builder kind{} mode{}: {e}", row["kind"], row["field4_1"]))?;
                },
                5 => {
                    let _: v2::msgs::DedustX1000BidaskDammSwapPayloadMsg = TLB::from_cell(payload)
                        .map_err(|e| anyhow::anyhow!("builder kind{} mode{}: {e}", row["kind"], row["field4_1"]))?;
                },
                6 => {
                    let _: v2::msgs::DedustX1000CoffeeSwapPayloadMsg = TLB::from_cell(payload)
                        .map_err(|e| anyhow::anyhow!("builder kind{} mode{}: {e}", row["kind"], row["field4_1"]))?;
                },
                7 => {
                    let _: v2::msgs::DedustX1000MoonSwapPayloadMsg = TLB::from_cell(payload)
                        .map_err(|e| anyhow::anyhow!("builder kind{} mode{}: {e}", row["kind"], row["field4_1"]))?;
                },
                8 => {
                    let _: v2::msgs::DedustX1000PayloadFa45056bMsg = TLB::from_cell(payload)
                        .map_err(|e| anyhow::anyhow!("builder kind{} mode{}: {e}", row["kind"], row["field4_1"]))?;
                },
                9 => {
                    let p: v2::msgs::DedustX1000MoonClmmSwapPayloadMsg = TLB::from_cell(payload)
                        .map_err(|e| anyhow::anyhow!("builder kind{} mode{}: {e}", row["kind"], row["field4_1"]))?;
                    assert_eq!(p.sqrt_price_limit_x128, U256::from_u8(123));
                },
                10 => {
                    let _: v2::msgs::DedustX1000BidaskClmmSwapPayloadMsg = TLB::from_cell(payload)
                        .map_err(|e| anyhow::anyhow!("builder kind{} mode{}: {e}", row["kind"], row["field4_1"]))?;
                },
                value => anyhow::bail!("unexpected Jetton builder kind {value}"),
            }
        }
    }
    Ok(())
}

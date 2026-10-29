//! ston_minter msgs records.
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::adapters::InternalAddress;
use ton::ton_core::types::tlb_core::adapters::NullableAddress;
use ton::ton_core::{
    TLB,
    cell::{CellBuilder, CellParser, TonCell},
    errors::{TonCoreError, TonCoreResult},
    traits::tlb::{TLB, TLBPrefix},
    types::tlb_core::{TLBCoins, TLBEitherRef, TLBRef},
};
use tonabis_core::adapters::maybe_ref_nullable_address::MaybeRefNullableAddress;
/// Developer buy continuation referenced by factory/minter initialization.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct StonksStonMinterVirtualDevBuyMsg {
    pub ton_amount: TLBCoins,
    #[tlb(adapter = "NullableAddress")]
    pub recipient: Option<TonAddress>,
    pub min_jettons_out: TLBCoins,
}

/// Administrator mint request with a referenced internal-transfer body.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x00000015, bits_len = 32)]
pub struct StonksStonMinterJettonMintMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub recipient: Option<TonAddress>,
    pub forward_ton_amount: TLBCoins,
    pub internal_transfer: TLBRef<TonCell>,
}

/// Burn acknowledgement carrying the owner and excess-value recipient.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x7bdd97de, bits_len = 32)]
pub struct StonksStonMinterJettonBurnNotificationMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    #[tlb(adapter = "NullableAddress")]
    pub owner: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub response_address: Option<TonAddress>,
}

/// TEP-89 wallet discovery request with the include-owner flag.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x2c76b973, bits_len = 32)]
pub struct StonksStonMinterProvideWalletAddressMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub owner_address: Option<TonAddress>,
    pub include_address: bool,
}

/// Replace the minter administrator wire address.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x00000003, bits_len = 32)]
pub struct StonksStonMinterChangeAdminMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub new_admin_address: Option<TonAddress>,
}

/// Replace referenced Jetton metadata.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x00000004, bits_len = 32)]
pub struct StonksStonMinterChangeContentMsg {
    pub query_id: u64,
    pub jetton_content: TLBRef<TonCell>,
}

/// Factory initialization including a mandatory referenced developer-buy record.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xcc1a9802, bits_len = 32)]
pub struct StonksStonMinterVirtualMinterInitMsg {
    pub query_id: u64,
    pub presale_open: bool,
    pub migration_ton_target: TLBCoins,
    pub tons_collected: TLBCoins,
    pub base_jetton_reserve: TLBCoins,
    pub additional_ton_reserve: TLBCoins,
    pub additional_jetton_reserve: TLBCoins,
    pub max_buy_percent: u16,
    pub migration_config: TLBRef<TonCell>,
    pub dev_buy: TLBRef<StonksStonMinterVirtualDevBuyMsg>,
}

/// Older fixed-layout buy request; current forwarding uses `forwarded::BuyMsg`.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xcc1a97ae, bits_len = 32)]
pub struct StonksStonMinterBuyFromPresaleMsg {
    pub query_id: u64,
    pub ton_amount: TLBCoins,
    pub min_jettons_out: TLBCoins,
}

/// Older fixed-layout wallet-to-minter sell notification.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xcc1a97b0, bits_len = 32)]
pub struct StonksStonMinterPresaleSellNotificationForMinterMsg {
    pub query_id: u64,
    pub jetton_amount: TLBCoins,
    #[tlb(adapter = "NullableAddress")]
    pub sell_initiator: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub tons_recipient: Option<TonAddress>,
    pub min_ton_out: TLBCoins,
}

/// Older fixed-layout owner sell request.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xcc1a97af, bits_len = 32)]
pub struct StonksStonMinterAskToPresaleSellMsg {
    pub query_id: u64,
    pub jetton_amount: TLBCoins,
    #[tlb(adapter = "NullableAddress")]
    pub tons_recipient: Option<TonAddress>,
    pub min_ton_out: TLBCoins,
}

/// Internal token transfer with its source-defined context and raw forward tail.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0x178d4519, bits_len = 32)]
pub struct StonksStonMinterJettonInternalTransferMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    #[tlb(adapter = "NullableAddress")]
    pub from_address: Option<TonAddress>,
    #[tlb(adapter = "NullableAddress")]
    pub response_address: Option<TonAddress>,
    pub forward_ton_amount: TLBCoins,
    pub forward_payload: TonCell,
}

/// Excess-value acknowledgement identified by query ID.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xd53276db, bits_len = 32)]
pub struct StonksStonMinterJettonExcessesMsg {
    pub query_id: u64,
}

/// TEP-89 discovery response with optional referenced owner address.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xd1735400, bits_len = 32)]
pub struct StonksStonMinterTakeWalletAddressMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub wallet_address: Option<TonAddress>,
    #[tlb(adapter = "MaybeRefNullableAddress")]
    pub owner_address: Option<Option<TonAddress>>,
}

/// Trade log with input/output amounts, recipient and leftover value.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xcc1a97b3, bits_len = 32)]
pub struct StonksStonMinterPresaleTradeEventMsg {
    pub query_id: u64,
    pub is_buy: bool,
    pub amount_in: TLBCoins,
    pub amount_out: TLBCoins,
    #[tlb(adapter = "NullableAddress")]
    pub recipient: Option<TonAddress>,
    pub leftover: TLBCoins,
}

/// Known typed bodies and losslessly retained unknown messages.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum StonksStonMinterLegacyMessage {
    JettonMint(StonksStonMinterJettonMintMsg),
    JettonBurnNotification(StonksStonMinterJettonBurnNotificationMsg),
    ProvideWalletAddress(StonksStonMinterProvideWalletAddressMsg),
    ChangeAdmin(StonksStonMinterChangeAdminMsg),
    ChangeContent(StonksStonMinterChangeContentMsg),
    VirtualMinterInit(StonksStonMinterVirtualMinterInitMsg),
    BuyFromPresale(StonksStonMinterBuyFromPresaleMsg),
    PresaleSellNotificationForMinter(StonksStonMinterPresaleSellNotificationForMinterMsg),
    AskToPresaleSell(StonksStonMinterAskToPresaleSellMsg),
    JettonInternalTransfer(StonksStonMinterJettonInternalTransferMsg),
    JettonExcesses(StonksStonMinterJettonExcessesMsg),
    TakeWalletAddress(StonksStonMinterTakeWalletAddressMsg),
    PresaleTradeEvent(StonksStonMinterPresaleTradeEventMsg),

    /// Controller initialization without a developer buy.
    ControllerInit(StonksStonMinterControllerInitMsg),
    /// Owner wallet status request.
    WalletStatus(StonksStonMinterWalletStatusRequestMsg),
    /// Controller TON withdrawal.
    Withdraw(StonksStonMinterWithdrawTonMsg),
    /// Controller migration retry.
    Retry(StonksStonMinterRetryMigrationMsg),
    Unknown(TonCell),
}

/// Controller initialization without the factory's optional dev-buy second reference.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xcc1a97b5, bits_len = 32)]
pub struct StonksStonMinterControllerInitMsg {
    /// Correlation id.
    pub query_id: u64,
    /// Initial lifecycle flag.
    pub presale_open: bool,
    /// Target collection.
    pub migration_ton_target: TLBCoins,
    /// Initial physical TON.
    pub tons_collected: TLBCoins,
    /// Issued base reserve.
    pub base_jetton_reserve: TLBCoins,
    /// Virtual TON.
    pub additional_ton_reserve: TLBCoins,
    /// Virtual tokens.
    pub additional_jetton_reserve: TLBCoins,
    /// Per-buy cap.
    pub max_buy_percent: u16,
    /// Referenced target-specific migration config.
    pub migration_config: TLBRef<TonCell>,
}

/// Owner wallet status request, allowed only in the source-defined lifecycle.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xcc1a97b1, bits_len = 32)]
pub struct StonksStonMinterWalletStatusRequestMsg {
    /// Correlation id.
    pub query_id: u64,
    /// Owner whose wallet status should be updated.
    #[tlb(adapter = "NullableAddress")]
    pub owner: Option<TonAddress>,
}

/// Protocol-controller TON withdrawal.
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xcc1a97b8, bits_len = 32)]
pub struct StonksStonMinterWithdrawTonMsg {
    /// Correlation id.
    pub query_id: u64,
    /// Requested TON amount.
    pub amount: TLBCoins,
}

/// Protocol-controller retry of a pending migration (no query id in this layout).
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xcc1a97b9, bits_len = 32)]
pub struct StonksStonMinterRetryMigrationMsg {}

/// Current STON v10/v11 sell notification enforces its preliminary standard-address
/// recipient requirement while preserving the exact optional payload suffix.
#[derive(Debug, Clone, PartialEq)]
pub struct StonksStonMinterCurrentSellNotificationMsg {
    fields: StonksStonMinterSellNotificationMsg,
}

impl StonksStonMinterCurrentSellNotificationMsg {
    /// Validate the revision-specific TON recipient rule.
    pub fn new(fields: StonksStonMinterSellNotificationMsg) -> TonCoreResult<Self> {
        let recipient_cell = fields.tons_recipient.to_cell()?;
        let mut parser = recipient_cell.parser();
        if parser.read_num::<u8>(2)? != 2 {
            ton::ton_core::bail_ton_core_data!("STON v10/v11 requires a standard TON recipient");
        }
        Ok(Self { fields })
    }
    /// The complete source-order fields, including a missing versus present suffix.
    pub fn fields(&self) -> &StonksStonMinterSellNotificationMsg {
        &self.fields
    }
}

impl TLB for StonksStonMinterCurrentSellNotificationMsg {
    const PREFIX: TLBPrefix = TLBPrefix {
        value: 0xcc1a97b0,
        bits_len: 32,
    };
    fn read_definition(p: &mut CellParser) -> TonCoreResult<Self> {
        Self::new(StonksStonMinterSellNotificationMsg::read_definition(p)?)
    }
    fn write_definition(&self, b: &mut CellBuilder) -> TonCoreResult<()> {
        self.fields.write_definition(b)
    }
}

/// Current STON v10/v11 dispatcher with recipient/forwarding rules distinct from
/// the older fixed-message minter interface.
#[derive(Debug, Clone, PartialEq, ton::ton_core::TLB)]
pub enum StonksStonMinterCurrentMessage {
    Buy(StonksStonMinterBuyMsg),
    Sell(StonksStonMinterCurrentSellNotificationMsg),
    JettonMint(StonksStonMinterJettonMintMsg),
    JettonBurnNotification(StonksStonMinterJettonBurnNotificationMsg),
    ProvideWalletAddress(StonksStonMinterProvideWalletAddressMsg),
    ChangeAdmin(StonksStonMinterChangeAdminMsg),
    ChangeContent(StonksStonMinterChangeContentMsg),
    VirtualMinterInit(StonksStonMinterVirtualMinterInitMsg),
    AskToPresaleSell(StonksStonMinterAskToPresaleSellMsg),
    JettonInternalTransfer(StonksStonMinterJettonInternalTransferMsg),
    JettonExcesses(StonksStonMinterJettonExcessesMsg),
    TakeWalletAddress(StonksStonMinterTakeWalletAddressMsg),
    PresaleTradeEvent(StonksStonMinterPresaleTradeEventMsg),
    ControllerInit(StonksStonMinterControllerInitMsg),
    WalletStatus(StonksStonMinterWalletStatusRequestMsg),
    Withdraw(StonksStonMinterWithdrawTonMsg),
    Retry(StonksStonMinterRetryMigrationMsg),
    Unknown(TonCell),
}

/// Current buy layout with source-defined optional recipient/payload suffix.
#[derive(Debug, Clone, PartialEq)]
pub struct StonksStonMinterBuyMsg {
    /// Correlation id.
    pub query_id: u64,
    /// Gross TON request.
    pub ton_amount: TLBCoins,
    /// Token output protection.
    pub min_jettons_out: TLBCoins,
    /// Omitted means original sender. An explicit recipient must be a std address.
    pub recipient: Option<TonAddress>,
    /// Omitted means an empty forwarded payload.
    pub payload: Option<TLBEitherRef<TonCell>>,
}

impl TLB for StonksStonMinterBuyMsg {
    const PREFIX: TLBPrefix = TLBPrefix {
        value: 0xcc1a97ae,
        bits_len: 32,
    };
    fn read_definition(p: &mut CellParser) -> TonCoreResult<Self> {
        let query_id = u64::read(p)?;
        let ton_amount = TLBCoins::read(p)?;
        let min_jettons_out = TLBCoins::read(p)?;
        let recipient = if p.data_bits_left()? > 1 && p.clone().read_num::<u8>(2)? == 2 {
            Some(InternalAddress.read(p)?)
        } else {
            None
        };
        let payload = if p.data_bits_left()? == 0 && p.refs_left() == 0 {
            None
        } else {
            Some(TLBEitherRef::read(p)?)
        };
        p.ensure_empty()?;
        Ok(Self {
            query_id,
            ton_amount,
            min_jettons_out,
            recipient,
            payload,
        })
    }
    fn write_definition(&self, b: &mut CellBuilder) -> TonCoreResult<()> {
        self.query_id.write(b)?;
        self.ton_amount.write(b)?;
        self.min_jettons_out.write(b)?;
        if let Some(recipient) = &self.recipient {
            InternalAddress.write(b, recipient)?;
        }
        if let Some(payload) = &self.payload {
            payload.write(b)?;
        }
        Ok(())
    }
}

/// Current user-to-wallet sell extension (the payload is TON-payout data).
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xcc1a97af, bits_len = 32)]
pub struct StonksStonMinterSellWithPayloadMsg {
    /// Correlation id.
    pub query_id: u64,
    /// Tokens sold.
    pub jetton_amount: TLBCoins,
    /// Optional TON recipient, encoded as a TON address without an extra flag.
    #[tlb(adapter = "NullableAddress")]
    pub tons_recipient: Option<TonAddress>,
    /// Net TON output protection.
    pub min_ton_out: TLBCoins,
    /// Standard Either selector and payout data.
    pub payload: TLBEitherRef<TonCell>,
}

/// Source-defined optional payout tail; omission has no option marker on wire.
#[derive(Debug, Clone, PartialEq, ton::ton_core::TLB)]
#[tlb(prefix = 0xcc1a97b0, bits_len = 32, ensure_empty = true)]
pub struct StonksStonMinterSellNotificationMsg {
    /// Correlation id.
    pub query_id: u64,
    /// Tokens sold.
    pub jetton_amount: TLBCoins,
    /// Original owner.
    #[tlb(adapter = "NullableAddress")]
    pub sell_initiator: Option<TonAddress>,
    /// Optional payout recipient.
    #[tlb(adapter = "NullableAddress")]
    pub tons_recipient: Option<TonAddress>,
    /// Net TON output protection.
    pub min_ton_out: TLBCoins,
    /// Absent suffix or a complete Either payload.
    #[tlb(adapter = "PayoutSuffix")]
    pub payload: Option<TLBEitherRef<TonCell>>,
}

/// User-to-wallet sell with source-defined optional payout suffix.
#[derive(Debug, Clone, PartialEq, ton::ton_core::TLB)]
#[tlb(prefix = 0xcc1a97af, bits_len = 32, ensure_empty = true)]
pub struct StonksStonMinterSellMsg {
    /// Correlation id.
    pub query_id: u64,
    /// Tokens sold.
    pub jetton_amount: TLBCoins,
    /// Optional TON recipient.
    #[tlb(adapter = "NullableAddress")]
    pub tons_recipient: Option<TonAddress>,
    /// Net TON protection.
    pub min_ton_out: TLBCoins,
    /// Omitted suffix or complete Either payload.
    #[tlb(adapter = "PayoutSuffix")]
    pub payload: Option<TLBEitherRef<TonCell>>,
}

/// Validate the required Either selector while retaining source-defined raw tails.
pub(crate) fn validate_tail(cell: &TonCell) -> TonCoreResult<()> {
    let mut p = cell.parser();
    if p.read_bit()? {
        p.read_next_ref()?;
    }
    Ok(())
}

/// A missing suffix has no Maybe bit. A present suffix is a complete Either
/// payload; the enclosing derived record checks that no extra suffix remains.
struct PayoutSuffix;
impl PayoutSuffix {
    fn read(&self, parser: &mut CellParser) -> TonCoreResult<Option<TLBEitherRef<TonCell>>> {
        if parser.data_bits_left()? == 0 && parser.refs_left() == 0 {
            Ok(None)
        } else {
            Ok(Some(TLBEitherRef::read(parser)?))
        }
    }
    fn write(&self, builder: &mut CellBuilder, value: &Option<TLBEitherRef<TonCell>>) -> TonCoreResult<()> {
        if let Some(value) = value {
            value.write(builder)?;
        }
        Ok(())
    }
}

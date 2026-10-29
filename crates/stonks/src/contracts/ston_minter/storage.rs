//! ston_minter storage records.
use ton::block_tlb::StateInit;
use ton::ton_core::TLB;
use ton::ton_core::cell::TonCell;
use ton::ton_core::errors::TonCoreResult;
use ton::ton_core::traits::tlb::TLB as _;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::TLBCoins;
use ton::ton_core::types::tlb_core::TLBRef;
use ton::ton_core::types::tlb_core::adapters::NullableAddress;
/// Shared minter state, with an explicit optional bonding phase.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct StonksVirtualMinterStorage {
    pub total_supply: TLBCoins,
    #[tlb(adapter = "NullableAddress")]
    pub admin_address: Option<TonAddress>,
    pub jetton_content: TLBRef<TonCell>,
    pub jetton_wallet_code: TLBRef<TonCell>,
    pub bonding_state: Option<TLBRef<StonksVirtualMinterBondingState>>,
}

/// Bonding reserves and referenced target-specific launch/migration configurations.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct StonksVirtualMinterBondingState {
    pub presale_open: bool,
    pub migration_ton_target: TLBCoins,
    pub tons_collected: TLBCoins,
    pub base_jetton_reserve: TLBCoins,
    pub additional_ton_reserve: TLBCoins,
    pub additional_jetton_reserve: TLBCoins,
    pub max_buy_percent: u16,
    pub tokens_sold: TLBCoins,
    pub launch_config: TLBRef<TonCell>,
    pub migration_config: TLBRef<TonCell>,
}

/// STON launch state differs from DeDust by omitting the expected-wallet address.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct StonksStonMinterLaunchState {
    /// Migration is active.
    pub active: bool,
    /// Minter has emitted destination funding; settlement is not proven.
    pub completed: bool,
    /// Pending query id.
    pub query_id: u64,
    /// Physical TON committed.
    pub ton_amount: TLBCoins,
    /// Physical unsold tokens committed.
    pub token_amount: TLBCoins,
}

/// Complete source-order STON migration config. The two addresses and TON field
/// are retained independently; this record does not infer destination authority.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct StonksStonMinterMigrationConfig {
    /// First stored destination-specific address.
    #[tlb(adapter = "NullableAddress")]
    pub first_address: Option<TonAddress>,
    /// Stored TON-denominated configuration amount.
    pub ton_amount: TLBCoins,
    /// Second stored destination-specific address.
    #[tlb(adapter = "NullableAddress")]
    pub second_address: Option<TonAddress>,
    /// Referenced executable destination template.
    pub destination_template: TLBRef<TonCell>,
}

impl StonksVirtualMinterStorage {
    /// Factory-created full supply, metadata and code, with no bonding state yet.
    pub fn initial_state(
        code: TonCell,
        factory: Option<TonAddress>,
        content: TonCell,
        wallet_code: TonCell,
    ) -> TonCoreResult<StateInit> {
        Ok(StateInit::new(
            code,
            Self {
                total_supply: TLBCoins::new(100_000_000_000_000_000),
                admin_address: factory,
                jetton_content: TLBRef::new(content),
                jetton_wallet_code: TLBRef::new(wallet_code),
                bonding_state: None,
            }
            .to_cell()?,
        ))
    }
}

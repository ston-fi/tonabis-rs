//! Generated schema records from the pinned TON ABI catalog.
//! Ownership and provenance: tools/reference/dedust/bindings-generation.json; guidance: tools/reference/dedust/README.md.
use crate::contracts::classic::common::DedustClassicOperatorCode;
use crate::contracts::classic::{asset::DedustClassicAsset, common::method_results::DedustClassicVersionedCode};
use std::collections::HashMap;
use ton::{
    tlb_adapters::{DictKeyAdapterUint, DictValAdapterTLB, TLBHashMapE},
    ton_core::{
        TLB,
        cell::{CellBuilder, CellParser, TonCell},
        errors::TonCoreError,
        traits::tlb::TLB,
        types::{
            TonAddress,
            tlb_core::{
                TLBCoins, TLBRef,
                adapters::{InternalAddress, NullableAddress},
            },
        },
    },
};
/// Runtime Factory storage loaded by the verified v2 code root data cell.
#[derive(Debug, Clone, PartialEq)]
pub struct DedustFactoryV2Storage {
    pub owner_address: TonAddress,
    pub pending_owner_address: Option<TonAddress>,
    /// Unix timestamp after which `accept_ownership` can complete.
    pub can_accept_after: u32,
    pub version: u16,
    pub blank_code: TLBRef<TonCell>,
    pub config: TLBRef<DedustFactoryV2Config>,
    /// Optional trailing Operator configuration. Captured V2 loader accepts the older root with no trailing fields; absence remains distinct from version0/null.
    pub operator: Option<DedustClassicOperatorCode>,
}
impl TLB for DedustFactoryV2Storage {
    fn read_definition(p: &mut CellParser) -> Result<Self, TonCoreError> {
        Ok(Self {
            owner_address: InternalAddress.read(p)?,
            pending_owner_address: NullableAddress.read(p)?,
            can_accept_after: p.read_num(32)?,
            version: p.read_num(16)?,
            blank_code: <TLBRef<TonCell>>::read(p)?,
            config: <TLBRef<DedustFactoryV2Config>>::read(p)?,
            operator: if p.data_bits_left()? > 0 {
                Some(DedustClassicOperatorCode::read(p)?)
            } else {
                None
            },
        })
    }
    fn write_definition(&self, b: &mut CellBuilder) -> Result<(), TonCoreError> {
        InternalAddress.write(b, &self.owner_address)?;
        NullableAddress.write(b, &self.pending_owner_address)?;
        b.write_num(&self.can_accept_after, 32)?;
        b.write_num(&self.version, 16)?;
        self.blank_code.write(b)?;
        self.config.write(b)?;
        if let Some(value) = &self.operator {
            value.write(b)?;
        }
        Ok(())
    }
}
/// Referenced Factory child-code configuration cell.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustFactoryV2Config {
    pub pool_code_version: u16,
    pub pool_code: TLBRef<TonCell>,
    pub liquidity_deposit_code_version: u16,
    pub liquidity_deposit_code: TLBRef<TonCell>,
    /// Bounded uint4 dictionary of inline version16/ref-code entries, confirmed by getter129291 DICTUGET4.
    #[tlb(adapter = "TLBHashMapE::<DictKeyAdapterUint<u8>, DictValAdapterTLB<DedustClassicVersionedCode>>::new(4)")]
    pub vault_code_by_asset_type: HashMap<u8, DedustClassicVersionedCode>,
    pub lp_wallet_code: TLBRef<TonCell>,
}
/// Pool identity used in deterministic pool and liquidity-deposit derivation.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustFactoryV2PoolParams {
    pub is_stable: bool,
    pub asset0: DedustClassicAsset,
    pub asset1: DedustClassicAsset,
}
/// Factory proof for a Vault-derived request.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustFactoryV2VaultProof {
    #[tlb(adapter = "InternalAddress")]
    pub factory_address: TonAddress,
    pub contract_type: u8,
    pub asset: DedustClassicAsset,
}
/// Referenced extra values used by `create_liquidity_deposit`.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustFactoryV2CreateLiquidityDepositExtra {
    pub asset0_target_balance: TLBCoins,
    pub asset1_target_balance: TLBCoins,
    pub deposit_asset: DedustClassicAsset,
    pub deposit_amount: TLBCoins,
    pub min_lp_amount: TLBCoins,
}
/// DeDust Factory v2 exit-code catalog.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum DedustFactoryV2Errors {
    /// Incoming administration message sender is not the current Factory owner.
    InvalidOwnerSender = 256,
    /// Ownership acceptance sender is not the pending owner.
    InvalidPendingOwnerSender = 257,
    /// Ownership acceptance delay has not elapsed.
    OwnershipDelayNotElapsed = 258,
    /// Requested child implementation code is not installed.
    MissingChildCode = 260,
    /// Asset tag or proof shape is invalid for this Factory.
    InvalidAssetOrProof = 261,
    /// Pool asset order is invalid.
    InvalidAssetOrder = 262,
    /// Attached value cannot cover the required deployment budget.
    InsufficientValue = 263,
    /// Liquidity-deposit creation sender is not the authenticated Vault.
    InvalidVaultSender = 264,
    /// Replacement code version is not the next version.
    InvalidCodeVersion = 275,
    /// Operator-only request sender is not the deterministic Operator contract.
    InvalidOperatorSender = 296,
    /// Incoming internal message opcode is unsupported.
    UnsupportedOperation = 65535,
}

//! Fixed classic factory code dictionaries and numeric getters. Both captured
//! generations use uint4 asset-type keys and inline uint16/ref-code values.
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::adapters::InternalAddress;
use ton::ton_core::{
    TLB,
    cell::{CellBuilder, CellParser, TonCell},
    errors::TonCoreError,
    traits::tlb::TLB as Tlb,
    types::tlb_core::{TLBCoins, TLBRef},
};

/// Optional trailing V2 configuration; the entire record is absent on legacy
/// factory roots accepted by the captured V2 loader.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustClassicOperatorCode {
    pub version: u16,
    pub code: Option<TLBRef<TonCell>>,
}

/// Both classic pool constructor58662 generations read fee16/^LPcode/decimals8/8.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustClassicPoolParameters {
    pub trade_fee_numerator: u16,
    pub lp_wallet_code: TLBRef<TonCell>,
    pub asset0_decimals: u8,
    pub asset1_decimals: u8,
}
/// Fixed liquidity-deposit initializer. The final LDDICT/DROP ignores a suffix;
/// retain it after the complete known five-field prefix.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustClassicDepositParameters {
    pub min_lp_amount: TLBCoins,
    pub asset0_target_balance: TLBCoins,
    pub asset1_target_balance: TLBCoins,
    pub fulfill_payload: Option<TLBRef<TonCell>>,
    pub reject_payload: Option<TLBRef<TonCell>>,
    pub ignored_suffix: TonCell,
}
/// Native vault constructor58662 drops all initializer arguments before loading
/// its proof. The ignored argument cell is retained byte-for-byte.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustClassicNativeVaultParameters {
    pub ignored_arguments: TonCell,
}
/// Jetton vault constructor58662 tests SBITS: zero bits derives the minter from
/// its proof; otherwise LDMSGADDR/DROP reads one override and ignores the rest.
/// None differs from Some(addr_none). Parsing does not promise valid deployment.
#[derive(Debug, Clone, PartialEq)]
pub struct DedustClassicJettonVaultParameters {
    pub minter_override: Option<Option<TonAddress>>,
    pub ignored_suffix: TonCell,
}
impl Tlb for DedustClassicJettonVaultParameters {
    fn read_definition(p: &mut CellParser) -> Result<Self, TonCoreError> {
        let minter_override = if p.data_bits_left()? > 0 {
            Some(ton::ton_core::types::tlb_core::adapters::NullableAddress.read(p)?)
        } else {
            None
        };
        Ok(Self {
            minter_override,
            ignored_suffix: p.read_remaining()?,
        })
    }
    fn write_definition(&self, b: &mut CellBuilder) -> Result<(), TonCoreError> {
        if let Some(value) = &self.minter_override {
            ton::ton_core::types::tlb_core::adapters::NullableAddress.write(b, value)?;
        } else if self.ignored_suffix.data_len_bits() != 0 {
            return Err(TonCoreError::Custom(
                "absent Jetton initializer override requires zero data bits".into(),
            ));
        }
        b.write_cell(&self.ignored_suffix)
    }
}
/// Operator role comes from its blank proof; the fixed initializer adds its user.
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct DedustClassicOperatorParameters {
    #[tlb(adapter = "InternalAddress")]
    pub user_address: TonAddress,
}

/// V2 factory's fixed blank Operator child proof. The kind byte4 is part of
/// the committed record, distinct from the installed Operator runtime storage.
#[derive(Debug, Clone, PartialEq)]
pub struct DedustClassicOperatorProof {
    pub factory: TonAddress,
    pub role: u8,
}
impl Tlb for DedustClassicOperatorProof {
    fn read_definition(p: &mut CellParser) -> Result<Self, TonCoreError> {
        let factory = InternalAddress.read(p)?;
        let kind = p.read_num::<u8>(8)?;
        if kind != 4 {
            return Err(crate::contracts::utils::invalid_tag(
                "blank Operator proof kind",
                u64::from(kind),
            ));
        }
        Ok(Self {
            factory,
            role: p.read_num(8)?,
        })
    }
    fn write_definition(&self, b: &mut CellBuilder) -> Result<(), TonCoreError> {
        InternalAddress.write(b, &self.factory)?;
        b.write_num(&4u8, 8)?;
        b.write_num(&self.role, 8)
    }
}

pub mod method_results;
pub mod msgs;

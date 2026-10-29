//! Coffee msgs. See tools/reference/swap_coffee/README.md.
use crate::contracts::factory::storage::{CoffeeFactoryCodeCells, CoffeeFactoryCodeDictionaries};
use fastnum::U256;
use std::cmp::Ordering;
use ton::block_tlb::TVMStack;
use ton::errors::TonResult;
use ton::ton_core::types::tlb_core::adapters::InternalAddress;
use ton::ton_core::types::tlb_core::adapters::NullableAddress;
use ton::{
    block_tlb::StateInit,
    ton_core::{
        TLB,
        cell::{CellBuilder, CellParser, TonCell},
        errors::TonCoreResult,
        traits::tlb::TLB,
        types::{
            TonAddress,
            tlb_core::{TLBCoins, TLBRef, adapters::ConstLen},
        },
    },
};
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0b00, bits_len = 2)]
pub struct CoffeeFactoryNativeAsset {}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0b01, bits_len = 2)]
pub struct CoffeeFactoryJettonAsset {
    pub chain: u8,
    pub hash: U256,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0b10, bits_len = 2)]
pub struct CoffeeFactoryExtraAsset {
    pub id: u32,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0b000, bits_len = 3)]
pub struct CoffeeFactoryConstantProductAmm {}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0b001, bits_len = 3)]
pub struct CoffeeFactoryCurveFiStableAmm {}
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeeFactoryPoolParams {
    pub first: CoffeeFactoryAsset,
    pub second: CoffeeFactoryAsset,
    pub amm: CoffeeFactoryAmm,
    pub amm_settings: Option<TLBRef<TonCell>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeeFactoryPublicPoolCreationParams {
    #[tlb(adapter = "InternalAddress")]
    pub recipient: TonAddress,
    #[tlb(bits_len = 1)]
    pub use_recipient_on_failure: i8,
    pub notification_data: Option<TLBRef<TonCell>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeeFactoryPrivatePoolCreationParams {
    #[tlb(bits_len = 1)]
    pub is_active: u8,
    pub extra_settings: Option<TLBRef<TonCell>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeeFactoryPoolCreationParams {
    pub public_params: CoffeeFactoryPublicPoolCreationParams,
    pub private_params: CoffeeFactoryPrivatePoolCreationParams,
}
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeeFactoryNotificationDataSingle {
    #[tlb(adapter = "InternalAddress")]
    pub receiver: TonAddress,
    pub fwd_gas: TLBCoins,
    pub payload: TLBRef<TonCell>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeeFactoryNotificationData {
    pub on_success: Option<TLBRef<CoffeeFactoryNotificationDataSingle>>,
    pub on_failure: Option<TLBRef<CoffeeFactoryNotificationDataSingle>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0b00, bits_len = 2)]
pub struct CoffeeFactoryDepositNoCondition {}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0b01, bits_len = 2)]
pub struct CoffeeFactoryDepositLpQuantityCondition {
    pub min_lp_amount: TLBCoins,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0b10, bits_len = 2)]
pub struct CoffeeFactoryDepositReservesRatioCondition {
    pub denominator: u16,
    pub min_nominator: u16,
    pub max_nominator: u16,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0b11, bits_len = 2)]
pub struct CoffeeFactoryDepositComplexCondition {
    pub min_lp_amount: TLBCoins,
    pub denominator: u16,
    pub min_nominator: u16,
    pub max_nominator: u16,
}
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeeFactoryDepositLiquidityParamsTrimmed {
    #[tlb(adapter = "InternalAddress")]
    pub recipient: TonAddress,
    #[tlb(bits_len = 1)]
    pub use_recipient_on_failure: i8,
    #[tlb(adapter = "NullableAddress")]
    pub referral: Option<TonAddress>,
    pub deadline: u32,
    pub condition: CoffeeFactoryDepositLiquidityCondition,
    pub extra_settings: Option<TLBRef<TonCell>>,
    pub notification_data: Option<TLBRef<CoffeeFactoryNotificationData>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeeFactoryDepositLiquidityParams {
    pub params: TLBRef<CoffeeFactoryDepositLiquidityParamsTrimmed>,
    pub pool_params: TLBRef<CoffeeFactoryPoolParams>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeeFactoryContractUpdate {
    pub code: Option<TLBRef<TonCell>>,
    pub data: Option<TLBRef<TonCell>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0b00, bits_len = 2)]
pub struct CoffeeFactoryPoolNoUpdate {}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0b01, bits_len = 2)]
pub struct CoffeeFactoryPoolFeeUpdate {
    pub protocol_fee: u16,
    pub lp_fee: u16,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0b10, bits_len = 2)]
pub struct CoffeeFactoryPoolStatusUpdate {
    #[tlb(bits_len = 1)]
    pub is_active: u8,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0b11, bits_len = 2)]
pub struct CoffeeFactoryPoolFeeAndStatusUpdate {
    pub protocol_fee: u16,
    pub lp_fee: u16,
    #[tlb(bits_len = 1)]
    pub is_active: u8,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0b00, bits_len = 2)]
pub struct CoffeeFactoryVaultContractType {}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0b01, bits_len = 2)]
pub struct CoffeeFactoryPoolContractType {}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0b10, bits_len = 2)]
pub struct CoffeeFactoryLiquidityDepositoryContractType {}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0b11, bits_len = 2)]
pub struct CoffeeFactoryPoolCreatorContractType {}
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeeFactoryVaultProof {
    #[tlb(adapter = "InternalAddress")]
    pub factory_address: TonAddress,
    pub contract_type: CoffeeFactoryVaultContractType,
    pub asset: CoffeeFactoryAsset,
}
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeeFactoryPoolCreatorProof {
    #[tlb(adapter = "InternalAddress")]
    pub factory_address: TonAddress,
    pub contract_type: CoffeeFactoryPoolCreatorContractType,
    #[tlb(adapter = "InternalAddress")]
    pub owner: TonAddress,
    pub params: TLBRef<CoffeeFactoryPoolParams>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee23, bits_len = 32)]
pub struct CoffeeFactoryCreatePoolCreatorRequestMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    pub params: CoffeeFactoryPoolParams,
    pub creation_params: TLBRef<CoffeeFactoryPoolCreationParams>,
    #[tlb(adapter = "InternalAddress")]
    pub sender: TonAddress,
    pub proof: TLBRef<CoffeeFactoryVaultProof>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee27, bits_len = 32)]
pub struct CoffeeFactoryCreateLiquidityDepositoryRequestMsg {
    pub query_id: u64,
    pub amount: TLBCoins,
    pub params: CoffeeFactoryDepositLiquidityParams,
    #[tlb(adapter = "InternalAddress")]
    pub sender: TonAddress,
    pub proof: TLBRef<CoffeeFactoryVaultProof>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee25, bits_len = 32)]
pub struct CoffeeFactoryCreatePoolRequestMsg {
    pub query_id: u64,
    pub amount1: TLBCoins,
    pub amount2: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub tx_initiator: TonAddress,
    pub creation_params: TLBRef<CoffeeFactoryPoolCreationParams>,
    pub proof: TLBRef<CoffeeFactoryPoolCreatorProof>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee06, bits_len = 32)]
pub struct CoffeeFactoryCreateVaultMsg {
    pub query_id: u64,
    pub asset: CoffeeFactoryAsset,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee40, bits_len = 32)]
pub struct CoffeeFactoryUpdateAdminMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub address: Option<TonAddress>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee46, bits_len = 32)]
pub struct CoffeeFactoryUpdateWithdrawerMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub address: Option<TonAddress>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee44, bits_len = 32)]
pub struct CoffeeFactoryUpdateCodeCellsMsg {
    pub query_id: u64,
    pub first: TLBRef<CoffeeFactoryCodeCells>,
    pub second: TLBRef<CoffeeFactoryCodeDictionaries>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee45, bits_len = 32)]
pub struct CoffeeFactoryUpdateContractMsg {
    pub query_id: u64,
    #[tlb(adapter = "NullableAddress")]
    pub contract_address: Option<TonAddress>,
    pub update: CoffeeFactoryContractUpdate,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee41, bits_len = 32)]
pub struct CoffeeFactoryUpdatePoolMsg {
    pub query_id: u64,
    pub pool_params: CoffeeFactoryPoolParams,
    pub params: TLBRef<CoffeeFactoryPoolUpdateParams>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee42, bits_len = 32)]
pub struct CoffeeFactoryActivateVaultMsg {
    pub query_id: u64,
    pub asset: CoffeeFactoryAsset,
    #[tlb(adapter = "InternalAddress")]
    pub wallet: TonAddress,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee43, bits_len = 32)]
pub struct CoffeeFactoryWithdrawMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub pool_address: TonAddress,
    pub asset: CoffeeFactoryAsset,
    pub amount: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub receiver: TonAddress,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee22, bits_len = 32)]
pub struct CoffeeFactoryDeployMsg {
    pub query_id: u64,
    pub code: TLBRef<TonCell>,
    pub data: TLBRef<TonCell>,
    pub action: Option<TLBRef<CoffeeFactoryDeployAction>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee24, bits_len = 32)]
pub struct CoffeeFactoryCreatePoolCreatorInternalMsg {
    pub query_id: u64,
    pub asset: CoffeeFactoryAsset,
    pub amount: TLBCoins,
    pub creation_params: TLBRef<CoffeeFactoryPoolCreationParams>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee28, bits_len = 32)]
pub struct CoffeeFactoryCreateLiquidityDepositoryInternalMsg {
    pub query_id: u64,
    pub asset: CoffeeFactoryAsset,
    pub amount: TLBCoins,
    pub params: TLBRef<CoffeeFactoryDepositLiquidityParamsTrimmed>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee26, bits_len = 32)]
pub struct CoffeeFactoryCreatePoolInternalMsg {
    pub query_id: u64,
    pub amount1: TLBCoins,
    pub amount2: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub tx_initiator: TonAddress,
    #[tlb(adapter = "InternalAddress")]
    pub recipient: TonAddress,
    #[tlb(bits_len = 1)]
    pub use_recipient_on_failure: i8,
    pub extra_settings: Option<TLBRef<TonCell>>,
    pub notification_data: Option<TLBRef<CoffeeFactoryNotificationData>>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee2a, bits_len = 32)]
pub struct CoffeeFactoryCreateVaultInternalMsg {
    pub query_id: u64,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee2b, bits_len = 32)]
pub struct CoffeeFactoryUpdatePoolInternalMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub excesses_receiver: TonAddress,
    pub params: TLBRef<CoffeeFactoryPoolUpdateParams>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee2c, bits_len = 32)]
pub struct CoffeeFactoryActivateVaultInternalMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub wallet: TonAddress,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee2d, bits_len = 32)]
pub struct CoffeeFactoryWithdrawInternalMsg {
    pub query_id: u64,
    pub asset: CoffeeFactoryAsset,
    pub amount: TLBCoins,
    #[tlb(adapter = "InternalAddress")]
    pub receiver: TonAddress,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xc0ffee2e, bits_len = 32)]
pub struct CoffeeFactoryUpdateContractInternalMsg {
    pub query_id: u64,
    #[tlb(adapter = "InternalAddress")]
    pub excesses_receiver: TonAddress,
    pub update: CoffeeFactoryContractUpdate,
}
#[derive(Debug, Clone, PartialEq, TLB)]
#[tlb(prefix = 0xd53276db, bits_len = 32)]
pub struct CoffeeFactoryExcessesMsg {
    pub query_id: u64,
}
/// Distinct dispatch role; malformed recognized bodies fail.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum CoffeeFactoryAsset {
    NativeAsset(Box<CoffeeFactoryNativeAsset>),
    JettonAsset(Box<CoffeeFactoryJettonAsset>),
    ExtraAsset(Box<CoffeeFactoryExtraAsset>),
}
/// Distinct dispatch role; malformed recognized bodies fail.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum CoffeeFactoryAmm {
    ConstantProductAmm(Box<CoffeeFactoryConstantProductAmm>),
    CurveFiStableAmm(Box<CoffeeFactoryCurveFiStableAmm>),
}
/// Distinct dispatch role; malformed recognized bodies fail.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum CoffeeFactoryDepositLiquidityCondition {
    DepositNoCondition(Box<CoffeeFactoryDepositNoCondition>),
    DepositLpQuantityCondition(Box<CoffeeFactoryDepositLpQuantityCondition>),
    DepositReservesRatioCondition(Box<CoffeeFactoryDepositReservesRatioCondition>),
    DepositComplexCondition(Box<CoffeeFactoryDepositComplexCondition>),
}
/// Distinct dispatch role; malformed recognized bodies fail.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum CoffeeFactoryPoolUpdateParams {
    PoolNoUpdate(Box<CoffeeFactoryPoolNoUpdate>),
    PoolFeeUpdate(Box<CoffeeFactoryPoolFeeUpdate>),
    PoolStatusUpdate(Box<CoffeeFactoryPoolStatusUpdate>),
    PoolFeeAndStatusUpdate(Box<CoffeeFactoryPoolFeeAndStatusUpdate>),
}
/// Distinct dispatch role; malformed recognized bodies fail.
#[derive(Debug, Clone, PartialEq, TLB)]
pub enum CoffeeFactoryDeployAction {
    CreatePoolCreatorInternal(Box<CoffeeFactoryCreatePoolCreatorInternalMsg>),
    CreateLiquidityDepositoryInternal(Box<CoffeeFactoryCreateLiquidityDepositoryInternalMsg>),
    CreatePoolInternal(Box<CoffeeFactoryCreatePoolInternalMsg>),
    CreateVaultInternal(Box<CoffeeFactoryCreateVaultInternalMsg>),
}
/// Distinct dispatch role; malformed recognized bodies fail.
#[derive(Debug, Clone, PartialEq)]
pub enum CoffeeFactoryIncomingMessage {
    CreatePoolCreatorRequest(Box<CoffeeFactoryCreatePoolCreatorRequestMsg>),
    CreateLiquidityDepositoryRequest(Box<CoffeeFactoryCreateLiquidityDepositoryRequestMsg>),
    CreatePoolRequest(Box<CoffeeFactoryCreatePoolRequestMsg>),
    CreateVault(Box<CoffeeFactoryCreateVaultMsg>),
    UpdateAdmin(Box<CoffeeFactoryUpdateAdminMsg>),
    UpdateWithdrawer(Box<CoffeeFactoryUpdateWithdrawerMsg>),
    UpdateCodeCells(Box<CoffeeFactoryUpdateCodeCellsMsg>),
    UpdateContract(Box<CoffeeFactoryUpdateContractMsg>),
    UpdatePool(Box<CoffeeFactoryUpdatePoolMsg>),
    ActivateVault(Box<CoffeeFactoryActivateVaultMsg>),
    Withdraw(Box<CoffeeFactoryWithdrawMsg>),
    Unknown(TonCell),
}
impl ton::ton_core::traits::tlb::TLB for CoffeeFactoryIncomingMessage {
    fn read_definition(p: &mut CellParser) -> TonCoreResult<Self> {
        if p.data_bits_left()? >= 32 && p.lookup_bits(32)? == 0xc0ffee23 {
            return Ok(Self::CreatePoolCreatorRequest(Box::new(
                CoffeeFactoryCreatePoolCreatorRequestMsg::read(p)?,
            )));
        }
        if p.data_bits_left()? >= 32 && p.lookup_bits(32)? == 0xc0ffee27 {
            return Ok(Self::CreateLiquidityDepositoryRequest(Box::new(
                CoffeeFactoryCreateLiquidityDepositoryRequestMsg::read(p)?,
            )));
        }
        if p.data_bits_left()? >= 32 && p.lookup_bits(32)? == 0xc0ffee25 {
            return Ok(Self::CreatePoolRequest(Box::new(CoffeeFactoryCreatePoolRequestMsg::read(p)?)));
        }
        if p.data_bits_left()? >= 32 && p.lookup_bits(32)? == 0xc0ffee06 {
            return Ok(Self::CreateVault(Box::new(CoffeeFactoryCreateVaultMsg::read(p)?)));
        }
        if p.data_bits_left()? >= 32 && p.lookup_bits(32)? == 0xc0ffee40 {
            return Ok(Self::UpdateAdmin(Box::new(CoffeeFactoryUpdateAdminMsg::read(p)?)));
        }
        if p.data_bits_left()? >= 32 && p.lookup_bits(32)? == 0xc0ffee46 {
            return Ok(Self::UpdateWithdrawer(Box::new(CoffeeFactoryUpdateWithdrawerMsg::read(p)?)));
        }
        if p.data_bits_left()? >= 32 && p.lookup_bits(32)? == 0xc0ffee44 {
            return Ok(Self::UpdateCodeCells(Box::new(CoffeeFactoryUpdateCodeCellsMsg::read(p)?)));
        }
        if p.data_bits_left()? >= 32 && p.lookup_bits(32)? == 0xc0ffee45 {
            return Ok(Self::UpdateContract(Box::new(CoffeeFactoryUpdateContractMsg::read(p)?)));
        }
        if p.data_bits_left()? >= 32 && p.lookup_bits(32)? == 0xc0ffee41 {
            return Ok(Self::UpdatePool(Box::new(CoffeeFactoryUpdatePoolMsg::read(p)?)));
        }
        if p.data_bits_left()? >= 32 && p.lookup_bits(32)? == 0xc0ffee42 {
            return Ok(Self::ActivateVault(Box::new(CoffeeFactoryActivateVaultMsg::read(p)?)));
        }
        if p.data_bits_left()? >= 32 && p.lookup_bits(32)? == 0xc0ffee43 {
            return Ok(Self::Withdraw(Box::new(CoffeeFactoryWithdrawMsg::read(p)?)));
        }
        Ok(Self::Unknown(p.read_remaining()?))
    }
    fn write_definition(&self, b: &mut CellBuilder) -> TonCoreResult<()> {
        match self {
            Self::CreatePoolCreatorRequest(value) => value.write(b),
            Self::CreateLiquidityDepositoryRequest(value) => value.write(b),
            Self::CreatePoolRequest(value) => value.write(b),
            Self::CreateVault(value) => value.write(b),
            Self::UpdateAdmin(value) => value.write(b),
            Self::UpdateWithdrawer(value) => value.write(b),
            Self::UpdateCodeCells(value) => value.write(b),
            Self::UpdateContract(value) => value.write(b),
            Self::UpdatePool(value) => value.write(b),
            Self::ActivateVault(value) => value.write(b),
            Self::Withdraw(value) => value.write(b),
            Self::Unknown(value) => value.write(b),
        }
    }
}
/// Distinct dispatch role; malformed recognized bodies fail.
#[derive(Debug, Clone, PartialEq, ton::ton_core::TLB)]
pub enum CoffeeFactoryOutgoingMessage {
    Deploy(CoffeeFactoryDeployMsg),
    CreatePoolCreatorInternal(CoffeeFactoryCreatePoolCreatorInternalMsg),
    CreateLiquidityDepositoryInternal(CoffeeFactoryCreateLiquidityDepositoryInternalMsg),
    CreatePoolInternal(CoffeeFactoryCreatePoolInternalMsg),
    CreateVaultInternal(CoffeeFactoryCreateVaultInternalMsg),
    UpdatePoolInternal(CoffeeFactoryUpdatePoolInternalMsg),
    ActivateVaultInternal(CoffeeFactoryActivateVaultInternalMsg),
    WithdrawInternal(CoffeeFactoryWithdrawInternalMsg),
    UpdateContractInternal(CoffeeFactoryUpdateContractInternalMsg),
    Excesses(CoffeeFactoryExcessesMsg),

    Unknown(TonCell),
}

// Caller-code identity construction; parsing is not authentication.
impl CoffeeFactoryVaultProof {
    /// Exact code/data StateInit; caller supplies the factory's Init code.
    pub fn state_init(&self, init_code: &TonCell) -> TonCoreResult<StateInit> {
        Ok(StateInit::new(init_code.clone(), self.to_cell()?))
    }
    /// Basechain identity address; this does not prove factory admission.
    pub fn derive_basechain_address(&self, init_code: &TonCell) -> TonCoreResult<TonAddress> {
        self.state_init(init_code)?.derive_address(0)
    }
}
impl CoffeeFactoryPoolCreatorProof {
    /// Exact code/data StateInit; caller supplies the factory's Init code.
    pub fn state_init(&self, init_code: &TonCell) -> TonCoreResult<StateInit> {
        Ok(StateInit::new(init_code.clone(), self.to_cell()?))
    }
    /// Basechain identity address; this does not prove factory admission.
    pub fn derive_basechain_address(&self, init_code: &TonCell) -> TonCoreResult<TonAddress> {
        self.state_init(init_code)?.derive_address(0)
    }
}

// Canonical tagged asset order, separate from code authentication.
impl CoffeeFactoryAsset {
    /// Sorts tags, unsigned Jetton chain bytes/hashes and full uint32 extra IDs.
    /// Returns the ordered pair and whether it was reversed; identical assets fail.
    pub fn canonical_pair(pair: [Self; 2]) -> TonCoreResult<([Self; 2], bool)> {
        fn key(asset: &CoffeeFactoryAsset) -> (u8, u8, U256, u32) {
            match asset {
                CoffeeFactoryAsset::NativeAsset(_) => (0, 0, U256::ZERO, 0),
                CoffeeFactoryAsset::JettonAsset(v) => (1, v.chain, v.hash, 0),
                CoffeeFactoryAsset::ExtraAsset(v) => (2, 0, U256::ZERO, v.id),
            }
        }
        let [first, second] = pair;
        match key(&first).cmp(&key(&second)) {
            Ordering::Less => Ok(([first, second], false)),
            Ordering::Greater => Ok(([second, first], true)),
            Ordering::Equal => Err(crate::contracts::wire::wire_error("pool assets must differ")),
        }
    }
}

impl ton::block_tlb::ToTVMStack for CoffeeFactoryAsset {
    fn to_stack(&self, stack: &mut TVMStack) -> TonResult<()> {
        stack.push_cell_slice(self.to_cell()?);
        Ok(())
    }
}

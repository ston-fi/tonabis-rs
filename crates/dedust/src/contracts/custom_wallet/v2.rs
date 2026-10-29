//! DedustX1000WalletV2 contract API. Data records live in [`storage`].
use self::storage::DedustX1000WalletV2Storage;
use fastnum::{I512, U256};
use ton::{
    contracts::TonContract,
    errors::TonResult,
    ton_contract,
    ton_core::{ton_methods, types::Coins},
};

pub mod storage;
ton_contract!(DedustX1000WalletV2Contract<DedustX1000WalletV2Storage>);

pub mod msgs;

#[ton_methods]
impl DedustX1000WalletV2Contract {
    /// Sequence number expected in the next accepted signed request.
    pub async fn seqno(&self) -> TonResult<u32>;
    /// Selector 83648; returns the current balance in nanograms.
    #[ton_method(id = 83648)]
    pub async fn get_balance(&self) -> TonResult<Coins>;
    /// Selector 85802; reports whether the first runtime dictionary is empty.
    #[ton_method(id = 85802)]
    pub async fn is_pending_operations_empty(&self) -> TonResult<bool>;
    /// Numeric getter89828 returns the executable implementation-code cell hash (HASHCU), not an application-state hash.
    #[ton_method(id = 89828)]
    pub async fn get_implementation_code_hash(&self) -> TonResult<U256>;
    #[ton_method(id = 91459)]
    pub async fn get_stored_seqno(&self) -> TonResult<u32>;
    /// Selector 94056; reports whether the third runtime dictionary is empty.
    #[ton_method(id = 94056)]
    pub async fn is_execution_entries_empty(&self) -> TonResult<bool>;
    pub async fn storage_version(&self) -> TonResult<u8>;
    #[ton_method(id = 97027)]
    pub async fn get_counter(&self) -> TonResult<u32>;
    /// Numeric getter97376 returns1 when the first runtime dictionary contains the supplied uint64 key, otherwise0; DICTUGET/IFNOTJMP source order corrects reversed catalog prose.
    #[ton_method(id = 97376)]
    pub async fn get_pending_operation_presence(&self, key: I512) -> TonResult<u8>;
    /// Selector 98121; reports whether the second runtime dictionary is empty.
    #[ton_method(id = 98121)]
    pub async fn is_reserved_values_empty(&self) -> TonResult<bool>;
    /// Selector 118999; reports whether the first runtime dictionary is empty.
    #[ton_method(id = 118999)]
    pub async fn has_no_pending_operations(&self) -> TonResult<bool>;
}

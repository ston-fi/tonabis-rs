//! Persistent TL-B records and local caller-code StateInit construction. No deployment is performed.
use crate::contracts::native::wallet::storage::ToncoNativeWalletStorage;
use ton::block_tlb::StateInit;
use ton::ton_core::TLB;
use ton::ton_core::cell::TonCell;
use ton::ton_core::errors::TonCoreResult;
use ton::ton_core::traits::tlb::TLB;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::TLBCoins;
use ton::ton_core::types::tlb_core::TLBRef;

#[derive(Debug, Clone, PartialEq, TLB)]
pub struct ToncoNativeMinterStorage {
    pub version: u32,
    pub wallet_code: TLBRef<TonCell>,
    pub content: TLBRef<TonCell>,
}

impl ToncoNativeMinterStorage {
    /// Caller owns code identity and workchain selection.
    pub fn derive_wallet_address(
        &self,
        owner: TonAddress,
        minter: TonAddress,
        workchain: i32,
    ) -> TonCoreResult<TonAddress> {
        let data = ToncoNativeWalletStorage {
            balance: TLBCoins::new(0),
            owner,
            minter,
        }
        .to_cell()?;
        StateInit::new(self.wallet_code.clone(), data).derive_address(workchain)
    }

    /// Construct data and StateInit with explicit caller-selected code; sends nothing.
    pub fn state_init(&self, code: TonCell) -> TonCoreResult<StateInit> {
        Ok(StateInit::new(code, self.to_cell()?))
    }
    /// Caller owns code identity and workchain policy.
    pub fn derive_address(&self, code: TonCell, workchain: i32) -> TonCoreResult<TonAddress> {
        self.state_init(code)?.derive_address(workchain)
    }
}

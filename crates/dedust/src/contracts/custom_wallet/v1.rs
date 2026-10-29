//! DedustX1000WalletV1 contract API. Data records live in [`storage`].
use self::storage::DedustX1000WalletV1Storage;
use ton::ton_contract;

pub mod storage;
ton_contract!(DedustX1000WalletV1Contract<DedustX1000WalletV1Storage>);
pub mod msgs;

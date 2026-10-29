//! DedustUranusFactoryV3 contract API. Data records live in [`storage`].
use self::storage::DedustUranusFactoryV3Storage;
use ton::ton_contract;

pub mod storage;
ton_contract!(DedustUranusFactoryV3Contract<DedustUranusFactoryV3Storage>);
pub mod msgs;

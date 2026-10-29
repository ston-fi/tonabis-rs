//! V1 factory order child. Storage and wire callbacks are supported; this revision has no getters.
use crate::contracts::v1::order::storage::ToncoOrderV1Storage;
pub mod msgs;
pub mod storage;

use ton::ton_contract;

ton_contract!(ToncoOrderV1Contract<ToncoOrderV1Storage>);

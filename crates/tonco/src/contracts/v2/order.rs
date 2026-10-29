//! V2 factory order child. Storage and wire callbacks are supported; this revision has no getters.
use crate::contracts::v2::order::storage::ToncoOrderV2Storage;
pub mod msgs;
pub mod storage;

use ton::ton_contract;

ton_contract!(ToncoOrderV2Contract<ToncoOrderV2Storage>);

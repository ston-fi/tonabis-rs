//! Shared replay dictionary value used by both observed Coffee MEV generations.
use ton::ton_core::TLB;
/// Replay presence marker: the dictionary value contains no bits or references.
/// The unsigned64 key is the query id; presence, rather than a stored boolean,
/// denotes an executed query. Ordinary dictionary tails are tolerated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, TLB)]
pub struct CoffeeMevReplayMarker {}

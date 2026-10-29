//! Generated Coffee storage. See tools/reference/swap_coffee/README.md.
use ton::ton_core::TLB;
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::adapters::NullableAddress;
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeeCrossDexStorage {
    #[tlb(adapter = "NullableAddress")]
    pub admin_address: Option<TonAddress>,
}
#[derive(Debug, Clone, PartialEq, TLB)]
pub struct CoffeeCrossDexDeploymentStorage {
    #[tlb(adapter = "NullableAddress")]
    pub admin_address: Option<TonAddress>,
    pub deployment_discriminator: u32,
}

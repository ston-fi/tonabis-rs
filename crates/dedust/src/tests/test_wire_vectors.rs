//! Literal captured wire vectors transcribed from pinned catalog tests.
use crate::contracts::classic::liquidity_deposit_v1::msgs::DedustLiquidityDepositV1DepositAssetMsg;
use crate::contracts::classic::liquidity_deposit_v1::msgs::DedustLiquidityDepositV1DepositLiquidityAllMsg;
use crate::contracts::classic::liquidity_deposit_v1::msgs::DedustLiquidityDepositV1FulfillDepositMsg;
use crate::contracts::classic::liquidity_deposit_v1::msgs::DedustLiquidityDepositV1InstallMsg;
use crate::contracts::classic::liquidity_deposit_v1::msgs::DedustLiquidityDepositV1PayOutFromDepositMsg;
use crate::contracts::classic::pool_v2::msgs::DedustPoolV2ConfigureTradeFeeMsg;
use crate::contracts::classic::pool_v2::msgs::DedustPoolV2PayOutFromPoolMsg;
use crate::contracts::classic::pool_v2::msgs::DedustPoolV2ResetGasMsg;
use crate::contracts::classic::pool_v2::msgs::DedustPoolV2SwapEventMsg;
use crate::contracts::classic::pool_v2::msgs::DedustPoolV2SwapExternalMsg;
use crate::contracts::classic::pool_v2::msgs::DedustPoolV2SwapPeerMsg;
use crate::contracts::classic::vault_jetton_v2::msgs::DedustVaultJettonV2JettonTransferMsg;
use crate::contracts::classic::vault_jetton_v2::msgs::DedustVaultJettonV2PayOutFromPoolMsg;
use crate::contracts::classic::vault_jetton_v2::msgs::DedustVaultJettonV2SwapExternalMsg;
use crate::contracts::classic::vault_jetton_v2::msgs::DedustVaultJettonV2TransferNotificationMsg;
use crate::contracts::cpmm_v2::pool::msgs::DedustV2CpmmPayNativeMsg;
use crate::contracts::cpmm_v2::pool::msgs::DedustV2CpmmSwapEventMsg;
use crate::contracts::cpmm_v2::pool::storage::DedustV2CpmmStorage;
use crate::contracts::custom_wallet::v2::msgs::DedustX1000StonksPumpSellMsg;
use crate::contracts::custom_wallet::v2::msgs::DedustX1000UranusSellMsg;
use crate::contracts::custom_wallet::v2::msgs::DedustX1000WalletV2Event10937847Msg;
use crate::contracts::custom_wallet::v2::msgs::DedustX1000WalletV2EventBeaf9617Msg;
use crate::contracts::custom_wallet::v2::msgs::DedustX1000WalletV2ExecuteStoredMsg;
use crate::contracts::custom_wallet::v2::msgs::DedustX1000WalletV2ExecutionRequestedMsg;
use crate::contracts::custom_wallet::v2::msgs::DedustX1000WalletV2JettonTransferMsg;
use crate::contracts::custom_wallet::v2::msgs::DedustX1000WalletV2SignedRequestMsg;
use crate::contracts::uranus::factory_v3::msgs::DedustUranusFactoryV3DeployCustomizedMemeMsg;
use crate::contracts::uranus::factory_v3::msgs::DedustUranusFactoryV3DeployMemeMsg;
use crate::contracts::uranus::factory_v3::storage::DedustUranusFactoryV3Storage;
use crate::contracts::uranus::meme_v2::msgs::DedustUranusMemeV2BuyEventMsg;
use crate::contracts::uranus::meme_v2::msgs::DedustUranusMemeV2SellEventMsg;
use crate::contracts::uranus::meme_v3::msgs::DedustUranusMemeV3BuyEventMsg;
use crate::contracts::uranus::meme_v3::msgs::DedustUranusMemeV3BuyMsg;
use crate::contracts::uranus::meme_v3::msgs::DedustUranusMemeV3Opcode66c8ad72Msg;
use crate::contracts::uranus::meme_v3::msgs::DedustUranusMemeV3SellEventMsg;
use crate::contracts::uranus::meme_v3::storage::DedustUranusMemeV3Storage;
use crate::contracts::uranus::wallet_v3::msgs::DedustUranusMemeWalletV3SellTokensMsg;
use crate::contracts::uranus::wallet_v3::storage::DedustUranusMemeWalletV3Storage;
use ton::ton_core::{cell::TonCell, traits::tlb::TLB};

#[test]
fn test_wire_000() -> anyhow::Result<()> {
    let body = TonCell::from_boc_base64(
        "te6cckEBAwEAkQACI6Wny/iN2V4BAHrmz1AdzWUACAECACHEQlAPcFk91gAqXHAGonZY4QDLgBZoWGILz/BWQHkWxpyThZsEDXJRvnBXfBm2mPrww8xPoEALNCwxBef4KyA8i2NOScLNgga5KN84K74M20x9eGHmJ9AgBZoWGILz/BWQHkWxpyThZsEDXJRvnBXfBm2mPrww8xPst9DDeQ==",
    )?;
    let decoded: DedustV2CpmmPayNativeMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 10221304187690084047);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_wire_001() -> anyhow::Result<()> {
    let body = TonCell::from_boc_base64(
        "te6cckEBBAEAoAACp3jnm6SoDuaygAODGUIY5QbEQAs0LDEF5/grIDyLY05Jws2CBrko3zgrvgzbTH14YeYn2AFmhYYgvP8FZAeRbGnJOFmwQNclG+cFd8GbaY+vDDzE+wECAS1gEwvpwF9oA/ngt03Q0dBwIs8xF/+LKAMAG6APG5MKAPG5MB+NyYAEADdQf7XF0ABQ/2uLxRBwOrOV3OkWiPljufrS7hggwwZ3Ww==",
    )?;
    let decoded: DedustV2CpmmSwapEventMsg = TLB::from_cell(&body)?;
    assert!(decoded.x_to_y);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_wire_002() -> anyhow::Result<()> {
    let body = TonCell::from_boc_base64(
        "te6cckEBBAEAjgACoXjnm6QgCBOBCoMUlu1BQAB+DGNRG6j+xoNsCZKGb6AhuAWD7BBuZVQVz2Z9FKlzeABNTqRk8IeOQPwD/ZYlzklW5WOmC0bf7dYjA1q3bVRPZQECASVi9eIkm0tlB7BTWlhgExWBzykYAwAPGA5UUBYkGAQALQR/e9S1AAPAQAg3iEjCk4TV3er15wzgV7b36Q==",
    )?;
    let decoded: DedustV2CpmmSwapEventMsg = TLB::from_cell(&body)?;
    assert!(!decoded.x_to_y);
    assert_eq!(*decoded.amount_in, 16936993);
    assert_eq!(*decoded.amount_out, 26460674690);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_wire_003() -> anyhow::Result<()> {
    let body = TonCell::from_boc_base64(
        "te6ccgECCgEAAY0AAy9bgRZ5iL/8WTAJMvUX4GQCDuBMScpKCMABAgMCjyAB0c2lR6nbkJ5ApuKB/dvRAAsWDZloRHQcbaEVM2BGDVQA7uAIk//ySrqk9GZ43tEaFyEDD3I+LiBmGZnt1CuIRZQAZBOIlgQFAC1QzoqEowAAcF7vmU5TDbcFYZ51OZggIAJDgAZsyfMCfBf7JtYmZmxd63Ulv4FsArI0rBFdGa3wtgHWmggJAEUCgAdHNpUep25CeQKbigf3b0QALFg2ZaER0HG2hFTNgRg1UAIBSAYHAE0gAdHNpUep25CeQKbigf3b0QALFg2ZaER0HG2hFTNgRg1QAACTqAQACwAAASdQCACFoAdHNpUep25CeQKbigf3b0QALFg2ZaER0HG2hFTNgRg1UAPS4QN/Df9+0rkLLAcLlWBUmig1E5vQoZqYvbHwCBY1KgCFoB6XCBv4b/v2lchZYDhcqwKk0UGonN6FDNTF7Y+AQLGpUADo5tKj1O3ITyBTcUD+7eiABYsGzLQiOg420IqZsCMGqg==",
    )?;
    let decoded: DedustV2CpmmStorage = TLB::from_cell(&body)?;
    assert!(decoded.deposit_active);
    assert!(decoded.swap_active);
    assert_eq!(*decoded.liquidity, 612372435695794);
    assert_eq!(*decoded.reserve_x, 1264308124684);
    assert_eq!(*decoded.reserve_y, 296604912019870737);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_wire_004() -> anyhow::Result<()> {
    let body = TonCell::from_boc_base64("te6ccgEBBAEAKAADJZs6o/oAAAAAAAAAYwAFERFkHIoDAgEABBI0AAgiIiIiAAgRERER")?;
    let decoded: DedustLiquidityDepositV1InstallMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 99);
    assert_eq!(decoded.code_version, 5);
    assert_eq!(decoded.code.data_len_bits(), 32);
    assert_eq!(decoded.blank_code.data_len_bits(), 32);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_wire_005() -> anyhow::Result<()> {
    let body = TonCell::from_boc_base64(
        "te6cckEBAQEAMwAAYlQkD+UAAAAAAAAAABALETqZS1AkoWcZ9pE5Mo63WVlsOKJfWQKLFG/s3DYh3+MNdigEXoUu",
    )?;
    let decoded: DedustLiquidityDepositV1DepositAssetMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 0);
    assert_eq!(*decoded.amount, 882216);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_wire_006() -> anyhow::Result<()> {
    let body = TonCell::from_boc_base64(
        "te6cckEBBAEAywACXbVrlZgAAAAAAAAAAIAWENIZBy719taSMfRic6Pz8p5n4gY/wGcWER/FwBOj3SBAAQIBRYAL4Kyfa+wI8Htq4GOcOezRURqamttNvJtkRWlyQa37AABwAwBUBB3NZQAQCxE6mUtQJKFnGfaROTKOt1lZbDiiX1kCixRv7Nw2Id/jDXYoAIeAFhDSGQcu9fbWkjH0YnOj8/KeZ+IGP8BnFhEfxcATo90gEAsROplLUCShZxn2kTkyjrdZWWw4ol9ZAosUb+zcNiHf6KnxeLU=",
    )?;
    let decoded: DedustLiquidityDepositV1DepositLiquidityAllMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 0);
    assert_eq!(*decoded.min_lp_amount, 0);
    assert!(decoded.fulfill_payload.is_none());
    assert!(decoded.reject_payload.is_none());
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_wire_007() -> anyhow::Result<()> {
    let body = TonCell::from_boc_base64(
        "te6cckEBAgEAXgABKKrnklYAAAAAAAAAAEHccx+TDXYoAQCJgAvgrJ9r7Ajwe2rgY5w57NFRGpqa2028m2RFaXJBrfsAAEAQCxE6mUtQJKFnGfaROTKOt1lZbDiiX1kCixRv7Nw2Id/o/coJYg==",
    )?;
    let decoded: DedustLiquidityDepositV1FulfillDepositMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 0);
    assert_eq!(*decoded.asset0_deposit, 499593721);
    assert_eq!(*decoded.asset1_deposit, 882216);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_wire_008() -> anyhow::Result<()> {
    let body = TonCell::from_boc_base64(
        "te6cckEBAwEAfwABH2sLeH8AAAAAAAAAADBjMHQBAUWAC+Csn2vsCPB7auBjnDns0VEamprbTbybZEVpckGt+wAAcAIAh4AWENIZBy719taSMfRic6Pz8p5n4gY/wGcWER/FwBOj3SAQCxE6mUtQJKFnGfaROTKOt1lZbDiiX1kCixRv7Nw2Id/oohzMNQ==",
    )?;
    let decoded: DedustLiquidityDepositV1PayOutFromDepositMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 0);
    assert_eq!(*decoded.amount, 406279);
    assert!(decoded.payload.is_none());
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_wire_009() -> anyhow::Result<()> {
    let body = TonCell::from_boc_base64(
        "te6cckEBAwEAiAACbWHuVC0AAAAAAAAAAFAXXdPxaABNauqYaw65JWHTMvE5n2NdUvaMUxvGaLOCr1jl5QnO3jqfjJQBAgBFgAvgrJ9r7Ajwe2rgY5w57NFRGpqa2028m2RFaXJBrfsAACEAS2ohsk+ABNauqYaw65JWHTMvE5n2NdUvaMUxvGaLOCr1jl5QnO3hJw4gVA==",
    )?;
    let decoded: DedustPoolV2SwapExternalMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 0);
    assert_eq!(*decoded.amount, 6272401174);
    assert_eq!(*decoded.current.limit, 11139273);
    assert!(decoded.current.next.is_none());
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_wire_010() -> anyhow::Result<()> {
    let body = TonCell::from_boc_base64(
        "te6cckEBAgEAewABYq1OtvUAAAAAAAAAADqfoSgATWrqmGsOuSVh0zLxOZ9jXVL2jFMbxmizgq9Y5eUJzt4BAImAC+Csn2vsCPB7auBjnDns0VEamprbTbybZEVpckGt+wAAQBALETqZS1AkoWcZ9pE5Mo63WVlsOKJfWQKLFG/s3DYh3+jZQHs2",
    )?;
    let decoded: DedustPoolV2PayOutFromPoolMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 0);
    assert_eq!(*decoded.amount, 11139602);
    assert!(decoded.payload.is_none());
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_wire_011() -> anyhow::Result<()> {
    let body = TonCell::from_boc_base64(
        "te6cckEBAgEAYgABXpxhDeMBALETqZS1AkoWcZ9pE5Mo63WVlsOKJfWQKLFG/s3DYh3+UBdd0/FjqfoSAQBbgATWrqmGsOuSVh0zLxOZ9jXVL2jFMbxmizgq9Y5eUJzt41MbY8PG4apdIs6bBFp5+Sc=",
    )?;
    let decoded: DedustPoolV2SwapEventMsg = TLB::from_cell(&body)?;
    assert_eq!(*decoded.amount_in, 6272401174);
    assert_eq!(*decoded.amount_out, 11139602);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_wire_012() -> anyhow::Result<()> {
    let body = TonCell::from_boc_base64(
        "te6cckEBAwEA6QACqXKsqKpqIZW7AAAAABALETqZS1AkoWcZ9pE5Mo63WVlsOKJfWQKLFG/s3DYh3+MMB/SAH2FHoIKiENMKfMjywuY4KW74Ojsk0guowsdBdRZZuF1BAUABAgDLgAvgrJ9r7Ajwe2rgY5w57NFRGpqa2028m2RFaXJBrfsAAEEAU3MrWY3WcBS4+kzuwnhr5+GjJql721/pzg4gKW3dNdYQCxE6mUtQJKFnGfaROTKOt1lZbDiiX1kCixRv7Nw2Id/oAEtqIZXZgB9hR6CCohDTCnzI8sLmOClu+Do7JNILqMLHQXUWWbhdQbYGPvg=",
    )?;
    let decoded: DedustPoolV2SwapPeerMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 7647558272643235840);
    assert_eq!(*decoded.amount, 788468);
    assert!(decoded.current.next.is_none());
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_wire_013() -> anyhow::Result<()> {
    let body = TonCell::from_boc_base64("te6cckEBAQEAEAAAHMAVKX8AAAAAAAAAAAAFKlHsVg==")?;
    let decoded: DedustPoolV2ConfigureTradeFeeMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 0);
    assert_eq!(decoded.trade_fee, 5);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_wire_014() -> anyhow::Result<()> {
    let body = TonCell::from_boc_base64(
        "te6cckEBAQEAMAAAWybGOrcAAAAAAAAAAIAGbMnzAnwX+ybWJmZsXet1Jb+BbAKyNKwRXRmt8LYB1pBcU3m+",
    )?;
    let decoded: DedustPoolV2ResetGasMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 0);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_wire_015() -> anyhow::Result<()> {
    let body = TonCell::from_boc_base64(
        "te6cckEBAgEAWgABJW/0FtwAAAAAAAAAAEUCGHEaACABAIRpcGZzOi8vYmFma3JlaWhnbjVzaDJha3M0M2YyY3FicnlwYm03aGFyM20yeWk2NTVibHp3aGF1Z25rZDZjdW9qY3UHVNF9",
    )?;
    let decoded: DedustUranusFactoryV3DeployMemeMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 0);
    assert_eq!(decoded.preset_id, 4);
    assert_eq!(*decoded.initial_buy, 9000000000);
    assert!(decoded.partner_config.is_none());
    assert!(decoded.referrer_config.is_none());
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_wire_016() -> anyhow::Result<()> {
    let body = TonCell::from_boc_base64(
        "te6cckEBAgEAfAABg2MvXRwAAAAAaidXIxNgFdPveYAEuAHdwBEn/+SVdUnozPG9ojQuQgYe5HxcQMwzM9uoVxCLKCcQH0AABigJUC+QAQEAamlwZnM6Ly9RbVNERHQzVEJhNFFrQ1VwTHpRRkE3R3lINzV6UHhaMlVYa0dvenlEd3lOOUxGuNJKAA==",
    )?;
    let decoded: DedustUranusFactoryV3DeployCustomizedMemeMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 1780963107);
    assert_eq!(decoded.total_supply_preset_id, 1);
    assert_eq!(decoded.base_fee_preset_id, 3);
    assert_eq!(*decoded.raising_funds, 1500000000000);
    assert_eq!(decoded.on_sell_supply_percent, 75);
    assert_eq!(decoded.partner_fee_bps, 5000);
    assert_eq!(decoded.pool_partner_fee_bps, 4000);
    assert_eq!(decoded.pool_base_fee_preset_id, 3);
    assert_eq!(*decoded.initial_buy, 5000000000);
    assert!(decoded.partner_config.is_none());
    assert!(decoded.referrer_config.is_none());
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_wire_017() -> anyhow::Result<()> {
    let body = TonCell::from_boc_base64("te6cckEBAQEAFAAAI3lvWgwAAAAAaidXI1ASoF8gApdF25U=")?;
    let decoded: crate::contracts::uranus::factory_v3::msgs::DedustUranusFactoryV3InitMemeMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 1780963107);
    assert_eq!(*decoded.initial_buy, 5000000000);
    assert!(decoded.partner_config.is_none());
    assert!(decoded.referrer_config.is_none());
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_wire_018() -> anyhow::Result<()> {
    let body = TonCell::from_boc_base64("te6ccgEBAQEAJAAAQ4AGbMnzAnwX+ybWJmZsXet1Jb+BbAKyNKwRXRmt8LYB1pA=")?;
    let decoded: DedustUranusFactoryV3Storage = TLB::from_cell(&body)?;

    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_wire_019() -> anyhow::Result<()> {
    let body = TonCell::from_boc_base64("te6cckEBAQEAFAAAI3lvWgwAAAAAaidXI1ASoF8gApdF25U=")?;
    let decoded: crate::contracts::uranus::meme_v3::msgs::DedustUranusMemeV3InitMemeMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 1780963107);
    assert_eq!(*decoded.initial_buy, 5000000000);
    assert!(decoded.partner_config.is_none());
    assert!(decoded.referrer_config.is_none());
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_wire_020() -> anyhow::Result<()> {
    let body = TonCell::from_boc_base64(
        "te6cckEBAQEAPAAAc5SCZVeN2V4LAHrGqEO5rKAHA2PIbMkXsoABZDCJdrL1sgJgtuVtmscq/Zw2OexeBtzjy7c5YwnO+qQAV3oC",
    )?;
    let decoded: DedustUranusMemeV3BuyMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 10221304230639748776);
    assert_eq!(*decoded.amount, 1000000000);
    assert_eq!(*decoded.minimal_amount_out, 954137399859122);
    assert!(decoded.partner_config.is_none());
    assert!(decoded.referrer_config.is_none());
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_wire_021() -> anyhow::Result<()> {
    let body = TonCell::from_boc_base64(
        "te6cckEBAQEASgAAj6Cqa8KAAWQwiXay9bICYLblbZrHKv2cNjnsXgbc48u3OWMJzvqodul5dOB3wjfsn42mFtTEYPOIIBAEPKVwVhXCkqOmRWCKSOXztkU=",
    )?;
    let decoded: DedustUranusMemeV3BuyEventMsg = TLB::from_cell(&body)?;
    assert_eq!(*decoded.amount_in, 997506234);
    assert_eq!(*decoded.amount_out, 1053408364919917);
    assert_eq!(*decoded.fees.creator_fee, 748130);
    assert_eq!(*decoded.fees.protocol_fee, 498753);
    assert_eq!(*decoded.fees.partner_fee, 0);
    assert_eq!(*decoded.fees.referrer_fee, 0);
    assert_eq!(*decoded.current_supply, 152650388326768969);
    assert_eq!(*decoded.raised_funds, 125395731538);
    assert!(!decoded.is_graduated);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_wire_022() -> anyhow::Result<()> {
    let body = TonCell::from_boc_base64("te6cckEBAQEADgAAGGbIrXKN2V4LAHrmkcs/bH4=")?;
    let decoded: DedustUranusMemeV3Opcode66c8ad72Msg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 10221304230639756945);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_wire_023() -> anyhow::Result<()> {
    let body = TonCell::from_boc_base64(
        "te6cckEBAQEAWgAAsGRq1CSN2V4LAHrGv3L+PVVAewoBAYAJHnTszXmW//gPUMcIuew0bdQhWDuhCKtcM537TEmdaZABI86dma8y3/8B6hjhFz2GjbqEKwd0IRVrhnO/aYkzrTC4J6Os",
    )?;
    let decoded: crate::contracts::uranus::meme_v3::msgs::DedustUranusMemeV3SellMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 10221304230639748799);
    assert_eq!(*decoded.amount, 13479829282730144);
    assert_eq!(*decoded.minimal_amount_out, 1);
    assert!(decoded.partner_config.is_none());
    assert!(decoded.referrer_config.is_none());
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_wire_024() -> anyhow::Result<()> {
    let body = TonCell::from_boc_base64(
        "te6cckEBAQEASwAAkTqw/MyACR507M15lv/4D1DHCLnsNG3UIVg7oQirXDOd+0xJnWmOX8eqqA9hQKBV/7dqxwgwSGsCAwAQAqZtMbyBaIiiKIJMtRB643Og",
    )?;
    let decoded: DedustUranusMemeV3SellEventMsg = TLB::from_cell(&body)?;
    assert_eq!(*decoded.amount_in, 13479829282730144);
    assert_eq!(*decoded.amount_out, 11542575958);
    assert_eq!(*decoded.fees.creator_fee, 8656932);
    assert_eq!(*decoded.fees.protocol_fee, 5771288);
    assert_eq!(*decoded.fees.partner_fee, 0);
    assert_eq!(*decoded.fees.referrer_fee, 0);
    assert_eq!(*decoded.current_supply, 95480047296623684);
    assert_eq!(*decoded.raised_funds, 74156500392);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_wire_025() -> anyhow::Result<()> {
    let body = TonCell::from_boc_base64(
        "te6ccgECBAEAARcAAluAGRAfwRcy79N1JrezzahhsBTREg17FgAAIKEBvBbWdOyAAAf4ShaBkhB2AnEQAQIBw4AZR1/BLKcXvEPzcLYL6egGnFNcsMKM6ej2jy92UN1O21AAzZk+YE+C/2TaxMzNi71upLfwLYBWRpWCK6M1vhbAOtGAV0+95gAgDeC2s6dkAAJ/7xJxVKCi3FPruOPkz6vSAwCNgB3cARJ//klXVJ6MzxvaI0LkIGHuR8XEDMMzPbqFcQiygnEADJADKOv4JZTi94h+bhbBfT0A04prlhhRnT0e0eXuyhup22oAamlwZnM6Ly9RbVNERHQzVEJhNFFrQ1VwTHpRRkE3R3lINzV6UHhaMlVYa0dvenlEd3lOOUxG",
    )?;
    let decoded: DedustUranusMemeV3Storage = TLB::from_cell(&body)?;
    assert!(decoded.initialized);
    assert!(!decoded.migrated);
    assert_eq!(decoded.base_fee_bps, 100);
    assert_eq!(*decoded.total_supply, 1000000000000000000);
    assert_eq!(*decoded.creator_fee, 16524555);
    assert!(!decoded.curve.graduated);
    assert_eq!(*decoded.curve.alpha, 1144067796610169491);
    assert_eq!(*decoded.curve.beta, 814406779661);
    assert_eq!(*decoded.curve.on_sell_supply, 750000000000000000);
    assert_eq!(*decoded.curve.raised_funds, 5);
    assert_eq!(*decoded.curve.current_supply, 0);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_wire_026() -> anyhow::Result<()> {
    let body = TonCell::from_boc_base64(
        "te6cckEBAQEASAAAi6Cqa8KAGpQEp6DRHcliUlfi9qP3/3keHlYAAX/Z88jrIUvQbyGoU5xIMODnAM4Hm7aGldR8ZANnwA4OcAzgebtohTnEgwgNAQJ9",
    )?;
    let decoded: DedustUranusMemeV2BuyEventMsg = TLB::from_cell(&body)?;
    assert_eq!(*decoded.amount_in, 701375512);
    assert_eq!(*decoded.amount_out, 2031925140970932);
    assert_eq!(*decoded.fees.creator_fee, 4909630);
    assert_eq!(*decoded.fees.protocol_fee, 2104126);
    assert_eq!(*decoded.fees.partner_fee, 0);
    assert_eq!(*decoded.fees.referrer_fee, 0);
    assert_eq!(*decoded.current_supply, 2031925140970932);
    assert_eq!(*decoded.raised_funds, 701375512);
    assert!(!decoded.is_graduated);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_wire_027() -> anyhow::Result<()> {
    let body = TonCell::from_boc_base64(
        "te6cckEBAQEASwAAkTqw/MyAGpQEp6DRHcliUlfi9qP3/3keHlYAAX/Z88jrIUvQbyGuDaegTGlZuoTt1b2GjVNWY8kWxnki3A4zAnWASauwko0Fv7BYpaEx",
    )?;
    let decoded: DedustUranusMemeV2SellEventMsg = TLB::from_cell(&body)?;
    assert_eq!(*decoded.amount_in, 1921740807908573);
    assert_eq!(*decoded.amount_out, 661564908);
    assert_eq!(*decoded.fees.creator_fee, 4630955);
    assert_eq!(*decoded.fees.protocol_fee, 1984694);
    assert_eq!(*decoded.fees.partner_fee, 3969390);
    assert_eq!(*decoded.fees.referrer_fee, 0);
    assert_eq!(*decoded.current_supply, 7178963749492184);
    assert_eq!(*decoded.raised_funds, 2489855485);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_wire_028() -> anyhow::Result<()> {
    let body = TonCell::from_boc_base64(
        "te6cckEBAQEAOQAAbbdFniyN2V4LAHrGv3L+PVVAewoBAYAJHnTszXmW//gPUMcIuew0bdQhWDuhCKtcM537TEmdaYS2jFHP",
    )?;
    let decoded: DedustUranusMemeWalletV3SellTokensMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 10221304230639748799);
    assert_eq!(*decoded.amount, 13479829282730144);
    assert_eq!(*decoded.minimal_amount_out, 1);
    assert!(decoded.partner_config.is_none());
    assert!(decoded.referrer_config.is_none());
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_wire_029() -> anyhow::Result<()> {
    let body = TonCell::from_boc_base64(
        "te6cckEBAQEAWgAAsGRq1CSN2V4LAHrGv3L+PVVAewoBAYAJHnTszXmW//gPUMcIuew0bdQhWDuhCKtcM537TEmdaZABI86dma8y3/8B6hjhFz2GjbqEKwd0IRVrhnO/aYkzrTC4J6Os",
    )?;
    let decoded: crate::contracts::uranus::wallet_v3::msgs::DedustUranusMemeWalletV3SellMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 10221304230639748799);
    assert_eq!(*decoded.amount, 13479829282730144);
    assert_eq!(*decoded.minimal_amount_out, 1);
    assert!(decoded.partner_config.is_none());
    assert!(decoded.referrer_config.is_none());
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_wire_030() -> anyhow::Result<()> {
    let body = TonCell::from_boc_base64(
        "te6ccgEBAQEARgAAhwgAEdsbmTp1QXHXi3VSqu+BCmj31Ly7fVQ9EzFUhUMY5HUADp0Z4ohwdRVCCEoaYZ7V8R/J9hpjIsLn3g+Rg7m4hsSg",
    )?;
    let decoded: DedustUranusMemeWalletV3Storage = TLB::from_cell(&body)?;
    assert_eq!(*decoded.balance, 0);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_wire_031() -> anyhow::Result<()> {
    let body = TonCell::from_boc_base64(
        "te6cckEBAQEAMwAAYlQkD+UAAAAAAAAAABALETqZS1AkoWcZ9pE5Mo63WVlsOKJfWQKLFG/s3DYh3+MNdigEXoUu",
    )?;
    let decoded: DedustLiquidityDepositV1DepositAssetMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 0);
    assert_eq!(*decoded.amount, 882216);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_wire_032() -> anyhow::Result<()> {
    let body = TonCell::from_boc_base64(
        "te6cckEBBAEAywACXbVrlZgAAAAAAAAAAIAWENIZBy719taSMfRic6Pz8p5n4gY/wGcWER/FwBOj3SBAAQIBRYAL4Kyfa+wI8Htq4GOcOezRURqamttNvJtkRWlyQa37AABwAwBUBB3NZQAQCxE6mUtQJKFnGfaROTKOt1lZbDiiX1kCixRv7Nw2Id/jDXYoAIeAFhDSGQcu9fbWkjH0YnOj8/KeZ+IGP8BnFhEfxcATo90gEAsROplLUCShZxn2kTkyjrdZWWw4ol9ZAosUb+zcNiHf6KnxeLU=",
    )?;
    let decoded: DedustLiquidityDepositV1DepositLiquidityAllMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 0);
    assert_eq!(*decoded.min_lp_amount, 0);
    assert!(decoded.fulfill_payload.is_none());
    assert!(decoded.reject_payload.is_none());
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_wire_033() -> anyhow::Result<()> {
    let body = TonCell::from_boc_base64(
        "te6cckEBAgEAXgABKKrnklYAAAAAAAAAAEHccx+TDXYoAQCJgAvgrJ9r7Ajwe2rgY5w57NFRGpqa2028m2RFaXJBrfsAAEAQCxE6mUtQJKFnGfaROTKOt1lZbDiiX1kCixRv7Nw2Id/o/coJYg==",
    )?;
    let decoded: DedustLiquidityDepositV1FulfillDepositMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 0);
    assert_eq!(*decoded.asset0_deposit, 499593721);
    assert_eq!(*decoded.asset1_deposit, 882216);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_wire_034() -> anyhow::Result<()> {
    let body = TonCell::from_boc_base64(
        "te6cckEBAwEAfwABH2sLeH8AAAAAAAAAADBjMHQBAUWAC+Csn2vsCPB7auBjnDns0VEamprbTbybZEVpckGt+wAAcAIAh4AWENIZBy719taSMfRic6Pz8p5n4gY/wGcWER/FwBOj3SAQCxE6mUtQJKFnGfaROTKOt1lZbDiiX1kCixRv7Nw2Id/oohzMNQ==",
    )?;
    let decoded: DedustLiquidityDepositV1PayOutFromDepositMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 0);
    assert_eq!(*decoded.amount, 406279);
    assert!(decoded.payload.is_none());
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_wire_035() -> anyhow::Result<()> {
    let body = TonCell::from_boc_base64(
        "te6cckEBBAEAuQABZHNi0Jz0znEXAAAAAEBe4/4IAPRcRblvM+G48QKmqzkZyhrpzk8vWyCmVZoW/vCWx8XbAQJX46DUgoAHy/+VG7+ebYbZP+jeYqxVVqNzIpCLWthNl7zJOrFKsQUM/4rTA8ACAwBTgAxg/7Jlu0nQwx7ci/wMTGdPmN+Qtkb14/CAcQfgF1+nZwf065e96/9AAEtqIZpigA9FxFuW8z4bjxAqarORnKGunOTy9bIKZVmhb+8JbHxdoZTWyR4=",
    )?;
    let decoded: DedustVaultJettonV2TransferNotificationMsg = TLB::from_cell(&body)?;

    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_wire_036() -> anyhow::Result<()> {
    let body = TonCell::from_boc_base64(
        "te6cckEBBAEA1wADb2HuVC30znEXAAAAAEBe4/4IAPRcRblvM+G48QKmqzkZyhrpzk8vWyCmVZoW/vCWx8XaUM/4rTA8AQIDAIeAC+Csn2vsCPB7auBjnDns0VEamprbTbybZEVpckGt+wAAIgFiJ1MpagSULOM+0icmUdbrKy2HFEvrIFFijf2bhsQ7/QBTgAxg/7Jlu0nQwx7ci/wMTGdPmN+Qtkb14/CAcQfgF1+nZwf065e96/9AAEtqIZpigA9FxFuW8z4bjxAqarORnKGunOTy9bIKZVmhb+8JbHxdoVKTXDw=",
    )?;
    let decoded: DedustVaultJettonV2SwapExternalMsg = TLB::from_cell(&body)?;

    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_wire_037() -> anyhow::Result<()> {
    let body = TonCell::from_boc_base64(
        "te6cckEBAgEAfAABZK1OtvUAAAAAAAI8DkB8nCUYARnqKB/UVzJaYmkjDefS8mkalmJWXD0JMIprLqZISH74AQCJgAvgrJ9r7Ajwe2rgY5w57NFRGpqa2028m2RFaXJBrfsAAEAQCxE6mUtQJKFnGfaROTKOt1lZbDiiX1kCixRv7Nw2Id/o3vy6Sg==",
    )?;
    let decoded: DedustVaultJettonV2PayOutFromPoolMsg = TLB::from_cell(&body)?;
    assert!(decoded.payload.is_none());
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_wire_038() -> anyhow::Result<()> {
    let body = TonCell::from_boc_base64(
        "te6cckEBAQEAWgAAsA+KfqUAAAAAAAI8DkB8nCUYARnqKB/UVzJaYmkjDefS8mkalmJWXD0JMIprLqZISH75ACM9RQP6iuZLTE0kYbz6Xk0jUsxKy4ehJhFNZdTJCQ/fCB1+RTL5TEvi",
    )?;
    let decoded: DedustVaultJettonV2JettonTransferMsg = TLB::from_cell(&body)?;
    assert!(decoded.custom_payload.is_none());
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_wire_039() -> anyhow::Result<()> {
    let body = TonCell::from_boc_base64(
        "te6cckECHgEAAmMAAa/MIr6L5m9INsvIIybocmxOduIBVVLl2FOubfhe/+nNUVelbMXLGluRJ7KWDqGZ4pU8E2Uvf8ltZ8Anwl1v2YMAAAAqvQAAABoAAAAAgAABgGS5jHlAk2FAAQNfgAXW5Qv7d51Hn+02RuXIZc4rfIN3KuDzmKXyD8j43XMDyg4iVyWGGpW8Vkj4qcMOAgMEAiOlp8v4UreKyR8VOGFQbncmvDgFBgIBbgcIAQNAExkAIcRCUA9w8Xt4dQyTAAam9XGRANOAALFjBkfPf49HmFzHHQ2hM2AzjS9qk3D/WVLJ2LZOyeoII8NGAEAAWLGDI+e/x6PMLmOOhtCZsBnGl7VJuH+sqWTsWydk9QAgACxYwZHz3+PR5hcxx0NoTNgM40vapNw/1lSydi2TsnqEAgFuCQoCAUgPEAIBWAsMAgFIDQ4AF6jLGEAAAADiKE/4sAAXqLNgQAAAAOIphZtQABeom6hAAAAA4kdG2dAAF6iD8EAAAADiKGVd0AAXrgeZCAAAABxEt+lGAgEgERIAF62s2BAAAAA4iWb/BAIBIBMUAgEgFRYCASAXGAAXqJuoQAAAAOJQPxHQABeog/BAAAAA4iib4NAAF6hsOEAAAADiJY5qsAAXqFSAQAAAAOIllVWQAgNBDRsaAQNBCRsCEHDxe3h1DJMAHB0AhYAAsWMGR89/j0eYXMcdDaEzYDONL2qTcP9ZUsnYtk7J6hAAFhar9OrXdKcwZVErI7SK1O1MCq0ScGUYCxmp7aqofLoAQ4AF1uUL+3edR5/tNkblyGXOK3yDdyrg85il8g/I+N1zA9AKA5Qf",
    )?;
    let decoded: DedustX1000WalletV2SignedRequestMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.seqno, 10941);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_wire_040() -> anyhow::Result<()> {
    let body = TonCell::from_boc_base64("te6cckEBAQEADgAAGAEBAgFyz1b24dFP9m2QVa0=")?;
    let decoded: DedustX1000WalletV2ExecutionRequestedMsg = TLB::from_cell(&body)?;

    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_wire_041() -> anyhow::Result<()> {
    let body = TonCell::from_boc_base64("te6cckEBAQEADgAAGAEBAgJyz1b24dFP9kSc+rQ=")?;
    let decoded: DedustX1000WalletV2ExecuteStoredMsg = TLB::from_cell(&body)?;

    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_wire_042() -> anyhow::Result<()> {
    let body = TonCell::from_boc_base64(
        "te6cckEBAwEA/gABtg+KfqVWjEMAQKBrQnAdXe7jdkO4ALvt76z8/Rh1QLAgjAr2QtUsRzEs+uc4skr9PgOm2cqXAAFixgyPnv8ejzC5jjobQmbAZxpe1Sbh/rKlk7FsnZPUCDWk6QEBAeFmZN4qgAvDWJyINlcNyg7+HltsSw1DbL27gj+kOL8CgJfh019YkAAWLGDI+e/x6PMLmOOhtCZsBnGl7VJuH+sqWTsWydk9QgACxYwZHz3+PR5hcxx0NoTNgM40vapNw/1lSydi2TsnqAAAAAA1PPJdwAIAUwgACxYwZHz3+PR5hcxx0NoTNgM40vapNw/1lSydi2TsnqCAhYOwAAAAENVn5e4=",
    )?;
    let decoded: DedustX1000WalletV2JettonTransferMsg = TLB::from_cell(&body)?;
    assert_eq!(*decoded.amount, 516628432774203);
    assert!(decoded.custom_payload.is_none());
    assert_eq!(*decoded.forward_amount, 450000000);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_wire_043() -> anyhow::Result<()> {
    let body = TonCell::from_boc_base64(
        "te6cckEBAQEAOAAAa8wal68jj2ZXkT+nT3A3uOL2d0doAAsWMGR89/j0eYXMcdDaEzYDONL2qTcP9ZUsnYtk7J6gEJrR3p0=",
    )?;
    let decoded: DedustX1000StonksPumpSellMsg = TLB::from_cell(&body)?;
    assert_eq!(*decoded.amount, 980275541013622);
    assert_eq!(*decoded.min_gram_out, 0);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_wire_044() -> anyhow::Result<()> {
    let body = TonCell::from_boc_base64(
        "te6cckEBAQEANwAAabdFnixqR6abPzszsWG2MrCRyQCAALFjBkfPf49HmFzHHQ2hM2AzjS9qk3D/WVLJ2LZOyeoESwJGTg==",
    )?;
    let decoded: DedustX1000UranusSellMsg = TLB::from_cell(&body)?;
    assert_eq!(*decoded.amount, 30112737729680);
    assert_eq!(*decoded.min_gram_out, 0);
    assert!(decoded.partner.is_none());
    assert!(decoded.referrer.is_none());
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_wire_045() -> anyhow::Result<()> {
    let body = TonCell::from_boc_base64("te6cckEBAQEAEgAAIBCTeEdSt4rJHxU4YfKBJsJn1l5v")?;
    let decoded: DedustX1000WalletV2Event10937847Msg = TLB::from_cell(&body)?;

    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_wire_046() -> anyhow::Result<()> {
    let body = TonCell::from_boc_base64("te6ccgEBAQEADgAAGL6vlhdp/x/DhOrfaQ==")?;
    let decoded: DedustX1000WalletV2EventBeaf9617Msg = TLB::from_cell(&body)?;

    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

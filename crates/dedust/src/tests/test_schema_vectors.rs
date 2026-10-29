//! Independent Tolk-compiled synthetic wire vectors. Not executable-code or economic qualification.
#![allow(non_snake_case)]

use crate::contracts::classic::asset::DedustClassicAsset;
use crate::contracts::classic::asset::DedustClassicJettonAsset;
use crate::contracts::classic::asset::DedustClassicNativeAsset;
use crate::contracts::classic::factory_v1::msgs::DedustFactoryV1AcceptOwnershipMsg;
use crate::contracts::classic::factory_v1::msgs::DedustFactoryV1CancelOwnershipTransferMsg;
use crate::contracts::classic::factory_v1::msgs::DedustFactoryV1CreateLegacyJettonVaultMsg;
use crate::contracts::classic::factory_v1::msgs::DedustFactoryV1CreateLiquidityDepositMsg;
use crate::contracts::classic::factory_v1::msgs::DedustFactoryV1CreateStablePoolMsg;
use crate::contracts::classic::factory_v1::msgs::DedustFactoryV1CreateVaultMsg;
use crate::contracts::classic::factory_v1::msgs::DedustFactoryV1CreateVolatilePoolMsg;
use crate::contracts::classic::factory_v1::msgs::DedustFactoryV1DepositAssetMsg;
use crate::contracts::classic::factory_v1::msgs::DedustFactoryV1DestroyNonReadyVaultCommandMsg;
use crate::contracts::classic::factory_v1::msgs::DedustFactoryV1DestroyNonReadyVaultMsg;
use crate::contracts::classic::factory_v1::msgs::DedustFactoryV1IncomingMsg;
use crate::contracts::classic::factory_v1::msgs::DedustFactoryV1InstallLiquidityDepositCodeMsg;
use crate::contracts::classic::factory_v1::msgs::DedustFactoryV1InstallMsg;
use crate::contracts::classic::factory_v1::msgs::DedustFactoryV1InstallPoolCodeMsg;
use crate::contracts::classic::factory_v1::msgs::DedustFactoryV1InstallVaultCodeMsg;
use crate::contracts::classic::factory_v1::msgs::DedustFactoryV1OutgoingMsg;
use crate::contracts::classic::factory_v1::msgs::DedustFactoryV1ResetGasMsg;
use crate::contracts::classic::factory_v1::msgs::DedustFactoryV1TransferOwnershipMsg;
use crate::contracts::classic::factory_v1::msgs::DedustFactoryV1UpgradeChildCodeMsg;
use crate::contracts::classic::factory_v1::msgs::DedustFactoryV1UpgradeMsg;
use crate::contracts::classic::factory_v1::msgs::DedustFactoryV1UpgradePoolMsg;
use crate::contracts::classic::factory_v1::msgs::DedustFactoryV1UpgradeVaultMsg;
use crate::contracts::classic::factory_v1::storage::DedustFactoryV1Config;
use crate::contracts::classic::factory_v1::storage::DedustFactoryV1CreateLiquidityDepositExtra;
use crate::contracts::classic::factory_v1::storage::DedustFactoryV1PoolParams;
use crate::contracts::classic::factory_v1::storage::DedustFactoryV1Storage;
use crate::contracts::classic::factory_v1::storage::DedustFactoryV1VaultProof;
use crate::contracts::classic::factory_v2::msgs::DedustFactoryV2AcceptOwnershipMsg;
use crate::contracts::classic::factory_v2::msgs::DedustFactoryV2CancelOwnershipTransferMsg;
use crate::contracts::classic::factory_v2::msgs::DedustFactoryV2ChangeOperatorUserAddressMsg;
use crate::contracts::classic::factory_v2::msgs::DedustFactoryV2CreateLegacyJettonVaultMsg;
use crate::contracts::classic::factory_v2::msgs::DedustFactoryV2CreateLiquidityDepositMsg;
use crate::contracts::classic::factory_v2::msgs::DedustFactoryV2CreateStablePoolMsg;
use crate::contracts::classic::factory_v2::msgs::DedustFactoryV2CreateVaultMsg;
use crate::contracts::classic::factory_v2::msgs::DedustFactoryV2CreateVolatilePoolMsg;
use crate::contracts::classic::factory_v2::msgs::DedustFactoryV2DeployOperatorMsg;
use crate::contracts::classic::factory_v2::msgs::DedustFactoryV2DepositAssetMsg;
use crate::contracts::classic::factory_v2::msgs::DedustFactoryV2DestroyNonReadyVaultMsg;
use crate::contracts::classic::factory_v2::msgs::DedustFactoryV2IncomingMsg;
use crate::contracts::classic::factory_v2::msgs::DedustFactoryV2InstallLiquidityDepositCodeMsg;
use crate::contracts::classic::factory_v2::msgs::DedustFactoryV2InstallMsg;
use crate::contracts::classic::factory_v2::msgs::DedustFactoryV2InstallOperatorCodeMsg;
use crate::contracts::classic::factory_v2::msgs::DedustFactoryV2InstallPoolCodeMsg;
use crate::contracts::classic::factory_v2::msgs::DedustFactoryV2InstallVaultCodeMsg;
use crate::contracts::classic::factory_v2::msgs::DedustFactoryV2OutgoingMsg;
use crate::contracts::classic::factory_v2::msgs::DedustFactoryV2ResetGasMsg;
use crate::contracts::classic::factory_v2::msgs::DedustFactoryV2TransferOwnershipMsg;
use crate::contracts::classic::factory_v2::msgs::DedustFactoryV2UpgradeMsg;
use crate::contracts::classic::factory_v2::msgs::DedustFactoryV2UpgradeOperatorMsg;
use crate::contracts::classic::factory_v2::msgs::DedustFactoryV2UpgradePoolMsg;
use crate::contracts::classic::factory_v2::msgs::DedustFactoryV2UpgradeVaultMsg;
use crate::contracts::classic::factory_v2::storage::DedustFactoryV2Config;
use crate::contracts::classic::factory_v2::storage::DedustFactoryV2CreateLiquidityDepositExtra;
use crate::contracts::classic::factory_v2::storage::DedustFactoryV2PoolParams;
use crate::contracts::classic::factory_v2::storage::DedustFactoryV2Storage;
use crate::contracts::classic::factory_v2::storage::DedustFactoryV2VaultProof;
use crate::contracts::classic::liquidity_deposit_v1::msgs::DedustLiquidityDepositV1IncomingMsg;
use crate::contracts::classic::liquidity_deposit_v1::msgs::DedustLiquidityDepositV1OutgoingMsg;
use crate::contracts::classic::liquidity_deposit_v1::storage::DedustLiquidityDepositV1Storage;
use crate::contracts::classic::liquidity_deposit_v1::storage::DedustLiquidityDepositV1StorageDetails;
use crate::contracts::classic::liquidity_deposit_v2::msgs::DedustLiquidityDepositV2IncomingMsg;
use crate::contracts::classic::liquidity_deposit_v2::msgs::DedustLiquidityDepositV2OutgoingMsg;
use crate::contracts::classic::liquidity_deposit_v2::storage::DedustLiquidityDepositV2Storage;
use crate::contracts::classic::liquidity_deposit_v2::storage::DedustLiquidityDepositV2StorageDetails;
use crate::contracts::classic::pool_v1::msgs::DedustPoolV1DepositEventMsg;
use crate::contracts::classic::pool_v1::msgs::DedustPoolV1DepositLiquidityAllMsg;
use crate::contracts::classic::pool_v1::msgs::DedustPoolV1FulfillDepositMsg;
use crate::contracts::classic::pool_v1::msgs::DedustPoolV1IncomingMsg;
use crate::contracts::classic::pool_v1::msgs::DedustPoolV1JettonBurnNotificationMsg;
use crate::contracts::classic::pool_v1::msgs::DedustPoolV1JettonInternalTransferMsg;
use crate::contracts::classic::pool_v1::msgs::DedustPoolV1OutgoingMsg;
use crate::contracts::classic::pool_v1::msgs::DedustPoolV1PayOutFromPoolMsg;
use crate::contracts::classic::pool_v1::msgs::DedustPoolV1RejectDepositMsg;
use crate::contracts::classic::pool_v1::msgs::DedustPoolV1SwapEventMsg;
use crate::contracts::classic::pool_v1::msgs::DedustPoolV1SwapExternalMsg;
use crate::contracts::classic::pool_v1::msgs::DedustPoolV1SwapPeerMsg;
use crate::contracts::classic::pool_v1::msgs::DedustPoolV1UpgradeCodeMsg;
use crate::contracts::classic::pool_v1::msgs::DedustPoolV1WithdrawProtocolFeesMsg;
use crate::contracts::classic::pool_v1::msgs::DedustPoolV1WithdrawalEventMsg;
use crate::contracts::classic::pool_v1::storage::DedustPoolV1Config;
use crate::contracts::classic::pool_v1::storage::DedustPoolV1DepositLiquidityAllAmounts;
use crate::contracts::classic::pool_v1::storage::DedustPoolV1GivenIn;
use crate::contracts::classic::pool_v1::storage::DedustPoolV1GivenOut;
use crate::contracts::classic::pool_v1::storage::DedustPoolV1LiquidityDepositContractType;
use crate::contracts::classic::pool_v1::storage::DedustPoolV1LiquidityDepositProof;
use crate::contracts::classic::pool_v1::storage::DedustPoolV1LiquidityDepositProofParams;
use crate::contracts::classic::pool_v1::storage::DedustPoolV1PoolContractType;
use crate::contracts::classic::pool_v1::storage::DedustPoolV1PoolParams;
use crate::contracts::classic::pool_v1::storage::DedustPoolV1PoolProof;
use crate::contracts::classic::pool_v1::storage::DedustPoolV1PoolType;
use crate::contracts::classic::pool_v1::storage::DedustPoolV1StablePool;
use crate::contracts::classic::pool_v1::storage::DedustPoolV1Storage;
use crate::contracts::classic::pool_v1::storage::DedustPoolV1SwapEventTail;
use crate::contracts::classic::pool_v1::storage::DedustPoolV1SwapKind;
use crate::contracts::classic::pool_v1::storage::DedustPoolV1SwapParams;
use crate::contracts::classic::pool_v1::storage::DedustPoolV1SwapStep;
use crate::contracts::classic::pool_v1::storage::DedustPoolV1SwapStepParams;
use crate::contracts::classic::pool_v1::storage::DedustPoolV1VaultContractType;
use crate::contracts::classic::pool_v1::storage::DedustPoolV1VaultProof;
use crate::contracts::classic::pool_v1::storage::DedustPoolV1VolatilePool;
use crate::contracts::classic::pool_v2::msgs::DedustPoolV2CollectFeesMsg;
use crate::contracts::classic::pool_v2::msgs::DedustPoolV2ConfigureQuoteProviderMsg;
use crate::contracts::classic::pool_v2::msgs::DedustPoolV2ConfigureStartTimeMsg;
use crate::contracts::classic::pool_v2::msgs::DedustPoolV2ConfigureTradeFeeMsg;
use crate::contracts::classic::pool_v2::msgs::DedustPoolV2DepositEventMsg;
use crate::contracts::classic::pool_v2::msgs::DedustPoolV2DepositLiquidityAllMsg;
use crate::contracts::classic::pool_v2::msgs::DedustPoolV2FulfillDepositMsg;
use crate::contracts::classic::pool_v2::msgs::DedustPoolV2IncomingMsg;
use crate::contracts::classic::pool_v2::msgs::DedustPoolV2JettonBurnNotificationMsg;
use crate::contracts::classic::pool_v2::msgs::DedustPoolV2JettonInternalTransferMsg;
use crate::contracts::classic::pool_v2::msgs::DedustPoolV2OutgoingMsg;
use crate::contracts::classic::pool_v2::msgs::DedustPoolV2PayOutFromPoolMsg;
use crate::contracts::classic::pool_v2::msgs::DedustPoolV2ProvidePoolStateMsg;
use crate::contracts::classic::pool_v2::msgs::DedustPoolV2RejectDepositMsg;
use crate::contracts::classic::pool_v2::msgs::DedustPoolV2ResetGasMsg;
use crate::contracts::classic::pool_v2::msgs::DedustPoolV2SwapEventMsg;
use crate::contracts::classic::pool_v2::msgs::DedustPoolV2SwapExternalMsg;
use crate::contracts::classic::pool_v2::msgs::DedustPoolV2SwapPeerMsg;
use crate::contracts::classic::pool_v2::msgs::DedustPoolV2SyncAssetQuoteMsg;
use crate::contracts::classic::pool_v2::msgs::DedustPoolV2TakePoolStateMsg;
use crate::contracts::classic::pool_v2::msgs::DedustPoolV2WithdrawalEventMsg;
use crate::contracts::classic::pool_v2::storage::DedustPoolV2Config;
use crate::contracts::classic::pool_v2::storage::DedustPoolV2DepositLiquidityAllAmounts;
use crate::contracts::classic::pool_v2::storage::DedustPoolV2GivenIn;
use crate::contracts::classic::pool_v2::storage::DedustPoolV2GivenOut;
use crate::contracts::classic::pool_v2::storage::DedustPoolV2LiquidityDepositContractType;
use crate::contracts::classic::pool_v2::storage::DedustPoolV2LiquidityDepositProof;
use crate::contracts::classic::pool_v2::storage::DedustPoolV2LiquidityDepositProofParams;
use crate::contracts::classic::pool_v2::storage::DedustPoolV2PoolContractType;
use crate::contracts::classic::pool_v2::storage::DedustPoolV2PoolParams;
use crate::contracts::classic::pool_v2::storage::DedustPoolV2PoolProof;
use crate::contracts::classic::pool_v2::storage::DedustPoolV2PoolStateAssets;
use crate::contracts::classic::pool_v2::storage::DedustPoolV2PoolType;
use crate::contracts::classic::pool_v2::storage::DedustPoolV2StablePool;
use crate::contracts::classic::pool_v2::storage::DedustPoolV2Storage;
use crate::contracts::classic::pool_v2::storage::DedustPoolV2SwapEventTail;
use crate::contracts::classic::pool_v2::storage::DedustPoolV2SwapKind;
use crate::contracts::classic::pool_v2::storage::DedustPoolV2SwapParams;
use crate::contracts::classic::pool_v2::storage::DedustPoolV2SwapStep;
use crate::contracts::classic::pool_v2::storage::DedustPoolV2SwapStepParams;
use crate::contracts::classic::pool_v2::storage::DedustPoolV2VaultContractType;
use crate::contracts::classic::pool_v2::storage::DedustPoolV2VaultProof;
use crate::contracts::classic::pool_v2::storage::DedustPoolV2VolatilePool;
use crate::contracts::classic::vault_jetton_v1::msgs::DedustVaultJettonV1BouncedProvideWalletAddressMsg;
use crate::contracts::classic::vault_jetton_v1::msgs::DedustVaultJettonV1CreateLiquidityDepositMsg;
use crate::contracts::classic::vault_jetton_v1::msgs::DedustVaultJettonV1DepositLiquidityForwardPayloadMsg;
use crate::contracts::classic::vault_jetton_v1::msgs::DedustVaultJettonV1DestroyNonReadyMsg;
use crate::contracts::classic::vault_jetton_v1::msgs::DedustVaultJettonV1IncomingMsg;
use crate::contracts::classic::vault_jetton_v1::msgs::DedustVaultJettonV1JettonTransferMsg;
use crate::contracts::classic::vault_jetton_v1::msgs::DedustVaultJettonV1OutgoingMsg;
use crate::contracts::classic::vault_jetton_v1::msgs::DedustVaultJettonV1PayOutFromDepositMsg;
use crate::contracts::classic::vault_jetton_v1::msgs::DedustVaultJettonV1PayOutFromPoolMsg;
use crate::contracts::classic::vault_jetton_v1::msgs::DedustVaultJettonV1ProvideWalletAddressMsg;
use crate::contracts::classic::vault_jetton_v1::msgs::DedustVaultJettonV1SwapExternalMsg;
use crate::contracts::classic::vault_jetton_v1::msgs::DedustVaultJettonV1SwapForwardPayloadMsg;
use crate::contracts::classic::vault_jetton_v1::msgs::DedustVaultJettonV1TakeWalletAddressMsg;
use crate::contracts::classic::vault_jetton_v1::msgs::DedustVaultJettonV1TransferNotificationMsg;
use crate::contracts::classic::vault_jetton_v1::msgs::DedustVaultJettonV1UpgradeChildCodeMsg;
use crate::contracts::classic::vault_jetton_v1::storage::DedustVaultJettonV1CreateLiquidityDepositParams;
use crate::contracts::classic::vault_jetton_v1::storage::DedustVaultJettonV1GivenIn;
use crate::contracts::classic::vault_jetton_v1::storage::DedustVaultJettonV1GivenOut;
use crate::contracts::classic::vault_jetton_v1::storage::DedustVaultJettonV1PoolParams;
use crate::contracts::classic::vault_jetton_v1::storage::DedustVaultJettonV1PoolType;
use crate::contracts::classic::vault_jetton_v1::storage::DedustVaultJettonV1ProofData;
use crate::contracts::classic::vault_jetton_v1::storage::DedustVaultJettonV1StablePool;
use crate::contracts::classic::vault_jetton_v1::storage::DedustVaultJettonV1Storage;
use crate::contracts::classic::vault_jetton_v1::storage::DedustVaultJettonV1SwapKind;
use crate::contracts::classic::vault_jetton_v1::storage::DedustVaultJettonV1SwapParams;
use crate::contracts::classic::vault_jetton_v1::storage::DedustVaultJettonV1SwapStep;
use crate::contracts::classic::vault_jetton_v1::storage::DedustVaultJettonV1SwapStepParams;
use crate::contracts::classic::vault_jetton_v1::storage::DedustVaultJettonV1VolatilePool;
use crate::contracts::classic::vault_jetton_v2::msgs::DedustVaultJettonV2CreateLiquidityDepositMsg;
use crate::contracts::classic::vault_jetton_v2::msgs::DedustVaultJettonV2DepositLiquidityForwardPayloadMsg;
use crate::contracts::classic::vault_jetton_v2::msgs::DedustVaultJettonV2DestroyNonReadyMsg;
use crate::contracts::classic::vault_jetton_v2::msgs::DedustVaultJettonV2IncomingMsg;
use crate::contracts::classic::vault_jetton_v2::msgs::DedustVaultJettonV2JettonTransferMsg;
use crate::contracts::classic::vault_jetton_v2::msgs::DedustVaultJettonV2OutgoingMsg;
use crate::contracts::classic::vault_jetton_v2::msgs::DedustVaultJettonV2PayOutFromDepositMsg;
use crate::contracts::classic::vault_jetton_v2::msgs::DedustVaultJettonV2PayOutFromPoolMsg;
use crate::contracts::classic::vault_jetton_v2::msgs::DedustVaultJettonV2ProvideVaultStateMsg;
use crate::contracts::classic::vault_jetton_v2::msgs::DedustVaultJettonV2ProvideWalletAddressMsg;
use crate::contracts::classic::vault_jetton_v2::msgs::DedustVaultJettonV2ResetGasMsg;
use crate::contracts::classic::vault_jetton_v2::msgs::DedustVaultJettonV2SwapExternalMsg;
use crate::contracts::classic::vault_jetton_v2::msgs::DedustVaultJettonV2SwapForwardPayloadMsg;
use crate::contracts::classic::vault_jetton_v2::msgs::DedustVaultJettonV2TakeVaultStateMsg;
use crate::contracts::classic::vault_jetton_v2::msgs::DedustVaultJettonV2TakeWalletAddressMsg;
use crate::contracts::classic::vault_jetton_v2::msgs::DedustVaultJettonV2TransferNotificationMsg;
use crate::contracts::classic::vault_jetton_v2::storage::DedustVaultJettonV2ProofData;
use crate::contracts::classic::vault_jetton_v2::storage::DedustVaultJettonV2Storage;
use crate::contracts::classic::vault_native_v1::msgs::DedustVaultNativeV1CreateLiquidityDepositMsg;
use crate::contracts::classic::vault_native_v1::msgs::DedustVaultNativeV1DepositLiquidityMsg;
use crate::contracts::classic::vault_native_v1::msgs::DedustVaultNativeV1IncomingMsg;
use crate::contracts::classic::vault_native_v1::msgs::DedustVaultNativeV1OutgoingMsg;
use crate::contracts::classic::vault_native_v1::msgs::DedustVaultNativeV1PayOutFromDepositMsg;
use crate::contracts::classic::vault_native_v1::msgs::DedustVaultNativeV1PayOutFromPoolMsg;
use crate::contracts::classic::vault_native_v1::msgs::DedustVaultNativeV1PayoutMsg;
use crate::contracts::classic::vault_native_v1::msgs::DedustVaultNativeV1SwapExternalMsg;
use crate::contracts::classic::vault_native_v1::msgs::DedustVaultNativeV1SwapMsg;
use crate::contracts::classic::vault_native_v1::msgs::DedustVaultNativeV1UpgradeChildCodeMsg;
use crate::contracts::classic::vault_native_v1::storage::DedustVaultNativeV1CreateLiquidityDepositExtra;
use crate::contracts::classic::vault_native_v1::storage::DedustVaultNativeV1DepositLiquidityParams;
use crate::contracts::classic::vault_native_v1::storage::DedustVaultNativeV1GivenIn;
use crate::contracts::classic::vault_native_v1::storage::DedustVaultNativeV1GivenOut;
use crate::contracts::classic::vault_native_v1::storage::DedustVaultNativeV1LiquidityDepositProof;
use crate::contracts::classic::vault_native_v1::storage::DedustVaultNativeV1LiquidityDepositProofParams;
use crate::contracts::classic::vault_native_v1::storage::DedustVaultNativeV1PoolParams;
use crate::contracts::classic::vault_native_v1::storage::DedustVaultNativeV1PoolProof;
use crate::contracts::classic::vault_native_v1::storage::DedustVaultNativeV1Storage;
use crate::contracts::classic::vault_native_v1::storage::DedustVaultNativeV1SwapKind;
use crate::contracts::classic::vault_native_v1::storage::DedustVaultNativeV1SwapStep;
use crate::contracts::classic::vault_native_v1::storage::DedustVaultNativeV1SwapStepParams;
use crate::contracts::classic::vault_native_v1::storage::DedustVaultNativeV1VaultProof;
use crate::contracts::classic::vault_native_v2::msgs::DedustVaultNativeV2CreateLiquidityDepositMsg;
use crate::contracts::classic::vault_native_v2::msgs::DedustVaultNativeV2DepositLiquidityMsg;
use crate::contracts::classic::vault_native_v2::msgs::DedustVaultNativeV2IncomingMsg;
use crate::contracts::classic::vault_native_v2::msgs::DedustVaultNativeV2OutgoingMsg;
use crate::contracts::classic::vault_native_v2::msgs::DedustVaultNativeV2PayOutFromDepositMsg;
use crate::contracts::classic::vault_native_v2::msgs::DedustVaultNativeV2PayOutFromPoolMsg;
use crate::contracts::classic::vault_native_v2::msgs::DedustVaultNativeV2PayoutMsg;
use crate::contracts::classic::vault_native_v2::msgs::DedustVaultNativeV2ProvideVaultStateMsg;
use crate::contracts::classic::vault_native_v2::msgs::DedustVaultNativeV2ResetGasMsg;
use crate::contracts::classic::vault_native_v2::msgs::DedustVaultNativeV2SwapExternalMsg;
use crate::contracts::classic::vault_native_v2::msgs::DedustVaultNativeV2SwapMsg;
use crate::contracts::classic::vault_native_v2::msgs::DedustVaultNativeV2TakeVaultStateMsg;
use crate::contracts::classic::vault_native_v2::storage::DedustDepositLiquidityParams;
use crate::contracts::classic::vault_native_v2::storage::DedustVaultNativeV2ProofData;
use crate::contracts::classic::vault_native_v2::storage::DedustVaultNativeV2Storage;
use crate::contracts::cpmm_v2::pool::msgs::DedustV2CpmmCallback3216ca09Msg;
use crate::contracts::cpmm_v2::pool::msgs::DedustV2CpmmClaimAvailableFeesMsg;
use crate::contracts::cpmm_v2::pool::msgs::DedustV2CpmmClaimCreatorFeesMsg;
use crate::contracts::cpmm_v2::pool::msgs::DedustV2CpmmClaimPositionFeesMsg;
use crate::contracts::cpmm_v2::pool::msgs::DedustV2CpmmClaimPositionRewardMsg;
use crate::contracts::cpmm_v2::pool::msgs::DedustV2CpmmClaimProtocolFeesMsg;
use crate::contracts::cpmm_v2::pool::msgs::DedustV2CpmmClaimRewardMsg;
use crate::contracts::cpmm_v2::pool::msgs::DedustV2CpmmCreditLiquidityMsg;
use crate::contracts::cpmm_v2::pool::msgs::DedustV2CpmmDepositEventMsg;
use crate::contracts::cpmm_v2::pool::msgs::DedustV2CpmmDepositPayloadMsg;
use crate::contracts::cpmm_v2::pool::msgs::DedustV2CpmmExcessMsg;
use crate::contracts::cpmm_v2::pool::msgs::DedustV2CpmmExitLiquidityMsg;
use crate::contracts::cpmm_v2::pool::msgs::DedustV2CpmmFundRewardPayloadMsg;
use crate::contracts::cpmm_v2::pool::msgs::DedustV2CpmmIncomingMsg;
use crate::contracts::cpmm_v2::pool::msgs::DedustV2CpmmInitPoolMsg;
use crate::contracts::cpmm_v2::pool::msgs::DedustV2CpmmInitPoolResultMsg;
use crate::contracts::cpmm_v2::pool::msgs::DedustV2CpmmJoinLiquidityMsg;
use crate::contracts::cpmm_v2::pool::msgs::DedustV2CpmmOutgoingMsg;
use crate::contracts::cpmm_v2::pool::msgs::DedustV2CpmmPayJettonMsg;
use crate::contracts::cpmm_v2::pool::msgs::DedustV2CpmmPayNativeMsg;
use crate::contracts::cpmm_v2::pool::msgs::DedustV2CpmmPayoutPayloadMsg;
use crate::contracts::cpmm_v2::pool::msgs::DedustV2CpmmPayoutPositionFeesMsg;
use crate::contracts::cpmm_v2::pool::msgs::DedustV2CpmmPayoutRewardMsg;
use crate::contracts::cpmm_v2::pool::msgs::DedustV2CpmmProvidePoolStateMsg;
use crate::contracts::cpmm_v2::pool::msgs::DedustV2CpmmProvideWalletAddressMsg;
use crate::contracts::cpmm_v2::pool::msgs::DedustV2CpmmRefundDepositMsg;
use crate::contracts::cpmm_v2::pool::msgs::DedustV2CpmmSwapEventMsg;
use crate::contracts::cpmm_v2::pool::msgs::DedustV2CpmmSwapPayloadMsg;
use crate::contracts::cpmm_v2::pool::msgs::DedustV2CpmmTakePoolStateMsg;
use crate::contracts::cpmm_v2::pool::msgs::DedustV2CpmmTakeWalletAddressMsg;
use crate::contracts::cpmm_v2::pool::msgs::DedustV2CpmmTransferNotificationMsg;
use crate::contracts::cpmm_v2::pool::msgs::DedustV2CpmmUpgradePoolCodeMsg;
use crate::contracts::cpmm_v2::pool::msgs::DedustV2CpmmWithdrawLiquidityMsg;
use crate::contracts::cpmm_v2::pool::msgs::DedustV2CpmmWithdrawMsg;
use crate::contracts::cpmm_v2::pool::msgs::DedustV2CpmmWithdrawalEventMsg;
use crate::contracts::cpmm_v2::pool::storage::DedustV2CpmmActivation;
use crate::contracts::cpmm_v2::pool::storage::DedustV2CpmmActivationInitiator;
use crate::contracts::cpmm_v2::pool::storage::DedustV2CpmmActivationTime;
use crate::contracts::cpmm_v2::pool::storage::DedustV2CpmmAsset;
use crate::contracts::cpmm_v2::pool::storage::DedustV2CpmmBasicPayoutConfig;
use crate::contracts::cpmm_v2::pool::storage::DedustV2CpmmExtendedPayoutConfig;
use crate::contracts::cpmm_v2::pool::storage::DedustV2CpmmFeeIn;
use crate::contracts::cpmm_v2::pool::storage::DedustV2CpmmPartnerConfig;
use crate::contracts::cpmm_v2::pool::storage::DedustV2CpmmPayoutOptions;
use crate::contracts::cpmm_v2::pool::storage::DedustV2CpmmPoolConfig;
use crate::contracts::cpmm_v2::pool::storage::DedustV2CpmmPoolExtra;
use crate::contracts::cpmm_v2::pool::storage::DedustV2CpmmPoolFees;
use crate::contracts::cpmm_v2::pool::storage::DedustV2CpmmPoolReward;
use crate::contracts::cpmm_v2::pool::storage::DedustV2CpmmPoolRewards;
use crate::contracts::cpmm_v2::pool::storage::DedustV2CpmmPoolUpdatedState;
use crate::contracts::cpmm_v2::pool::storage::DedustV2CpmmPoolWallets;
use crate::contracts::cpmm_v2::pool::storage::DedustV2CpmmReferrerConfig;
use crate::contracts::cpmm_v2::pool::storage::DedustV2CpmmRewardConfig;
use crate::contracts::cpmm_v2::pool::storage::DedustV2CpmmStatus;
use crate::contracts::cpmm_v2::pool::storage::DedustV2CpmmStatusInitialized;
use crate::contracts::cpmm_v2::pool::storage::DedustV2CpmmStatusInitializing;
use crate::contracts::cpmm_v2::pool::storage::DedustV2CpmmStatusUninitialized;
use crate::contracts::cpmm_v2::pool::storage::DedustV2CpmmStorage;
use crate::contracts::cpmm_v2::pool::storage::DedustV2CpmmSwapFees;
use crate::contracts::cpmm_v2::pool::storage::DedustV2CpmmSwapStep;
use crate::contracts::custom_wallet::v1::storage::DedustX1000WalletV1Storage;
use crate::contracts::custom_wallet::v2::msgs::DedustX1000BidaskClmmSwapPayloadMsg;
use crate::contracts::custom_wallet::v2::msgs::DedustX1000BidaskDammSwapPayloadMsg;
use crate::contracts::custom_wallet::v2::msgs::DedustX1000CoffeeSwapPayloadMsg;
use crate::contracts::custom_wallet::v2::msgs::DedustX1000CpmmPayJettonPayloadMsg;
use crate::contracts::custom_wallet::v2::msgs::DedustX1000CpmmSwapPayloadMsg;
use crate::contracts::custom_wallet::v2::msgs::DedustX1000DedustSwapPayloadMsg;
use crate::contracts::custom_wallet::v2::msgs::DedustX1000JettonBurnMsg;
use crate::contracts::custom_wallet::v2::msgs::DedustX1000MoonClmmSwapPayloadMsg;
use crate::contracts::custom_wallet::v2::msgs::DedustX1000MoonSwapFailedMsg;
use crate::contracts::custom_wallet::v2::msgs::DedustX1000MoonSwapPayloadMsg;
use crate::contracts::custom_wallet::v2::msgs::DedustX1000MoonSwapSucceededMsg;
use crate::contracts::custom_wallet::v2::msgs::DedustX1000PayloadFa45056bMsg;
use crate::contracts::custom_wallet::v2::msgs::DedustX1000StonfiV1SwapPayloadMsg;
use crate::contracts::custom_wallet::v2::msgs::DedustX1000StonfiV2SwapPayloadMsg;
use crate::contracts::custom_wallet::v2::msgs::DedustX1000StonksPumpSellMsg;
use crate::contracts::custom_wallet::v2::msgs::DedustX1000ToncoSwapPayloadMsg;
use crate::contracts::custom_wallet::v2::msgs::DedustX1000UranusSellMsg;
use crate::contracts::custom_wallet::v2::msgs::DedustX1000WalletV2CallbackPayloadMsg;
use crate::contracts::custom_wallet::v2::msgs::DedustX1000WalletV2Event10937847Msg;
use crate::contracts::custom_wallet::v2::msgs::DedustX1000WalletV2EventBeaf9617Msg;
use crate::contracts::custom_wallet::v2::msgs::DedustX1000WalletV2EventMsg;
use crate::contracts::custom_wallet::v2::msgs::DedustX1000WalletV2ExecuteStoredMsg;
use crate::contracts::custom_wallet::v2::msgs::DedustX1000WalletV2ExecutionRequestedMsg;
use crate::contracts::custom_wallet::v2::msgs::DedustX1000WalletV2IncomingMsg;
use crate::contracts::custom_wallet::v2::msgs::DedustX1000WalletV2InternalCallbackMsg;
use crate::contracts::custom_wallet::v2::msgs::DedustX1000WalletV2JettonTransferMsg;
use crate::contracts::custom_wallet::v2::msgs::DedustX1000WalletV2OutgoingMsg;
use crate::contracts::custom_wallet::v2::msgs::DedustX1000WalletV2SignedRequestMsg;
use crate::contracts::custom_wallet::v2::storage::DedustX1000BidaskAdditionalData;
use crate::contracts::custom_wallet::v2::storage::DedustX1000BidaskDammReferral;
use crate::contracts::custom_wallet::v2::storage::DedustX1000BidaskNormalSlippage;
use crate::contracts::custom_wallet::v2::storage::DedustX1000BidaskPartialSlippage;
use crate::contracts::custom_wallet::v2::storage::DedustX1000BidaskSlippage;
use crate::contracts::custom_wallet::v2::storage::DedustX1000CoffeeNotification;
use crate::contracts::custom_wallet::v2::storage::DedustX1000CoffeeNotifications;
use crate::contracts::custom_wallet::v2::storage::DedustX1000CoffeeSwapParams;
use crate::contracts::custom_wallet::v2::storage::DedustX1000CoffeeSwapStep;
use crate::contracts::custom_wallet::v2::storage::DedustX1000CpmmNextStep;
use crate::contracts::custom_wallet::v2::storage::DedustX1000CpmmPartnerConfig;
use crate::contracts::custom_wallet::v2::storage::DedustX1000CpmmPayoutConfig;
use crate::contracts::custom_wallet::v2::storage::DedustX1000CpmmPayoutOptions;
use crate::contracts::custom_wallet::v2::storage::DedustX1000CpmmReferrerConfig;
use crate::contracts::custom_wallet::v2::storage::DedustX1000GivenIn;
use crate::contracts::custom_wallet::v2::storage::DedustX1000GivenOut;
use crate::contracts::custom_wallet::v2::storage::DedustX1000MoonCallback;
use crate::contracts::custom_wallet::v2::storage::DedustX1000MoonClmmCallback;
use crate::contracts::custom_wallet::v2::storage::DedustX1000MoonSwapParams;
use crate::contracts::custom_wallet::v2::storage::DedustX1000StonfiV2SwapBody;
use crate::contracts::custom_wallet::v2::storage::DedustX1000SwapKind;
use crate::contracts::custom_wallet::v2::storage::DedustX1000SwapParams;
use crate::contracts::custom_wallet::v2::storage::DedustX1000SwapStep;
use crate::contracts::custom_wallet::v2::storage::DedustX1000ToncoMultihop;
use crate::contracts::custom_wallet::v2::storage::DedustX1000UranusPartnerConfig;
use crate::contracts::custom_wallet::v2::storage::DedustX1000UranusReferrerConfig;
use crate::contracts::custom_wallet::v2::storage::DedustX1000WalletV2Storage;
use crate::contracts::operator::v2::msgs::DedustOperatorV2CollectFeesMsg;
use crate::contracts::operator::v2::msgs::DedustOperatorV2ConfigureStartTimeMsg;
use crate::contracts::operator::v2::msgs::DedustOperatorV2ForwardMsg;
use crate::contracts::operator::v2::msgs::DedustOperatorV2IncomingMsg;
use crate::contracts::operator::v2::msgs::DedustOperatorV2OutgoingMsg;
use crate::contracts::operator::v2::msgs::DedustOperatorV2SetUserAddressMsg;
use crate::contracts::uranus::factory_v3::msgs::DedustUranusFactoryV3DeployCustomizedMemeMsg;
use crate::contracts::uranus::factory_v3::msgs::DedustUranusFactoryV3DeployMemeMsg;
use crate::contracts::uranus::factory_v3::msgs::DedustUranusFactoryV3IncomingMsg;
use crate::contracts::uranus::factory_v3::msgs::DedustUranusFactoryV3OutgoingMsg;
use crate::contracts::uranus::factory_v3::storage::DedustUranusFactoryV3Storage;
use crate::contracts::uranus::meme_v2::msgs::DedustUranusMemeV2BurnNotificationMsg;
use crate::contracts::uranus::meme_v2::msgs::DedustUranusMemeV2BuyEventMsg;
use crate::contracts::uranus::meme_v2::msgs::DedustUranusMemeV2BuyMsg;
use crate::contracts::uranus::meme_v2::msgs::DedustUranusMemeV2ClaimCreatorFeeMsg;
use crate::contracts::uranus::meme_v2::msgs::DedustUranusMemeV2CpmmInitResultMsg;
use crate::contracts::uranus::meme_v2::msgs::DedustUranusMemeV2ExcessesMsg;
use crate::contracts::uranus::meme_v2::msgs::DedustUranusMemeV2IncomingMsg;
use crate::contracts::uranus::meme_v2::msgs::DedustUranusMemeV2InitMemeMsg;
use crate::contracts::uranus::meme_v2::msgs::DedustUranusMemeV2Opcode66c8ad72Msg;
use crate::contracts::uranus::meme_v2::msgs::DedustUranusMemeV2OpcodeF14b54f3Msg;
use crate::contracts::uranus::meme_v2::msgs::DedustUranusMemeV2OutgoingMsg;
use crate::contracts::uranus::meme_v2::msgs::DedustUranusMemeV2ProvideWalletAddressMsg;
use crate::contracts::uranus::meme_v2::msgs::DedustUranusMemeV2SellEventMsg;
use crate::contracts::uranus::meme_v2::msgs::DedustUranusMemeV2SellMsg;
use crate::contracts::uranus::meme_v2::msgs::DedustUranusMemeV2TakeWalletAddressMsg;
use crate::contracts::uranus::meme_v2::storage::DedustUranusMemeV2BondingCurve;
use crate::contracts::uranus::meme_v2::storage::DedustUranusMemeV2Config;
use crate::contracts::uranus::meme_v2::storage::DedustUranusMemeV2Metadata;
use crate::contracts::uranus::meme_v2::storage::DedustUranusMemeV2PartnerConfig;
use crate::contracts::uranus::meme_v2::storage::DedustUranusMemeV2ReferrerConfig;
use crate::contracts::uranus::meme_v2::storage::DedustUranusMemeV2Storage;
use crate::contracts::uranus::meme_v2::storage::DedustUranusMemeV2TradeFees;
use crate::contracts::uranus::meme_v3::msgs::DedustUranusMemeV3BuyEventMsg;
use crate::contracts::uranus::meme_v3::msgs::DedustUranusMemeV3BuyMsg;
use crate::contracts::uranus::meme_v3::msgs::DedustUranusMemeV3ClaimCreatorFeeMsg;
use crate::contracts::uranus::meme_v3::msgs::DedustUranusMemeV3ClaimPartnerFeeMsg;
use crate::contracts::uranus::meme_v3::msgs::DedustUranusMemeV3CpmmInitResultMsg;
use crate::contracts::uranus::meme_v3::msgs::DedustUranusMemeV3IncomingMsg;
use crate::contracts::uranus::meme_v3::msgs::DedustUranusMemeV3Opcode66c8ad72Msg;
use crate::contracts::uranus::meme_v3::msgs::DedustUranusMemeV3OpcodeF14b54f3Msg;
use crate::contracts::uranus::meme_v3::msgs::DedustUranusMemeV3OutgoingMsg;
use crate::contracts::uranus::meme_v3::msgs::DedustUranusMemeV3ProvideWalletAddressMsg;
use crate::contracts::uranus::meme_v3::msgs::DedustUranusMemeV3SellEventMsg;
use crate::contracts::uranus::meme_v3::msgs::DedustUranusMemeV3TakeWalletAddressMsg;
use crate::contracts::uranus::meme_v3::storage::DedustUranusMemeV3BondingCurve;
use crate::contracts::uranus::meme_v3::storage::DedustUranusMemeV3Config;
use crate::contracts::uranus::meme_v3::storage::DedustUranusMemeV3Storage;
use crate::contracts::uranus::meme_v3::storage::DedustUranusMemeV3TradeFees;
use crate::contracts::uranus::wallet_v2::msgs::DedustUranusMemeWalletV2IncomingMsg;
use crate::contracts::uranus::wallet_v2::msgs::DedustUranusMemeWalletV2OutgoingMsg;
use crate::contracts::uranus::wallet_v2::storage::DedustUranusMemeWalletV2Storage;
use crate::contracts::uranus::wallet_v3::msgs::DedustUranusMemeWalletV3IncomingMsg;
use crate::contracts::uranus::wallet_v3::msgs::DedustUranusMemeWalletV3OutgoingMsg;
use crate::contracts::uranus::wallet_v3::storage::DedustUranusMemeWalletV3Storage;
use fastnum::U256;
use fastnum::U512;
use std::collections::HashMap;
use std::{str::FromStr, sync::OnceLock};
use ton::ton_core::types::TonAddress;
use ton::ton_core::types::tlb_core::EitherRefLayout;
use ton::ton_core::types::tlb_core::VarLenBytes;
use ton::ton_core::{cell::TonCell, traits::tlb::TLB};
fn fixture(id: &str) -> anyhow::Result<TonCell> {
    static VALUES: OnceLock<Result<HashMap<String, String>, String>> = OnceLock::new();
    let values = VALUES
        .get_or_init(|| serde_json::from_str(include_str!("fixtures/schema-vectors.json")).map_err(|e| e.to_string()))
        .as_ref()
        .map_err(|e| anyhow::anyhow!("{e}"))?;
    let value = values.get(id).ok_or_else(|| anyhow::anyhow!("missing {id}"))?;
    Ok(TonCell::from_boc_base64(value)?)
}

#[test]
fn test_schema_DedustFactoryV1_DedustFactoryV1Storage() -> anyhow::Result<()> {
    let body = fixture("schema_DedustFactoryV1_DedustFactoryV1Storage")?;
    let decoded: DedustFactoryV1Storage = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.owner_address,
        TonAddress::from_str("0:000000000000000000000000000000000000000000000000000000000000000c")?
    );
    assert_eq!(
        decoded.pending_owner_address,
        Some(TonAddress::from_str(
            "0:000000000000000000000000000000000000000000000000000000000000000d"
        )?)
    );
    assert_eq!(decoded.can_accept_after, 1053);
    assert_eq!(decoded.version, 1126);
    assert_eq!(decoded.config.pool_code_version, 1418);
    assert_eq!(decoded.config.liquidity_deposit_code_version, 1564);
    assert!(decoded.config.vault_code_by_asset_type.is_empty());
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustFactoryV1_DedustFactoryV1Config() -> anyhow::Result<()> {
    let body = fixture("schema_DedustFactoryV1_DedustFactoryV1Config")?;
    let decoded: DedustFactoryV1Config = TLB::from_cell(&body)?;
    assert_eq!(decoded.pool_code_version, 1856);
    assert_eq!(decoded.liquidity_deposit_code_version, 2002);
    assert!(decoded.vault_code_by_asset_type.is_empty());
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustFactoryV1_DedustFactoryV1NativeAsset() -> anyhow::Result<()> {
    let body = fixture("schema_DedustFactoryV1_DedustFactoryV1NativeAsset")?;
    let decoded: DedustClassicNativeAsset = TLB::from_cell(&body)?;

    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustFactoryV1_DedustFactoryV1JettonAsset() -> anyhow::Result<()> {
    let body = fixture("schema_DedustFactoryV1_DedustFactoryV1JettonAsset")?;
    let decoded: DedustClassicJettonAsset = TLB::from_cell(&body)?;
    assert_eq!(decoded.workchain_id, -63);
    assert_eq!(
        decoded.address.to_hex(),
        "0000000000000000000000000000000000000000000000000000000000000988"
    );
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustFactoryV1_DedustFactoryV1PoolParams() -> anyhow::Result<()> {
    let body = fixture("schema_DedustFactoryV1_DedustFactoryV1PoolParams")?;
    let decoded: DedustFactoryV1PoolParams = TLB::from_cell(&body)?;
    assert!(decoded.is_stable);
    assert!(matches!(&decoded.asset0, DedustClassicAsset::Native(_)));
    assert!(matches!(&decoded.asset1, DedustClassicAsset::Native(_)));
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustFactoryV1_DedustFactoryV1VaultProof() -> anyhow::Result<()> {
    let body = fixture("schema_DedustFactoryV1_DedustFactoryV1VaultProof")?;
    let decoded: DedustFactoryV1VaultProof = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.factory_address,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000029")?
    );
    assert_eq!(decoded.contract_type, 25);
    let variant_43 = match &decoded.asset {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_43.workchain_id, -116);
    assert_eq!(
        variant_43.address.to_hex(),
        "0000000000000000000000000000000000000000000000000000000000000d3d"
    );
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustFactoryV1_DedustFactoryV1CreateLiquidityDepositExtra() -> anyhow::Result<()> {
    let body = fixture("schema_DedustFactoryV1_DedustFactoryV1CreateLiquidityDepositExtra")?;
    let decoded: DedustFactoryV1CreateLiquidityDepositExtra = TLB::from_cell(&body)?;
    assert_eq!(*decoded.asset0_target_balance, 3535);
    assert_eq!(*decoded.asset1_target_balance, 3608);
    assert!(matches!(&decoded.deposit_asset, DedustClassicAsset::Native(_)));
    assert_eq!(*decoded.deposit_amount, 3827);
    assert_eq!(*decoded.min_lp_amount, 3900);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustFactoryV1_DedustFactoryV1Install() -> anyhow::Result<()> {
    let body = fixture("schema_DedustFactoryV1_DedustFactoryV1Install")?;
    let decoded: DedustFactoryV1InstallMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 4046);
    assert_eq!(decoded.code_version, 4192);
    let dispatched = DedustFactoryV1OutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustFactoryV1OutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustFactoryV1_DedustFactoryV1UpgradeChildCode() -> anyhow::Result<()> {
    let body = fixture("schema_DedustFactoryV1_DedustFactoryV1UpgradeChildCode")?;
    let decoded: DedustFactoryV1UpgradeChildCodeMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 4484);
    assert_eq!(decoded.code_version, 4557);
    let dispatched = DedustFactoryV1OutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustFactoryV1OutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustFactoryV1_DedustFactoryV1DestroyNonReadyVaultCommand() -> anyhow::Result<()> {
    let body = fixture("schema_DedustFactoryV1_DedustFactoryV1DestroyNonReadyVaultCommand")?;
    let decoded: DedustFactoryV1DestroyNonReadyVaultCommandMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 4776);
    let dispatched = DedustFactoryV1OutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustFactoryV1OutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustFactoryV1_DedustFactoryV1DepositAsset() -> anyhow::Result<()> {
    let body = fixture("schema_DedustFactoryV1_DedustFactoryV1DepositAsset")?;
    let decoded: DedustFactoryV1DepositAssetMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 4922);
    assert!(matches!(&decoded.asset, DedustClassicAsset::Native(_)));
    assert_eq!(*decoded.amount, 5141);
    let dispatched = DedustFactoryV1OutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustFactoryV1OutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustFactoryV1_DedustFactoryV1TransferOwnership() -> anyhow::Result<()> {
    let body = fixture("schema_DedustFactoryV1_DedustFactoryV1TransferOwnership")?;
    let decoded: DedustFactoryV1TransferOwnershipMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 5287);
    assert_eq!(
        decoded.new_owner_address,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000049"
        )?)
    );
    let dispatched = DedustFactoryV1IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustFactoryV1IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustFactoryV1_DedustFactoryV1AcceptOwnership() -> anyhow::Result<()> {
    let body = fixture("schema_DedustFactoryV1_DedustFactoryV1AcceptOwnership")?;
    let decoded: DedustFactoryV1AcceptOwnershipMsg = TLB::from_cell(&body)?;
    let dispatched = DedustFactoryV1IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustFactoryV1IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustFactoryV1_DedustFactoryV1CancelOwnershipTransfer() -> anyhow::Result<()> {
    let body = fixture("schema_DedustFactoryV1_DedustFactoryV1CancelOwnershipTransfer")?;
    let decoded: DedustFactoryV1CancelOwnershipTransferMsg = TLB::from_cell(&body)?;
    let dispatched = DedustFactoryV1IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustFactoryV1IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustFactoryV1_DedustFactoryV1InstallVaultCode() -> anyhow::Result<()> {
    let body = fixture("schema_DedustFactoryV1_DedustFactoryV1InstallVaultCode")?;
    let decoded: DedustFactoryV1InstallVaultCodeMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 5798);
    assert_eq!(decoded.asset_type, 15);
    assert_eq!(decoded.code_version, 5944);
    let dispatched = DedustFactoryV1IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustFactoryV1IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustFactoryV1_DedustFactoryV1CreateVault() -> anyhow::Result<()> {
    let body = fixture("schema_DedustFactoryV1_DedustFactoryV1CreateVault")?;
    let decoded: DedustFactoryV1CreateVaultMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 6163);
    let variant_85 = match &decoded.asset {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_85.workchain_id, -110);
    assert_eq!(
        variant_85.address.to_hex(),
        "0000000000000000000000000000000000000000000000000000000000001937"
    );
    let dispatched = DedustFactoryV1IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustFactoryV1IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustFactoryV1_DedustFactoryV1CreateLegacyJettonVault() -> anyhow::Result<()> {
    let body = fixture("schema_DedustFactoryV1_DedustFactoryV1CreateLegacyJettonVault")?;
    let decoded: DedustFactoryV1CreateLegacyJettonVaultMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 6601);
    assert_eq!(
        decoded.minter_address,
        TonAddress::from_str("0:000000000000000000000000000000000000000000000000000000000000005b")?
    );
    assert_eq!(
        decoded.resolver_address,
        TonAddress::from_str("0:000000000000000000000000000000000000000000000000000000000000005c")?
    );
    let dispatched = DedustFactoryV1IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustFactoryV1IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustFactoryV1_DedustFactoryV1UpgradeVault() -> anyhow::Result<()> {
    let body = fixture("schema_DedustFactoryV1_DedustFactoryV1UpgradeVault")?;
    let decoded: DedustFactoryV1UpgradeVaultMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 6893);
    let variant_95 = match &decoded.asset {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_95.workchain_id, -72);
    assert_eq!(
        variant_95.address.to_hex(),
        "0000000000000000000000000000000000000000000000000000000000001c11"
    );
    let dispatched = DedustFactoryV1IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustFactoryV1IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustFactoryV1_DedustFactoryV1DestroyNonReadyVault() -> anyhow::Result<()> {
    let body = fixture("schema_DedustFactoryV1_DedustFactoryV1DestroyNonReadyVault")?;
    let decoded: DedustFactoryV1DestroyNonReadyVaultMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 7331);
    let variant_101 = match &decoded.asset {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_101.workchain_id, -126);
    assert_eq!(
        variant_101.address.to_hex(),
        "0000000000000000000000000000000000000000000000000000000000001dc7"
    );
    let dispatched = DedustFactoryV1IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustFactoryV1IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustFactoryV1_DedustFactoryV1Upgrade() -> anyhow::Result<()> {
    let body = fixture("schema_DedustFactoryV1_DedustFactoryV1Upgrade")?;
    let decoded: DedustFactoryV1UpgradeMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 7769);
    assert_eq!(decoded.code_version, 7842);
    let dispatched = DedustFactoryV1IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustFactoryV1IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustFactoryV1_DedustFactoryV1ResetGas() -> anyhow::Result<()> {
    let body = fixture("schema_DedustFactoryV1_DedustFactoryV1ResetGas")?;
    let decoded: DedustFactoryV1ResetGasMsg = TLB::from_cell(&body)?;
    let dispatched = DedustFactoryV1IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustFactoryV1IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustFactoryV1_DedustFactoryV1InstallPoolCode() -> anyhow::Result<()> {
    let body = fixture("schema_DedustFactoryV1_DedustFactoryV1InstallPoolCode")?;
    let decoded: DedustFactoryV1InstallPoolCodeMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 8207);
    assert_eq!(decoded.code_version, 8280);
    let dispatched = DedustFactoryV1IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustFactoryV1IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustFactoryV1_DedustFactoryV1CreateVolatilePool() -> anyhow::Result<()> {
    let body = fixture("schema_DedustFactoryV1_DedustFactoryV1CreateVolatilePool")?;
    let decoded: DedustFactoryV1CreateVolatilePoolMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 8499);
    let variant_117 = match &decoded.asset0 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_117.workchain_id, -14);
    assert_eq!(
        variant_117.address.to_hex(),
        "0000000000000000000000000000000000000000000000000000000000002257"
    );
    let variant_121 = match &decoded.asset1 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_121.workchain_id, -50);
    assert_eq!(
        variant_121.address.to_hex(),
        "000000000000000000000000000000000000000000000000000000000000237b"
    );
    let dispatched = DedustFactoryV1IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustFactoryV1IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustFactoryV1_DedustFactoryV1CreateStablePool() -> anyhow::Result<()> {
    let body = fixture("schema_DedustFactoryV1_DedustFactoryV1CreateStablePool")?;
    let decoded: DedustFactoryV1CreateStablePoolMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 9229);
    let variant_127 = match &decoded.asset0 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_127.workchain_id, -104);
    assert_eq!(
        variant_127.address.to_hex(),
        "0000000000000000000000000000000000000000000000000000000000002531"
    );
    assert_eq!(decoded.asset0_decimals, 122);
    assert!(matches!(&decoded.asset1, DedustClassicAsset::Native(_)));
    assert_eq!(decoded.asset1_decimals, 85);
    let dispatched = DedustFactoryV1IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustFactoryV1IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustFactoryV1_DedustFactoryV1UpgradePool() -> anyhow::Result<()> {
    let body = fixture("schema_DedustFactoryV1_DedustFactoryV1UpgradePool")?;
    let decoded: DedustFactoryV1UpgradePoolMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 9959);
    assert!(!decoded.pool_params.is_stable);
    let variant_139 = match &decoded.pool_params.asset0 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_139.workchain_id, -84);
    assert_eq!(
        variant_139.address.to_hex(),
        "000000000000000000000000000000000000000000000000000000000000289d"
    );
    let variant_143 = match &decoded.pool_params.asset1 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_143.workchain_id, -120);
    assert_eq!(
        variant_143.address.to_hex(),
        "00000000000000000000000000000000000000000000000000000000000029c1"
    );
    let dispatched = DedustFactoryV1IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustFactoryV1IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustFactoryV1_DedustFactoryV1InstallLiquidityDepositCode() -> anyhow::Result<()> {
    let body = fixture("schema_DedustFactoryV1_DedustFactoryV1InstallLiquidityDepositCode")?;
    let decoded: DedustFactoryV1InstallLiquidityDepositCodeMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 10835);
    assert_eq!(decoded.code_version, 10908);
    let dispatched = DedustFactoryV1IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustFactoryV1IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustFactoryV1_DedustFactoryV1CreateLiquidityDeposit() -> anyhow::Result<()> {
    let body = fixture("schema_DedustFactoryV1_DedustFactoryV1CreateLiquidityDeposit")?;
    let decoded: DedustFactoryV1CreateLiquidityDepositMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 11127);
    assert_eq!(
        decoded.proof.factory_address,
        TonAddress::from_str("0:000000000000000000000000000000000000000000000000000000000000009b")?
    );
    assert_eq!(decoded.proof.contract_type, 155);
    let variant_157 = match &decoded.proof.asset {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_157.workchain_id, -118);
    assert_eq!(
        variant_157.address.to_hex(),
        "0000000000000000000000000000000000000000000000000000000000002dbf"
    );
    assert_eq!(
        decoded.owner_address,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000000a1")?
    );
    assert!(decoded.pool_params.is_stable);
    assert!(matches!(&decoded.pool_params.asset0, DedustClassicAsset::Native(_)));
    assert!(matches!(&decoded.pool_params.asset1, DedustClassicAsset::Native(_)));
    assert_eq!(*decoded.extra.asset0_target_balance, 12441);
    assert_eq!(*decoded.extra.asset1_target_balance, 12514);
    assert!(matches!(&decoded.extra.deposit_asset, DedustClassicAsset::Native(_)));
    assert_eq!(*decoded.extra.deposit_amount, 12733);
    assert_eq!(*decoded.extra.min_lp_amount, 12806);
    let dispatched = DedustFactoryV1IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustFactoryV1IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustFactoryV2_DedustFactoryV2Storage() -> anyhow::Result<()> {
    let body = fixture("schema_DedustFactoryV2_DedustFactoryV2Storage")?;
    let decoded: DedustFactoryV2Storage = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.owner_address,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000000b5")?
    );
    assert_eq!(
        decoded.pending_owner_address,
        Some(TonAddress::from_str(
            "0:00000000000000000000000000000000000000000000000000000000000000b6"
        )?)
    );
    assert_eq!(decoded.can_accept_after, 13390);
    assert_eq!(decoded.version, 13463);
    assert_eq!(decoded.config.pool_code_version, 13755);
    assert_eq!(decoded.config.liquidity_deposit_code_version, 13901);
    assert!(decoded.config.vault_code_by_asset_type.is_empty());
    assert_eq!(
        decoded
            .operator
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing operator"))?
            .version,
        14120
    );
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustFactoryV2_DedustFactoryV2Config() -> anyhow::Result<()> {
    let body = fixture("schema_DedustFactoryV2_DedustFactoryV2Config")?;
    let decoded: DedustFactoryV2Config = TLB::from_cell(&body)?;
    assert_eq!(decoded.pool_code_version, 14412);
    assert_eq!(decoded.liquidity_deposit_code_version, 14558);
    assert!(decoded.vault_code_by_asset_type.is_empty());
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustFactoryV2_DedustFactoryV2NativeAsset() -> anyhow::Result<()> {
    let body = fixture("schema_DedustFactoryV2_DedustFactoryV2NativeAsset")?;
    let decoded: DedustClassicNativeAsset = TLB::from_cell(&body)?;

    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustFactoryV2_DedustFactoryV2JettonAsset() -> anyhow::Result<()> {
    let body = fixture("schema_DedustFactoryV2_DedustFactoryV2JettonAsset")?;
    let decoded: DedustClassicJettonAsset = TLB::from_cell(&body)?;
    assert_eq!(decoded.workchain_id, -75);
    assert_eq!(
        decoded.address.to_hex(),
        "0000000000000000000000000000000000000000000000000000000000003a94"
    );
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustFactoryV2_DedustFactoryV2PoolParams() -> anyhow::Result<()> {
    let body = fixture("schema_DedustFactoryV2_DedustFactoryV2PoolParams")?;
    let decoded: DedustFactoryV2PoolParams = TLB::from_cell(&body)?;
    assert!(decoded.is_stable);
    assert!(matches!(&decoded.asset0, DedustClassicAsset::Native(_)));
    assert!(matches!(&decoded.asset1, DedustClassicAsset::Native(_)));
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustFactoryV2_DedustFactoryV2VaultProof() -> anyhow::Result<()> {
    let body = fixture("schema_DedustFactoryV2_DedustFactoryV2VaultProof")?;
    let decoded: DedustFactoryV2VaultProof = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.factory_address,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000000d5")?
    );
    assert_eq!(decoded.contract_type, 37);
    let variant_215 = match &decoded.asset {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_215.workchain_id, 0);
    assert_eq!(
        variant_215.address.to_hex(),
        "0000000000000000000000000000000000000000000000000000000000003e49"
    );
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustFactoryV2_DedustFactoryV2CreateLiquidityDepositExtra() -> anyhow::Result<()> {
    let body = fixture("schema_DedustFactoryV2_DedustFactoryV2CreateLiquidityDepositExtra")?;
    let decoded: DedustFactoryV2CreateLiquidityDepositExtra = TLB::from_cell(&body)?;
    assert_eq!(*decoded.asset0_target_balance, 16091);
    assert_eq!(*decoded.asset1_target_balance, 16164);
    assert!(matches!(&decoded.deposit_asset, DedustClassicAsset::Native(_)));
    assert_eq!(*decoded.deposit_amount, 16383);
    assert_eq!(*decoded.min_lp_amount, 16456);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustFactoryV2_DedustFactoryV2Install() -> anyhow::Result<()> {
    let body = fixture("schema_DedustFactoryV2_DedustFactoryV2Install")?;
    let decoded: DedustFactoryV2InstallMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 16602);
    assert_eq!(decoded.code_version, 16748);
    let dispatched = DedustFactoryV2OutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustFactoryV2OutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustFactoryV2_DedustFactoryV2DepositAsset() -> anyhow::Result<()> {
    let body = fixture("schema_DedustFactoryV2_DedustFactoryV2DepositAsset")?;
    let decoded: DedustFactoryV2DepositAssetMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 17040);
    assert!(matches!(&decoded.asset, DedustClassicAsset::Native(_)));
    assert_eq!(*decoded.amount, 17259);
    let dispatched = DedustFactoryV2OutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustFactoryV2OutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustFactoryV2_DedustFactoryV2TransferOwnership() -> anyhow::Result<()> {
    let body = fixture("schema_DedustFactoryV2_DedustFactoryV2TransferOwnership")?;
    let decoded: DedustFactoryV2TransferOwnershipMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 17405);
    assert_eq!(
        decoded.new_owner_address,
        Some(TonAddress::from_str(
            "0:00000000000000000000000000000000000000000000000000000000000000ef"
        )?)
    );
    let dispatched = DedustFactoryV2IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustFactoryV2IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustFactoryV2_DedustFactoryV2AcceptOwnership() -> anyhow::Result<()> {
    let body = fixture("schema_DedustFactoryV2_DedustFactoryV2AcceptOwnership")?;
    let decoded: DedustFactoryV2AcceptOwnershipMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 17624);
    let dispatched = DedustFactoryV2IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustFactoryV2IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustFactoryV2_DedustFactoryV2CancelOwnershipTransfer() -> anyhow::Result<()> {
    let body = fixture("schema_DedustFactoryV2_DedustFactoryV2CancelOwnershipTransfer")?;
    let decoded: DedustFactoryV2CancelOwnershipTransferMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 17770);
    let dispatched = DedustFactoryV2IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustFactoryV2IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustFactoryV2_DedustFactoryV2InstallVaultCode() -> anyhow::Result<()> {
    let body = fixture("schema_DedustFactoryV2_DedustFactoryV2InstallVaultCode")?;
    let decoded: DedustFactoryV2InstallVaultCodeMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 17916);
    assert_eq!(decoded.asset_type, 5);
    assert_eq!(decoded.code_version, 18062);
    let dispatched = DedustFactoryV2IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustFactoryV2IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustFactoryV2_DedustFactoryV2CreateVault() -> anyhow::Result<()> {
    let body = fixture("schema_DedustFactoryV2_DedustFactoryV2CreateVault")?;
    let decoded: DedustFactoryV2CreateVaultMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 18281);
    let variant_251 = match &decoded.asset {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_251.workchain_id, -68);
    assert_eq!(
        variant_251.address.to_hex(),
        "000000000000000000000000000000000000000000000000000000000000488d"
    );
    let dispatched = DedustFactoryV2IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustFactoryV2IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustFactoryV2_DedustFactoryV2CreateLegacyJettonVault() -> anyhow::Result<()> {
    let body = fixture("schema_DedustFactoryV2_DedustFactoryV2CreateLegacyJettonVault")?;
    let decoded: DedustFactoryV2CreateLegacyJettonVaultMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 18719);
    assert_eq!(
        decoded.minter_address,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000101")?
    );
    assert_eq!(
        decoded.resolver_address,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000102")?
    );
    let dispatched = DedustFactoryV2IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustFactoryV2IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustFactoryV2_DedustFactoryV2UpgradeVault() -> anyhow::Result<()> {
    let body = fixture("schema_DedustFactoryV2_DedustFactoryV2UpgradeVault")?;
    let decoded: DedustFactoryV2UpgradeVaultMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 19011);
    let variant_261 = match &decoded.asset {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_261.workchain_id, -30);
    assert_eq!(
        variant_261.address.to_hex(),
        "0000000000000000000000000000000000000000000000000000000000004b67"
    );
    let dispatched = DedustFactoryV2IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustFactoryV2IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustFactoryV2_DedustFactoryV2Upgrade() -> anyhow::Result<()> {
    let body = fixture("schema_DedustFactoryV2_DedustFactoryV2Upgrade")?;
    let decoded: DedustFactoryV2UpgradeMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 19449);
    assert_eq!(decoded.code_version, 19522);
    let dispatched = DedustFactoryV2IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustFactoryV2IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustFactoryV2_DedustFactoryV2ResetGas() -> anyhow::Result<()> {
    let body = fixture("schema_DedustFactoryV2_DedustFactoryV2ResetGas")?;
    let decoded: DedustFactoryV2ResetGasMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 19741);
    let dispatched = DedustFactoryV2IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustFactoryV2IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustFactoryV2_DedustFactoryV2DeployOperator() -> anyhow::Result<()> {
    let body = fixture("schema_DedustFactoryV2_DedustFactoryV2DeployOperator")?;
    let decoded: DedustFactoryV2DeployOperatorMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 19887);
    assert_eq!(decoded.role, 248);
    assert_eq!(
        decoded.user_address,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000112")?
    );
    let dispatched = DedustFactoryV2IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustFactoryV2IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustFactoryV2_DedustFactoryV2ChangeOperatorUserAddress() -> anyhow::Result<()> {
    let body = fixture("schema_DedustFactoryV2_DedustFactoryV2ChangeOperatorUserAddress")?;
    let decoded: DedustFactoryV2ChangeOperatorUserAddressMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 20179);
    assert_eq!(decoded.role, 28);
    assert_eq!(
        decoded.user_address,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000116")?
    );
    let dispatched = DedustFactoryV2IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustFactoryV2IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustFactoryV2_DedustFactoryV2InstallOperatorCode() -> anyhow::Result<()> {
    let body = fixture("schema_DedustFactoryV2_DedustFactoryV2InstallOperatorCode")?;
    let decoded: DedustFactoryV2InstallOperatorCodeMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 20471);
    assert_eq!(decoded.code_version, 20544);
    let dispatched = DedustFactoryV2IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustFactoryV2IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustFactoryV2_DedustFactoryV2UpgradeOperator() -> anyhow::Result<()> {
    let body = fixture("schema_DedustFactoryV2_DedustFactoryV2UpgradeOperator")?;
    let decoded: DedustFactoryV2UpgradeOperatorMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 20763);
    assert_eq!(decoded.role, 100);
    let dispatched = DedustFactoryV2IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustFactoryV2IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustFactoryV2_DedustFactoryV2InstallPoolCode() -> anyhow::Result<()> {
    let body = fixture("schema_DedustFactoryV2_DedustFactoryV2InstallPoolCode")?;
    let decoded: DedustFactoryV2InstallPoolCodeMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 20982);
    assert_eq!(decoded.code_version, 21055);
    let dispatched = DedustFactoryV2IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustFactoryV2IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustFactoryV2_DedustFactoryV2CreateVolatilePool() -> anyhow::Result<()> {
    let body = fixture("schema_DedustFactoryV2_DedustFactoryV2CreateVolatilePool")?;
    let decoded: DedustFactoryV2CreateVolatilePoolMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 21274);
    assert!(matches!(&decoded.asset0, DedustClassicAsset::Native(_)));
    assert!(matches!(&decoded.asset1, DedustClassicAsset::Native(_)));
    let dispatched = DedustFactoryV2IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustFactoryV2IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustFactoryV2_DedustFactoryV2CreateStablePool() -> anyhow::Result<()> {
    let body = fixture("schema_DedustFactoryV2_DedustFactoryV2CreateStablePool")?;
    let decoded: DedustFactoryV2CreateStablePoolMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 21712);
    assert!(matches!(&decoded.asset0, DedustClassicAsset::Native(_)));
    assert_eq!(decoded.asset0_decimals, 171);
    let variant_301 = match &decoded.asset1 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_301.workchain_id, -6);
    assert_eq!(
        variant_301.address.to_hex(),
        "00000000000000000000000000000000000000000000000000000000000056cf"
    );
    assert_eq!(decoded.asset1_decimals, 24);
    let dispatched = DedustFactoryV2IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustFactoryV2IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustFactoryV2_DedustFactoryV2UpgradePool() -> anyhow::Result<()> {
    let body = fixture("schema_DedustFactoryV2_DedustFactoryV2UpgradePool")?;
    let decoded: DedustFactoryV2UpgradePoolMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 22442);
    assert!(decoded.pool_params.is_stable);
    assert!(matches!(&decoded.pool_params.asset0, DedustClassicAsset::Native(_)));
    assert!(matches!(&decoded.pool_params.asset1, DedustClassicAsset::Native(_)));
    let dispatched = DedustFactoryV2IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustFactoryV2IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustFactoryV2_DedustFactoryV2InstallLiquidityDepositCode() -> anyhow::Result<()> {
    let body = fixture("schema_DedustFactoryV2_DedustFactoryV2InstallLiquidityDepositCode")?;
    let decoded: DedustFactoryV2InstallLiquidityDepositCodeMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 23026);
    assert_eq!(decoded.code_version, 23099);
    let dispatched = DedustFactoryV2IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustFactoryV2IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustFactoryV2_DedustFactoryV2CreateLiquidityDeposit() -> anyhow::Result<()> {
    let body = fixture("schema_DedustFactoryV2_DedustFactoryV2CreateLiquidityDeposit")?;
    let decoded: DedustFactoryV2CreateLiquidityDepositMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 23318);
    assert_eq!(
        decoded.proof.factory_address,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000142")?
    );
    assert_eq!(decoded.proof.contract_type, 58);
    assert!(matches!(&decoded.proof.asset, DedustClassicAsset::Native(_)));
    assert_eq!(
        decoded.owner_address,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000146")?
    );
    assert!(!decoded.pool_params.is_stable);
    let variant_329 = match &decoded.pool_params.asset0 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_329.workchain_id, -2);
    assert_eq!(
        variant_329.address.to_hex(),
        "0000000000000000000000000000000000000000000000000000000000005ecb"
    );
    let variant_333 = match &decoded.pool_params.asset1 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_333.workchain_id, -38);
    assert_eq!(
        variant_333.address.to_hex(),
        "0000000000000000000000000000000000000000000000000000000000005fef"
    );
    assert_eq!(*decoded.extra.asset0_target_balance, 24778);
    assert_eq!(*decoded.extra.asset1_target_balance, 24851);
    let variant_341 = match &decoded.extra.deposit_asset {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_341.workchain_id, -110);
    assert_eq!(
        variant_341.address.to_hex(),
        "0000000000000000000000000000000000000000000000000000000000006237"
    );
    assert_eq!(*decoded.extra.deposit_amount, 25216);
    assert_eq!(*decoded.extra.min_lp_amount, 25289);
    let dispatched = DedustFactoryV2IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustFactoryV2IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustFactoryV2_DedustFactoryV2DestroyNonReadyVault() -> anyhow::Result<()> {
    let body = fixture("schema_DedustFactoryV2_DedustFactoryV2DestroyNonReadyVault")?;
    let decoded: DedustFactoryV2DestroyNonReadyVaultMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 25727);
    let variant_353 = match &decoded.asset {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_353.workchain_id, -90);
    assert_eq!(
        variant_353.address.to_hex(),
        "00000000000000000000000000000000000000000000000000000000000065a3"
    );
    let dispatched = DedustFactoryV2IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustFactoryV2IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV1_DedustPoolV1Storage() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV1_DedustPoolV1Storage")?;
    let decoded: DedustPoolV1Storage = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.proof.factory_addr,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000168")?
    );
    assert!(matches!(&decoded.proof.params.pool_type, DedustPoolV1PoolType::Stable(_)));
    let variant_365 = match &decoded.proof.params.asset0 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_365.workchain_id, -70);
    assert_eq!(
        variant_365.address.to_hex(),
        "000000000000000000000000000000000000000000000000000000000000690f"
    );
    let variant_369 = match &decoded.proof.params.asset1 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_369.workchain_id, -106);
    assert_eq!(
        variant_369.address.to_hex(),
        "0000000000000000000000000000000000000000000000000000000000006a33"
    );
    assert_eq!(decoded.config.asset0_decimals, 160);
    assert_eq!(decoded.config.asset1_decimals, 233);
    assert_eq!(
        decoded.config.asset0_vault_addr,
        TonAddress::from_str("0:000000000000000000000000000000000000000000000000000000000000017b")?
    );
    assert_eq!(
        decoded.config.asset1_vault_addr,
        TonAddress::from_str("0:000000000000000000000000000000000000000000000000000000000000017c")?
    );
    assert!(decoded.config.fee_collector_addr.is_none());
    assert_eq!(decoded.version, 27917);
    assert_eq!(decoded.trade_fee_numerator, 27990);
    assert_eq!(*decoded.total_supply, 28063);
    assert_eq!(*decoded.reserve0, 28136);
    assert_eq!(*decoded.reserve1, 28209);
    assert_eq!(*decoded.asset0_protocol_fee, 28282);
    assert_eq!(*decoded.asset1_protocol_fee, 28355);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV1_DedustPoolV1Config() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV1_DedustPoolV1Config")?;
    let decoded: DedustPoolV1Config = TLB::from_cell(&body)?;
    assert_eq!(decoded.asset0_decimals, 85);
    assert_eq!(decoded.asset1_decimals, 158);
    assert_eq!(
        decoded.asset0_vault_addr,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000188")?
    );
    assert_eq!(
        decoded.asset1_vault_addr,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000189")?
    );
    assert_eq!(
        decoded.fee_collector_addr,
        Some(TonAddress::from_str(
            "0:000000000000000000000000000000000000000000000000000000000000018a"
        )?)
    );
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV1_DedustPoolV1NativeAsset() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV1_DedustPoolV1NativeAsset")?;
    let decoded: DedustClassicNativeAsset = TLB::from_cell(&body)?;

    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV1_DedustPoolV1JettonAsset() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV1_DedustPoolV1JettonAsset")?;
    let decoded: DedustClassicJettonAsset = TLB::from_cell(&body)?;
    assert_eq!(decoded.workchain_id, -84);
    assert_eq!(
        decoded.address.to_hex(),
        "000000000000000000000000000000000000000000000000000000000000719d"
    );
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV1_DedustPoolV1VolatilePool() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV1_DedustPoolV1VolatilePool")?;
    let decoded: DedustPoolV1VolatilePool = TLB::from_cell(&body)?;

    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV1_DedustPoolV1StablePool() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV1_DedustPoolV1StablePool")?;
    let decoded: DedustPoolV1StablePool = TLB::from_cell(&body)?;

    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV1_DedustPoolV1PoolParams() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV1_DedustPoolV1PoolParams")?;
    let decoded: DedustPoolV1PoolParams = TLB::from_cell(&body)?;
    assert!(matches!(&decoded.pool_type, DedustPoolV1PoolType::Volatile(_)));
    assert!(matches!(&decoded.asset0, DedustClassicAsset::Native(_)));
    assert!(matches!(&decoded.asset1, DedustClassicAsset::Native(_)));
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV1_DedustPoolV1VaultContractType() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV1_DedustPoolV1VaultContractType")?;
    let decoded: DedustPoolV1VaultContractType = TLB::from_cell(&body)?;

    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV1_DedustPoolV1PoolContractType() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV1_DedustPoolV1PoolContractType")?;
    let decoded: DedustPoolV1PoolContractType = TLB::from_cell(&body)?;

    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV1_DedustPoolV1LiquidityDepositContractType() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV1_DedustPoolV1LiquidityDepositContractType")?;
    let decoded: DedustPoolV1LiquidityDepositContractType = TLB::from_cell(&body)?;

    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV1_DedustPoolV1VaultProof() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV1_DedustPoolV1VaultProof")?;
    let decoded: DedustPoolV1VaultProof = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.factory_addr,
        TonAddress::from_str("0:000000000000000000000000000000000000000000000000000000000000019c")?
    );
    assert!(matches!(&decoded.params, DedustClassicAsset::Native(_)));
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV1_DedustPoolV1PoolProof() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV1_DedustPoolV1PoolProof")?;
    let decoded: DedustPoolV1PoolProof = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.factory_addr,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000001a1")?
    );
    assert!(matches!(&decoded.params.pool_type, DedustPoolV1PoolType::Volatile(_)));
    assert!(matches!(&decoded.params.asset0, DedustClassicAsset::Native(_)));
    assert!(matches!(&decoded.params.asset1, DedustClassicAsset::Native(_)));
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV1_DedustPoolV1LiquidityDepositProofParams() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV1_DedustPoolV1LiquidityDepositProofParams")?;
    let decoded: DedustPoolV1LiquidityDepositProofParams = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.owner_addr,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000001ab")?
    );
    assert!(matches!(&decoded.pool_params.pool_type, DedustPoolV1PoolType::Stable(_)));
    let variant_431 = match &decoded.pool_params.asset0 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_431.workchain_id, -24);
    assert_eq!(
        variant_431.address.to_hex(),
        "0000000000000000000000000000000000000000000000000000000000007be1"
    );
    let variant_435 = match &decoded.pool_params.asset1 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_435.workchain_id, -60);
    assert_eq!(
        variant_435.address.to_hex(),
        "0000000000000000000000000000000000000000000000000000000000007d05"
    );
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV1_DedustPoolV1LiquidityDepositProof() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV1_DedustPoolV1LiquidityDepositProof")?;
    let decoded: DedustPoolV1LiquidityDepositProof = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.factory_addr,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000001b8")?
    );
    assert_eq!(
        decoded.params.owner_addr,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000001bc")?
    );
    assert!(matches!(
        &decoded.params.pool_params.pool_type,
        DedustPoolV1PoolType::Volatile(_)
    ));
    assert!(matches!(&decoded.params.pool_params.asset0, DedustClassicAsset::Native(_)));
    assert!(matches!(&decoded.params.pool_params.asset1, DedustClassicAsset::Native(_)));
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV1_DedustPoolV1GivenIn() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV1_DedustPoolV1GivenIn")?;
    let decoded: DedustPoolV1GivenIn = TLB::from_cell(&body)?;

    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV1_DedustPoolV1GivenOut() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV1_DedustPoolV1GivenOut")?;
    let decoded: DedustPoolV1GivenOut = TLB::from_cell(&body)?;

    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV1_DedustPoolV1SwapStepParams() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV1_DedustPoolV1SwapStepParams")?;
    let decoded: DedustPoolV1SwapStepParams = TLB::from_cell(&body)?;
    assert!(matches!(&decoded.kind, DedustPoolV1SwapKind::GivenOut(_)));
    assert_eq!(*decoded.limit, 33392);
    assert_eq!(
        decoded
            .next
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .pool_addr,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000001cd")?
    );
    assert!(matches!(
        &decoded
            .next
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .params
            .kind,
        DedustPoolV1SwapKind::GivenOut(_)
    ));
    assert_eq!(
        *decoded
            .next
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .params
            .limit,
        33976
    );
    assert!(
        decoded
            .next
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .params
            .next
            .is_none()
    );
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV1_DedustPoolV1SwapStep() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV1_DedustPoolV1SwapStep")?;
    let decoded: DedustPoolV1SwapStep = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.pool_addr,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000001d4")?
    );
    assert!(matches!(&decoded.params.kind, DedustPoolV1SwapKind::GivenIn(_)));
    assert_eq!(*decoded.params.limit, 34487);
    assert_eq!(
        decoded
            .params
            .next
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .pool_addr,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000001dc")?
    );
    assert!(matches!(
        &decoded
            .params
            .next
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .params
            .kind,
        DedustPoolV1SwapKind::GivenIn(_)
    ));
    assert_eq!(
        *decoded
            .params
            .next
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .params
            .limit,
        35071
    );
    assert!(
        decoded
            .params
            .next
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .params
            .next
            .is_none()
    );
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV1_DedustPoolV1SwapParams() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV1_DedustPoolV1SwapParams")?;
    let decoded: DedustPoolV1SwapParams = TLB::from_cell(&body)?;
    assert_eq!(decoded.deadline, 35290);
    assert_eq!(
        decoded.recipient_addr,
        Some(TonAddress::from_str(
            "0:00000000000000000000000000000000000000000000000000000000000001e4"
        )?)
    );
    assert_eq!(
        decoded.referral_addr,
        Some(TonAddress::from_str(
            "0:00000000000000000000000000000000000000000000000000000000000001e5"
        )?)
    );
    assert!(decoded.fulfill_payload.is_none());
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV1_DedustPoolV1DepositLiquidityAllAmounts() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV1_DedustPoolV1DepositLiquidityAllAmounts")?;
    let decoded: DedustPoolV1DepositLiquidityAllAmounts = TLB::from_cell(&body)?;
    assert!(matches!(&decoded.asset0, DedustClassicAsset::Native(_)));
    assert_eq!(*decoded.asset0_amount, 35947);
    let variant_493 = match &decoded.asset1 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_493.workchain_id, -70);
    assert_eq!(
        variant_493.address.to_hex(),
        "0000000000000000000000000000000000000000000000000000000000008d8f"
    );
    assert_eq!(*decoded.asset1_amount, 36312);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV1_DedustPoolV1UpgradeCode() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV1_DedustPoolV1UpgradeCode")?;
    let decoded: DedustPoolV1UpgradeCodeMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 36458);
    assert_eq!(decoded.code_version, 36531);
    let dispatched = DedustPoolV1IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustPoolV1IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV1_DedustPoolV1SwapExternal() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV1_DedustPoolV1SwapExternal")?;
    let decoded: DedustPoolV1SwapExternalMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 36750);
    assert_eq!(
        decoded.proof.factory_addr,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000001fa")?
    );
    assert!(matches!(&decoded.proof.params, DedustClassicAsset::Native(_)));
    assert_eq!(*decoded.amount, 37261);
    assert_eq!(
        decoded.sender_addr,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000001ff")?
    );
    assert!(matches!(&decoded.current.kind, DedustPoolV1SwapKind::GivenOut(_)));
    assert_eq!(*decoded.current.limit, 37626);
    assert!(decoded.current.next.is_none());
    assert_eq!(decoded.swap_params.deadline, 37918);
    assert_eq!(
        decoded.swap_params.recipient_addr,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000208"
        )?)
    );
    assert_eq!(
        decoded.swap_params.referral_addr,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000209"
        )?)
    );
    assert!(decoded.swap_params.fulfill_payload.is_none());
    let dispatched = DedustPoolV1IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustPoolV1IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV1_DedustPoolV1SwapPeer() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV1_DedustPoolV1SwapPeer")?;
    let decoded: DedustPoolV1SwapPeerMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 38429);
    assert_eq!(
        decoded.proof.factory_addr,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000211")?
    );
    assert!(matches!(&decoded.proof.params.pool_type, DedustPoolV1PoolType::Volatile(_)));
    assert!(matches!(&decoded.proof.params.asset0, DedustClassicAsset::Native(_)));
    assert!(matches!(&decoded.proof.params.asset1, DedustClassicAsset::Native(_)));
    assert!(matches!(&decoded.asset, DedustClassicAsset::Native(_)));
    assert_eq!(*decoded.amount, 39451);
    assert_eq!(
        decoded.sender_addr,
        TonAddress::from_str("0:000000000000000000000000000000000000000000000000000000000000021d")?
    );
    assert!(matches!(&decoded.current.kind, DedustPoolV1SwapKind::GivenOut(_)));
    assert_eq!(*decoded.current.limit, 39816);
    assert!(decoded.current.next.is_none());
    assert_eq!(decoded.swap_params.deadline, 40108);
    assert_eq!(
        decoded.swap_params.recipient_addr,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000226"
        )?)
    );
    assert_eq!(
        decoded.swap_params.referral_addr,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000227"
        )?)
    );
    assert!(decoded.swap_params.fulfill_payload.is_none());
    let dispatched = DedustPoolV1IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustPoolV1IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    let dispatched = DedustPoolV1OutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustPoolV1OutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV1_DedustPoolV1DepositLiquidityAll() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV1_DedustPoolV1DepositLiquidityAll")?;
    let decoded: DedustPoolV1DepositLiquidityAllMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 40619);
    assert_eq!(
        decoded.proof.factory_addr,
        TonAddress::from_str("0:000000000000000000000000000000000000000000000000000000000000022f")?
    );
    assert_eq!(
        decoded.proof.params.owner_addr,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000233")?
    );
    assert!(matches!(
        &decoded.proof.params.pool_params.pool_type,
        DedustPoolV1PoolType::Stable(_)
    ));
    let variant_567 = match &decoded.proof.params.pool_params.asset0 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_567.workchain_id, -96);
    assert_eq!(
        variant_567.address.to_hex(),
        "000000000000000000000000000000000000000000000000000000000000a2a9"
    );
    let variant_571 = match &decoded.proof.params.pool_params.asset1 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_571.workchain_id, -4);
    assert_eq!(
        variant_571.address.to_hex(),
        "000000000000000000000000000000000000000000000000000000000000a3cd"
    );
    assert_eq!(
        decoded.owner_addr,
        TonAddress::from_str("0:000000000000000000000000000000000000000000000000000000000000023f")?
    );
    assert_eq!(*decoded.min_lp_amount, 42079);
    let variant_579 = match &decoded.deposit_amounts.asset0 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_579.workchain_id, -76);
    assert_eq!(
        variant_579.address.to_hex(),
        "000000000000000000000000000000000000000000000000000000000000a615"
    );
    assert_eq!(*decoded.deposit_amounts.asset0_amount, 42590);
    assert!(matches!(&decoded.deposit_amounts.asset1, DedustClassicAsset::Native(_)));
    assert_eq!(*decoded.deposit_amounts.asset1_amount, 42809);
    let dispatched = DedustPoolV1IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustPoolV1IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV1_DedustPoolV1JettonBurnNotification() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV1_DedustPoolV1JettonBurnNotification")?;
    let decoded: DedustPoolV1JettonBurnNotificationMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 43247);
    assert_eq!(*decoded.amount, 43320);
    assert_eq!(
        decoded.sender,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000252")?
    );
    assert_eq!(
        decoded.response_destination,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000253"
        )?)
    );
    let dispatched = DedustPoolV1IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustPoolV1IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV1_DedustPoolV1WithdrawProtocolFees() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV1_DedustPoolV1WithdrawProtocolFees")?;
    let decoded: DedustPoolV1WithdrawProtocolFeesMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 43612);
    assert_eq!(
        decoded.recipient_addr,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000256")?
    );
    let dispatched = DedustPoolV1IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustPoolV1IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV1_DedustPoolV1PayOutFromPool() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV1_DedustPoolV1PayOutFromPool")?;
    let decoded: DedustPoolV1PayOutFromPoolMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 43977);
    assert_eq!(
        decoded.proof.factory_addr,
        TonAddress::from_str("0:000000000000000000000000000000000000000000000000000000000000025d")?
    );
    assert!(matches!(&decoded.proof.params.pool_type, DedustPoolV1PoolType::Volatile(_)));
    assert!(matches!(&decoded.proof.params.asset0, DedustClassicAsset::Native(_)));
    assert!(matches!(&decoded.proof.params.asset1, DedustClassicAsset::Native(_)));
    assert_eq!(*decoded.amount, 44853);
    assert!(decoded.recipient_addr.is_none());
    let dispatched = DedustPoolV1OutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustPoolV1OutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV1_DedustPoolV1FulfillDeposit() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV1_DedustPoolV1FulfillDeposit")?;
    let decoded: DedustPoolV1FulfillDepositMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 45218);
    assert_eq!(
        decoded.proof.factory_addr,
        TonAddress::from_str("0:000000000000000000000000000000000000000000000000000000000000026e")?
    );
    assert!(matches!(&decoded.proof.params.pool_type, DedustPoolV1PoolType::Stable(_)));
    let variant_627 = match &decoded.proof.params.asset0 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_627.workchain_id, -124);
    assert_eq!(
        variant_627.address.to_hex(),
        "000000000000000000000000000000000000000000000000000000000000b3c5"
    );
    let variant_631 = match &decoded.proof.params.asset1 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_631.workchain_id, -32);
    assert_eq!(
        variant_631.address.to_hex(),
        "000000000000000000000000000000000000000000000000000000000000b4e9"
    );
    assert_eq!(*decoded.asset0_deposit, 46386);
    assert_eq!(*decoded.asset1_deposit, 46459);
    let dispatched = DedustPoolV1OutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustPoolV1OutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV1_DedustPoolV1RejectDeposit() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV1_DedustPoolV1RejectDeposit")?;
    let decoded: DedustPoolV1RejectDepositMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 46605);
    assert_eq!(
        decoded.proof.factory_addr,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000281")?
    );
    assert!(matches!(&decoded.proof.params.pool_type, DedustPoolV1PoolType::Volatile(_)));
    assert!(matches!(&decoded.proof.params.asset0, DedustClassicAsset::Native(_)));
    assert!(matches!(&decoded.proof.params.asset1, DedustClassicAsset::Native(_)));
    let dispatched = DedustPoolV1OutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustPoolV1OutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV1_DedustPoolV1JettonInternalTransfer() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV1_DedustPoolV1JettonInternalTransfer")?;
    let decoded: DedustPoolV1JettonInternalTransferMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 47700);
    assert_eq!(*decoded.amount, 47773);
    assert_eq!(
        decoded.from_address,
        TonAddress::from_str("0:000000000000000000000000000000000000000000000000000000000000028f")?
    );
    assert_eq!(
        decoded.response_address,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000290"
        )?)
    );
    assert_eq!(*decoded.forward_ton_amount, 47992);
    let dispatched = DedustPoolV1OutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustPoolV1OutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV1_DedustPoolV1SwapEventTail() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV1_DedustPoolV1SwapEventTail")?;
    let decoded: DedustPoolV1SwapEventTail = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.sender_addr,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000294")?
    );
    assert_eq!(
        decoded.referral_addr,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000295"
        )?)
    );
    assert_eq!(*decoded.reserve0, 48357);
    assert_eq!(*decoded.reserve1, 48430);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV1_DedustPoolV1SwapEvent() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV1_DedustPoolV1SwapEvent")?;
    let decoded: DedustPoolV1SwapEventMsg = TLB::from_cell(&body)?;
    let variant_665 = match &decoded.asset_in {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_665.workchain_id, -82);
    assert_eq!(
        variant_665.address.to_hex(),
        "000000000000000000000000000000000000000000000000000000000000be9b"
    );
    let variant_669 = match &decoded.asset_out {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_669.workchain_id, -118);
    assert_eq!(
        variant_669.address.to_hex(),
        "000000000000000000000000000000000000000000000000000000000000bfbf"
    );
    assert_eq!(*decoded.amount_in, 49160);
    assert_eq!(*decoded.amount_out, 49233);
    assert_eq!(
        decoded.details.sender_addr,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000002a5")?
    );
    assert!(decoded.details.referral_addr.is_none());
    assert_eq!(*decoded.details.reserve0, 49598);
    assert_eq!(*decoded.details.reserve1, 49671);
    let dispatched = DedustPoolV1OutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustPoolV1OutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV1_DedustPoolV1DepositEvent() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV1_DedustPoolV1DepositEvent")?;
    let decoded: DedustPoolV1DepositEventMsg = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.sender_addr,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000002aa")?
    );
    assert_eq!(*decoded.amount0, 49890);
    assert_eq!(*decoded.amount1, 49963);
    assert_eq!(*decoded.reserve0, 50036);
    assert_eq!(*decoded.reserve1, 50109);
    assert_eq!(*decoded.liquidity, 50182);
    let dispatched = DedustPoolV1OutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustPoolV1OutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV1_DedustPoolV1WithdrawalEvent() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV1_DedustPoolV1WithdrawalEvent")?;
    let decoded: DedustPoolV1WithdrawalEventMsg = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.sender_addr,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000002b1")?
    );
    assert_eq!(*decoded.liquidity, 50401);
    assert_eq!(*decoded.amount0, 50474);
    assert_eq!(*decoded.amount1, 50547);
    assert_eq!(*decoded.reserve0, 50620);
    assert_eq!(*decoded.reserve1, 50693);
    let dispatched = DedustPoolV1OutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustPoolV1OutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV2_DedustPoolV2Storage() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV2_DedustPoolV2Storage")?;
    let decoded: DedustPoolV2Storage = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.proof.factory_addr,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000002ba")?
    );
    assert!(matches!(&decoded.proof.params.pool_type, DedustPoolV2PoolType::Stable(_)));
    let variant_703 = match &decoded.proof.params.asset0 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_703.workchain_id, -40);
    assert_eq!(
        variant_703.address.to_hex(),
        "000000000000000000000000000000000000000000000000000000000000c971"
    );
    let variant_707 = match &decoded.proof.params.asset1 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_707.workchain_id, -76);
    assert_eq!(
        variant_707.address.to_hex(),
        "000000000000000000000000000000000000000000000000000000000000ca95"
    );
    assert_eq!(decoded.config.asset0_decimals, 2);
    assert_eq!(decoded.config.asset1_decimals, 75);
    assert_eq!(
        decoded.config.asset0_vault_addr,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000002cd")?
    );
    assert_eq!(
        decoded.config.asset1_vault_addr,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000002ce")?
    );
    assert_eq!(
        decoded.config.fee_collector_addr,
        Some(TonAddress::from_str(
            "0:00000000000000000000000000000000000000000000000000000000000002cf"
        )?)
    );
    assert_eq!(
        decoded
            .config
            .quote_provider_data
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing providers"))?
            .provider0,
        None
    );
    assert_eq!(decoded.version, 52591);
    assert_eq!(decoded.trade_fee_numerator, 52664);
    assert_eq!(*decoded.total_supply, 52737);
    assert_eq!(*decoded.reserve0, 52810);
    assert_eq!(*decoded.reserve1, 52883);
    assert_eq!(*decoded.asset0_protocol_fee, 52956);
    assert_eq!(*decoded.asset1_protocol_fee, 53029);
    assert_eq!(decoded.normalization_factor0, 53102);
    assert_eq!(decoded.normalization_factor1, 53175);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV2_DedustPoolV2Config() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV2_DedustPoolV2Config")?;
    let decoded: DedustPoolV2Config = TLB::from_cell(&body)?;
    assert_eq!(decoded.asset0_decimals, 73);
    assert_eq!(decoded.asset1_decimals, 146);
    assert_eq!(
        decoded.asset0_vault_addr,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000002dc")?
    );
    assert_eq!(
        decoded.asset1_vault_addr,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000002dd")?
    );
    assert_eq!(
        decoded.fee_collector_addr,
        Some(TonAddress::from_str(
            "0:00000000000000000000000000000000000000000000000000000000000002de"
        )?)
    );
    assert_eq!(
        decoded
            .quote_provider_data
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing providers"))?
            .provider0,
        None
    );
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV2_DedustPoolV2NativeAsset() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV2_DedustPoolV2NativeAsset")?;
    let decoded: DedustClassicNativeAsset = TLB::from_cell(&body)?;

    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV2_DedustPoolV2JettonAsset() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV2_DedustPoolV2JettonAsset")?;
    let decoded: DedustClassicJettonAsset = TLB::from_cell(&body)?;
    assert_eq!(decoded.workchain_id, -72);
    assert_eq!(
        decoded.address.to_hex(),
        "000000000000000000000000000000000000000000000000000000000000d291"
    );
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV2_DedustPoolV2VolatilePool() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV2_DedustPoolV2VolatilePool")?;
    let decoded: DedustPoolV2VolatilePool = TLB::from_cell(&body)?;

    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV2_DedustPoolV2StablePool() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV2_DedustPoolV2StablePool")?;
    let decoded: DedustPoolV2StablePool = TLB::from_cell(&body)?;

    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV2_DedustPoolV2PoolParams() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV2_DedustPoolV2PoolParams")?;
    let decoded: DedustPoolV2PoolParams = TLB::from_cell(&body)?;
    assert!(matches!(&decoded.pool_type, DedustPoolV2PoolType::Volatile(_)));
    assert!(matches!(&decoded.asset0, DedustClassicAsset::Native(_)));
    assert!(matches!(&decoded.asset1, DedustClassicAsset::Native(_)));
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV2_DedustPoolV2VaultContractType() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV2_DedustPoolV2VaultContractType")?;
    let decoded: DedustPoolV2VaultContractType = TLB::from_cell(&body)?;

    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV2_DedustPoolV2PoolContractType() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV2_DedustPoolV2PoolContractType")?;
    let decoded: DedustPoolV2PoolContractType = TLB::from_cell(&body)?;

    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV2_DedustPoolV2LiquidityDepositContractType() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV2_DedustPoolV2LiquidityDepositContractType")?;
    let decoded: DedustPoolV2LiquidityDepositContractType = TLB::from_cell(&body)?;

    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV2_DedustPoolV2VaultProof() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV2_DedustPoolV2VaultProof")?;
    let decoded: DedustPoolV2VaultProof = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.factory_addr,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000002f0")?
    );
    assert!(matches!(&decoded.params, DedustClassicAsset::Native(_)));
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV2_DedustPoolV2PoolProof() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV2_DedustPoolV2PoolProof")?;
    let decoded: DedustPoolV2PoolProof = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.factory_addr,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000002f5")?
    );
    assert!(matches!(&decoded.params.pool_type, DedustPoolV2PoolType::Volatile(_)));
    assert!(matches!(&decoded.params.asset0, DedustClassicAsset::Native(_)));
    assert!(matches!(&decoded.params.asset1, DedustClassicAsset::Native(_)));
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV2_DedustPoolV2LiquidityDepositProofParams() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV2_DedustPoolV2LiquidityDepositProofParams")?;
    let decoded: DedustPoolV2LiquidityDepositProofParams = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.owner_addr,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000002ff")?
    );
    assert!(matches!(&decoded.pool_params.pool_type, DedustPoolV2PoolType::Stable(_)));
    let variant_771 = match &decoded.pool_params.asset0 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_771.workchain_id, -12);
    assert_eq!(
        variant_771.address.to_hex(),
        "000000000000000000000000000000000000000000000000000000000000dcd5"
    );
    let variant_775 = match &decoded.pool_params.asset1 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_775.workchain_id, -48);
    assert_eq!(
        variant_775.address.to_hex(),
        "000000000000000000000000000000000000000000000000000000000000ddf9"
    );
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV2_DedustPoolV2LiquidityDepositProof() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV2_DedustPoolV2LiquidityDepositProof")?;
    let decoded: DedustPoolV2LiquidityDepositProof = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.factory_addr,
        TonAddress::from_str("0:000000000000000000000000000000000000000000000000000000000000030c")?
    );
    assert_eq!(
        decoded.params.owner_addr,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000310")?
    );
    assert!(matches!(
        &decoded.params.pool_params.pool_type,
        DedustPoolV2PoolType::Volatile(_)
    ));
    assert!(matches!(&decoded.params.pool_params.asset0, DedustClassicAsset::Native(_)));
    assert!(matches!(&decoded.params.pool_params.asset1, DedustClassicAsset::Native(_)));
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV2_DedustPoolV2GivenIn() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV2_DedustPoolV2GivenIn")?;
    let decoded: DedustPoolV2GivenIn = TLB::from_cell(&body)?;

    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV2_DedustPoolV2GivenOut() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV2_DedustPoolV2GivenOut")?;
    let decoded: DedustPoolV2GivenOut = TLB::from_cell(&body)?;

    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV2_DedustPoolV2SwapStepParams() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV2_DedustPoolV2SwapStepParams")?;
    let decoded: DedustPoolV2SwapStepParams = TLB::from_cell(&body)?;
    assert!(matches!(&decoded.kind, DedustPoolV2SwapKind::GivenOut(_)));
    assert_eq!(*decoded.limit, 58212);
    assert!(decoded.next.is_none());
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV2_DedustPoolV2SwapStep() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV2_DedustPoolV2SwapStep")?;
    let decoded: DedustPoolV2SwapStep = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.pool_addr,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000320")?
    );
    assert!(matches!(&decoded.params.kind, DedustPoolV2SwapKind::GivenIn(_)));
    assert_eq!(*decoded.params.limit, 58723);
    assert_eq!(
        decoded
            .params
            .next
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .pool_addr,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000328")?
    );
    assert!(matches!(
        &decoded
            .params
            .next
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .params
            .kind,
        DedustPoolV2SwapKind::GivenIn(_)
    ));
    assert_eq!(
        *decoded
            .params
            .next
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .params
            .limit,
        59307
    );
    assert!(
        decoded
            .params
            .next
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .params
            .next
            .is_none()
    );
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV2_DedustPoolV2SwapParams() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV2_DedustPoolV2SwapParams")?;
    let decoded: DedustPoolV2SwapParams = TLB::from_cell(&body)?;
    assert_eq!(decoded.deadline, 59526);
    assert!(decoded.recipient_addr.is_none());
    assert_eq!(
        decoded.referral_addr,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000331"
        )?)
    );
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV2_DedustPoolV2DepositLiquidityAllAmounts() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV2_DedustPoolV2DepositLiquidityAllAmounts")?;
    let decoded: DedustPoolV2DepositLiquidityAllAmounts = TLB::from_cell(&body)?;
    let variant_823 = match &decoded.asset0 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_823.workchain_id, -96);
    assert_eq!(
        variant_823.address.to_hex(),
        "000000000000000000000000000000000000000000000000000000000000eba9"
    );
    assert_eq!(*decoded.asset0_amount, 60402);
    assert!(matches!(&decoded.asset1, DedustClassicAsset::Native(_)));
    assert_eq!(*decoded.asset1_amount, 60621);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV2_DedustPoolV2SwapExternal() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV2_DedustPoolV2SwapExternal")?;
    let decoded: DedustPoolV2SwapExternalMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 60767);
    assert_eq!(
        decoded.proof.factory_addr,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000343")?
    );
    let variant_837 = match &decoded.proof.params {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_837.workchain_id, -94);
    assert_eq!(
        variant_837.address.to_hex(),
        "000000000000000000000000000000000000000000000000000000000000efa7"
    );
    assert_eq!(*decoded.amount, 61424);
    assert_eq!(
        decoded.sender_addr,
        TonAddress::from_str("0:000000000000000000000000000000000000000000000000000000000000034a")?
    );
    assert!(matches!(&decoded.current.kind, DedustPoolV2SwapKind::GivenIn(_)));
    assert_eq!(*decoded.current.limit, 61789);
    assert_eq!(
        decoded
            .current
            .next
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .pool_addr,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000352")?
    );
    assert!(matches!(
        &decoded
            .current
            .next
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .params
            .kind,
        DedustPoolV2SwapKind::GivenIn(_)
    ));
    assert_eq!(
        *decoded
            .current
            .next
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .params
            .limit,
        62373
    );
    assert!(
        decoded
            .current
            .next
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .params
            .next
            .is_none()
    );
    assert_eq!(decoded.swap_params.deadline, 62665);
    assert_eq!(
        decoded.swap_params.recipient_addr,
        Some(TonAddress::from_str(
            "0:000000000000000000000000000000000000000000000000000000000000035b"
        )?)
    );
    assert_eq!(
        decoded.swap_params.referral_addr,
        Some(TonAddress::from_str(
            "0:000000000000000000000000000000000000000000000000000000000000035c"
        )?)
    );
    assert!(decoded.swap_params.fulfill_payload.is_none());
    let dispatched = DedustPoolV2IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustPoolV2IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV2_DedustPoolV2SwapPeer() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV2_DedustPoolV2SwapPeer")?;
    let decoded: DedustPoolV2SwapPeerMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 63176);
    assert_eq!(
        decoded.proof.factory_addr,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000364")?
    );
    assert!(matches!(&decoded.proof.params.pool_type, DedustPoolV2PoolType::Stable(_)));
    let variant_873 = match &decoded.proof.params.asset0 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_873.workchain_id, -34);
    assert_eq!(
        variant_873.address.to_hex(),
        "000000000000000000000000000000000000000000000000000000000000f9eb"
    );
    let variant_877 = match &decoded.proof.params.asset1 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_877.workchain_id, -70);
    assert_eq!(
        variant_877.address.to_hex(),
        "000000000000000000000000000000000000000000000000000000000000fb0f"
    );
    let variant_881 = match &decoded.asset {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_881.workchain_id, -106);
    assert_eq!(
        variant_881.address.to_hex(),
        "000000000000000000000000000000000000000000000000000000000000fc33"
    );
    assert_eq!(*decoded.amount, 64636);
    assert_eq!(
        decoded.sender_addr,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000376")?
    );
    assert!(matches!(&decoded.current.kind, DedustPoolV2SwapKind::GivenIn(_)));
    assert_eq!(*decoded.current.limit, 65001);
    assert!(decoded.current.next.is_none());
    assert_eq!(decoded.swap_params.deadline, 65293);
    assert_eq!(
        decoded.swap_params.recipient_addr,
        Some(TonAddress::from_str(
            "0:000000000000000000000000000000000000000000000000000000000000037f"
        )?)
    );
    assert_eq!(
        decoded.swap_params.referral_addr,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000380"
        )?)
    );
    assert!(decoded.swap_params.fulfill_payload.is_none());
    let dispatched = DedustPoolV2IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustPoolV2IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    let dispatched = DedustPoolV2OutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustPoolV2OutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV2_DedustPoolV2DepositLiquidityAll() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV2_DedustPoolV2DepositLiquidityAll")?;
    let decoded: DedustPoolV2DepositLiquidityAllMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 65804);
    assert_eq!(
        decoded.proof.factory_addr,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000388")?
    );
    assert_eq!(
        decoded.proof.params.owner_addr,
        TonAddress::from_str("0:000000000000000000000000000000000000000000000000000000000000038c")?
    );
    assert!(matches!(
        &decoded.proof.params.pool_params.pool_type,
        DedustPoolV2PoolType::Volatile(_)
    ));
    assert!(matches!(
        &decoded.proof.params.pool_params.asset0,
        DedustClassicAsset::Native(_)
    ));
    assert!(matches!(
        &decoded.proof.params.pool_params.asset1,
        DedustClassicAsset::Native(_)
    ));
    assert_eq!(
        decoded.owner_addr,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000394")?
    );
    assert_eq!(*decoded.min_lp_amount, 66972);
    assert!(matches!(&decoded.deposit_amounts.asset0, DedustClassicAsset::Native(_)));
    assert_eq!(*decoded.deposit_amounts.asset0_amount, 67337);
    let variant_923 = match &decoded.deposit_amounts.asset1 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_923.workchain_id, -100);
    assert_eq!(
        variant_923.address.to_hex(),
        "000000000000000000000000000000000000000000000000000000000001082d"
    );
    assert_eq!(*decoded.deposit_amounts.asset1_amount, 67702);
    assert!(decoded.reject_payload.is_none());
    let dispatched = DedustPoolV2IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustPoolV2IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV2_DedustPoolV2JettonBurnNotification() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV2_DedustPoolV2JettonBurnNotification")?;
    let decoded: DedustPoolV2JettonBurnNotificationMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 68067);
    assert_eq!(*decoded.amount, 68140);
    assert_eq!(
        decoded.sender,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000003a6")?
    );
    assert_eq!(
        decoded.response_destination,
        Some(TonAddress::from_str(
            "0:00000000000000000000000000000000000000000000000000000000000003a7"
        )?)
    );
    let dispatched = DedustPoolV2IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustPoolV2IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV2_DedustPoolV2ConfigureQuoteProvider() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV2_DedustPoolV2ConfigureQuoteProvider")?;
    let decoded: DedustPoolV2ConfigureQuoteProviderMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 68432);
    assert!(matches!(&decoded.asset, DedustClassicAsset::Native(_)));
    assert_eq!(
        decoded.quote_provider_addr,
        Some(TonAddress::from_str(
            "0:00000000000000000000000000000000000000000000000000000000000003ac"
        )?)
    );
    let dispatched = DedustPoolV2IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustPoolV2IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV2_DedustPoolV2SyncAssetQuote() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV2_DedustPoolV2SyncAssetQuote")?;
    let decoded: DedustPoolV2SyncAssetQuoteMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 68797);
    let variant_943 = match &decoded.asset {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_943.workchain_id, -24);
    assert_eq!(
        variant_943.address.to_hex(),
        "0000000000000000000000000000000000000000000000000000000000010de1"
    );
    let dispatched = DedustPoolV2IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustPoolV2IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV2_DedustPoolV2ProvidePoolState() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV2_DedustPoolV2ProvidePoolState")?;
    let decoded: DedustPoolV2ProvidePoolStateMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 69235);
    assert!(decoded.include_assets);
    let dispatched = DedustPoolV2IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustPoolV2IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV2_DedustPoolV2ConfigureTradeFee() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV2_DedustPoolV2ConfigureTradeFee")?;
    let decoded: DedustPoolV2ConfigureTradeFeeMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 69454);
    assert_eq!(decoded.trade_fee, 3991);
    let dispatched = DedustPoolV2IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustPoolV2IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV2_DedustPoolV2ResetGas() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV2_DedustPoolV2ResetGas")?;
    let decoded: DedustPoolV2ResetGasMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 69673);
    assert_eq!(
        decoded.response_destination,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000003bb")?
    );
    let dispatched = DedustPoolV2IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustPoolV2IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV2_DedustPoolV2PayOutFromPool() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV2_DedustPoolV2PayOutFromPool")?;
    let decoded: DedustPoolV2PayOutFromPoolMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 69892);
    assert_eq!(
        decoded.proof.factory_addr,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000003c0")?
    );
    assert!(matches!(&decoded.proof.params.pool_type, DedustPoolV2PoolType::Stable(_)));
    let variant_965 = match &decoded.proof.params.asset0 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_965.workchain_id, -94);
    assert_eq!(
        variant_965.address.to_hex(),
        "0000000000000000000000000000000000000000000000000000000000011427"
    );
    let variant_969 = match &decoded.proof.params.asset1 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_969.workchain_id, -2);
    assert_eq!(
        variant_969.address.to_hex(),
        "000000000000000000000000000000000000000000000000000000000001154b"
    );
    assert_eq!(*decoded.amount, 71060);
    assert_eq!(
        decoded.recipient_addr,
        Some(TonAddress::from_str(
            "0:00000000000000000000000000000000000000000000000000000000000003ce"
        )?)
    );
    assert!(decoded.payload.is_none());
    let dispatched = DedustPoolV2OutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustPoolV2OutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV2_DedustPoolV2PoolStateAssets() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV2_DedustPoolV2PoolStateAssets")?;
    let decoded: DedustPoolV2PoolStateAssets = TLB::from_cell(&body)?;
    let variant_977 = match &decoded.asset0 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_977.workchain_id, -74);
    assert_eq!(
        variant_977.address.to_hex(),
        "0000000000000000000000000000000000000000000000000000000000011793"
    );
    let variant_981 = match &decoded.asset1 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_981.workchain_id, -110);
    assert_eq!(
        variant_981.address.to_hex(),
        "00000000000000000000000000000000000000000000000000000000000118b7"
    );
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV2_DedustPoolV2TakePoolState() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV2_DedustPoolV2TakePoolState")?;
    let decoded: DedustPoolV2TakePoolStateMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 72009);
    assert_eq!(*decoded.reserve0, 72082);
    assert_eq!(*decoded.reserve1, 72155);
    assert_eq!(*decoded.total_supply, 72228);
    assert!(decoded.assets.is_none());
    let dispatched = DedustPoolV2OutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustPoolV2OutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV2_DedustPoolV2FulfillDeposit() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV2_DedustPoolV2FulfillDeposit")?;
    let decoded: DedustPoolV2FulfillDepositMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 72447);
    assert_eq!(
        decoded.proof.factory_addr,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000003e3")?
    );
    assert!(matches!(&decoded.proof.params.pool_type, DedustPoolV2PoolType::Volatile(_)));
    assert!(matches!(&decoded.proof.params.asset0, DedustClassicAsset::Native(_)));
    assert!(matches!(&decoded.proof.params.asset1, DedustClassicAsset::Native(_)));
    assert_eq!(*decoded.asset0_deposit, 73323);
    assert_eq!(*decoded.asset1_deposit, 73396);
    let dispatched = DedustPoolV2OutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustPoolV2OutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV2_DedustPoolV2RejectDeposit() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV2_DedustPoolV2RejectDeposit")?;
    let decoded: DedustPoolV2RejectDepositMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 73542);
    assert_eq!(
        decoded.proof.factory_addr,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000003f2")?
    );
    assert!(matches!(&decoded.proof.params.pool_type, DedustPoolV2PoolType::Stable(_)));
    let variant_1015 = match &decoded.proof.params.asset0 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_1015.workchain_id, -32);
    assert_eq!(
        variant_1015.address.to_hex(),
        "0000000000000000000000000000000000000000000000000000000000012269"
    );
    let variant_1019 = match &decoded.proof.params.asset1 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_1019.workchain_id, -68);
    assert_eq!(
        variant_1019.address.to_hex(),
        "000000000000000000000000000000000000000000000000000000000001238d"
    );
    assert!(decoded.payload.is_none());
    let dispatched = DedustPoolV2OutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustPoolV2OutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV2_DedustPoolV2JettonInternalTransfer() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV2_DedustPoolV2JettonInternalTransfer")?;
    let decoded: DedustPoolV2JettonInternalTransferMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 74856);
    assert_eq!(*decoded.amount, 74929);
    assert_eq!(
        decoded.from_address,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000403")?
    );
    assert_eq!(
        decoded.response_address,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000404"
        )?)
    );
    assert_eq!(*decoded.forward_ton_amount, 75148);
    let dispatched = DedustPoolV2OutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustPoolV2OutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV2_DedustPoolV2SwapEventTail() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV2_DedustPoolV2SwapEventTail")?;
    let decoded: DedustPoolV2SwapEventTail = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.sender_addr,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000408")?
    );
    assert_eq!(
        decoded.referral_addr,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000409"
        )?)
    );
    assert_eq!(*decoded.reserve0, 75513);
    assert_eq!(*decoded.reserve1, 75586);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV2_DedustPoolV2SwapEvent() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV2_DedustPoolV2SwapEvent")?;
    let decoded: DedustPoolV2SwapEventMsg = TLB::from_cell(&body)?;
    let variant_1037 = match &decoded.asset_in {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_1037.workchain_id, -102);
    assert_eq!(
        variant_1037.address.to_hex(),
        "00000000000000000000000000000000000000000000000000000000000128af"
    );
    let variant_1041 = match &decoded.asset_out {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_1041.workchain_id, -10);
    assert_eq!(
        variant_1041.address.to_hex(),
        "00000000000000000000000000000000000000000000000000000000000129d3"
    );
    assert_eq!(*decoded.amount_in, 76316);
    assert_eq!(*decoded.amount_out, 76389);
    assert_eq!(
        decoded.details.sender_addr,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000419")?
    );
    assert!(decoded.details.referral_addr.is_none());
    assert_eq!(*decoded.details.reserve0, 76754);
    assert_eq!(*decoded.details.reserve1, 76827);
    let dispatched = DedustPoolV2OutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustPoolV2OutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV2_DedustPoolV2DepositEvent() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV2_DedustPoolV2DepositEvent")?;
    let decoded: DedustPoolV2DepositEventMsg = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.sender_addr,
        TonAddress::from_str("0:000000000000000000000000000000000000000000000000000000000000041e")?
    );
    assert_eq!(*decoded.amount0, 77046);
    assert_eq!(*decoded.amount1, 77119);
    assert_eq!(*decoded.reserve0, 77192);
    assert_eq!(*decoded.reserve1, 77265);
    assert_eq!(*decoded.liquidity, 77338);
    let dispatched = DedustPoolV2OutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustPoolV2OutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV2_DedustPoolV2WithdrawalEvent() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV2_DedustPoolV2WithdrawalEvent")?;
    let decoded: DedustPoolV2WithdrawalEventMsg = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.sender_addr,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000425")?
    );
    assert_eq!(*decoded.liquidity, 77557);
    assert_eq!(*decoded.amount0, 77630);
    assert_eq!(*decoded.amount1, 77703);
    assert_eq!(*decoded.reserve0, 77776);
    assert_eq!(*decoded.reserve1, 77849);
    let dispatched = DedustPoolV2OutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustPoolV2OutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV2_DedustPoolV2CollectFees() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV2_DedustPoolV2CollectFees")?;
    let decoded: DedustPoolV2CollectFeesMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 77995);
    assert_eq!(*decoded.asset0_amount, 78068);
    assert_eq!(
        decoded.asset0_destination,
        TonAddress::from_str("0:000000000000000000000000000000000000000000000000000000000000042e")?
    );
    assert!(decoded.asset0_payload.is_none());
    assert_eq!(*decoded.asset1_amount, 78287);
    assert_eq!(
        decoded.asset1_destination,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000431"
        )?)
    );
    assert!(decoded.asset1_payload.is_none());
    let dispatched = DedustPoolV2IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustPoolV2IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustPoolV2_DedustPoolV2ConfigureStartTime() -> anyhow::Result<()> {
    let body = fixture("schema_DedustPoolV2_DedustPoolV2ConfigureStartTime")?;
    let decoded: DedustPoolV2ConfigureStartTimeMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 78579);
    assert_eq!(decoded.start_at, 78652);
    let dispatched = DedustPoolV2IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustPoolV2IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultNativeV1_DedustVaultNativeV1Storage() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultNativeV1_DedustVaultNativeV1Storage")?;
    let decoded: DedustVaultNativeV1Storage = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.cached_proof.factory_address,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000439")?
    );
    assert_eq!(decoded.cached_proof.contract_type, 169);
    let variant_1083 = match &decoded.cached_proof.asset {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_1083.workchain_id, -4);
    assert_eq!(
        variant_1083.address.to_hex(),
        "00000000000000000000000000000000000000000000000000000000000135cd"
    );
    assert_eq!(
        decoded.factory_address,
        TonAddress::from_str("0:000000000000000000000000000000000000000000000000000000000000043f")?
    );
    assert_eq!(decoded.version, 13992);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultNativeV1_DedustVaultNativeV1VaultProof() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultNativeV1_DedustVaultNativeV1VaultProof")?;
    let decoded: DedustVaultNativeV1VaultProof = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.factory_address,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000443")?
    );
    assert_eq!(decoded.contract_type, 131);
    let variant_1093 = match &decoded.asset {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_1093.workchain_id, -94);
    assert_eq!(
        variant_1093.address.to_hex(),
        "00000000000000000000000000000000000000000000000000000000000138a7"
    );
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultNativeV1_DedustVaultNativeV1NativeAsset() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultNativeV1_DedustVaultNativeV1NativeAsset")?;
    let decoded: DedustClassicNativeAsset = TLB::from_cell(&body)?;

    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultNativeV1_DedustVaultNativeV1JettonAsset() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultNativeV1_DedustVaultNativeV1JettonAsset")?;
    let decoded: DedustClassicJettonAsset = TLB::from_cell(&body)?;
    assert_eq!(decoded.workchain_id, -2);
    assert_eq!(
        decoded.address.to_hex(),
        "00000000000000000000000000000000000000000000000000000000000139cb"
    );
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultNativeV1_DedustVaultNativeV1PoolParams() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultNativeV1_DedustVaultNativeV1PoolParams")?;
    let decoded: DedustVaultNativeV1PoolParams = TLB::from_cell(&body)?;
    assert!(!decoded.is_stable);
    let variant_1103 = match &decoded.asset0 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_1103.workchain_id, -56);
    assert_eq!(
        variant_1103.address.to_hex(),
        "0000000000000000000000000000000000000000000000000000000000013b81"
    );
    let variant_1107 = match &decoded.asset1 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_1107.workchain_id, -92);
    assert_eq!(
        variant_1107.address.to_hex(),
        "0000000000000000000000000000000000000000000000000000000000013ca5"
    );
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultNativeV1_DedustVaultNativeV1PoolProof() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultNativeV1_DedustVaultNativeV1PoolProof")?;
    let decoded: DedustVaultNativeV1PoolProof = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.factory_address,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000458")?
    );
    assert_eq!(decoded.contract_type, 128);
    assert!(decoded.pool_params.is_stable);
    assert!(matches!(&decoded.pool_params.asset0, DedustClassicAsset::Native(_)));
    assert!(matches!(&decoded.pool_params.asset1, DedustClassicAsset::Native(_)));
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultNativeV1_DedustVaultNativeV1LiquidityDepositProof() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultNativeV1_DedustVaultNativeV1LiquidityDepositProof")?;
    let decoded: DedustVaultNativeV1LiquidityDepositProof = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.factory_address,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000461")?
    );
    assert_eq!(decoded.contract_type, 17);
    assert_eq!(
        decoded.params.owner_address,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000465")?
    );
    assert!(decoded.params.pool_params.is_stable);
    assert!(matches!(&decoded.params.pool_params.asset0, DedustClassicAsset::Native(_)));
    assert!(matches!(&decoded.params.pool_params.asset1, DedustClassicAsset::Native(_)));
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultNativeV1_DedustVaultNativeV1LiquidityDepositProofParams() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultNativeV1_DedustVaultNativeV1LiquidityDepositProofParams")?;
    let decoded: DedustVaultNativeV1LiquidityDepositProofParams = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.owner_address,
        TonAddress::from_str("0:000000000000000000000000000000000000000000000000000000000000046d")?
    );
    assert!(decoded.pool_params.is_stable);
    assert!(matches!(&decoded.pool_params.asset0, DedustClassicAsset::Native(_)));
    assert!(matches!(&decoded.pool_params.asset1, DedustClassicAsset::Native(_)));
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultNativeV1_DedustVaultNativeV1GivenIn() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultNativeV1_DedustVaultNativeV1GivenIn")?;
    let decoded: DedustVaultNativeV1GivenIn = TLB::from_cell(&body)?;

    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultNativeV1_DedustVaultNativeV1GivenOut() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultNativeV1_DedustVaultNativeV1GivenOut")?;
    let decoded: DedustVaultNativeV1GivenOut = TLB::from_cell(&body)?;

    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultNativeV1_DedustVaultNativeV1SwapStepParams() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultNativeV1_DedustVaultNativeV1SwapStepParams")?;
    let decoded: DedustVaultNativeV1SwapStepParams = TLB::from_cell(&body)?;
    assert!(matches!(&decoded.kind, DedustVaultNativeV1SwapKind::GivenOut(_)));
    assert_eq!(*decoded.limit, 83616);
    assert!(decoded.next.is_none());
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultNativeV1_DedustVaultNativeV1SwapStep() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultNativeV1_DedustVaultNativeV1SwapStep")?;
    let decoded: DedustVaultNativeV1SwapStep = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.pool_address,
        TonAddress::from_str("0:000000000000000000000000000000000000000000000000000000000000047b")?
    );
    assert!(matches!(&decoded.params.kind, DedustVaultNativeV1SwapKind::GivenOut(_)));
    assert_eq!(*decoded.params.limit, 84054);
    assert!(decoded.params.next.is_none());
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultNativeV1_DedustVaultNativeV1Swap() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultNativeV1_DedustVaultNativeV1Swap")?;
    let decoded: DedustVaultNativeV1SwapMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 84200);
    assert_eq!(*decoded.amount, 84273);
    assert_eq!(
        decoded.step.pool_address,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000484")?
    );
    assert!(matches!(&decoded.step.params.kind, DedustVaultNativeV1SwapKind::GivenIn(_)));
    assert_eq!(*decoded.step.params.limit, 84711);
    assert!(decoded.step.params.next.is_none());
    assert_eq!(decoded.swap_params.deadline, 1000);
    let dispatched = DedustVaultNativeV1IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustVaultNativeV1IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultNativeV1_DedustVaultNativeV1DepositLiquidityParams() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultNativeV1_DedustVaultNativeV1DepositLiquidityParams")?;
    let decoded: DedustVaultNativeV1DepositLiquidityParams = TLB::from_cell(&body)?;
    assert_eq!(*decoded.min_lp_amount, 84857);
    assert_eq!(*decoded.asset0_target_balance, 84930);
    assert_eq!(*decoded.asset1_target_balance, 85003);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultNativeV1_DedustVaultNativeV1DepositLiquidity() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultNativeV1_DedustVaultNativeV1DepositLiquidity")?;
    let decoded: DedustVaultNativeV1DepositLiquidityMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 85149);
    assert_eq!(*decoded.amount, 85222);
    assert!(decoded.pool_params.is_stable);
    assert!(matches!(&decoded.pool_params.asset0, DedustClassicAsset::Native(_)));
    assert!(matches!(&decoded.pool_params.asset1, DedustClassicAsset::Native(_)));
    assert_eq!(*decoded.params.min_lp_amount, 85879);
    assert_eq!(*decoded.params.asset0_target_balance, 85952);
    assert_eq!(*decoded.params.asset1_target_balance, 86025);
    assert!(decoded.fulfill_payload.is_none());
    let dispatched = DedustVaultNativeV1IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustVaultNativeV1IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultNativeV1_DedustVaultNativeV1PayOutFromPool() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultNativeV1_DedustVaultNativeV1PayOutFromPool")?;
    let decoded: DedustVaultNativeV1PayOutFromPoolMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 86390);
    assert_eq!(
        decoded.proof.factory_address,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000004a2")?
    );
    assert_eq!(decoded.proof.contract_type, 154);
    assert!(decoded.proof.pool_params.is_stable);
    assert!(matches!(&decoded.proof.pool_params.asset0, DedustClassicAsset::Native(_)));
    assert!(matches!(&decoded.proof.pool_params.asset1, DedustClassicAsset::Native(_)));
    assert_eq!(*decoded.amount, 87193);
    assert_eq!(
        decoded.recipient_address,
        Some(TonAddress::from_str(
            "0:00000000000000000000000000000000000000000000000000000000000004ab"
        )?)
    );
    let dispatched = DedustVaultNativeV1IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustVaultNativeV1IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultNativeV1_DedustVaultNativeV1PayOutFromDeposit() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultNativeV1_DedustVaultNativeV1PayOutFromDeposit")?;
    let decoded: DedustVaultNativeV1PayOutFromDepositMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 87558);
    assert_eq!(
        decoded.proof.factory_address,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000004b2")?
    );
    assert_eq!(decoded.proof.contract_type, 42);
    assert_eq!(
        decoded.proof.params.owner_address,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000004b6")?
    );
    assert!(!decoded.proof.params.pool_params.is_stable);
    let variant_1209 = match &decoded.proof.params.pool_params.asset0 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_1209.workchain_id, -114);
    assert_eq!(
        variant_1209.address.to_hex(),
        "00000000000000000000000000000000000000000000000000000000000159bb"
    );
    let variant_1213 = match &decoded.proof.params.pool_params.asset1 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_1213.workchain_id, -22);
    assert_eq!(
        variant_1213.address.to_hex(),
        "0000000000000000000000000000000000000000000000000000000000015adf"
    );
    assert_eq!(*decoded.amount, 88872);
    assert!(decoded.payload.is_none());
    let dispatched = DedustVaultNativeV1IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustVaultNativeV1IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultNativeV1_DedustVaultNativeV1UpgradeChildCode() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultNativeV1_DedustVaultNativeV1UpgradeChildCode")?;
    let decoded: DedustVaultNativeV1UpgradeChildCodeMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 89091);
    assert_eq!(decoded.code_version, 23628);
    let dispatched = DedustVaultNativeV1IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustVaultNativeV1IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultNativeV1_DedustVaultNativeV1SwapExternal() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultNativeV1_DedustVaultNativeV1SwapExternal")?;
    let decoded: DedustVaultNativeV1SwapExternalMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 89383);
    assert_eq!(
        decoded.proof.factory_address,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000004cb")?
    );
    assert_eq!(decoded.proof.contract_type, 75);
    let variant_1229 = match &decoded.proof.asset {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_1229.workchain_id, -38);
    assert_eq!(
        variant_1229.address.to_hex(),
        "0000000000000000000000000000000000000000000000000000000000015f6f"
    );
    assert_eq!(*decoded.amount, 90040);
    assert_eq!(
        decoded.sender_address,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000004d2")?
    );
    assert!(matches!(&decoded.current.kind, DedustVaultNativeV1SwapKind::GivenIn(_)));
    assert_eq!(*decoded.current.limit, 90405);
    assert!(decoded.current.next.is_none());
    assert_eq!(decoded.swap_params.deadline, 1000);
    let dispatched = DedustVaultNativeV1OutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustVaultNativeV1OutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultNativeV1_DedustVaultNativeV1CreateLiquidityDepositExtra() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultNativeV1_DedustVaultNativeV1CreateLiquidityDepositExtra")?;
    let decoded: DedustVaultNativeV1CreateLiquidityDepositExtra = TLB::from_cell(&body)?;
    assert_eq!(*decoded.asset0_target_balance, 90551);
    assert_eq!(*decoded.asset1_target_balance, 90624);
    assert!(matches!(&decoded.deposit_asset, DedustClassicAsset::Native(_)));
    assert_eq!(*decoded.deposit_amount, 90843);
    assert_eq!(*decoded.min_lp_amount, 90916);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultNativeV1_DedustVaultNativeV1CreateLiquidityDeposit() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultNativeV1_DedustVaultNativeV1CreateLiquidityDeposit")?;
    let decoded: DedustVaultNativeV1CreateLiquidityDepositMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 91062);
    assert_eq!(
        decoded.proof.factory_address,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000004e2")?
    );
    assert_eq!(decoded.proof.contract_type, 218);
    assert!(matches!(&decoded.proof.asset, DedustClassicAsset::Native(_)));
    assert_eq!(
        decoded.owner_address,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000004e6")?
    );
    assert!(!decoded.pool_params.is_stable);
    let variant_1257 = match &decoded.pool_params.asset0 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_1257.workchain_id, -34);
    assert_eq!(
        variant_1257.address.to_hex(),
        "000000000000000000000000000000000000000000000000000000000001676b"
    );
    let variant_1261 = match &decoded.pool_params.asset1 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_1261.workchain_id, -70);
    assert_eq!(
        variant_1261.address.to_hex(),
        "000000000000000000000000000000000000000000000000000000000001688f"
    );
    assert_eq!(*decoded.extra.asset0_target_balance, 92522);
    assert_eq!(*decoded.extra.asset1_target_balance, 92595);
    let variant_1269 = match &decoded.extra.deposit_asset {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_1269.workchain_id, -14);
    assert_eq!(
        variant_1269.address.to_hex(),
        "0000000000000000000000000000000000000000000000000000000000016ad7"
    );
    assert_eq!(*decoded.extra.deposit_amount, 92960);
    assert_eq!(*decoded.extra.min_lp_amount, 93033);
    assert!(decoded.fulfill_payload.is_none());
    let dispatched = DedustVaultNativeV1OutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustVaultNativeV1OutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultNativeV1_DedustVaultNativeV1Payout() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultNativeV1_DedustVaultNativeV1Payout")?;
    let decoded: DedustVaultNativeV1PayoutMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 93398);
    let dispatched = DedustVaultNativeV1OutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustVaultNativeV1OutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultNativeV2_DedustVaultNativeV2Storage() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultNativeV2_DedustVaultNativeV2Storage")?;
    let decoded: DedustVaultNativeV2Storage = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.cached_proof.factory_addr,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000505")?
    );
    assert_eq!(decoded.cached_proof.contract_type, 213);
    let variant_1287 = match &decoded.cached_proof.asset {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_1287.workchain_id, -48);
    assert_eq!(
        variant_1287.address.to_hex(),
        "0000000000000000000000000000000000000000000000000000000000016ff9"
    );
    assert_eq!(
        decoded.factory_addr,
        TonAddress::from_str("0:000000000000000000000000000000000000000000000000000000000000050b")?
    );
    assert_eq!(decoded.version, 28884);
    assert_eq!(*decoded.native_balance, 94493);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultNativeV2_DedustVaultNativeV2ProofData() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultNativeV2_DedustVaultNativeV2ProofData")?;
    let decoded: DedustVaultNativeV2ProofData = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.factory_addr,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000510")?
    );
    assert_eq!(decoded.contract_type, 248);
    assert!(matches!(&decoded.asset, DedustClassicAsset::Native(_)));
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultNativeV2_DedustNativeAsset() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultNativeV2_DedustNativeAsset")?;
    let decoded: DedustClassicNativeAsset = TLB::from_cell(&body)?;

    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultNativeV2_DedustJettonAsset() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultNativeV2_DedustJettonAsset")?;
    let decoded: DedustClassicJettonAsset = TLB::from_cell(&body)?;
    assert_eq!(decoded.workchain_id, -101);
    assert_eq!(
        decoded.address.to_hex(),
        "00000000000000000000000000000000000000000000000000000000000173ae"
    );
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultNativeV2_DedustVolatilePool() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultNativeV2_DedustVolatilePool")?;
    let decoded: crate::contracts::classic::vault_native_v2::storage::DedustVolatilePool = TLB::from_cell(&body)?;

    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultNativeV2_DedustStablePool() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultNativeV2_DedustStablePool")?;
    let decoded: crate::contracts::classic::vault_native_v2::storage::DedustStablePool = TLB::from_cell(&body)?;

    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultNativeV2_DedustPoolParams() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultNativeV2_DedustPoolParams")?;
    let decoded: crate::contracts::classic::vault_native_v2::storage::DedustVaultNativeV2PoolParams =
        TLB::from_cell(&body)?;
    assert!(matches!(
        &decoded.pool_type,
        crate::contracts::classic::vault_native_v2::storage::DedustPoolType::Stable(_)
    ));
    let variant_1309 = match &decoded.asset0 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_1309.workchain_id, -118);
    assert_eq!(
        variant_1309.address.to_hex(),
        "000000000000000000000000000000000000000000000000000000000001763f"
    );
    let variant_1313 = match &decoded.asset1 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_1313.workchain_id, -26);
    assert_eq!(
        variant_1313.address.to_hex(),
        "0000000000000000000000000000000000000000000000000000000000017763"
    );
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultNativeV2_DedustGivenIn() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultNativeV2_DedustGivenIn")?;
    let decoded: crate::contracts::classic::vault_native_v2::storage::DedustGivenIn = TLB::from_cell(&body)?;

    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultNativeV2_DedustGivenOut() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultNativeV2_DedustGivenOut")?;
    let decoded: crate::contracts::classic::vault_native_v2::storage::DedustGivenOut = TLB::from_cell(&body)?;

    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultNativeV2_DedustSwapStepParams() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultNativeV2_DedustSwapStepParams")?;
    let decoded: crate::contracts::classic::vault_native_v2::storage::DedustSwapStepParams = TLB::from_cell(&body)?;
    assert!(matches!(
        &decoded.kind,
        crate::contracts::classic::vault_native_v2::storage::DedustSwapKind::GivenIn(_)
    ));
    assert_eq!(*decoded.limit, 96537);
    assert!(decoded.next.is_none());
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultNativeV2_DedustSwapStep() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultNativeV2_DedustSwapStep")?;
    let decoded: crate::contracts::classic::vault_native_v2::storage::DedustSwapStep = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.pool_addr,
        TonAddress::from_str("0:000000000000000000000000000000000000000000000000000000000000052d")?
    );
    assert!(matches!(
        &decoded.params.kind,
        crate::contracts::classic::vault_native_v2::storage::DedustSwapKind::GivenOut(_)
    ));
    assert_eq!(*decoded.params.limit, 97048);
    assert_eq!(
        decoded
            .params
            .next
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .pool_addr,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000535")?
    );
    assert!(matches!(
        &decoded
            .params
            .next
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .params
            .kind,
        crate::contracts::classic::vault_native_v2::storage::DedustSwapKind::GivenOut(_)
    ));
    assert_eq!(
        *decoded
            .params
            .next
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .params
            .limit,
        97632
    );
    assert!(
        decoded
            .params
            .next
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .params
            .next
            .is_none()
    );
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultNativeV2_DedustSwapParams() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultNativeV2_DedustSwapParams")?;
    let decoded: crate::contracts::classic::vault_native_v2::storage::DedustSwapParams = TLB::from_cell(&body)?;
    assert_eq!(decoded.deadline, 97851);
    assert!(decoded.recipient_addr.is_none());
    assert_eq!(
        decoded.referral_addr,
        Some(TonAddress::from_str(
            "0:000000000000000000000000000000000000000000000000000000000000053e"
        )?)
    );
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultNativeV2_DedustDepositLiquidityParams() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultNativeV2_DedustDepositLiquidityParams")?;
    let decoded: DedustDepositLiquidityParams = TLB::from_cell(&body)?;
    assert_eq!(*decoded.min_lp_amount, 98435);
    assert_eq!(*decoded.asset0_target_balance, 98508);
    assert_eq!(*decoded.asset1_target_balance, 98581);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultNativeV2_DedustVaultNativeV2Swap() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultNativeV2_DedustVaultNativeV2Swap")?;
    let decoded: DedustVaultNativeV2SwapMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 99019);
    assert_eq!(*decoded.amount, 99092);
    assert_eq!(
        decoded.step.pool_addr,
        TonAddress::from_str("0:000000000000000000000000000000000000000000000000000000000000054f")?
    );
    assert!(matches!(
        &decoded.step.params.kind,
        crate::contracts::classic::vault_native_v2::storage::DedustSwapKind::GivenOut(_)
    ));
    assert_eq!(*decoded.step.params.limit, 99530);
    assert_eq!(
        decoded
            .step
            .params
            .next
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .pool_addr,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000557")?
    );
    assert!(matches!(
        &decoded
            .step
            .params
            .next
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .params
            .kind,
        crate::contracts::classic::vault_native_v2::storage::DedustSwapKind::GivenOut(_)
    ));
    assert_eq!(
        *decoded
            .step
            .params
            .next
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .params
            .limit,
        100114
    );
    assert!(
        decoded
            .step
            .params
            .next
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .params
            .next
            .is_none()
    );
    assert_eq!(decoded.swap_params.deadline, 100406);
    assert_eq!(
        decoded.swap_params.recipient_addr,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000560"
        )?)
    );
    assert!(decoded.swap_params.referral_addr.is_none());
    assert!(decoded.swap_params.reject_payload.is_none());
    let dispatched = DedustVaultNativeV2IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustVaultNativeV2IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultNativeV2_DedustVaultNativeV2DepositLiquidity() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultNativeV2_DedustVaultNativeV2DepositLiquidity")?;
    let decoded: DedustVaultNativeV2DepositLiquidityMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 100917);
    assert_eq!(*decoded.amount, 100990);
    assert!(matches!(
        &decoded.pool_params.pool_type,
        crate::contracts::classic::vault_native_v2::storage::DedustPoolType::Stable(_)
    ));
    let variant_1387 = match &decoded.pool_params.asset0 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_1387.workchain_id, -52);
    assert_eq!(
        variant_1387.address.to_hex(),
        "0000000000000000000000000000000000000000000000000000000000018c7d"
    );
    let variant_1391 = match &decoded.pool_params.asset1 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_1391.workchain_id, -88);
    assert_eq!(
        variant_1391.address.to_hex(),
        "0000000000000000000000000000000000000000000000000000000000018da1"
    );
    assert_eq!(*decoded.params.min_lp_amount, 102012);
    assert_eq!(*decoded.params.asset0_target_balance, 102085);
    assert_eq!(*decoded.params.asset1_target_balance, 102158);
    let dispatched = DedustVaultNativeV2IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustVaultNativeV2IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultNativeV2_DedustVaultNativeV2PayOutFromPool() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultNativeV2_DedustVaultNativeV2PayOutFromPool")?;
    let decoded: DedustVaultNativeV2PayOutFromPoolMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 102596);
    assert_eq!(decoded.proof.contract_type, 2);
    assert_eq!(*decoded.amount, 102669);
    assert!(decoded.recipient_addr.is_none());
    let dispatched = DedustVaultNativeV2IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustVaultNativeV2IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultNativeV2_DedustVaultNativeV2PayOutFromDeposit() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultNativeV2_DedustVaultNativeV2PayOutFromDeposit")?;
    let decoded: DedustVaultNativeV2PayOutFromDepositMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 103034);
    assert_eq!(decoded.proof.contract_type, 3);
    assert_eq!(*decoded.amount, 103107);
    assert!(decoded.payload.is_none());
    let dispatched = DedustVaultNativeV2IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustVaultNativeV2IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultNativeV2_DedustVaultNativeV2ProvideVaultState() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultNativeV2_DedustVaultNativeV2ProvideVaultState")?;
    let decoded: DedustVaultNativeV2ProvideVaultStateMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 103326);
    let dispatched = DedustVaultNativeV2IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustVaultNativeV2IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultNativeV2_DedustVaultNativeV2ResetGas() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultNativeV2_DedustVaultNativeV2ResetGas")?;
    let decoded: DedustVaultNativeV2ResetGasMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 103472);
    assert_eq!(
        decoded.response_destination,
        TonAddress::from_str("0:000000000000000000000000000000000000000000000000000000000000058a")?
    );
    let dispatched = DedustVaultNativeV2IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustVaultNativeV2IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultNativeV2_DedustVaultNativeV2SwapExternal() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultNativeV2_DedustVaultNativeV2SwapExternal")?;
    let decoded: DedustVaultNativeV2SwapExternalMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 103691);
    assert_eq!(decoded.proof.contract_type, 1);
    assert_eq!(*decoded.amount, 103764);
    assert_eq!(
        decoded.sender_addr,
        TonAddress::from_str("0:000000000000000000000000000000000000000000000000000000000000058e")?
    );
    assert!(matches!(
        &decoded.current.kind,
        crate::contracts::classic::vault_native_v2::storage::DedustSwapKind::GivenIn(_)
    ));
    assert_eq!(*decoded.current.limit, 104129);
    assert_eq!(
        decoded
            .current
            .next
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .pool_addr,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000596")?
    );
    assert!(matches!(
        &decoded
            .current
            .next
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .params
            .kind,
        crate::contracts::classic::vault_native_v2::storage::DedustSwapKind::GivenIn(_)
    ));
    assert_eq!(
        *decoded
            .current
            .next
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .params
            .limit,
        104713
    );
    assert!(
        decoded
            .current
            .next
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .params
            .next
            .is_none()
    );
    assert_eq!(decoded.swap_params.deadline, 105005);
    assert_eq!(
        decoded.swap_params.recipient_addr,
        Some(TonAddress::from_str(
            "0:000000000000000000000000000000000000000000000000000000000000059f"
        )?)
    );
    assert!(decoded.swap_params.referral_addr.is_none());
    assert!(decoded.swap_params.reject_payload.is_none());
    let dispatched = DedustVaultNativeV2OutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustVaultNativeV2OutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultNativeV2_DedustVaultNativeV2CreateLiquidityDeposit() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultNativeV2_DedustVaultNativeV2CreateLiquidityDeposit")?;
    let decoded: DedustVaultNativeV2CreateLiquidityDepositMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 105516);
    assert_eq!(decoded.proof.contract_type, 1);
    assert_eq!(
        decoded.owner_addr,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000005a6")?
    );
    assert!(matches!(
        &decoded.pool_params.pool_type,
        crate::contracts::classic::vault_native_v2::storage::DedustPoolType::Volatile(_)
    ));
    assert!(matches!(&decoded.pool_params.asset0, DedustClassicAsset::Native(_)));
    assert!(matches!(&decoded.pool_params.asset1, DedustClassicAsset::Native(_)));
    assert_eq!(*decoded.deposit_params.asset0_target_balance, 106319);
    assert_eq!(*decoded.deposit_params.asset1_target_balance, 106392);
    assert!(matches!(&decoded.deposit_params.deposit_asset, DedustClassicAsset::Native(_)));
    assert_eq!(*decoded.deposit_params.deposit_amount, 106611);
    assert_eq!(*decoded.deposit_params.min_lp_amount, 106684);
    assert!(decoded.reject_payload.is_none());
    let dispatched = DedustVaultNativeV2OutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustVaultNativeV2OutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultNativeV2_DedustCreateLiquidityDepositParams() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultNativeV2_DedustCreateLiquidityDepositParams")?;
    let decoded: crate::contracts::classic::vault_native_v2::storage::DedustVaultNativeV2CreateLiquidityDepositParams =
        TLB::from_cell(&body)?;
    assert_eq!(*decoded.asset0_target_balance, 107049);
    assert_eq!(*decoded.asset1_target_balance, 107122);
    assert!(matches!(&decoded.deposit_asset, DedustClassicAsset::Native(_)));
    assert_eq!(*decoded.deposit_amount, 107341);
    assert_eq!(*decoded.min_lp_amount, 107414);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultNativeV2_DedustVaultNativeV2Payout() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultNativeV2_DedustVaultNativeV2Payout")?;
    let decoded: DedustVaultNativeV2PayoutMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 107560);
    let dispatched = DedustVaultNativeV2OutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustVaultNativeV2OutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultNativeV2_DedustVaultNativeV2TakeVaultState() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultNativeV2_DedustVaultNativeV2TakeVaultState")?;
    let decoded: DedustVaultNativeV2TakeVaultStateMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 107852);
    assert!(!decoded.is_ready);
    let dispatched = DedustVaultNativeV2OutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustVaultNativeV2OutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultJettonV1_DedustVaultJettonV1Storage() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultJettonV1_DedustVaultJettonV1Storage")?;
    let decoded: DedustVaultJettonV1Storage = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.cached_proof.factory_addr,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000005ca")?
    );
    assert_eq!(decoded.cached_proof.contract_type, 2);
    assert!(matches!(&decoded.cached_proof.asset, DedustClassicAsset::Native(_)));
    assert!(decoded.is_ready);
    assert_eq!(
        decoded.jetton_minter_addr,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000005d0")?
    );
    assert_eq!(
        decoded.jetton_wallet_addr,
        Some(TonAddress::from_str(
            "0:00000000000000000000000000000000000000000000000000000000000005d1"
        )?)
    );
    assert_eq!(decoded.version, 43265);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultJettonV1_DedustVaultJettonV1ProofData() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultJettonV1_DedustVaultJettonV1ProofData")?;
    let decoded: DedustVaultJettonV1ProofData = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.factory_addr,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000005d4")?
    );
    assert_eq!(decoded.contract_type, 220);
    assert!(matches!(&decoded.asset, DedustClassicAsset::Native(_)));
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultJettonV1_DedustVaultJettonV1NativeAsset() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultJettonV1_DedustVaultJettonV1NativeAsset")?;
    let decoded: DedustClassicNativeAsset = TLB::from_cell(&body)?;

    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultJettonV1_DedustVaultJettonV1JettonAsset() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultJettonV1_DedustVaultJettonV1JettonAsset")?;
    let decoded: DedustClassicJettonAsset = TLB::from_cell(&body)?;
    assert_eq!(decoded.workchain_id, -73);
    assert_eq!(
        decoded.address.to_hex(),
        "000000000000000000000000000000000000000000000000000000000001ab92"
    );
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultJettonV1_DedustVaultJettonV1VolatilePool() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultJettonV1_DedustVaultJettonV1VolatilePool")?;
    let decoded: DedustVaultJettonV1VolatilePool = TLB::from_cell(&body)?;

    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultJettonV1_DedustVaultJettonV1StablePool() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultJettonV1_DedustVaultJettonV1StablePool")?;
    let decoded: DedustVaultJettonV1StablePool = TLB::from_cell(&body)?;

    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultJettonV1_DedustVaultJettonV1PoolParams() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultJettonV1_DedustVaultJettonV1PoolParams")?;
    let decoded: DedustVaultJettonV1PoolParams = TLB::from_cell(&body)?;
    assert!(matches!(&decoded.pool_type, DedustVaultJettonV1PoolType::Stable(_)));
    let variant_1505 = match &decoded.asset0 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_1505.workchain_id, -90);
    assert_eq!(
        variant_1505.address.to_hex(),
        "000000000000000000000000000000000000000000000000000000000001ae23"
    );
    let variant_1509 = match &decoded.asset1 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_1509.workchain_id, -126);
    assert_eq!(
        variant_1509.address.to_hex(),
        "000000000000000000000000000000000000000000000000000000000001af47"
    );
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultJettonV1_DedustVaultJettonV1GivenIn() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultJettonV1_DedustVaultJettonV1GivenIn")?;
    let decoded: DedustVaultJettonV1GivenIn = TLB::from_cell(&body)?;

    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultJettonV1_DedustVaultJettonV1GivenOut() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultJettonV1_DedustVaultJettonV1GivenOut")?;
    let decoded: DedustVaultJettonV1GivenOut = TLB::from_cell(&body)?;

    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultJettonV1_DedustVaultJettonV1SwapStepParams() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultJettonV1_DedustVaultJettonV1SwapStepParams")?;
    let decoded: DedustVaultJettonV1SwapStepParams = TLB::from_cell(&body)?;
    assert!(matches!(&decoded.kind, DedustVaultJettonV1SwapKind::GivenIn(_)));
    assert_eq!(*decoded.limit, 110845);
    assert_eq!(
        decoded
            .next
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .pool_addr,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000005f2")?
    );
    assert!(matches!(
        &decoded
            .next
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .params
            .kind,
        DedustVaultJettonV1SwapKind::GivenIn(_)
    ));
    assert_eq!(
        *decoded
            .next
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .params
            .limit,
        111429
    );
    assert!(
        decoded
            .next
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .params
            .next
            .is_none()
    );
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultJettonV1_DedustVaultJettonV1SwapStep() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultJettonV1_DedustVaultJettonV1SwapStep")?;
    let decoded: DedustVaultJettonV1SwapStep = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.pool_addr,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000005f9")?
    );
    assert!(matches!(&decoded.params.kind, DedustVaultJettonV1SwapKind::GivenOut(_)));
    assert_eq!(*decoded.params.limit, 111940);
    assert_eq!(
        decoded
            .params
            .next
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .pool_addr,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000601")?
    );
    assert!(matches!(
        &decoded
            .params
            .next
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .params
            .kind,
        DedustVaultJettonV1SwapKind::GivenOut(_)
    ));
    assert_eq!(
        *decoded
            .params
            .next
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .params
            .limit,
        112524
    );
    assert!(
        decoded
            .params
            .next
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .params
            .next
            .is_none()
    );
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultJettonV1_DedustVaultJettonV1SwapParams() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultJettonV1_DedustVaultJettonV1SwapParams")?;
    let decoded: DedustVaultJettonV1SwapParams = TLB::from_cell(&body)?;
    assert_eq!(decoded.deadline, 112743);
    assert!(decoded.recipient_addr.is_none());
    assert_eq!(
        decoded.referral_addr,
        Some(TonAddress::from_str(
            "0:000000000000000000000000000000000000000000000000000000000000060a"
        )?)
    );
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultJettonV1_DedustVaultJettonV1TransferNotification() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultJettonV1_DedustVaultJettonV1TransferNotification")?;
    let decoded: DedustVaultJettonV1TransferNotificationMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 113327);
    assert_eq!(*decoded.amount, 113400);
    assert_eq!(
        decoded.sender,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000612")?
    );
    let dispatched = DedustVaultJettonV1IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustVaultJettonV1IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultJettonV1_DedustVaultJettonV1SwapForwardPayload() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultJettonV1_DedustVaultJettonV1SwapForwardPayload")?;
    let decoded: DedustVaultJettonV1SwapForwardPayloadMsg = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.step.pool_addr,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000616")?
    );
    assert!(matches!(&decoded.step.params.kind, DedustVaultJettonV1SwapKind::GivenIn(_)));
    assert_eq!(*decoded.step.params.limit, 114057);
    assert!(decoded.step.params.next.is_none());
    assert_eq!(decoded.swap_params.deadline, 114349);
    assert_eq!(
        decoded.swap_params.recipient_addr,
        Some(TonAddress::from_str(
            "0:000000000000000000000000000000000000000000000000000000000000061f"
        )?)
    );
    assert_eq!(
        decoded.swap_params.referral_addr,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000620"
        )?)
    );
    assert!(decoded.swap_params.fulfill_payload.is_none());
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultJettonV1_DedustVaultJettonV1DepositLiquidityForwardPayload() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultJettonV1_DedustVaultJettonV1DepositLiquidityForwardPayload")?;
    let decoded: DedustVaultJettonV1DepositLiquidityForwardPayloadMsg = TLB::from_cell(&body)?;
    assert!(matches!(
        &decoded.pool_params.pool_type,
        DedustVaultJettonV1PoolType::Volatile(_)
    ));
    assert!(matches!(&decoded.pool_params.asset0, DedustClassicAsset::Native(_)));
    assert!(matches!(&decoded.pool_params.asset1, DedustClassicAsset::Native(_)));
    assert_eq!(*decoded.min_lp_amount, 115371);
    assert_eq!(*decoded.asset0_target_balance, 115444);
    assert_eq!(*decoded.asset1_target_balance, 115517);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultJettonV1_DedustVaultJettonV1PayOutFromPool() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultJettonV1_DedustVaultJettonV1PayOutFromPool")?;
    let decoded: DedustVaultJettonV1PayOutFromPoolMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 115955);
    assert_eq!(decoded.proof.contract_type, 2);
    assert_eq!(*decoded.amount, 116028);
    assert!(decoded.recipient_addr.is_none());
    let dispatched = DedustVaultJettonV1IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustVaultJettonV1IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultJettonV1_DedustVaultJettonV1PayOutFromDeposit() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultJettonV1_DedustVaultJettonV1PayOutFromDeposit")?;
    let decoded: DedustVaultJettonV1PayOutFromDepositMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 116393);
    assert_eq!(decoded.proof.contract_type, 3);
    assert_eq!(*decoded.amount, 116466);
    assert!(decoded.payload.is_none());
    let dispatched = DedustVaultJettonV1IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustVaultJettonV1IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultJettonV1_DedustVaultJettonV1UpgradeChildCode() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultJettonV1_DedustVaultJettonV1UpgradeChildCode")?;
    let decoded: DedustVaultJettonV1UpgradeChildCodeMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 116685);
    assert_eq!(decoded.code_version, 51222);
    let dispatched = DedustVaultJettonV1IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustVaultJettonV1IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultJettonV1_DedustVaultJettonV1TakeWalletAddress() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultJettonV1_DedustVaultJettonV1TakeWalletAddress")?;
    let decoded: DedustVaultJettonV1TakeWalletAddressMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 116977);
    assert_eq!(
        decoded.wallet_address,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000643")?
    );
    assert!(!decoded.include_address);
    assert!(decoded.owner_address.is_none());
    let dispatched = DedustVaultJettonV1IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustVaultJettonV1IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultJettonV1_DedustVaultJettonV1DestroyNonReady() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultJettonV1_DedustVaultJettonV1DestroyNonReady")?;
    let decoded: DedustVaultJettonV1DestroyNonReadyMsg = TLB::from_cell(&body)?;
    let dispatched = DedustVaultJettonV1IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustVaultJettonV1IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultJettonV1_DedustVaultJettonV1SwapExternal() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultJettonV1_DedustVaultJettonV1SwapExternal")?;
    let decoded: DedustVaultJettonV1SwapExternalMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 117488);
    assert_eq!(decoded.proof.contract_type, 1);
    assert_eq!(*decoded.amount, 117561);
    assert_eq!(
        decoded.sender_addr,
        TonAddress::from_str("0:000000000000000000000000000000000000000000000000000000000000064b")?
    );
    assert!(matches!(&decoded.current.kind, DedustVaultJettonV1SwapKind::GivenOut(_)));
    assert_eq!(*decoded.current.limit, 117926);
    assert_eq!(
        decoded
            .current
            .next
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .pool_addr,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000653")?
    );
    assert!(matches!(
        &decoded
            .current
            .next
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .params
            .kind,
        DedustVaultJettonV1SwapKind::GivenOut(_)
    ));
    assert_eq!(
        *decoded
            .current
            .next
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .params
            .limit,
        118510
    );
    assert!(
        decoded
            .current
            .next
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .params
            .next
            .is_none()
    );
    assert_eq!(decoded.swap_params.deadline, 118802);
    assert_eq!(
        decoded.swap_params.recipient_addr,
        Some(TonAddress::from_str(
            "0:000000000000000000000000000000000000000000000000000000000000065c"
        )?)
    );
    assert!(decoded.swap_params.referral_addr.is_none());
    assert!(decoded.swap_params.reject_payload.is_none());
    let dispatched = DedustVaultJettonV1OutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustVaultJettonV1OutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultJettonV1_DedustVaultJettonV1CreateLiquidityDeposit() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultJettonV1_DedustVaultJettonV1CreateLiquidityDeposit")?;
    let decoded: DedustVaultJettonV1CreateLiquidityDepositMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 119313);
    assert_eq!(decoded.proof.contract_type, 1);
    assert_eq!(
        decoded.owner_addr,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000663")?
    );
    assert!(matches!(&decoded.pool_params.pool_type, DedustVaultJettonV1PoolType::Stable(_)));
    let variant_1639 = match &decoded.pool_params.asset0 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_1639.workchain_id, -16);
    assert_eq!(
        variant_1639.address.to_hex(),
        "000000000000000000000000000000000000000000000000000000000001d459"
    );
    let variant_1643 = match &decoded.pool_params.asset1 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_1643.workchain_id, -52);
    assert_eq!(
        variant_1643.address.to_hex(),
        "000000000000000000000000000000000000000000000000000000000001d57d"
    );
    assert_eq!(*decoded.deposit_params.asset0_target_balance, 120408);
    assert_eq!(*decoded.deposit_params.asset1_target_balance, 120481);
    let variant_1651 = match &decoded.deposit_params.deposit_asset {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_1651.workchain_id, -124);
    assert_eq!(
        variant_1651.address.to_hex(),
        "000000000000000000000000000000000000000000000000000000000001d7c5"
    );
    assert_eq!(*decoded.deposit_params.deposit_amount, 120846);
    assert_eq!(*decoded.deposit_params.min_lp_amount, 120919);
    assert!(decoded.reject_payload.is_none());
    let dispatched = DedustVaultJettonV1OutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustVaultJettonV1OutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultJettonV1_DedustVaultJettonV1CreateLiquidityDepositParams() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultJettonV1_DedustVaultJettonV1CreateLiquidityDepositParams")?;
    let decoded: DedustVaultJettonV1CreateLiquidityDepositParams = TLB::from_cell(&body)?;
    assert_eq!(*decoded.asset0_target_balance, 121284);
    assert_eq!(*decoded.asset1_target_balance, 121357);
    let variant_1663 = match &decoded.deposit_asset {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_1663.workchain_id, -104);
    assert_eq!(
        variant_1663.address.to_hex(),
        "000000000000000000000000000000000000000000000000000000000001db31"
    );
    assert_eq!(*decoded.deposit_amount, 121722);
    assert_eq!(*decoded.min_lp_amount, 121795);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultJettonV1_DedustVaultJettonV1JettonTransfer() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultJettonV1_DedustVaultJettonV1JettonTransfer")?;
    let decoded: DedustVaultJettonV1JettonTransferMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 121941);
    assert_eq!(*decoded.amount, 122014);
    assert_eq!(
        decoded.destination,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000688"
        )?)
    );
    assert_eq!(
        decoded.response_destination,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000689"
        )?)
    );
    assert!(decoded.custom_payload.is_none());
    assert_eq!(*decoded.forward_ton_amount, 122306);
    let dispatched = DedustVaultJettonV1OutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustVaultJettonV1OutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultJettonV1_DedustVaultJettonV1ProvideWalletAddress() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultJettonV1_DedustVaultJettonV1ProvideWalletAddress")?;
    let decoded: DedustVaultJettonV1ProvideWalletAddressMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 122525);
    assert_eq!(
        decoded.owner_address,
        TonAddress::from_str("0:000000000000000000000000000000000000000000000000000000000000068f")?
    );
    assert!(!decoded.include_address);
    let dispatched = DedustVaultJettonV1OutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustVaultJettonV1OutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultJettonV1_DedustVaultJettonV1BouncedProvideWalletAddress() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultJettonV1_DedustVaultJettonV1BouncedProvideWalletAddress")?;
    let decoded: DedustVaultJettonV1BouncedProvideWalletAddressMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.original_opcode, 122817);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultJettonV2_DedustVaultJettonV2Storage() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultJettonV2_DedustVaultJettonV2Storage")?;
    let decoded: DedustVaultJettonV2Storage = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.cached_proof.factory_addr,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000697")?
    );
    assert_eq!(decoded.cached_proof.contract_type, 119);
    let variant_1689 = match &decoded.cached_proof.asset {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_1689.workchain_id, -82);
    assert_eq!(
        variant_1689.address.to_hex(),
        "000000000000000000000000000000000000000000000000000000000001e29b"
    );
    assert!(!decoded.is_ready);
    assert_eq!(
        decoded.jetton_minter_addr,
        TonAddress::from_str("0:000000000000000000000000000000000000000000000000000000000000069f")?
    );
    assert_eq!(
        decoded.jetton_wallet_addr,
        Some(TonAddress::from_str(
            "0:00000000000000000000000000000000000000000000000000000000000006a0"
        )?)
    );
    assert_eq!(decoded.version, 58376);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultJettonV2_DedustVaultJettonV2ProofData() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultJettonV2_DedustVaultJettonV2ProofData")?;
    let decoded: DedustVaultJettonV2ProofData = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.factory_addr,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000006a3")?
    );
    assert_eq!(decoded.contract_type, 227);
    let variant_1701 = match &decoded.asset {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_1701.workchain_id, -62);
    assert_eq!(
        variant_1701.address.to_hex(),
        "000000000000000000000000000000000000000000000000000000000001e607"
    );
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultJettonV2_DedustNativeAsset() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultJettonV2_DedustNativeAsset")?;
    let decoded: DedustClassicNativeAsset = TLB::from_cell(&body)?;

    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultJettonV2_DedustJettonAsset() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultJettonV2_DedustJettonAsset")?;
    let decoded: DedustClassicJettonAsset = TLB::from_cell(&body)?;
    assert_eq!(decoded.workchain_id, -98);
    assert_eq!(
        decoded.address.to_hex(),
        "000000000000000000000000000000000000000000000000000000000001e72b"
    );
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultJettonV2_DedustVolatilePool() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultJettonV2_DedustVolatilePool")?;
    let decoded: crate::contracts::classic::vault_native_v2::storage::DedustVolatilePool = TLB::from_cell(&body)?;

    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultJettonV2_DedustStablePool() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultJettonV2_DedustStablePool")?;
    let decoded: crate::contracts::classic::vault_native_v2::storage::DedustStablePool = TLB::from_cell(&body)?;

    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultJettonV2_DedustPoolParams() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultJettonV2_DedustPoolParams")?;
    let decoded: crate::contracts::classic::vault_native_v2::storage::DedustVaultNativeV2PoolParams =
        TLB::from_cell(&body)?;
    assert!(matches!(
        &decoded.pool_type,
        crate::contracts::classic::vault_native_v2::storage::DedustPoolType::Volatile(_)
    ));
    assert!(matches!(&decoded.asset0, DedustClassicAsset::Native(_)));
    assert!(matches!(&decoded.asset1, DedustClassicAsset::Native(_)));
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultJettonV2_DedustGivenIn() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultJettonV2_DedustGivenIn")?;
    let decoded: crate::contracts::classic::vault_native_v2::storage::DedustGivenIn = TLB::from_cell(&body)?;

    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultJettonV2_DedustGivenOut() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultJettonV2_DedustGivenOut")?;
    let decoded: crate::contracts::classic::vault_native_v2::storage::DedustGivenOut = TLB::from_cell(&body)?;

    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultJettonV2_DedustSwapStepParams() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultJettonV2_DedustSwapStepParams")?;
    let decoded: crate::contracts::classic::vault_native_v2::storage::DedustSwapStepParams = TLB::from_cell(&body)?;
    assert!(matches!(
        &decoded.kind,
        crate::contracts::classic::vault_native_v2::storage::DedustSwapKind::GivenOut(_)
    ));
    assert_eq!(*decoded.limit, 125810);
    assert_eq!(
        decoded
            .next
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .pool_addr,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000006bf")?
    );
    assert!(matches!(
        &decoded
            .next
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .params
            .kind,
        crate::contracts::classic::vault_native_v2::storage::DedustSwapKind::GivenOut(_)
    ));
    assert_eq!(
        *decoded
            .next
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .params
            .limit,
        126394
    );
    assert!(
        decoded
            .next
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .params
            .next
            .is_none()
    );
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultJettonV2_DedustSwapStep() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultJettonV2_DedustSwapStep")?;
    let decoded: crate::contracts::classic::vault_native_v2::storage::DedustSwapStep = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.pool_addr,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000006c6")?
    );
    assert!(matches!(
        &decoded.params.kind,
        crate::contracts::classic::vault_native_v2::storage::DedustSwapKind::GivenIn(_)
    ));
    assert_eq!(*decoded.params.limit, 126905);
    assert_eq!(
        decoded
            .params
            .next
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .pool_addr,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000006ce")?
    );
    assert!(matches!(
        &decoded
            .params
            .next
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .params
            .kind,
        crate::contracts::classic::vault_native_v2::storage::DedustSwapKind::GivenIn(_)
    ));
    assert_eq!(
        *decoded
            .params
            .next
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .params
            .limit,
        127489
    );
    assert!(
        decoded
            .params
            .next
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .params
            .next
            .is_none()
    );
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultJettonV2_DedustSwapParams() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultJettonV2_DedustSwapParams")?;
    let decoded: crate::contracts::classic::vault_native_v2::storage::DedustSwapParams = TLB::from_cell(&body)?;
    assert_eq!(decoded.deadline, 127708);
    assert_eq!(
        decoded.recipient_addr,
        Some(TonAddress::from_str(
            "0:00000000000000000000000000000000000000000000000000000000000006d6"
        )?)
    );
    assert_eq!(
        decoded.referral_addr,
        Some(TonAddress::from_str(
            "0:00000000000000000000000000000000000000000000000000000000000006d7"
        )?)
    );
    assert!(decoded.fulfill_payload.is_none());
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultJettonV2_DedustVaultJettonV2TransferNotification() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultJettonV2_DedustVaultJettonV2TransferNotification")?;
    let decoded: DedustVaultJettonV2TransferNotificationMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 128219);
    assert_eq!(*decoded.amount, 128292);
    assert_eq!(
        decoded.sender,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000006de")?
    );
    let dispatched = DedustVaultJettonV2IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustVaultJettonV2IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultJettonV2_DedustVaultJettonV2SwapForwardPayload() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultJettonV2_DedustVaultJettonV2SwapForwardPayload")?;
    let decoded: DedustVaultJettonV2SwapForwardPayloadMsg = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.step.pool_addr,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000006e2")?
    );
    assert!(matches!(
        &decoded.step.params.kind,
        crate::contracts::classic::vault_native_v2::storage::DedustSwapKind::GivenIn(_)
    ));
    assert_eq!(*decoded.step.params.limit, 128949);
    assert!(decoded.step.params.next.is_none());
    assert_eq!(decoded.swap_params.deadline, 129241);
    assert_eq!(
        decoded.swap_params.recipient_addr,
        Some(TonAddress::from_str(
            "0:00000000000000000000000000000000000000000000000000000000000006eb"
        )?)
    );
    assert_eq!(
        decoded.swap_params.referral_addr,
        Some(TonAddress::from_str(
            "0:00000000000000000000000000000000000000000000000000000000000006ec"
        )?)
    );
    assert!(decoded.swap_params.fulfill_payload.is_none());
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultJettonV2_DedustVaultJettonV2DepositLiquidityForwardPayload() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultJettonV2_DedustVaultJettonV2DepositLiquidityForwardPayload")?;
    let decoded: DedustVaultJettonV2DepositLiquidityForwardPayloadMsg = TLB::from_cell(&body)?;
    assert!(matches!(
        &decoded.pool_params.pool_type,
        crate::contracts::classic::vault_native_v2::storage::DedustPoolType::Volatile(_)
    ));
    assert!(matches!(&decoded.pool_params.asset0, DedustClassicAsset::Native(_)));
    assert!(matches!(&decoded.pool_params.asset1, DedustClassicAsset::Native(_)));
    assert_eq!(*decoded.min_lp_amount, 130263);
    assert_eq!(*decoded.asset0_target_balance, 130336);
    assert_eq!(*decoded.asset1_target_balance, 130409);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultJettonV2_DedustVaultJettonV2PayOutFromPool() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultJettonV2_DedustVaultJettonV2PayOutFromPool")?;
    let decoded: DedustVaultJettonV2PayOutFromPoolMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 130847);
    assert_eq!(decoded.proof.contract_type, 2);
    assert_eq!(*decoded.amount, 130920);
    assert!(decoded.recipient_addr.is_none());
    let dispatched = DedustVaultJettonV2IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustVaultJettonV2IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultJettonV2_DedustVaultJettonV2PayOutFromDeposit() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultJettonV2_DedustVaultJettonV2PayOutFromDeposit")?;
    let decoded: DedustVaultJettonV2PayOutFromDepositMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 131285);
    assert_eq!(decoded.proof.contract_type, 3);
    assert_eq!(*decoded.amount, 131358);
    assert!(decoded.payload.is_none());
    let dispatched = DedustVaultJettonV2IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustVaultJettonV2IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultJettonV2_DedustVaultJettonV2ProvideVaultState() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultJettonV2_DedustVaultJettonV2ProvideVaultState")?;
    let decoded: DedustVaultJettonV2ProvideVaultStateMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 131577);
    let dispatched = DedustVaultJettonV2IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustVaultJettonV2IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultJettonV2_DedustVaultJettonV2ResetGas() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultJettonV2_DedustVaultJettonV2ResetGas")?;
    let decoded: DedustVaultJettonV2ResetGasMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 131723);
    assert_eq!(
        decoded.response_destination,
        TonAddress::from_str("0:000000000000000000000000000000000000000000000000000000000000070d")?
    );
    let dispatched = DedustVaultJettonV2IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustVaultJettonV2IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultJettonV2_DedustVaultJettonV2DestroyNonReady() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultJettonV2_DedustVaultJettonV2DestroyNonReady")?;
    let decoded: DedustVaultJettonV2DestroyNonReadyMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 131942);
    let dispatched = DedustVaultJettonV2IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustVaultJettonV2IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultJettonV2_DedustVaultJettonV2SwapExternal() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultJettonV2_DedustVaultJettonV2SwapExternal")?;
    let decoded: DedustVaultJettonV2SwapExternalMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 132088);
    assert_eq!(decoded.proof.contract_type, 1);
    assert_eq!(*decoded.amount, 132161);
    assert_eq!(
        decoded.sender_addr,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000713")?
    );
    assert!(matches!(
        &decoded.current.kind,
        crate::contracts::classic::vault_native_v2::storage::DedustSwapKind::GivenOut(_)
    ));
    assert_eq!(*decoded.current.limit, 132526);
    assert_eq!(
        decoded
            .current
            .next
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .pool_addr,
        TonAddress::from_str("0:000000000000000000000000000000000000000000000000000000000000071b")?
    );
    assert!(matches!(
        &decoded
            .current
            .next
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .params
            .kind,
        crate::contracts::classic::vault_native_v2::storage::DedustSwapKind::GivenOut(_)
    ));
    assert_eq!(
        *decoded
            .current
            .next
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .params
            .limit,
        133110
    );
    assert!(
        decoded
            .current
            .next
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .params
            .next
            .is_none()
    );
    assert_eq!(decoded.swap_params.deadline, 133402);
    assert_eq!(
        decoded.swap_params.recipient_addr,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000724"
        )?)
    );
    assert_eq!(
        decoded.swap_params.referral_addr,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000725"
        )?)
    );
    assert!(decoded.swap_params.fulfill_payload.is_none());
    let dispatched = DedustVaultJettonV2OutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustVaultJettonV2OutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultJettonV2_DedustVaultJettonV2CreateLiquidityDeposit() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultJettonV2_DedustVaultJettonV2CreateLiquidityDeposit")?;
    let decoded: DedustVaultJettonV2CreateLiquidityDepositMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 133913);
    assert_eq!(decoded.proof.contract_type, 1);
    assert_eq!(
        decoded.owner_addr,
        TonAddress::from_str("0:000000000000000000000000000000000000000000000000000000000000072b")?
    );
    assert!(matches!(
        &decoded.pool_params.pool_type,
        crate::contracts::classic::vault_native_v2::storage::DedustPoolType::Stable(_)
    ));
    let variant_1839 = match &decoded.pool_params.asset0 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_1839.workchain_id, -24);
    assert_eq!(
        variant_1839.address.to_hex(),
        "0000000000000000000000000000000000000000000000000000000000020d61"
    );
    let variant_1843 = match &decoded.pool_params.asset1 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_1843.workchain_id, -60);
    assert_eq!(
        variant_1843.address.to_hex(),
        "0000000000000000000000000000000000000000000000000000000000020e85"
    );
    assert_eq!(*decoded.deposit_params.asset0_target_balance, 135008);
    assert_eq!(*decoded.deposit_params.asset1_target_balance, 135081);
    let variant_1851 = match &decoded.deposit_params.deposit_asset {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_1851.workchain_id, -4);
    assert_eq!(
        variant_1851.address.to_hex(),
        "00000000000000000000000000000000000000000000000000000000000210cd"
    );
    assert_eq!(*decoded.deposit_params.deposit_amount, 135446);
    assert_eq!(*decoded.deposit_params.min_lp_amount, 135519);
    assert!(decoded.fulfill_payload.is_none());
    let dispatched = DedustVaultJettonV2OutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustVaultJettonV2OutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultJettonV2_DedustCreateLiquidityDepositParams() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultJettonV2_DedustCreateLiquidityDepositParams")?;
    let decoded: crate::contracts::classic::vault_jetton_v2::storage::DedustVaultJettonV2CreateLiquidityDepositParams =
        TLB::from_cell(&body)?;
    assert_eq!(*decoded.asset0_target_balance, 135884);
    assert_eq!(*decoded.asset1_target_balance, 135957);
    let variant_1863 = match &decoded.deposit_asset {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_1863.workchain_id, -112);
    assert_eq!(
        variant_1863.address.to_hex(),
        "0000000000000000000000000000000000000000000000000000000000021439"
    );
    assert_eq!(*decoded.deposit_amount, 136322);
    assert_eq!(*decoded.min_lp_amount, 136395);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultJettonV2_DedustVaultJettonV2JettonTransfer() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultJettonV2_DedustVaultJettonV2JettonTransfer")?;
    let decoded: DedustVaultJettonV2JettonTransferMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 136541);
    assert_eq!(*decoded.amount, 136614);
    assert!(decoded.destination.is_none());
    assert_eq!(
        decoded.response_destination,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000751"
        )?)
    );
    assert_eq!(*decoded.forward_ton_amount, 136979);
    let dispatched = DedustVaultJettonV2OutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustVaultJettonV2OutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultJettonV2_DedustVaultJettonV2TakeVaultState() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultJettonV2_DedustVaultJettonV2TakeVaultState")?;
    let decoded: DedustVaultJettonV2TakeVaultStateMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 137198);
    assert!(!decoded.is_ready);
    let dispatched = DedustVaultJettonV2OutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustVaultJettonV2OutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultJettonV2_DedustVaultJettonV2ProvideWalletAddress() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultJettonV2_DedustVaultJettonV2ProvideWalletAddress")?;
    let decoded: DedustVaultJettonV2ProvideWalletAddressMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 137417);
    assert_eq!(
        decoded.owner_address,
        TonAddress::from_str("0:000000000000000000000000000000000000000000000000000000000000075b")?
    );
    assert!(!decoded.include_address);
    let dispatched = DedustVaultJettonV2OutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustVaultJettonV2OutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustVaultJettonV2_DedustVaultJettonV2TakeWalletAddress() -> anyhow::Result<()> {
    let body = fixture("schema_DedustVaultJettonV2_DedustVaultJettonV2TakeWalletAddress")?;
    let decoded: DedustVaultJettonV2TakeWalletAddressMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 137709);
    assert_eq!(
        decoded.wallet_address,
        TonAddress::from_str("0:000000000000000000000000000000000000000000000000000000000000075f")?
    );
    assert!(!decoded.include_address);
    assert_eq!(
        (decoded
            .owner_address
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?)
        .to_cell()?
        .hash()?,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000763")?
            .to_msg_address_int()
            .to_cell()?
            .hash()?
    );
    let dispatched = DedustVaultJettonV2IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustVaultJettonV2IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustLiquidityDepositV1_DedustLiquidityDepositV1Storage() -> anyhow::Result<()> {
    let body = fixture("schema_DedustLiquidityDepositV1_DedustLiquidityDepositV1Storage")?;
    let decoded: DedustLiquidityDepositV1Storage = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.proof.factory_addr,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000767")?
    );
    assert_eq!(decoded.proof.contract_type, 199);
    assert_eq!(
        decoded.proof.params.owner_addr,
        TonAddress::from_str("0:000000000000000000000000000000000000000000000000000000000000076b")?
    );
    assert!(decoded.proof.params.pool_params.is_stable);
    assert!(matches!(
        &decoded.proof.params.pool_params.asset0,
        DedustClassicAsset::Native(_)
    ));
    assert!(matches!(
        &decoded.proof.params.pool_params.asset1,
        DedustClassicAsset::Native(_)
    ));
    assert_eq!(
        decoded.factory_addr,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000772")?
    );
    assert_eq!(
        decoded.owner_addr,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000774")?
    );
    assert_eq!(*decoded.asset0_balance, 139388);
    assert_eq!(*decoded.asset1_balance, 139461);
    assert!(decoded.is_processing);
    assert_eq!(*decoded.min_lp_amount, 139607);
    assert!(decoded.details.reject_payload.is_none());
    assert!(decoded.details.pool_params.is_stable);
    assert!(matches!(&decoded.details.pool_params.asset0, DedustClassicAsset::Native(_)));
    assert!(matches!(&decoded.details.pool_params.asset1, DedustClassicAsset::Native(_)));
    assert_eq!(*decoded.details.asset0_target_balance, 140483);
    assert_eq!(*decoded.details.asset1_target_balance, 140556);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustLiquidityDepositV1_DedustLiquidityDepositV1StorageDetails() -> anyhow::Result<()> {
    let body = fixture("schema_DedustLiquidityDepositV1_DedustLiquidityDepositV1StorageDetails")?;
    let decoded: DedustLiquidityDepositV1StorageDetails = TLB::from_cell(&body)?;
    assert!(decoded.reject_payload.is_none());
    assert!(decoded.pool_params.is_stable);
    assert!(matches!(&decoded.pool_params.asset0, DedustClassicAsset::Native(_)));
    assert!(matches!(&decoded.pool_params.asset1, DedustClassicAsset::Native(_)));
    assert_eq!(*decoded.asset0_target_balance, 141359);
    assert_eq!(*decoded.asset1_target_balance, 141432);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustLiquidityDepositV1_DedustAssetNative() -> anyhow::Result<()> {
    let body = fixture("schema_DedustLiquidityDepositV1_DedustAssetNative")?;
    let decoded: DedustClassicNativeAsset = TLB::from_cell(&body)?;

    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustLiquidityDepositV1_DedustAssetJetton() -> anyhow::Result<()> {
    let body = fixture("schema_DedustLiquidityDepositV1_DedustAssetJetton")?;
    let decoded: DedustClassicJettonAsset = TLB::from_cell(&body)?;
    assert_eq!(decoded.workchain_id, -83);
    assert_eq!(
        decoded.address.to_hex(),
        "000000000000000000000000000000000000000000000000000000000002299c"
    );
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustLiquidityDepositV1_DedustPoolParams() -> anyhow::Result<()> {
    let body = fixture("schema_DedustLiquidityDepositV1_DedustPoolParams")?;
    let decoded: crate::contracts::classic::liquidity_deposit_v1::storage::DedustLiquidityDepositV1PoolParams =
        TLB::from_cell(&body)?;
    assert!(decoded.is_stable);
    assert!(matches!(&decoded.asset0, DedustClassicAsset::Native(_)));
    assert!(matches!(&decoded.asset1, DedustClassicAsset::Native(_)));
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustLiquidityDepositV1_DedustLiquidityDepositProofParams() -> anyhow::Result<()> {
    let body = fixture("schema_DedustLiquidityDepositV1_DedustLiquidityDepositProofParams")?;
    let decoded: crate::contracts::classic::liquidity_deposit_v1::storage::DedustLiquidityDepositV1ProofParams =
        TLB::from_cell(&body)?;
    assert_eq!(
        decoded.owner_addr,
        TonAddress::from_str("0:000000000000000000000000000000000000000000000000000000000000079d")?
    );
    assert!(decoded.pool_params.is_stable);
    assert!(matches!(&decoded.pool_params.asset0, DedustClassicAsset::Native(_)));
    assert!(matches!(&decoded.pool_params.asset1, DedustClassicAsset::Native(_)));
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustLiquidityDepositV1_DedustLiquidityDepositProof() -> anyhow::Result<()> {
    let body = fixture("schema_DedustLiquidityDepositV1_DedustLiquidityDepositProof")?;
    let decoded: crate::contracts::classic::liquidity_deposit_v1::storage::DedustLiquidityDepositV1Proof =
        TLB::from_cell(&body)?;
    assert_eq!(
        decoded.factory_addr,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000007a5")?
    );
    assert_eq!(decoded.contract_type, 117);
    assert_eq!(
        decoded.params.owner_addr,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000007a9")?
    );
    assert!(decoded.params.pool_params.is_stable);
    assert!(matches!(&decoded.params.pool_params.asset0, DedustClassicAsset::Native(_)));
    assert!(matches!(&decoded.params.pool_params.asset1, DedustClassicAsset::Native(_)));
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustLiquidityDepositV1_DedustPoolProof() -> anyhow::Result<()> {
    let body = fixture("schema_DedustLiquidityDepositV1_DedustPoolProof")?;
    let decoded: crate::contracts::classic::liquidity_deposit_v1::storage::DedustLiquidityDepositV1PoolProof =
        TLB::from_cell(&body)?;
    assert_eq!(
        decoded.factory_addr,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000007b1")?
    );
    assert_eq!(decoded.contract_type, 225);
    assert!(!decoded.params.is_stable);
    let variant_1973 = match &decoded.params.asset0 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_1973.workchain_id, -78);
    assert_eq!(
        variant_1973.address.to_hex(),
        "0000000000000000000000000000000000000000000000000000000000023397"
    );
    let variant_1977 = match &decoded.params.asset1 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_1977.workchain_id, -114);
    assert_eq!(
        variant_1977.address.to_hex(),
        "00000000000000000000000000000000000000000000000000000000000234bb"
    );
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustLiquidityDepositV1_DedustLiquidityDepositInstall() -> anyhow::Result<()> {
    let body = fixture("schema_DedustLiquidityDepositV1_DedustLiquidityDepositInstall")?;
    let decoded: crate::contracts::classic::liquidity_deposit_v1::msgs::DedustLiquidityDepositV1InstallMsg =
        TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 144717);
    assert_eq!(decoded.code_version, 13791);
    assert_eq!(*decoded.extra_params.min_lp_amount, 17);
    assert_eq!(*decoded.extra_params.asset0_target_balance, 100);
    assert_eq!(*decoded.extra_params.asset1_target_balance, 200);
    let dispatched = DedustLiquidityDepositV1IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustLiquidityDepositV1IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustLiquidityDepositV1_DedustDepositAsset() -> anyhow::Result<()> {
    let body = fixture("schema_DedustLiquidityDepositV1_DedustDepositAsset")?;
    let decoded: crate::contracts::classic::liquidity_deposit_v1::msgs::DedustLiquidityDepositV1DepositAssetMsg =
        TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 145082);
    assert!(matches!(&decoded.asset, DedustClassicAsset::Native(_)));
    assert_eq!(*decoded.amount, 145301);
    let dispatched = DedustLiquidityDepositV1IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustLiquidityDepositV1IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustLiquidityDepositV1_DedustFulfillDeposit() -> anyhow::Result<()> {
    let body = fixture("schema_DedustLiquidityDepositV1_DedustFulfillDeposit")?;
    let decoded: crate::contracts::classic::liquidity_deposit_v1::msgs::DedustLiquidityDepositV1FulfillDepositMsg =
        TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 145447);
    assert_eq!(
        decoded.proof.factory_addr,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000007cb")?
    );
    assert_eq!(decoded.proof.contract_type, 75);
    assert!(!decoded.proof.params.is_stable);
    let variant_1999 = match &decoded.proof.params.asset0 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_1999.workchain_id, -56);
    assert_eq!(
        variant_1999.address.to_hex(),
        "0000000000000000000000000000000000000000000000000000000000023b01"
    );
    let variant_2003 = match &decoded.proof.params.asset1 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_2003.workchain_id, -92);
    assert_eq!(
        variant_2003.address.to_hex(),
        "0000000000000000000000000000000000000000000000000000000000023c25"
    );
    assert_eq!(*decoded.asset0_deposit, 146542);
    assert_eq!(*decoded.asset1_deposit, 146615);
    let dispatched = DedustLiquidityDepositV1IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustLiquidityDepositV1IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustLiquidityDepositV1_DedustRejectDeposit() -> anyhow::Result<()> {
    let body = fixture("schema_DedustLiquidityDepositV1_DedustRejectDeposit")?;
    let decoded: crate::contracts::classic::liquidity_deposit_v1::msgs::DedustLiquidityDepositV1RejectDepositMsg =
        TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 146761);
    assert_eq!(
        decoded.proof.factory_addr,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000007dd")?
    );
    assert_eq!(decoded.proof.contract_type, 109);
    assert!(!decoded.proof.params.is_stable);
    let variant_2017 = match &decoded.proof.params.asset0 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_2017.workchain_id, -90);
    assert_eq!(
        variant_2017.address.to_hex(),
        "0000000000000000000000000000000000000000000000000000000000024023"
    );
    let variant_2021 = match &decoded.proof.params.asset1 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_2021.workchain_id, -126);
    assert_eq!(
        variant_2021.address.to_hex(),
        "0000000000000000000000000000000000000000000000000000000000024147"
    );
    assert!(decoded.payload.is_none());
    let dispatched = DedustLiquidityDepositV1IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustLiquidityDepositV1IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustLiquidityDepositV1_DedustCancelDeposit() -> anyhow::Result<()> {
    let body = fixture("schema_DedustLiquidityDepositV1_DedustCancelDeposit")?;
    let decoded: crate::contracts::classic::liquidity_deposit_v1::msgs::DedustLiquidityDepositV1CancelDepositMsg =
        TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 148002);
    assert!(decoded.payload.is_none());
    let dispatched = DedustLiquidityDepositV1IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustLiquidityDepositV1IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustLiquidityDepositV1_DedustDepositLiquidityAllAssets() -> anyhow::Result<()> {
    let body = fixture("schema_DedustLiquidityDepositV1_DedustDepositLiquidityAllAssets")?;
    let decoded: crate::contracts::classic::liquidity_deposit_v1::storage::DedustLiquidityDepositV1DepositLiquidityAllAssets =
        TLB::from_cell(&body)?;
    assert!(matches!(&decoded.asset0, DedustClassicAsset::Native(_)));
    assert_eq!(*decoded.asset0_amount, 148367);
    let variant_2033 = match &decoded.asset1 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_2033.workchain_id, -106);
    assert_eq!(
        variant_2033.address.to_hex(),
        "00000000000000000000000000000000000000000000000000000000000244b3"
    );
    assert_eq!(*decoded.asset1_amount, 148732);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustLiquidityDepositV1_DedustDepositLiquidityAll() -> anyhow::Result<()> {
    let body = fixture("schema_DedustLiquidityDepositV1_DedustDepositLiquidityAll")?;
    let decoded: crate::contracts::classic::liquidity_deposit_v1::msgs::DedustLiquidityDepositV1DepositLiquidityAllMsg =
        TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 148878);
    assert_eq!(
        decoded.proof.factory_addr,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000007fa")?
    );
    assert_eq!(decoded.proof.contract_type, 178);
    assert_eq!(
        decoded.proof.params.owner_addr,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000007fe")?
    );
    assert!(!decoded.proof.params.pool_params.is_stable);
    let variant_2049 = match &decoded.proof.params.pool_params.asset0 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_2049.workchain_id, -122);
    assert_eq!(
        variant_2049.address.to_hex(),
        "0000000000000000000000000000000000000000000000000000000000024943"
    );
    let variant_2053 = match &decoded.proof.params.pool_params.asset1 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_2053.workchain_id, -30);
    assert_eq!(
        variant_2053.address.to_hex(),
        "0000000000000000000000000000000000000000000000000000000000024a67"
    );
    assert_eq!(
        decoded.owner_addr,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000809")?
    );
    assert_eq!(*decoded.min_lp_amount, 150265);
    let variant_2061 = match &decoded.assets.asset0 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_2061.workchain_id, -102);
    assert_eq!(
        variant_2061.address.to_hex(),
        "0000000000000000000000000000000000000000000000000000000000024caf"
    );
    assert_eq!(*decoded.assets.asset0_amount, 150776);
    assert!(matches!(&decoded.assets.asset1, DedustClassicAsset::Native(_)));
    assert_eq!(*decoded.assets.asset1_amount, 150995);
    let dispatched = DedustLiquidityDepositV1OutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustLiquidityDepositV1OutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustLiquidityDepositV1_DedustPayOutFromDeposit() -> anyhow::Result<()> {
    let body = fixture("schema_DedustLiquidityDepositV1_DedustPayOutFromDeposit")?;
    let decoded: crate::contracts::classic::liquidity_deposit_v1::msgs::DedustLiquidityDepositV1PayOutFromDepositMsg =
        TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 151433);
    assert_eq!(
        decoded.proof.factory_addr,
        TonAddress::from_str("0:000000000000000000000000000000000000000000000000000000000000081d")?
    );
    assert_eq!(decoded.proof.contract_type, 173);
    assert_eq!(
        decoded.proof.params.owner_addr,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000821")?
    );
    assert!(decoded.proof.params.pool_params.is_stable);
    assert!(matches!(
        &decoded.proof.params.pool_params.asset0,
        DedustClassicAsset::Native(_)
    ));
    assert!(matches!(
        &decoded.proof.params.pool_params.asset1,
        DedustClassicAsset::Native(_)
    ));
    assert_eq!(*decoded.amount, 152455);
    let dispatched = DedustLiquidityDepositV1OutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustLiquidityDepositV1OutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustLiquidityDepositV2_DedustLiquidityDepositV2Storage() -> anyhow::Result<()> {
    let body = fixture("schema_DedustLiquidityDepositV2_DedustLiquidityDepositV2Storage")?;
    let decoded: DedustLiquidityDepositV2Storage = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.proof.factory_addr,
        TonAddress::from_str("0:000000000000000000000000000000000000000000000000000000000000082e")?
    );
    assert_eq!(decoded.proof.contract_type, 134);
    assert_eq!(
        decoded.proof.params.owner_addr,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000832")?
    );
    assert!(!decoded.proof.params.pool_params.is_stable);
    let variant_2101 = match &decoded.proof.params.pool_params.asset0 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_2101.workchain_id, -78);
    assert_eq!(
        variant_2101.address.to_hex(),
        "0000000000000000000000000000000000000000000000000000000000025817"
    );
    let variant_2105 = match &decoded.proof.params.pool_params.asset1 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_2105.workchain_id, -114);
    assert_eq!(
        variant_2105.address.to_hex(),
        "000000000000000000000000000000000000000000000000000000000002593b"
    );
    assert_eq!(
        decoded.factory_addr,
        TonAddress::from_str("0:000000000000000000000000000000000000000000000000000000000000083d")?
    );
    assert_eq!(
        decoded.owner_addr,
        TonAddress::from_str("0:000000000000000000000000000000000000000000000000000000000000083f")?
    );
    assert_eq!(*decoded.asset0_balance, 154207);
    assert_eq!(*decoded.asset1_balance, 154280);
    assert!(!decoded.is_processing);
    assert_eq!(*decoded.min_lp_amount, 154426);
    assert!(decoded.details.fulfill_payload.is_none());
    assert!(!decoded.details.pool_params.is_stable);
    let variant_2123 = match &decoded.details.pool_params.asset0 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_2123.workchain_id, -20);
    assert_eq!(
        variant_2123.address.to_hex(),
        "0000000000000000000000000000000000000000000000000000000000025e5d"
    );
    let variant_2127 = match &decoded.details.pool_params.asset1 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_2127.workchain_id, -56);
    assert_eq!(
        variant_2127.address.to_hex(),
        "0000000000000000000000000000000000000000000000000000000000025f81"
    );
    assert_eq!(*decoded.details.asset0_target_balance, 155594);
    assert_eq!(*decoded.details.asset1_target_balance, 155667);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustLiquidityDepositV2_DedustLiquidityDepositV2StorageDetails() -> anyhow::Result<()> {
    let body = fixture("schema_DedustLiquidityDepositV2_DedustLiquidityDepositV2StorageDetails")?;
    let decoded: DedustLiquidityDepositV2StorageDetails = TLB::from_cell(&body)?;
    assert!(decoded.reject_payload.is_none());
    assert!(!decoded.pool_params.is_stable);
    let variant_2139 = match &decoded.pool_params.asset0 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_2139.workchain_id, -36);
    assert_eq!(
        variant_2139.address.to_hex(),
        "00000000000000000000000000000000000000000000000000000000000262ed"
    );
    let variant_2143 = match &decoded.pool_params.asset1 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_2143.workchain_id, -72);
    assert_eq!(
        variant_2143.address.to_hex(),
        "0000000000000000000000000000000000000000000000000000000000026411"
    );
    assert_eq!(*decoded.asset0_target_balance, 156762);
    assert_eq!(*decoded.asset1_target_balance, 156835);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustLiquidityDepositV2_DedustAssetNative() -> anyhow::Result<()> {
    let body = fixture("schema_DedustLiquidityDepositV2_DedustAssetNative")?;
    let decoded: DedustClassicNativeAsset = TLB::from_cell(&body)?;

    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustLiquidityDepositV2_DedustAssetJetton() -> anyhow::Result<()> {
    let body = fixture("schema_DedustLiquidityDepositV2_DedustAssetJetton")?;
    let decoded: DedustClassicJettonAsset = TLB::from_cell(&body)?;
    assert_eq!(decoded.workchain_id, -126);
    assert_eq!(
        decoded.address.to_hex(),
        "00000000000000000000000000000000000000000000000000000000000265c7"
    );
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustLiquidityDepositV2_DedustPoolParams() -> anyhow::Result<()> {
    let body = fixture("schema_DedustLiquidityDepositV2_DedustPoolParams")?;
    let decoded: crate::contracts::classic::liquidity_deposit_v2::storage::DedustLiquidityDepositV2PoolParams =
        TLB::from_cell(&body)?;
    assert!(!decoded.is_stable);
    let variant_2155 = match &decoded.asset0 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_2155.workchain_id, -52);
    assert_eq!(
        variant_2155.address.to_hex(),
        "000000000000000000000000000000000000000000000000000000000002677d"
    );
    let variant_2159 = match &decoded.asset1 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_2159.workchain_id, -88);
    assert_eq!(
        variant_2159.address.to_hex(),
        "00000000000000000000000000000000000000000000000000000000000268a1"
    );
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustLiquidityDepositV2_DedustLiquidityDepositProofParams() -> anyhow::Result<()> {
    let body = fixture("schema_DedustLiquidityDepositV2_DedustLiquidityDepositProofParams")?;
    let decoded: crate::contracts::classic::liquidity_deposit_v2::storage::DedustLiquidityDepositV2ProofParams =
        TLB::from_cell(&body)?;
    assert_eq!(
        decoded.owner_addr,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000874")?
    );
    assert!(!decoded.pool_params.is_stable);
    let variant_2167 = match &decoded.pool_params.asset0 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_2167.workchain_id, -32);
    assert_eq!(
        variant_2167.address.to_hex(),
        "0000000000000000000000000000000000000000000000000000000000026ae9"
    );
    let variant_2171 = match &decoded.pool_params.asset1 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_2171.workchain_id, -68);
    assert_eq!(
        variant_2171.address.to_hex(),
        "0000000000000000000000000000000000000000000000000000000000026c0d"
    );
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustLiquidityDepositV2_DedustLiquidityDepositProof() -> anyhow::Result<()> {
    let body = fixture("schema_DedustLiquidityDepositV2_DedustLiquidityDepositProof")?;
    let decoded: crate::contracts::classic::liquidity_deposit_v2::storage::DedustLiquidityDepositV2Proof =
        TLB::from_cell(&body)?;
    assert_eq!(
        decoded.factory_addr,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000880")?
    );
    assert_eq!(decoded.contract_type, 232);
    assert_eq!(
        decoded.params.owner_addr,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000884")?
    );
    assert!(!decoded.params.pool_params.is_stable);
    let variant_2183 = match &decoded.params.pool_params.asset0 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_2183.workchain_id, -48);
    assert_eq!(
        variant_2183.address.to_hex(),
        "0000000000000000000000000000000000000000000000000000000000026f79"
    );
    let variant_2187 = match &decoded.params.pool_params.asset1 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_2187.workchain_id, -84);
    assert_eq!(
        variant_2187.address.to_hex(),
        "000000000000000000000000000000000000000000000000000000000002709d"
    );
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustLiquidityDepositV2_DedustPoolProof() -> anyhow::Result<()> {
    let body = fixture("schema_DedustLiquidityDepositV2_DedustPoolProof")?;
    let decoded: crate::contracts::classic::liquidity_deposit_v2::storage::DedustLiquidityDepositV2PoolProof =
        TLB::from_cell(&body)?;
    assert_eq!(
        decoded.factory_addr,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000890")?
    );
    assert_eq!(decoded.contract_type, 120);
    assert!(decoded.params.is_stable);
    assert!(matches!(&decoded.params.asset0, DedustClassicAsset::Native(_)));
    assert!(matches!(&decoded.params.asset1, DedustClassicAsset::Native(_)));
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustLiquidityDepositV2_DedustLiquidityDepositInstall() -> anyhow::Result<()> {
    let body = fixture("schema_DedustLiquidityDepositV2_DedustLiquidityDepositInstall")?;
    let decoded: crate::contracts::classic::liquidity_deposit_v2::msgs::DedustLiquidityDepositV2InstallMsg =
        TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 160704);
    assert_eq!(decoded.code_version, 29778);
    assert_eq!(*decoded.extra_params.min_lp_amount, 17);
    assert_eq!(*decoded.extra_params.asset0_target_balance, 100);
    assert_eq!(*decoded.extra_params.asset1_target_balance, 200);
    let dispatched = DedustLiquidityDepositV2IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustLiquidityDepositV2IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustLiquidityDepositV2_DedustDepositAsset() -> anyhow::Result<()> {
    let body = fixture("schema_DedustLiquidityDepositV2_DedustDepositAsset")?;
    let decoded: crate::contracts::classic::liquidity_deposit_v2::msgs::DedustLiquidityDepositV2DepositAssetMsg =
        TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 161069);
    let variant_2207 = match &decoded.asset {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_2207.workchain_id, -8);
    assert_eq!(
        variant_2207.address.to_hex(),
        "0000000000000000000000000000000000000000000000000000000000027651"
    );
    assert_eq!(*decoded.amount, 161434);
    let dispatched = DedustLiquidityDepositV2IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustLiquidityDepositV2IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustLiquidityDepositV2_DedustFulfillDeposit() -> anyhow::Result<()> {
    let body = fixture("schema_DedustLiquidityDepositV2_DedustFulfillDeposit")?;
    let decoded: crate::contracts::classic::liquidity_deposit_v2::msgs::DedustLiquidityDepositV2FulfillDepositMsg =
        TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 161580);
    assert_eq!(
        decoded.proof.factory_addr,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000008a8")?
    );
    assert_eq!(decoded.proof.contract_type, 80);
    assert!(decoded.proof.params.is_stable);
    assert!(matches!(&decoded.proof.params.asset0, DedustClassicAsset::Native(_)));
    assert!(matches!(&decoded.proof.params.asset1, DedustClassicAsset::Native(_)));
    assert_eq!(*decoded.asset0_deposit, 162383);
    assert_eq!(*decoded.asset1_deposit, 162456);
    let dispatched = DedustLiquidityDepositV2IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustLiquidityDepositV2IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustLiquidityDepositV2_DedustRejectDeposit() -> anyhow::Result<()> {
    let body = fixture("schema_DedustLiquidityDepositV2_DedustRejectDeposit")?;
    let decoded: crate::contracts::classic::liquidity_deposit_v2::msgs::DedustLiquidityDepositV2RejectDepositMsg =
        TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 162602);
    assert_eq!(
        decoded.proof.factory_addr,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000008b6")?
    );
    assert_eq!(decoded.proof.contract_type, 78);
    assert!(decoded.proof.params.is_stable);
    assert!(matches!(&decoded.proof.params.asset0, DedustClassicAsset::Native(_)));
    assert!(matches!(&decoded.proof.params.asset1, DedustClassicAsset::Native(_)));
    assert!(decoded.payload.is_none());
    let dispatched = DedustLiquidityDepositV2IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustLiquidityDepositV2IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustLiquidityDepositV2_DedustCancelDeposit() -> anyhow::Result<()> {
    let body = fixture("schema_DedustLiquidityDepositV2_DedustCancelDeposit")?;
    let decoded: crate::contracts::classic::liquidity_deposit_v2::msgs::DedustLiquidityDepositV2CancelDepositMsg =
        TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 163551);
    assert!(decoded.payload.is_none());
    let dispatched = DedustLiquidityDepositV2IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustLiquidityDepositV2IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustLiquidityDepositV2_DedustDepositLiquidityAllAssets() -> anyhow::Result<()> {
    let body = fixture("schema_DedustLiquidityDepositV2_DedustDepositLiquidityAllAssets")?;
    let decoded: crate::contracts::classic::liquidity_deposit_v2::storage::DedustLiquidityDepositV2DepositLiquidityAllAssets =
        TLB::from_cell(&body)?;
    let variant_2243 = match &decoded.asset0 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_2243.workchain_id, -76);
    assert_eq!(
        variant_2243.address.to_hex(),
        "0000000000000000000000000000000000000000000000000000000000028095"
    );
    assert_eq!(*decoded.asset0_amount, 164062);
    assert!(matches!(&decoded.asset1, DedustClassicAsset::Native(_)));
    assert_eq!(*decoded.asset1_amount, 164281);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustLiquidityDepositV2_DedustDepositLiquidityAll() -> anyhow::Result<()> {
    let body = fixture("schema_DedustLiquidityDepositV2_DedustDepositLiquidityAll")?;
    let decoded: crate::contracts::classic::liquidity_deposit_v2::msgs::DedustLiquidityDepositV2DepositLiquidityAllMsg =
        TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 164427);
    assert_eq!(
        decoded.proof.factory_addr,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000008cf")?
    );
    assert_eq!(decoded.proof.contract_type, 111);
    assert_eq!(
        decoded.proof.params.owner_addr,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000008d3")?
    );
    assert!(decoded.proof.params.pool_params.is_stable);
    assert!(matches!(
        &decoded.proof.params.pool_params.asset0,
        DedustClassicAsset::Native(_)
    ));
    assert!(matches!(
        &decoded.proof.params.pool_params.asset1,
        DedustClassicAsset::Native(_)
    ));
    assert_eq!(
        decoded.owner_addr,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000008da")?
    );
    assert_eq!(*decoded.min_lp_amount, 165522);
    assert!(matches!(&decoded.assets.asset0, DedustClassicAsset::Native(_)));
    assert_eq!(*decoded.assets.asset0_amount, 165887);
    let variant_2273 = match &decoded.assets.asset1 {
        DedustClassicAsset::Jetton(value) => value,
        _ => anyhow::bail!("wrong DedustClassicAsset variant"),
    };
    assert_eq!(variant_2273.workchain_id, -90);
    assert_eq!(
        variant_2273.address.to_hex(),
        "0000000000000000000000000000000000000000000000000000000000028923"
    );
    assert_eq!(*decoded.assets.asset1_amount, 166252);
    assert!(decoded.reject_payload.is_none());
    let dispatched = DedustLiquidityDepositV2OutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustLiquidityDepositV2OutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustLiquidityDepositV2_DedustPayOutFromDeposit() -> anyhow::Result<()> {
    let body = fixture("schema_DedustLiquidityDepositV2_DedustPayOutFromDeposit")?;
    let decoded: crate::contracts::classic::liquidity_deposit_v2::msgs::DedustLiquidityDepositV2PayOutFromDepositMsg =
        TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 166617);
    assert_eq!(
        decoded.proof.factory_addr,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000008ed")?
    );
    assert_eq!(decoded.proof.contract_type, 253);
    assert_eq!(
        decoded.proof.params.owner_addr,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000008f1")?
    );
    assert!(decoded.proof.params.pool_params.is_stable);
    assert!(matches!(
        &decoded.proof.params.pool_params.asset0,
        DedustClassicAsset::Native(_)
    ));
    assert!(matches!(
        &decoded.proof.params.pool_params.asset1,
        DedustClassicAsset::Native(_)
    ));
    assert_eq!(*decoded.amount, 167639);
    let dispatched = DedustLiquidityDepositV2OutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustLiquidityDepositV2OutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustV2Cpmm_DedustV2CpmmStatusUninitialized() -> anyhow::Result<()> {
    let body = fixture("schema_DedustV2Cpmm_DedustV2CpmmStatusUninitialized")?;
    let decoded: DedustV2CpmmStatusUninitialized = TLB::from_cell(&body)?;

    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustV2Cpmm_DedustV2CpmmStatusInitializing() -> anyhow::Result<()> {
    let body = fixture("schema_DedustV2Cpmm_DedustV2CpmmStatusInitializing")?;
    let decoded: DedustV2CpmmStatusInitializing = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.initiator_address,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000008fd")?
    );
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustV2Cpmm_DedustV2CpmmStatusInitialized() -> anyhow::Result<()> {
    let body = fixture("schema_DedustV2Cpmm_DedustV2CpmmStatusInitialized")?;
    let decoded: DedustV2CpmmStatusInitialized = TLB::from_cell(&body)?;

    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustV2Cpmm_DedustV2CpmmStorage() -> anyhow::Result<()> {
    let body = fixture("schema_DedustV2Cpmm_DedustV2CpmmStorage")?;
    let decoded: DedustV2CpmmStorage = TLB::from_cell(&body)?;
    assert!(decoded.config.asset_x.value.is_none());
    assert_eq!(
        decoded.config.asset_y.value,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000905"
        )?)
    );
    assert!(decoded.config.creator_address.is_none());
    assert_eq!(decoded.config.base_fee_bps, 37662);
    assert_eq!(decoded.config.creator_fee_bps, 37735);
    assert!(decoded.config.deposit_activation.is_none());
    let variant_2316 = match &**(decoded
        .config
        .swap_activation
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("missing optional"))?)
    {
        DedustV2CpmmActivation::Time(value) => value,
        _ => anyhow::bail!("wrong DedustV2CpmmActivation variant"),
    };
    assert_eq!(variant_2316.active_since, 169245);
    assert_eq!(decoded.config.fee_in, DedustV2CpmmFeeIn::Both);
    assert!(decoded.config.custom_resolvers.is_empty());
    assert!(decoded.config.allowed_rewards.is_empty());
    assert_eq!(*decoded.fees.protocol_fee_x, 169683);
    assert_eq!(*decoded.fees.protocol_fee_y, 169756);
    assert_eq!(*decoded.fees.creator_fee_x, 169829);
    assert_eq!(*decoded.fees.creator_fee_y, 169902);
    assert_eq!(*decoded.fees.x_fee_per_token, U256::from_u64(170048));
    assert_eq!(*decoded.fees.y_fee_per_token, U256::from_u64(170194));
    assert_eq!(
        decoded.extra.controller,
        TonAddress::from_str("0:000000000000000000000000000000000000000000000000000000000000091e")?
    );
    assert!(decoded.extra.wallets.wallets_by_assets.is_empty());
    assert!(decoded.extra.wallets.assets_by_wallets.is_empty());
    assert!(decoded.extra.wallets.resolutions.is_empty());
    assert!(decoded.rewards.entries.is_empty());
    let variant_2341 = match &decoded.status {
        DedustV2CpmmStatus::Initializing(value) => value,
        _ => anyhow::bail!("wrong DedustV2CpmmStatus variant"),
    };
    assert_eq!(
        variant_2341.initiator_address,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000927")?
    );
    assert!(!decoded.deposit_active);
    assert!(decoded.swap_active);
    assert_eq!(*decoded.liquidity, 171289);
    assert_eq!(*decoded.reserve_x, 171362);
    assert_eq!(*decoded.reserve_y, 171435);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustV2Cpmm_DedustV2CpmmSwapPayload() -> anyhow::Result<()> {
    let body = fixture("schema_DedustV2Cpmm_DedustV2CpmmSwapPayload")?;
    let decoded: DedustV2CpmmSwapPayloadMsg = TLB::from_cell(&body)?;
    assert_eq!(*decoded.minimal_amount_out, 171581);
    assert_eq!(decoded.deadline, 171654);
    assert!(decoded.next.is_none());
    assert_eq!(
        decoded
            .partner_config
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .partner_id,
        U256::from_u64(171946)
    );
    assert_eq!(
        decoded
            .partner_config
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .partner_fee_bps,
        40947
    );
    assert_eq!(
        decoded
            .referrer_config
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .referrer_id,
        U256::from_u64(172238)
    );
    assert_eq!(
        decoded
            .referrer_config
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .referrer_fee_bps,
        41239
    );
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustV2Cpmm_DedustV2CpmmDepositPayload() -> anyhow::Result<()> {
    let body = fixture("schema_DedustV2Cpmm_DedustV2CpmmDepositPayload")?;
    let decoded: DedustV2CpmmDepositPayloadMsg = TLB::from_cell(&body)?;
    assert_eq!(*decoded.amount_x, 172457);
    assert_eq!(*decoded.amount_y, 172530);
    assert_eq!(*decoded.minimal_liquidity, 172603);
    assert_eq!(decoded.locked_liquidity_share, 41604);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustV2Cpmm_DedustV2CpmmFundRewardPayload() -> anyhow::Result<()> {
    let body = fixture("schema_DedustV2Cpmm_DedustV2CpmmFundRewardPayload")?;
    let decoded: DedustV2CpmmFundRewardPayloadMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.till, 172822);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustV2Cpmm_DedustV2CpmmPayJetton() -> anyhow::Result<()> {
    let body = fixture("schema_DedustV2Cpmm_DedustV2CpmmPayJetton")?;
    let decoded: DedustV2CpmmPayJettonMsg = TLB::from_cell(&body)?;
    assert!(decoded.payout_config.fulfill.destination.is_none());
    assert_eq!(*decoded.payout_config.fulfill.extra_gas, 173333);
    assert!(decoded.payout_config.fulfill.payload.is_none());
    assert!(!decoded.payout_config.fulfill.wrap_payload);
    assert_eq!(
        decoded.payout_config.reject.destination,
        Some(TonAddress::from_str(
            "0:000000000000000000000000000000000000000000000000000000000000094a"
        )?)
    );
    assert_eq!(*decoded.payout_config.reject.extra_gas, 173698);
    assert!(decoded.payout_config.reject.payload.is_none());
    assert!(decoded.payout_config.reject.wrap_payload);
    assert!(decoded.payout_config.excesses_to.is_none());
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustV2Cpmm_DedustV2CpmmSwapStep() -> anyhow::Result<()> {
    let body = fixture("schema_DedustV2Cpmm_DedustV2CpmmSwapStep")?;
    let decoded: DedustV2CpmmSwapStep = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.pool,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000950")?
    );
    assert_eq!(*decoded.minimal_amount_out, 174136);
    assert_eq!(decoded.deadline, 174209);
    assert_eq!(
        decoded.next.as_ref().ok_or_else(|| anyhow::anyhow!("missing optional"))?.pool,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000956")?
    );
    assert_eq!(
        *decoded
            .next
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .minimal_amount_out,
        174574
    );
    assert_eq!(
        decoded
            .next
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .deadline,
        174647
    );
    assert!(
        decoded
            .next
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .next
            .is_none()
    );
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustV2Cpmm_DedustV2CpmmPartnerConfig() -> anyhow::Result<()> {
    let body = fixture("schema_DedustV2Cpmm_DedustV2CpmmPartnerConfig")?;
    let decoded: DedustV2CpmmPartnerConfig = TLB::from_cell(&body)?;
    assert_eq!(decoded.partner_id, U256::from_u64(174866));
    assert_eq!(decoded.partner_fee_bps, 43867);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustV2Cpmm_DedustV2CpmmReferrerConfig() -> anyhow::Result<()> {
    let body = fixture("schema_DedustV2Cpmm_DedustV2CpmmReferrerConfig")?;
    let decoded: DedustV2CpmmReferrerConfig = TLB::from_cell(&body)?;
    assert_eq!(decoded.referrer_id, U256::from_u64(175085));
    assert_eq!(decoded.referrer_fee_bps, 44086);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustV2Cpmm_DedustV2CpmmAsset() -> anyhow::Result<()> {
    let body = fixture("schema_DedustV2Cpmm_DedustV2CpmmAsset")?;
    let decoded: DedustV2CpmmAsset = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.value,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000961"
        )?)
    );
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustV2Cpmm_DedustV2CpmmQ120X120() -> anyhow::Result<()> {
    let body = fixture("schema_DedustV2Cpmm_DedustV2CpmmQ120X120")?;
    let decoded: VarLenBytes<U256, 5> = TLB::from_cell(&body)?;
    assert_eq!(*decoded, U256::from_u64(175450));
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustV2Cpmm_DedustV2CpmmPoolConfig() -> anyhow::Result<()> {
    let body = fixture("schema_DedustV2Cpmm_DedustV2CpmmPoolConfig")?;
    let decoded: DedustV2CpmmPoolConfig = TLB::from_cell(&body)?;
    assert!(decoded.asset_x.value.is_none());
    assert_eq!(
        decoded.asset_y.value,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000968"
        )?)
    );
    assert!(decoded.creator_address.is_none());
    assert_eq!(decoded.base_fee_bps, 44889);
    assert_eq!(decoded.creator_fee_bps, 44962);
    assert!(decoded.deposit_activation.is_none());
    let variant_2415 = match &**(decoded
        .swap_activation
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("missing optional"))?)
    {
        DedustV2CpmmActivation::Initiator(value) => value,
        _ => anyhow::bail!("wrong DedustV2CpmmActivation variant"),
    };
    assert_eq!(
        variant_2415.initiator_address,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000971")?
    );
    assert_eq!(decoded.fee_in, DedustV2CpmmFeeIn::Both);
    assert!(decoded.custom_resolvers.is_empty());
    assert!(decoded.allowed_rewards.is_empty());
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustV2Cpmm_DedustV2CpmmActivationTime() -> anyhow::Result<()> {
    let body = fixture("schema_DedustV2Cpmm_DedustV2CpmmActivationTime")?;
    let decoded: DedustV2CpmmActivationTime = TLB::from_cell(&body)?;
    assert_eq!(decoded.active_since, 176837);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustV2Cpmm_DedustV2CpmmActivationInitiator() -> anyhow::Result<()> {
    let body = fixture("schema_DedustV2Cpmm_DedustV2CpmmActivationInitiator")?;
    let decoded: DedustV2CpmmActivationInitiator = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.initiator_address,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000978")?
    );
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustV2Cpmm_DedustV2CpmmRewardConfig() -> anyhow::Result<()> {
    let body = fixture("schema_DedustV2Cpmm_DedustV2CpmmRewardConfig")?;
    let decoded: DedustV2CpmmRewardConfig = TLB::from_cell(&body)?;
    assert!(decoded.asset.value.is_none());
    assert_eq!(decoded.max_duration, 177275);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustV2Cpmm_DedustV2CpmmPoolRewards() -> anyhow::Result<()> {
    let body = fixture("schema_DedustV2Cpmm_DedustV2CpmmPoolRewards")?;
    let decoded: DedustV2CpmmPoolRewards = TLB::from_cell(&body)?;
    assert!(decoded.entries.is_empty());
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustV2Cpmm_DedustV2CpmmPoolReward() -> anyhow::Result<()> {
    let body = fixture("schema_DedustV2Cpmm_DedustV2CpmmPoolReward")?;
    let decoded: DedustV2CpmmPoolReward = TLB::from_cell(&body)?;
    assert_eq!(decoded.remaining_time, 177567);
    assert_eq!(*decoded.remaining_budget, 177640);
    assert_eq!(*decoded.rewards_per_token, U256::from_u64(177786));
    assert_eq!(decoded.last_update, 177859);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustV2Cpmm_DedustV2CpmmPoolFees() -> anyhow::Result<()> {
    let body = fixture("schema_DedustV2Cpmm_DedustV2CpmmPoolFees")?;
    let decoded: DedustV2CpmmPoolFees = TLB::from_cell(&body)?;
    assert_eq!(*decoded.protocol_fee_x, 178005);
    assert_eq!(*decoded.protocol_fee_y, 178078);
    assert_eq!(*decoded.creator_fee_x, 178151);
    assert_eq!(*decoded.creator_fee_y, 178224);
    assert_eq!(*decoded.x_fee_per_token, U256::from_u64(178370));
    assert_eq!(*decoded.y_fee_per_token, U256::from_u64(178516));
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustV2Cpmm_DedustV2CpmmPoolWallets() -> anyhow::Result<()> {
    let body = fixture("schema_DedustV2Cpmm_DedustV2CpmmPoolWallets")?;
    let decoded: DedustV2CpmmPoolWallets = TLB::from_cell(&body)?;
    assert!(decoded.wallets_by_assets.is_empty());
    assert!(decoded.assets_by_wallets.is_empty());
    assert!(decoded.resolutions.is_empty());
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustV2Cpmm_DedustV2CpmmPoolExtra() -> anyhow::Result<()> {
    let body = fixture("schema_DedustV2Cpmm_DedustV2CpmmPoolExtra")?;
    let decoded: DedustV2CpmmPoolExtra = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.controller,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000993")?
    );
    assert!(decoded.wallets.wallets_by_assets.is_empty());
    assert!(decoded.wallets.assets_by_wallets.is_empty());
    assert!(decoded.wallets.resolutions.is_empty());
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustV2Cpmm_DedustV2CpmmPoolUpdatedState() -> anyhow::Result<()> {
    let body = fixture("schema_DedustV2Cpmm_DedustV2CpmmPoolUpdatedState")?;
    let decoded: DedustV2CpmmPoolUpdatedState = TLB::from_cell(&body)?;
    assert_eq!(*decoded.reserve_x, 179392);
    assert_eq!(*decoded.reserve_y, 179465);
    assert_eq!(*decoded.liquidity, 179538);
    assert_eq!(*decoded.fees.protocol_fee_x, 179757);
    assert_eq!(*decoded.fees.protocol_fee_y, 179830);
    assert_eq!(*decoded.fees.creator_fee_x, 179903);
    assert_eq!(*decoded.fees.creator_fee_y, 179976);
    assert_eq!(*decoded.fees.x_fee_per_token, U256::from_u64(180122));
    assert_eq!(*decoded.fees.y_fee_per_token, U256::from_u64(180268));
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustV2Cpmm_DedustV2CpmmSwapFees() -> anyhow::Result<()> {
    let body = fixture("schema_DedustV2Cpmm_DedustV2CpmmSwapFees")?;
    let decoded: DedustV2CpmmSwapFees = TLB::from_cell(&body)?;
    assert!(decoded.is_x);
    assert_eq!(*decoded.lp_fee, 180487);
    assert_eq!(*decoded.creator_fee, 180560);
    assert_eq!(*decoded.protocol_fee, 180633);
    assert_eq!(*decoded.partner_fee, 180706);
    assert_eq!(*decoded.referrer_fee, 180779);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustV2Cpmm_DedustV2CpmmExtendedPayoutConfig() -> anyhow::Result<()> {
    let body = fixture("schema_DedustV2Cpmm_DedustV2CpmmExtendedPayoutConfig")?;
    let decoded: DedustV2CpmmExtendedPayoutConfig = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.fulfill.destination,
        Some(TonAddress::from_str(
            "0:00000000000000000000000000000000000000000000000000000000000009af"
        )?)
    );
    assert_eq!(*decoded.fulfill.extra_gas, 181071);
    assert!(decoded.fulfill.payload.is_none());
    assert!(!decoded.fulfill.wrap_payload);
    assert!(decoded.reject.destination.is_none());
    assert_eq!(*decoded.reject.extra_gas, 181436);
    assert!(!decoded.reject.wrap_payload);
    assert_eq!(
        decoded.excesses_to,
        Some(TonAddress::from_str(
            "0:00000000000000000000000000000000000000000000000000000000000009b9"
        )?)
    );
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustV2Cpmm_DedustV2CpmmBasicPayoutConfig() -> anyhow::Result<()> {
    let body = fixture("schema_DedustV2Cpmm_DedustV2CpmmBasicPayoutConfig")?;
    let decoded: DedustV2CpmmBasicPayoutConfig = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.options.destination,
        Some(TonAddress::from_str(
            "0:00000000000000000000000000000000000000000000000000000000000009bc"
        )?)
    );
    assert_eq!(*decoded.options.extra_gas, 182020);
    assert!(!decoded.options.wrap_payload);
    assert_eq!(
        decoded.excesses_to,
        Some(TonAddress::from_str(
            "0:00000000000000000000000000000000000000000000000000000000000009c1"
        )?)
    );
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustV2Cpmm_DedustV2CpmmPayoutOptions() -> anyhow::Result<()> {
    let body = fixture("schema_DedustV2Cpmm_DedustV2CpmmPayoutOptions")?;
    let decoded: DedustV2CpmmPayoutOptions = TLB::from_cell(&body)?;
    assert!(decoded.destination.is_none());
    assert_eq!(*decoded.extra_gas, 182531);
    assert!(decoded.wrap_payload);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustV2Cpmm_DedustV2CpmmPayNative() -> anyhow::Result<()> {
    let body = fixture("schema_DedustV2Cpmm_DedustV2CpmmPayNative")?;
    let decoded: DedustV2CpmmPayNativeMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 182896);
    assert_eq!(*decoded.amount, 182969);
    assert!(decoded.payout_config.fulfill.destination.is_none());
    assert_eq!(*decoded.payout_config.fulfill.extra_gas, 183407);
    assert!(decoded.payout_config.fulfill.payload.is_none());
    assert!(!decoded.payout_config.fulfill.wrap_payload);
    assert_eq!(
        decoded.payout_config.reject.destination,
        Some(TonAddress::from_str(
            "0:00000000000000000000000000000000000000000000000000000000000009d4"
        )?)
    );
    assert_eq!(*decoded.payout_config.reject.extra_gas, 183772);
    assert!(decoded.payout_config.reject.payload.is_none());
    assert!(decoded.payout_config.reject.wrap_payload);
    assert!(decoded.payout_config.excesses_to.is_none());
    let dispatched = DedustV2CpmmIncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustV2CpmmIncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustV2Cpmm_DedustV2CpmmInitPool() -> anyhow::Result<()> {
    let body = fixture("schema_DedustV2Cpmm_DedustV2CpmmInitPool")?;
    let decoded: DedustV2CpmmInitPoolMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 184137);
    assert!(decoded.custom_payload.is_none());
    let dispatched = DedustV2CpmmIncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustV2CpmmIncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustV2Cpmm_DedustV2CpmmTakeWalletAddress() -> anyhow::Result<()> {
    let body = fixture("schema_DedustV2Cpmm_DedustV2CpmmTakeWalletAddress")?;
    let decoded: DedustV2CpmmTakeWalletAddressMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 184356);
    assert_eq!(
        decoded.wallet_address,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000009de")?
    );
    assert_eq!(
        (decoded
            .owner_address
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?)
        .to_cell()?
        .hash()?,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000009e1")?
            .to_msg_address_int()
            .to_cell()?
            .hash()?
    );
    let dispatched = DedustV2CpmmIncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustV2CpmmIncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustV2Cpmm_DedustV2CpmmTransferNotification() -> anyhow::Result<()> {
    let body = fixture("schema_DedustV2Cpmm_DedustV2CpmmTransferNotification")?;
    let decoded: DedustV2CpmmTransferNotificationMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 184794);
    assert_eq!(*decoded.amount, 184867);
    assert_eq!(
        decoded.sender,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000009e5")?
    );
    assert_eq!(decoded.forward_payload.layout, EitherRefLayout::ToCell);
    let dispatched = DedustV2CpmmIncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustV2CpmmIncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustV2Cpmm_DedustV2CpmmJoinLiquidity() -> anyhow::Result<()> {
    let body = fixture("schema_DedustV2Cpmm_DedustV2CpmmJoinLiquidity")?;
    let decoded: DedustV2CpmmJoinLiquidityMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 185159);
    assert_eq!(*decoded.config.target_x, 100);
    assert_eq!(*decoded.config.target_y, 200);
    assert_eq!(*decoded.config.minimal_liquidity, 17);
    assert_eq!(decoded.config.locked_liquidity_share, 5000);
    assert_eq!(*decoded.amount_x, 185232);
    assert_eq!(*decoded.amount_y, 185305);
    assert_eq!(
        decoded.initiator_address,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000009eb")?
    );
    let dispatched = DedustV2CpmmIncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustV2CpmmIncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustV2Cpmm_DedustV2CpmmRefundDeposit() -> anyhow::Result<()> {
    let body = fixture("schema_DedustV2Cpmm_DedustV2CpmmRefundDeposit")?;
    let decoded: DedustV2CpmmRefundDepositMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 185524);
    assert_eq!(*decoded.config.target_x, 100);
    assert_eq!(*decoded.config.target_y, 200);
    assert_eq!(*decoded.config.minimal_liquidity, 17);
    assert_eq!(decoded.config.locked_liquidity_share, 5000);
    assert_eq!(*decoded.amount_x, 185597);
    assert_eq!(*decoded.amount_y, 185670);
    assert_eq!(
        decoded.initiator_address,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000009f0")?
    );
    let dispatched = DedustV2CpmmIncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustV2CpmmIncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustV2Cpmm_DedustV2CpmmClaimPositionFees() -> anyhow::Result<()> {
    let body = fixture("schema_DedustV2Cpmm_DedustV2CpmmClaimPositionFees")?;
    let decoded: DedustV2CpmmClaimPositionFeesMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 185889);
    assert_eq!(
        decoded.excesses_to,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000009f3")?
    );
    let dispatched = DedustV2CpmmIncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustV2CpmmIncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustV2Cpmm_DedustV2CpmmPayoutPositionFees() -> anyhow::Result<()> {
    let body = fixture("schema_DedustV2Cpmm_DedustV2CpmmPayoutPositionFees")?;
    let decoded: DedustV2CpmmPayoutPositionFeesMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 186108);
    assert_eq!(*decoded.x_fees, 186181);
    assert_eq!(*decoded.y_fees, 186254);
    assert_eq!(
        decoded.owner_address,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000009f8")?
    );
    assert_eq!(
        decoded.excesses_to,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000009f9")?
    );
    let dispatched = DedustV2CpmmIncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustV2CpmmIncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustV2Cpmm_DedustV2CpmmClaimProtocolFees() -> anyhow::Result<()> {
    let body = fixture("schema_DedustV2Cpmm_DedustV2CpmmClaimProtocolFees")?;
    let decoded: DedustV2CpmmClaimProtocolFeesMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 186546);
    assert_eq!(
        decoded.to,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000009fc")?
    );
    assert_eq!(
        decoded.excesses_to,
        TonAddress::from_str("0:00000000000000000000000000000000000000000000000000000000000009fd")?
    );
    let dispatched = DedustV2CpmmIncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustV2CpmmIncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustV2Cpmm_DedustV2CpmmClaimCreatorFees() -> anyhow::Result<()> {
    let body = fixture("schema_DedustV2Cpmm_DedustV2CpmmClaimCreatorFees")?;
    let decoded: DedustV2CpmmClaimCreatorFeesMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 186838);
    assert_eq!(
        decoded.to,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000a00")?
    );
    assert_eq!(
        decoded.excesses_to,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000a01")?
    );
    let dispatched = DedustV2CpmmIncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustV2CpmmIncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustV2Cpmm_DedustV2CpmmWithdraw() -> anyhow::Result<()> {
    let body = fixture("schema_DedustV2Cpmm_DedustV2CpmmWithdraw")?;
    let decoded: DedustV2CpmmWithdrawMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 187130);
    assert_eq!(*decoded.liquidity, 187203);
    assert_eq!(*decoded.minimal_x_out, 187276);
    assert_eq!(*decoded.minimal_y_out, 187349);
    assert!(decoded.auto_claim_fees);
    assert!(decoded.payout_config.options.destination.is_none());
    assert_eq!(*decoded.payout_config.options.extra_gas, 187787);
    assert!(decoded.payout_config.options.payload.is_none());
    assert!(!decoded.payout_config.options.wrap_payload);
    assert_eq!(
        decoded.payout_config.excesses_to,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000a0f"
        )?)
    );
    let dispatched = DedustV2CpmmIncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustV2CpmmIncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustV2Cpmm_DedustV2CpmmExitLiquidity() -> anyhow::Result<()> {
    let body = fixture("schema_DedustV2Cpmm_DedustV2CpmmExitLiquidity")?;
    let decoded: DedustV2CpmmExitLiquidityMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 188152);
    assert_eq!(*decoded.liquidity, 188225);
    assert_eq!(*decoded.x_fees, 188298);
    assert_eq!(*decoded.y_fees, 188371);
    assert_eq!(
        decoded.owner_address,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000a15")?
    );
    assert_eq!(*decoded.minimal_amount_x, 188517);
    assert_eq!(*decoded.minimal_amount_y, 188590);
    assert_eq!(
        decoded.payout_config.options.destination,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000a1b"
        )?)
    );
    assert_eq!(*decoded.payout_config.options.extra_gas, 188955);
    assert!(decoded.payout_config.options.payload.is_none());
    assert!(!decoded.payout_config.options.wrap_payload);
    assert_eq!(
        decoded.payout_config.excesses_to,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000a1f"
        )?)
    );
    let dispatched = DedustV2CpmmIncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustV2CpmmIncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustV2Cpmm_DedustV2CpmmProvidePoolState() -> anyhow::Result<()> {
    let body = fixture("schema_DedustV2Cpmm_DedustV2CpmmProvidePoolState")?;
    let decoded: DedustV2CpmmProvidePoolStateMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 189320);
    assert!(!decoded.include_config);
    assert!(decoded.include_fees);
    assert!(!decoded.include_code_hash);
    assert!(decoded.include_rewards);
    let dispatched = DedustV2CpmmIncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustV2CpmmIncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustV2Cpmm_DedustV2CpmmClaimReward() -> anyhow::Result<()> {
    let body = fixture("schema_DedustV2Cpmm_DedustV2CpmmClaimReward")?;
    let decoded: DedustV2CpmmClaimRewardMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 189758);
    assert_eq!(decoded.reward_index, 3);
    assert_eq!(
        decoded.excesses_to,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000a29")?
    );
    let dispatched = DedustV2CpmmIncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustV2CpmmIncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustV2Cpmm_DedustV2CpmmPayoutReward() -> anyhow::Result<()> {
    let body = fixture("schema_DedustV2Cpmm_DedustV2CpmmPayoutReward")?;
    let decoded: DedustV2CpmmPayoutRewardMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 190050);
    assert_eq!(*decoded.amount, 190123);
    assert_eq!(decoded.reward_index, 0);
    assert_eq!(
        decoded.owner_address,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000a2e")?
    );
    assert_eq!(
        decoded.excesses_to,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000a2f")?
    );
    let dispatched = DedustV2CpmmIncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustV2CpmmIncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustV2Cpmm_DedustV2CpmmUpgradePoolCode() -> anyhow::Result<()> {
    let body = fixture("schema_DedustV2Cpmm_DedustV2CpmmUpgradePoolCode")?;
    let decoded: DedustV2CpmmUpgradePoolCodeMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 190488);
    let dispatched = DedustV2CpmmIncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustV2CpmmIncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustV2Cpmm_DedustV2CpmmProvideWalletAddress() -> anyhow::Result<()> {
    let body = fixture("schema_DedustV2Cpmm_DedustV2CpmmProvideWalletAddress")?;
    let decoded: DedustV2CpmmProvideWalletAddressMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 190707);
    assert!(decoded.owner_address.is_none());
    assert!(!decoded.include_owner_address);
    let dispatched = DedustV2CpmmOutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustV2CpmmOutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustV2Cpmm_DedustV2CpmmExcess() -> anyhow::Result<()> {
    let body = fixture("schema_DedustV2Cpmm_DedustV2CpmmExcess")?;
    let decoded: DedustV2CpmmExcessMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 190999);
    let dispatched = DedustV2CpmmOutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustV2CpmmOutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustV2Cpmm_DedustV2CpmmInitPoolResult() -> anyhow::Result<()> {
    let body = fixture("schema_DedustV2Cpmm_DedustV2CpmmInitPoolResult")?;
    let decoded: DedustV2CpmmInitPoolResultMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 191145);
    assert_eq!(decoded.exit_code, -191218);
    let dispatched = DedustV2CpmmOutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustV2CpmmOutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustV2Cpmm_DedustV2CpmmCallback3216ca09() -> anyhow::Result<()> {
    let body = fixture("schema_DedustV2Cpmm_DedustV2CpmmCallback3216ca09")?;
    let decoded: DedustV2CpmmCallback3216ca09Msg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 191510);
    assert_eq!(*decoded.amount0, 191583);
    assert_eq!(decoded.code, -191656);
    let dispatched = DedustV2CpmmOutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustV2CpmmOutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustV2Cpmm_DedustV2CpmmWithdrawLiquidity() -> anyhow::Result<()> {
    let body = fixture("schema_DedustV2Cpmm_DedustV2CpmmWithdrawLiquidity")?;
    let decoded: DedustV2CpmmWithdrawLiquidityMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 191948);
    assert_eq!(*decoded.liquidity, 192021);
    assert_eq!(*decoded.minimal_x_out, 192094);
    assert_eq!(*decoded.minimal_y_out, 192167);
    assert_eq!(*decoded.x_fee_per_token, U256::from_u64(192313));
    assert_eq!(*decoded.y_fee_per_token, U256::from_u64(192459));
    assert!(decoded.rewards.entries.is_empty());
    assert!(decoded.auto_claim_fees);
    assert!(decoded.payout_config.options.destination.is_none());
    assert_eq!(*decoded.payout_config.options.extra_gas, 193043);
    assert!(decoded.payout_config.options.payload.is_none());
    assert!(!decoded.payout_config.options.wrap_payload);
    assert_eq!(
        decoded.payout_config.excesses_to,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000a57"
        )?)
    );
    let dispatched = DedustV2CpmmOutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustV2CpmmOutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustV2Cpmm_DedustV2CpmmCreditLiquidity() -> anyhow::Result<()> {
    let body = fixture("schema_DedustV2Cpmm_DedustV2CpmmCreditLiquidity")?;
    let decoded: DedustV2CpmmCreditLiquidityMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 193408);
    assert_eq!(*decoded.liquidity, 193481);
    assert_eq!(decoded.locked_liquidity_share, 62482);
    assert_eq!(*decoded.notify_extra_gas, 193627);
    assert!(decoded.excesses_to.is_none());
    assert_eq!(*decoded.x_fee_per_token, U256::from_u64(193992));
    assert_eq!(*decoded.y_fee_per_token, U256::from_u64(194138));
    assert!(decoded.rewards.entries.is_empty());
    let dispatched = DedustV2CpmmOutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustV2CpmmOutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustV2Cpmm_DedustV2CpmmClaimAvailableFees() -> anyhow::Result<()> {
    let body = fixture("schema_DedustV2Cpmm_DedustV2CpmmClaimAvailableFees")?;
    let decoded: DedustV2CpmmClaimAvailableFeesMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 194430);
    assert!(decoded.excesses_to.is_none());
    assert_eq!(*decoded.x_fee_per_token, U256::from_u64(194649));
    assert_eq!(*decoded.y_fee_per_token, U256::from_u64(194795));
    let dispatched = DedustV2CpmmOutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustV2CpmmOutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustV2Cpmm_DedustV2CpmmTakePoolState() -> anyhow::Result<()> {
    let body = fixture("schema_DedustV2Cpmm_DedustV2CpmmTakePoolState")?;
    let decoded: DedustV2CpmmTakePoolStateMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 194941);
    let variant_2671 = match &decoded.status {
        DedustV2CpmmStatus::Initializing(value) => value,
        _ => anyhow::bail!("wrong DedustV2CpmmStatus variant"),
    };
    assert_eq!(
        variant_2671.initiator_address,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000a71")?
    );
    assert!(!decoded.deposit_active);
    assert!(decoded.swap_active);
    assert_eq!(*decoded.liquidity, 195379);
    assert_eq!(*decoded.reserve_x, 195452);
    assert_eq!(*decoded.reserve_y, 195525);
    assert!(decoded.config.is_none());
    assert_eq!(
        *decoded
            .fees
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .protocol_fee_x,
        195890
    );
    assert_eq!(
        *decoded
            .fees
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .protocol_fee_y,
        195963
    );
    assert_eq!(
        *decoded
            .fees
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .creator_fee_x,
        196036
    );
    assert_eq!(
        *decoded
            .fees
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .creator_fee_y,
        196109
    );
    assert_eq!(
        *decoded
            .fees
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .x_fee_per_token,
        U256::from_u64(196255)
    );
    assert_eq!(
        *decoded
            .fees
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .y_fee_per_token,
        U256::from_u64(196401)
    );
    assert!(decoded.rewards.is_none());
    assert_eq!(
        (*decoded.code_hash.as_ref().ok_or_else(|| anyhow::anyhow!("missing optional"))?),
        U256::from_u64(196620)
    );
    let dispatched = DedustV2CpmmOutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustV2CpmmOutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustV2Cpmm_DedustV2CpmmClaimPositionReward() -> anyhow::Result<()> {
    let body = fixture("schema_DedustV2Cpmm_DedustV2CpmmClaimPositionReward")?;
    let decoded: DedustV2CpmmClaimPositionRewardMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 196766);
    assert_eq!(decoded.reward_index, 3);
    assert_eq!(decoded.reward.remaining_time, 196985);
    assert_eq!(*decoded.reward.remaining_budget, 197058);
    assert_eq!(*decoded.reward.rewards_per_token, U256::from_u64(197204));
    assert_eq!(decoded.reward.last_update, 197277);
    assert!(decoded.excesses_to.is_none());
    let dispatched = DedustV2CpmmOutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustV2CpmmOutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustV2Cpmm_DedustV2CpmmPayoutPayload() -> anyhow::Result<()> {
    let body = fixture("schema_DedustV2Cpmm_DedustV2CpmmPayoutPayload")?;
    let decoded: DedustV2CpmmPayoutPayloadMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.exit_code, -197496);
    assert!(decoded.payload.is_none());
    let dispatched = DedustV2CpmmOutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustV2CpmmOutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustV2Cpmm_DedustV2CpmmSwapEvent() -> anyhow::Result<()> {
    let body = fixture("schema_DedustV2Cpmm_DedustV2CpmmSwapEvent")?;
    let decoded: DedustV2CpmmSwapEventMsg = TLB::from_cell(&body)?;
    assert!(!decoded.x_to_y);
    assert_eq!(*decoded.amount_in, 197788);
    assert_eq!(*decoded.amount_out, 197861);
    assert_eq!(
        decoded.initiator_address,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000a97")?
    );
    assert_eq!(
        decoded.recipient_address,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000a98")?
    );
    assert_eq!(*decoded.next_state.reserve_x, 198226);
    assert_eq!(*decoded.next_state.reserve_y, 198299);
    assert_eq!(*decoded.next_state.liquidity, 198372);
    assert_eq!(*decoded.next_state.fees.protocol_fee_x, 198591);
    assert_eq!(*decoded.next_state.fees.protocol_fee_y, 198664);
    assert_eq!(*decoded.next_state.fees.creator_fee_x, 198737);
    assert_eq!(*decoded.next_state.fees.creator_fee_y, 198810);
    assert_eq!(*decoded.next_state.fees.x_fee_per_token, U256::from_u64(198956));
    assert_eq!(*decoded.next_state.fees.y_fee_per_token, U256::from_u64(199102));
    assert!(!decoded.fees.is_x);
    assert_eq!(*decoded.fees.lp_fee, 199394);
    assert_eq!(*decoded.fees.creator_fee, 199467);
    assert_eq!(*decoded.fees.protocol_fee, 199540);
    assert_eq!(*decoded.fees.partner_fee, 199613);
    assert_eq!(*decoded.fees.referrer_fee, 199686);
    let dispatched = DedustV2CpmmOutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustV2CpmmOutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustV2Cpmm_DedustV2CpmmDepositEvent() -> anyhow::Result<()> {
    let body = fixture("schema_DedustV2Cpmm_DedustV2CpmmDepositEvent")?;
    let decoded: DedustV2CpmmDepositEventMsg = TLB::from_cell(&body)?;
    assert_eq!(*decoded.amount_x_in, 199832);
    assert_eq!(*decoded.amount_y_in, 199905);
    assert_eq!(*decoded.liquidity, 199978);
    assert_eq!(
        decoded.initiator_address,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000ab4")?
    );
    assert_eq!(
        decoded.recipient_address,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000ab5")?
    );
    assert_eq!(*decoded.next_state.reserve_x, 200343);
    assert_eq!(*decoded.next_state.reserve_y, 200416);
    assert_eq!(*decoded.next_state.liquidity, 200489);
    assert_eq!(*decoded.next_state.fees.protocol_fee_x, 200708);
    assert_eq!(*decoded.next_state.fees.protocol_fee_y, 200781);
    assert_eq!(*decoded.next_state.fees.creator_fee_x, 200854);
    assert_eq!(*decoded.next_state.fees.creator_fee_y, 200927);
    assert_eq!(*decoded.next_state.fees.x_fee_per_token, U256::from_u64(201073));
    assert_eq!(*decoded.next_state.fees.y_fee_per_token, U256::from_u64(201219));
    let dispatched = DedustV2CpmmOutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustV2CpmmOutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustV2Cpmm_DedustV2CpmmWithdrawalEvent() -> anyhow::Result<()> {
    let body = fixture("schema_DedustV2Cpmm_DedustV2CpmmWithdrawalEvent")?;
    let decoded: DedustV2CpmmWithdrawalEventMsg = TLB::from_cell(&body)?;
    assert_eq!(*decoded.liquidity, 201365);
    assert_eq!(*decoded.amount_x_out, 201438);
    assert_eq!(*decoded.amount_y_out, 201511);
    assert_eq!(
        decoded.initiator_address,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000ac9")?
    );
    assert_eq!(
        decoded.recipient_address,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000aca")?
    );
    assert_eq!(*decoded.next_state.reserve_x, 201876);
    assert_eq!(*decoded.next_state.reserve_y, 201949);
    assert_eq!(*decoded.next_state.liquidity, 202022);
    assert_eq!(*decoded.next_state.fees.protocol_fee_x, 202241);
    assert_eq!(*decoded.next_state.fees.protocol_fee_y, 202314);
    assert_eq!(*decoded.next_state.fees.creator_fee_x, 202387);
    assert_eq!(*decoded.next_state.fees.creator_fee_y, 202460);
    assert_eq!(*decoded.next_state.fees.x_fee_per_token, U256::from_u64(202606));
    assert_eq!(*decoded.next_state.fees.y_fee_per_token, U256::from_u64(202752));
    let dispatched = DedustV2CpmmOutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustV2CpmmOutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustUranusFactoryV3_DedustUranusFactoryV3Storage() -> anyhow::Result<()> {
    let body = fixture("schema_DedustUranusFactoryV3_DedustUranusFactoryV3Storage")?;
    let decoded: DedustUranusFactoryV3Storage = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.controller_address,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000adb")?
    );
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustUranusFactoryV3_DedustUranusMemeMetadata() -> anyhow::Result<()> {
    let body = fixture("schema_DedustUranusFactoryV3_DedustUranusMemeMetadata")?;
    let decoded: crate::contracts::uranus::factory_v3::storage::DedustUranusFactoryV3MemeMetadata =
        TLB::from_cell(&body)?;

    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustUranusFactoryV3_DedustUranusPartnerConfig() -> anyhow::Result<()> {
    let body = fixture("schema_DedustUranusFactoryV3_DedustUranusPartnerConfig")?;
    let decoded: crate::contracts::uranus::factory_v3::storage::DedustUranusFactoryV3PartnerConfig =
        TLB::from_cell(&body)?;
    assert_eq!(decoded.partner_id, U256::from_u64(203190));
    assert_eq!(decoded.partner_fee_bps, 6655);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustUranusFactoryV3_DedustUranusReferrerConfig() -> anyhow::Result<()> {
    let body = fixture("schema_DedustUranusFactoryV3_DedustUranusReferrerConfig")?;
    let decoded: crate::contracts::uranus::factory_v3::storage::DedustUranusFactoryV3ReferrerConfig =
        TLB::from_cell(&body)?;
    assert_eq!(decoded.referrer_id, U256::from_u64(203409));
    assert_eq!(decoded.referrer_fee_bps, 6874);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustUranusFactoryV3_DedustUranusDeployMeme() -> anyhow::Result<()> {
    let body = fixture("schema_DedustUranusFactoryV3_DedustUranusDeployMeme")?;
    let decoded: DedustUranusFactoryV3DeployMemeMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 203628);
    assert_eq!(decoded.preset_id, 5);
    assert_eq!(*decoded.initial_buy, 203920);
    assert_eq!(
        decoded
            .partner_config
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .partner_id,
        U256::from_u64(204139)
    );
    assert_eq!(
        decoded
            .partner_config
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .partner_fee_bps,
        7604
    );
    assert_eq!(
        decoded
            .referrer_config
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .referrer_id,
        U256::from_u64(204431)
    );
    assert_eq!(
        decoded
            .referrer_config
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .referrer_fee_bps,
        7896
    );
    let dispatched = DedustUranusFactoryV3IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustUranusFactoryV3IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustUranusFactoryV3_DedustUranusDeployCustomizedMeme() -> anyhow::Result<()> {
    let body = fixture("schema_DedustUranusFactoryV3_DedustUranusDeployCustomizedMeme")?;
    let decoded: DedustUranusFactoryV3DeployCustomizedMemeMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 204650);
    assert_eq!(decoded.total_supply_preset_id, 3);
    assert_eq!(decoded.base_fee_preset_id, 12);
    assert_eq!(*decoded.raising_funds, 204869);
    assert_eq!(decoded.on_sell_supply_percent, 142);
    assert_eq!(
        decoded.partner_address,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000af8")?
    );
    assert_eq!(decoded.partner_fee_bps, 8480);
    assert_eq!(decoded.pool_partner_fee_bps, 8553);
    assert_eq!(decoded.pool_base_fee_preset_id, 8626);
    assert_eq!(
        decoded.pool_liquidity_owner_address,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000afc"
        )?)
    );
    assert_eq!(*decoded.initial_buy, 205453);
    assert_eq!(
        decoded
            .partner_config
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .partner_id,
        U256::from_u64(205745)
    );
    assert_eq!(
        decoded
            .partner_config
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .partner_fee_bps,
        9210
    );
    assert!(decoded.referrer_config.is_none());
    let dispatched = DedustUranusFactoryV3IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustUranusFactoryV3IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustUranusFactoryV3_DedustUranusInitMeme() -> anyhow::Result<()> {
    let body = fixture("schema_DedustUranusFactoryV3_DedustUranusInitMeme")?;
    let decoded: crate::contracts::uranus::factory_v3::msgs::DedustUranusFactoryV3InitMemeMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 206037);
    assert_eq!(*decoded.initial_buy, 206110);
    assert_eq!(
        decoded
            .partner_config
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .partner_id,
        U256::from_u64(206329)
    );
    assert_eq!(
        decoded
            .partner_config
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .partner_fee_bps,
        9794
    );
    assert_eq!(
        decoded
            .referrer_config
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .referrer_id,
        U256::from_u64(206621)
    );
    assert_eq!(
        decoded
            .referrer_config
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .referrer_fee_bps,
        10086
    );
    let dispatched = DedustUranusFactoryV3OutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustUranusFactoryV3OutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustUranusMemeV2_DedustUranusMemeV2Metadata() -> anyhow::Result<()> {
    let body = fixture("schema_DedustUranusMemeV2_DedustUranusMemeV2Metadata")?;
    let decoded: DedustUranusMemeV2Metadata = TLB::from_cell(&body)?;

    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustUranusMemeV2_DedustUranusMemeV2PartnerConfig() -> anyhow::Result<()> {
    let body = fixture("schema_DedustUranusMemeV2_DedustUranusMemeV2PartnerConfig")?;
    let decoded: DedustUranusMemeV2PartnerConfig = TLB::from_cell(&body)?;
    assert_eq!(decoded.partner_id, U256::from_u64(206986));
    assert_eq!(decoded.partner_fee_bps, 10451);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustUranusMemeV2_DedustUranusMemeV2ReferrerConfig() -> anyhow::Result<()> {
    let body = fixture("schema_DedustUranusMemeV2_DedustUranusMemeV2ReferrerConfig")?;
    let decoded: DedustUranusMemeV2ReferrerConfig = TLB::from_cell(&body)?;
    assert_eq!(decoded.referrer_id, U256::from_u64(207205));
    assert_eq!(decoded.referrer_fee_bps, 10670);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustUranusMemeV2_DedustUranusMemeV2Config() -> anyhow::Result<()> {
    let body = fixture("schema_DedustUranusMemeV2_DedustUranusMemeV2Config")?;
    let decoded: DedustUranusMemeV2Config = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.creator_address,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000b19")?
    );
    assert_eq!(
        decoded.controller_address,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000b1a")?
    );
    assert_eq!(*decoded.raising_funds, 207716);
    assert_eq!(*decoded.liquidity_tokens, 207789);
    assert_eq!(decoded.seed, 207862);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustUranusMemeV2_DedustUranusMemeV2BondingCurve() -> anyhow::Result<()> {
    let body = fixture("schema_DedustUranusMemeV2_DedustUranusMemeV2BondingCurve")?;
    let decoded: DedustUranusMemeV2BondingCurve = TLB::from_cell(&body)?;
    assert!(decoded.graduated);
    assert_eq!(*decoded.alpha, 208081);
    assert_eq!(*decoded.beta, 208154);
    assert_eq!(*decoded.on_sell_supply, 208227);
    assert_eq!(*decoded.raised_funds, 208300);
    assert_eq!(*decoded.current_supply, 208373);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustUranusMemeV2_DedustUranusMemeV2Storage() -> anyhow::Result<()> {
    let body = fixture("schema_DedustUranusMemeV2_DedustUranusMemeV2Storage")?;
    let decoded: DedustUranusMemeV2Storage = TLB::from_cell(&body)?;
    assert!(!decoded.initialized);
    assert!(decoded.migrated);
    assert_eq!(
        decoded.config.creator_address,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000b2c")?
    );
    assert_eq!(
        decoded.config.controller_address,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000b2d")?
    );
    assert_eq!(*decoded.config.raising_funds, 209103);
    assert_eq!(*decoded.config.liquidity_tokens, 209176);
    assert_eq!(decoded.config.seed, 209249);
    assert_eq!(decoded.base_fee_bps, 12714);
    assert!(decoded.curve.graduated);
    assert_eq!(*decoded.curve.alpha, 209541);
    assert_eq!(*decoded.curve.beta, 209614);
    assert_eq!(*decoded.curve.on_sell_supply, 209687);
    assert_eq!(*decoded.curve.raised_funds, 209760);
    assert_eq!(*decoded.curve.current_supply, 209833);
    assert_eq!(*decoded.total_supply, 209906);
    assert_eq!(*decoded.creator_fee, 209979);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustUranusMemeV2_DedustUranusMemeV2TradeFees() -> anyhow::Result<()> {
    let body = fixture("schema_DedustUranusMemeV2_DedustUranusMemeV2TradeFees")?;
    let decoded: DedustUranusMemeV2TradeFees = TLB::from_cell(&body)?;
    assert_eq!(*decoded.creator_fee, 210125);
    assert_eq!(*decoded.protocol_fee, 210198);
    assert_eq!(*decoded.partner_fee, 210271);
    assert_eq!(*decoded.referrer_fee, 210344);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustUranusMemeV2_DedustUranusMemeV2InitMeme() -> anyhow::Result<()> {
    let body = fixture("schema_DedustUranusMemeV2_DedustUranusMemeV2InitMeme")?;
    let decoded: DedustUranusMemeV2InitMemeMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 210490);
    assert_eq!(*decoded.initial_buy, 210563);
    assert_eq!(
        decoded
            .partner_config
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .partner_id,
        U256::from_u64(210782)
    );
    assert_eq!(
        decoded
            .partner_config
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .partner_fee_bps,
        14247
    );
    assert!(decoded.referrer_config.is_none());
    let dispatched = DedustUranusMemeV2IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustUranusMemeV2IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustUranusMemeV2_DedustUranusMemeV2BuyMessage() -> anyhow::Result<()> {
    let body = fixture("schema_DedustUranusMemeV2_DedustUranusMemeV2BuyMessage")?;
    let decoded: DedustUranusMemeV2BuyMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 211074);
    assert_eq!(*decoded.amount, 211147);
    assert_eq!(*decoded.minimal_amount_out, 211220);
    assert_eq!(
        decoded.excesses_to,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000b4e")?
    );
    assert!(decoded.partner_config.is_none());
    assert_eq!(
        decoded
            .referrer_config
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .referrer_id,
        U256::from_u64(211658)
    );
    assert_eq!(
        decoded
            .referrer_config
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .referrer_fee_bps,
        15123
    );
    let dispatched = DedustUranusMemeV2IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustUranusMemeV2IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustUranusMemeV2_DedustUranusMemeV2SellMessage() -> anyhow::Result<()> {
    let body = fixture("schema_DedustUranusMemeV2_DedustUranusMemeV2SellMessage")?;
    let decoded: DedustUranusMemeV2SellMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 211877);
    assert_eq!(*decoded.amount, 211950);
    assert_eq!(*decoded.minimal_amount_out, 212023);
    assert_eq!(
        decoded.r#from,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000b59")?
    );
    assert_eq!(
        decoded.excesses_to,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000b5a"
        )?)
    );
    assert!(decoded.partner_config.is_none());
    assert_eq!(
        decoded
            .referrer_config
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .referrer_id,
        U256::from_u64(212534)
    );
    assert_eq!(
        decoded
            .referrer_config
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .referrer_fee_bps,
        15999
    );
    let dispatched = DedustUranusMemeV2IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustUranusMemeV2IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustUranusMemeV2_DedustUranusMemeV2BurnNotification() -> anyhow::Result<()> {
    let body = fixture("schema_DedustUranusMemeV2_DedustUranusMemeV2BurnNotification")?;
    let decoded: DedustUranusMemeV2BurnNotificationMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 212753);
    assert_eq!(*decoded.amount, 212826);
    assert_eq!(
        decoded.sender,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000b64")?
    );
    assert_eq!(
        decoded.response_destination,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000b65"
        )?)
    );
    let dispatched = DedustUranusMemeV2IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustUranusMemeV2IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustUranusMemeV2_DedustUranusMemeV2ProvideWalletAddress() -> anyhow::Result<()> {
    let body = fixture("schema_DedustUranusMemeV2_DedustUranusMemeV2ProvideWalletAddress")?;
    let decoded: DedustUranusMemeV2ProvideWalletAddressMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 213118);
    assert_eq!(
        decoded.owner_address,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000b68"
        )?)
    );
    assert!(decoded.include_address);
    let dispatched = DedustUranusMemeV2IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustUranusMemeV2IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustUranusMemeV2_DedustUranusMemeV2ClaimCreatorFee() -> anyhow::Result<()> {
    let body = fixture("schema_DedustUranusMemeV2_DedustUranusMemeV2ClaimCreatorFee")?;
    let decoded: DedustUranusMemeV2ClaimCreatorFeeMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 213410);
    assert_eq!(
        decoded.to,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000b6c"
        )?)
    );
    assert!(decoded.excesses_to.is_none());
    let dispatched = DedustUranusMemeV2IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustUranusMemeV2IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustUranusMemeV2_DedustUranusMemeV2OpcodeF14b54f3() -> anyhow::Result<()> {
    let body = fixture("schema_DedustUranusMemeV2_DedustUranusMemeV2OpcodeF14b54f3")?;
    let decoded: DedustUranusMemeV2OpcodeF14b54f3Msg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 213702);
    assert!(decoded.to.is_none());
    let dispatched = DedustUranusMemeV2IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustUranusMemeV2IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustUranusMemeV2_DedustUranusMemeV2Excesses() -> anyhow::Result<()> {
    let body = fixture("schema_DedustUranusMemeV2_DedustUranusMemeV2Excesses")?;
    let decoded: DedustUranusMemeV2ExcessesMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 213921);
    let dispatched = DedustUranusMemeV2IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustUranusMemeV2IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    let dispatched = DedustUranusMemeV2OutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustUranusMemeV2OutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustUranusMemeV2_DedustUranusMemeV2CpmmInitResult() -> anyhow::Result<()> {
    let body = fixture("schema_DedustUranusMemeV2_DedustUranusMemeV2CpmmInitResult")?;
    let decoded: DedustUranusMemeV2CpmmInitResultMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 214067);
    assert_eq!(decoded.exit_code, -214140);
    assert!(decoded.payload.is_none());
    let dispatched = DedustUranusMemeV2IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustUranusMemeV2IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustUranusMemeV2_DedustUranusMemeV2BuyEvent() -> anyhow::Result<()> {
    let body = fixture("schema_DedustUranusMemeV2_DedustUranusMemeV2BuyEvent")?;
    let decoded: DedustUranusMemeV2BuyEventMsg = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.trader_address,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000b78")?
    );
    assert_eq!(*decoded.amount_in, 214432);
    assert_eq!(*decoded.amount_out, 214505);
    assert_eq!(*decoded.fees.creator_fee, 214651);
    assert_eq!(*decoded.fees.protocol_fee, 214724);
    assert_eq!(*decoded.fees.partner_fee, 214797);
    assert_eq!(*decoded.fees.referrer_fee, 214870);
    assert_eq!(*decoded.current_supply, 214943);
    assert_eq!(*decoded.raised_funds, 215016);
    assert!(!decoded.is_graduated);
    let dispatched = DedustUranusMemeV2OutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustUranusMemeV2OutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustUranusMemeV2_DedustUranusMemeV2SellEvent() -> anyhow::Result<()> {
    let body = fixture("schema_DedustUranusMemeV2_DedustUranusMemeV2SellEvent")?;
    let decoded: DedustUranusMemeV2SellEventMsg = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.trader_address,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000b84")?
    );
    assert_eq!(*decoded.amount_in, 215308);
    assert_eq!(*decoded.amount_out, 215381);
    assert_eq!(*decoded.fees.creator_fee, 215527);
    assert_eq!(*decoded.fees.protocol_fee, 215600);
    assert_eq!(*decoded.fees.partner_fee, 215673);
    assert_eq!(*decoded.fees.referrer_fee, 215746);
    assert_eq!(*decoded.current_supply, 215819);
    assert_eq!(*decoded.raised_funds, 215892);
    let dispatched = DedustUranusMemeV2OutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustUranusMemeV2OutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustUranusMemeV2_DedustUranusMemeV2Opcode66c8ad72() -> anyhow::Result<()> {
    let body = fixture("schema_DedustUranusMemeV2_DedustUranusMemeV2Opcode66c8ad72")?;
    let decoded: DedustUranusMemeV2Opcode66c8ad72Msg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 216038);
    let dispatched = DedustUranusMemeV2OutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustUranusMemeV2OutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustUranusMemeV2_DedustUranusMemeV2TakeWalletAddress() -> anyhow::Result<()> {
    let body = fixture("schema_DedustUranusMemeV2_DedustUranusMemeV2TakeWalletAddress")?;
    let decoded: DedustUranusMemeV2TakeWalletAddressMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 216184);
    assert_eq!(
        decoded.wallet_address,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000b92"
        )?)
    );
    assert_eq!(
        (decoded
            .owner_address
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?)
        .to_cell()?
        .hash()?,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000b95")?
            .to_msg_address_int()
            .to_cell()?
            .hash()?
    );
    let dispatched = DedustUranusMemeV2OutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustUranusMemeV2OutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustUranusMemeV3_DedustUranusMemeMetadata() -> anyhow::Result<()> {
    let body = fixture("schema_DedustUranusMemeV3_DedustUranusMemeMetadata")?;
    let decoded: crate::contracts::uranus::meme_v3::storage::DedustUranusMemeV3Metadata = TLB::from_cell(&body)?;

    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustUranusMemeV3_DedustUranusPartnerConfig() -> anyhow::Result<()> {
    let body = fixture("schema_DedustUranusMemeV3_DedustUranusPartnerConfig")?;
    let decoded: crate::contracts::uranus::meme_v3::storage::DedustUranusMemeV3PartnerConfig = TLB::from_cell(&body)?;
    assert_eq!(decoded.partner_id, U256::from_u64(216768));
    assert_eq!(decoded.partner_fee_bps, 20233);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustUranusMemeV3_DedustUranusReferrerConfig() -> anyhow::Result<()> {
    let body = fixture("schema_DedustUranusMemeV3_DedustUranusReferrerConfig")?;
    let decoded: crate::contracts::uranus::meme_v3::storage::DedustUranusMemeV3ReferrerConfig = TLB::from_cell(&body)?;
    assert_eq!(decoded.referrer_id, U256::from_u64(216987));
    assert_eq!(decoded.referrer_fee_bps, 20452);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustUranusMemeV3_DedustUranusMemeConfig() -> anyhow::Result<()> {
    let body = fixture("schema_DedustUranusMemeV3_DedustUranusMemeConfig")?;
    let decoded: DedustUranusMemeV3Config = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.creator_address,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000b9f")?
    );
    assert_eq!(
        decoded.controller_address,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000ba0")?
    );
    assert_eq!(*decoded.raising_funds, 217498);
    assert_eq!(*decoded.liquidity_tokens, 217571);
    assert_eq!(decoded.seed, 217644);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustUranusMemeV3_DedustUranusBondingCurve() -> anyhow::Result<()> {
    let body = fixture("schema_DedustUranusMemeV3_DedustUranusBondingCurve")?;
    let decoded: DedustUranusMemeV3BondingCurve = TLB::from_cell(&body)?;
    assert!(decoded.graduated);
    assert_eq!(*decoded.alpha, 217863);
    assert_eq!(*decoded.beta, 217936);
    assert_eq!(*decoded.on_sell_supply, 218009);
    assert_eq!(*decoded.raised_funds, 218082);
    assert_eq!(*decoded.current_supply, 218155);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustUranusMemeV3_DedustUranusMemeV3Storage() -> anyhow::Result<()> {
    let body = fixture("schema_DedustUranusMemeV3_DedustUranusMemeV3Storage")?;
    let decoded: DedustUranusMemeV3Storage = TLB::from_cell(&body)?;
    assert!(!decoded.initialized);
    assert!(decoded.migrated);
    assert_eq!(
        decoded.config.creator_address,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000bb2")?
    );
    assert_eq!(
        decoded.config.controller_address,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000bb3")?
    );
    assert_eq!(*decoded.config.raising_funds, 218885);
    assert_eq!(*decoded.config.liquidity_tokens, 218958);
    assert_eq!(decoded.config.seed, 219031);
    assert_eq!(decoded.base_fee_bps, 22496);
    assert!(decoded.curve.graduated);
    assert_eq!(*decoded.curve.alpha, 219323);
    assert_eq!(*decoded.curve.beta, 219396);
    assert_eq!(*decoded.curve.on_sell_supply, 219469);
    assert_eq!(*decoded.curve.raised_funds, 219542);
    assert_eq!(*decoded.curve.current_supply, 219615);
    assert_eq!(*decoded.total_supply, 219688);
    assert_eq!(*decoded.creator_fee, 219761);
    assert_eq!(*decoded.partner_fee, 37);
    assert_eq!(decoded.partner_fee_bps, 5000);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustUranusMemeV3_DedustUranusTradeFees() -> anyhow::Result<()> {
    let body = fixture("schema_DedustUranusMemeV3_DedustUranusTradeFees")?;
    let decoded: DedustUranusMemeV3TradeFees = TLB::from_cell(&body)?;
    assert_eq!(*decoded.creator_fee, 219907);
    assert_eq!(*decoded.protocol_fee, 219980);
    assert_eq!(*decoded.partner_fee, 220053);
    assert_eq!(*decoded.referrer_fee, 220126);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustUranusMemeV3_DedustUranusInitMeme() -> anyhow::Result<()> {
    let body = fixture("schema_DedustUranusMemeV3_DedustUranusInitMeme")?;
    let decoded: crate::contracts::uranus::meme_v3::msgs::DedustUranusMemeV3InitMemeMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 220272);
    assert_eq!(*decoded.initial_buy, 220345);
    assert_eq!(
        decoded
            .partner_config
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .partner_id,
        U256::from_u64(220564)
    );
    assert_eq!(
        decoded
            .partner_config
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .partner_fee_bps,
        24029
    );
    assert_eq!(
        decoded
            .referrer_config
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .referrer_id,
        U256::from_u64(220856)
    );
    assert_eq!(
        decoded
            .referrer_config
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .referrer_fee_bps,
        24321
    );
    let dispatched = DedustUranusMemeV3IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustUranusMemeV3IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustUranusMemeV3_DedustUranusBuyMessage() -> anyhow::Result<()> {
    let body = fixture("schema_DedustUranusMemeV3_DedustUranusBuyMessage")?;
    let decoded: DedustUranusMemeV3BuyMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 221075);
    assert_eq!(*decoded.amount, 221148);
    assert_eq!(*decoded.minimal_amount_out, 221221);
    assert_eq!(
        decoded.excesses_to,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000bd7")?
    );
    assert_eq!(
        decoded
            .partner_config
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .partner_id,
        U256::from_u64(221586)
    );
    assert_eq!(
        decoded
            .partner_config
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .partner_fee_bps,
        25051
    );
    assert_eq!(
        decoded
            .referrer_config
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .referrer_id,
        U256::from_u64(221951)
    );
    assert_eq!(
        decoded
            .referrer_config
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .referrer_fee_bps,
        25416
    );
    let dispatched = DedustUranusMemeV3IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustUranusMemeV3IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustUranusMemeV3_DedustUranusSellMessage() -> anyhow::Result<()> {
    let body = fixture("schema_DedustUranusMemeV3_DedustUranusSellMessage")?;
    let decoded: crate::contracts::uranus::meme_v3::msgs::DedustUranusMemeV3SellMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 222170);
    assert_eq!(*decoded.amount, 222243);
    assert_eq!(*decoded.minimal_amount_out, 222316);
    assert_eq!(
        decoded.r#from,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000be6")?
    );
    assert_eq!(
        decoded.excesses_to,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000be7"
        )?)
    );
    assert!(decoded.partner_config.is_none());
    assert_eq!(
        decoded
            .referrer_config
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .referrer_id,
        U256::from_u64(222827)
    );
    assert_eq!(
        decoded
            .referrer_config
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .referrer_fee_bps,
        26292
    );
    let dispatched = DedustUranusMemeV3IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustUranusMemeV3IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustUranusMemeV3_DedustUranusBurnNotification() -> anyhow::Result<()> {
    let body = fixture("schema_DedustUranusMemeV3_DedustUranusBurnNotification")?;
    let decoded: crate::contracts::uranus::meme_v3::msgs::DedustUranusMemeV3BurnNotificationMsg =
        TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 223046);
    assert_eq!(*decoded.amount, 223119);
    assert_eq!(
        decoded.sender,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000bf1")?
    );
    assert_eq!(
        decoded.response_destination,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000bf2"
        )?)
    );
    let dispatched = DedustUranusMemeV3IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustUranusMemeV3IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustUranusMemeV3_DedustUranusProvideWalletAddress() -> anyhow::Result<()> {
    let body = fixture("schema_DedustUranusMemeV3_DedustUranusProvideWalletAddress")?;
    let decoded: DedustUranusMemeV3ProvideWalletAddressMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 223411);
    assert_eq!(
        decoded.owner_address,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000bf5"
        )?)
    );
    assert!(!decoded.include_address);
    let dispatched = DedustUranusMemeV3IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustUranusMemeV3IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustUranusMemeV3_DedustUranusClaimCreatorFee() -> anyhow::Result<()> {
    let body = fixture("schema_DedustUranusMemeV3_DedustUranusClaimCreatorFee")?;
    let decoded: DedustUranusMemeV3ClaimCreatorFeeMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 223703);
    assert_eq!(
        decoded.to,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000bf9"
        )?)
    );
    assert!(decoded.excesses_to.is_none());
    let dispatched = DedustUranusMemeV3IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustUranusMemeV3IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustUranusMemeV3_DedustUranusClaimPartnerFee() -> anyhow::Result<()> {
    let body = fixture("schema_DedustUranusMemeV3_DedustUranusClaimPartnerFee")?;
    let decoded: DedustUranusMemeV3ClaimPartnerFeeMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 223995);
    assert!(decoded.to.is_none());
    assert_eq!(
        decoded.excesses_to,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000bfe"
        )?)
    );
    let dispatched = DedustUranusMemeV3IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustUranusMemeV3IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustUranusMemeV3_DedustUranusOpcodeF14b54f3() -> anyhow::Result<()> {
    let body = fixture("schema_DedustUranusMemeV3_DedustUranusOpcodeF14b54f3")?;
    let decoded: DedustUranusMemeV3OpcodeF14b54f3Msg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 224287);
    assert_eq!(
        decoded.to,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000c01"
        )?)
    );
    let dispatched = DedustUranusMemeV3IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustUranusMemeV3IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustUranusMemeV3_DedustUranusExcesses() -> anyhow::Result<()> {
    let body = fixture("schema_DedustUranusMemeV3_DedustUranusExcesses")?;
    let decoded: crate::contracts::uranus::meme_v3::msgs::DedustUranusMemeV3ExcessesMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 224506);
    let dispatched = DedustUranusMemeV3IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustUranusMemeV3IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    let dispatched = DedustUranusMemeV3OutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustUranusMemeV3OutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustUranusMemeV3_DedustUranusCpmmInitResult() -> anyhow::Result<()> {
    let body = fixture("schema_DedustUranusMemeV3_DedustUranusCpmmInitResult")?;
    let decoded: DedustUranusMemeV3CpmmInitResultMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 224652);
    assert_eq!(decoded.exit_code, -224725);
    let dispatched = DedustUranusMemeV3IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustUranusMemeV3IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustUranusMemeV3_DedustUranusBuyEvent() -> anyhow::Result<()> {
    let body = fixture("schema_DedustUranusMemeV3_DedustUranusBuyEvent")?;
    let decoded: DedustUranusMemeV3BuyEventMsg = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.trader_address,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000c0a")?
    );
    assert_eq!(*decoded.amount_in, 225090);
    assert_eq!(*decoded.amount_out, 225163);
    assert_eq!(*decoded.fees.creator_fee, 225309);
    assert_eq!(*decoded.fees.protocol_fee, 225382);
    assert_eq!(*decoded.fees.partner_fee, 225455);
    assert_eq!(*decoded.fees.referrer_fee, 225528);
    assert_eq!(*decoded.current_supply, 225601);
    assert_eq!(*decoded.raised_funds, 225674);
    assert!(!decoded.is_graduated);
    let dispatched = DedustUranusMemeV3OutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustUranusMemeV3OutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustUranusMemeV3_DedustUranusSellEvent() -> anyhow::Result<()> {
    let body = fixture("schema_DedustUranusMemeV3_DedustUranusSellEvent")?;
    let decoded: DedustUranusMemeV3SellEventMsg = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.trader_address,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000c16")?
    );
    assert_eq!(*decoded.amount_in, 225966);
    assert_eq!(*decoded.amount_out, 226039);
    assert_eq!(*decoded.fees.creator_fee, 226185);
    assert_eq!(*decoded.fees.protocol_fee, 226258);
    assert_eq!(*decoded.fees.partner_fee, 226331);
    assert_eq!(*decoded.fees.referrer_fee, 226404);
    assert_eq!(*decoded.current_supply, 226477);
    assert_eq!(*decoded.raised_funds, 226550);
    let dispatched = DedustUranusMemeV3OutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustUranusMemeV3OutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustUranusMemeV3_DedustUranusOpcode66c8ad72() -> anyhow::Result<()> {
    let body = fixture("schema_DedustUranusMemeV3_DedustUranusOpcode66c8ad72")?;
    let decoded: DedustUranusMemeV3Opcode66c8ad72Msg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 226696);
    let dispatched = DedustUranusMemeV3OutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustUranusMemeV3OutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustUranusMemeV3_DedustUranusTakeWalletAddress() -> anyhow::Result<()> {
    let body = fixture("schema_DedustUranusMemeV3_DedustUranusTakeWalletAddress")?;
    let decoded: DedustUranusMemeV3TakeWalletAddressMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 226842);
    assert!(decoded.wallet_address.is_none());
    assert_eq!(
        (decoded
            .owner_address
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?)
        .to_cell()?
        .hash()?,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000c27")?
            .to_msg_address_int()
            .to_cell()?
            .hash()?
    );
    let dispatched = DedustUranusMemeV3OutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustUranusMemeV3OutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustUranusMemeWalletV2_DedustUranusPartnerConfig() -> anyhow::Result<()> {
    let body = fixture("schema_DedustUranusMemeWalletV2_DedustUranusPartnerConfig")?;
    let decoded: crate::contracts::uranus::wallet_v2::storage::DedustUranusMemeWalletV2PartnerConfig =
        TLB::from_cell(&body)?;
    assert_eq!(decoded.partner_id, U256::from_u64(227280));
    assert_eq!(decoded.partner_fee_bps, 30745);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustUranusMemeWalletV2_DedustUranusReferrerConfig() -> anyhow::Result<()> {
    let body = fixture("schema_DedustUranusMemeWalletV2_DedustUranusReferrerConfig")?;
    let decoded: crate::contracts::uranus::wallet_v2::storage::DedustUranusMemeWalletV2ReferrerConfig =
        TLB::from_cell(&body)?;
    assert_eq!(decoded.referrer_id, U256::from_u64(227499));
    assert_eq!(decoded.referrer_fee_bps, 30964);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustUranusMemeWalletV2_DedustUranusMemeWalletV3Storage() -> anyhow::Result<()> {
    let body = fixture("schema_DedustUranusMemeWalletV2_DedustUranusMemeWalletV3Storage")?;
    let decoded: DedustUranusMemeWalletV2Storage = TLB::from_cell(&body)?;
    assert_eq!(*decoded.balance, 227718);
    assert_eq!(
        tonabis_core::address::nullable_address_cell(&decoded.owner_address)?.hash()?,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000c30")?
            .to_msg_address_int()
            .to_cell()?
            .hash()?
    );
    assert_eq!(
        tonabis_core::address::nullable_address_cell(&decoded.meme_address)?.hash()?,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000c31")?
            .to_msg_address_int()
            .to_cell()?
            .hash()?
    );
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustUranusMemeWalletV2_DedustUranusJettonTransfer() -> anyhow::Result<()> {
    let body = fixture("schema_DedustUranusMemeWalletV2_DedustUranusJettonTransfer")?;
    let decoded: crate::contracts::uranus::wallet_v2::msgs::DedustUranusMemeWalletV2JettonTransferMsg =
        TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 228010);
    assert_eq!(*decoded.amount, 228083);
    assert_eq!(
        tonabis_core::address::nullable_address_cell(&decoded.destination)?.hash()?,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000c35")?
            .to_msg_address_int()
            .to_cell()?
            .hash()?
    );
    assert!(decoded.response_destination.is_none());
    assert_eq!(*decoded.forward_ton_amount, 228448);
    let dispatched = DedustUranusMemeWalletV2IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustUranusMemeWalletV2IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustUranusMemeWalletV2_DedustUranusJettonInternalTransfer() -> anyhow::Result<()> {
    let body = fixture("schema_DedustUranusMemeWalletV2_DedustUranusJettonInternalTransfer")?;
    let decoded: crate::contracts::uranus::wallet_v2::msgs::DedustUranusMemeWalletV2JettonInternalTransferMsg =
        TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 228667);
    assert_eq!(*decoded.amount, 228740);
    assert_eq!(
        tonabis_core::address::nullable_address_cell(&decoded.r#from)?.hash()?,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000c3e")?
            .to_msg_address_int()
            .to_cell()?
            .hash()?
    );
    assert!(decoded.response_address.is_none());
    assert_eq!(*decoded.forward_ton_amount, 228959);
    let dispatched = DedustUranusMemeWalletV2IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustUranusMemeWalletV2IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    let dispatched = DedustUranusMemeWalletV2OutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustUranusMemeWalletV2OutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustUranusMemeWalletV2_DedustUranusJettonBurn() -> anyhow::Result<()> {
    let body = fixture("schema_DedustUranusMemeWalletV2_DedustUranusJettonBurn")?;
    let decoded: crate::contracts::uranus::wallet_v2::msgs::DedustUranusMemeWalletV2JettonBurnMsg =
        TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 229178);
    assert_eq!(*decoded.amount, 229251);
    assert!(decoded.response_destination.is_none());
    let dispatched = DedustUranusMemeWalletV2IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustUranusMemeWalletV2IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustUranusMemeWalletV2_DedustUranusSellTokensMessage() -> anyhow::Result<()> {
    let body = fixture("schema_DedustUranusMemeWalletV2_DedustUranusSellTokensMessage")?;
    let decoded: crate::contracts::uranus::wallet_v2::msgs::DedustUranusMemeWalletV2SellTokensMsg =
        TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 229616);
    assert_eq!(*decoded.amount, 229689);
    assert_eq!(*decoded.minimal_amount_out, 229762);
    assert_eq!(
        decoded.excesses_to,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000c4c"
        )?)
    );
    assert_eq!(
        decoded
            .partner_config
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .partner_id,
        U256::from_u64(230127)
    );
    assert_eq!(
        decoded
            .partner_config
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .partner_fee_bps,
        33592
    );
    assert_eq!(
        decoded
            .referrer_config
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .referrer_id,
        U256::from_u64(230492)
    );
    assert_eq!(
        decoded
            .referrer_config
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .referrer_fee_bps,
        33957
    );
    let dispatched = DedustUranusMemeWalletV2IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustUranusMemeWalletV2IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustUranusMemeWalletV2_DedustUranusSellMessage() -> anyhow::Result<()> {
    let body = fixture("schema_DedustUranusMemeWalletV2_DedustUranusSellMessage")?;
    let decoded: crate::contracts::uranus::wallet_v2::msgs::DedustUranusMemeWalletV2SellMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 230711);
    assert_eq!(*decoded.amount, 230784);
    assert_eq!(*decoded.minimal_amount_out, 230857);
    assert_eq!(
        tonabis_core::address::nullable_address_cell(&decoded.r#from)?.hash()?,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000c5b")?
            .to_msg_address_int()
            .to_cell()?
            .hash()?
    );
    assert_eq!(
        decoded.excesses_to,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000c5c"
        )?)
    );
    assert!(decoded.partner_config.is_none());
    assert_eq!(
        decoded
            .referrer_config
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .referrer_id,
        U256::from_u64(231368)
    );
    assert_eq!(
        decoded
            .referrer_config
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .referrer_fee_bps,
        34833
    );
    let dispatched = DedustUranusMemeWalletV2OutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustUranusMemeWalletV2OutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustUranusMemeWalletV2_DedustUranusBurnNotification() -> anyhow::Result<()> {
    let body = fixture("schema_DedustUranusMemeWalletV2_DedustUranusBurnNotification")?;
    let decoded: crate::contracts::uranus::wallet_v2::msgs::DedustUranusMemeWalletV2BurnNotificationMsg =
        TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 231587);
    assert_eq!(*decoded.amount, 231660);
    assert_eq!(
        tonabis_core::address::nullable_address_cell(&decoded.sender)?.hash()?,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000c66")?
            .to_msg_address_int()
            .to_cell()?
            .hash()?
    );
    assert_eq!(
        decoded.response_destination,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000c67"
        )?)
    );
    let dispatched = DedustUranusMemeWalletV2OutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustUranusMemeWalletV2OutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustUranusMemeWalletV2_DedustUranusTransferNotification() -> anyhow::Result<()> {
    let body = fixture("schema_DedustUranusMemeWalletV2_DedustUranusTransferNotification")?;
    let decoded: crate::contracts::uranus::wallet_v2::msgs::DedustUranusMemeWalletV2TransferNotificationMsg =
        TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 231952);
    assert_eq!(*decoded.amount, 232025);
    assert_eq!(
        tonabis_core::address::nullable_address_cell(&decoded.sender)?.hash()?,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000c6b")?
            .to_msg_address_int()
            .to_cell()?
            .hash()?
    );
    let dispatched = DedustUranusMemeWalletV2OutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustUranusMemeWalletV2OutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustUranusMemeWalletV2_DedustUranusExcesses() -> anyhow::Result<()> {
    let body = fixture("schema_DedustUranusMemeWalletV2_DedustUranusExcesses")?;
    let decoded: crate::contracts::uranus::wallet_v2::msgs::DedustUranusMemeWalletV2ExcessesMsg =
        TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 232317);
    let dispatched = DedustUranusMemeWalletV2OutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustUranusMemeWalletV2OutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustUranusMemeWalletV3_DedustUranusPartnerConfig() -> anyhow::Result<()> {
    let body = fixture("schema_DedustUranusMemeWalletV3_DedustUranusPartnerConfig")?;
    let decoded: crate::contracts::uranus::wallet_v3::storage::DedustUranusMemeWalletV3PartnerConfig =
        TLB::from_cell(&body)?;
    assert_eq!(decoded.partner_id, U256::from_u64(232463));
    assert_eq!(decoded.partner_fee_bps, 35928);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustUranusMemeWalletV3_DedustUranusReferrerConfig() -> anyhow::Result<()> {
    let body = fixture("schema_DedustUranusMemeWalletV3_DedustUranusReferrerConfig")?;
    let decoded: crate::contracts::uranus::wallet_v3::storage::DedustUranusMemeWalletV3ReferrerConfig =
        TLB::from_cell(&body)?;
    assert_eq!(decoded.referrer_id, U256::from_u64(232682));
    assert_eq!(decoded.referrer_fee_bps, 36147);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustUranusMemeWalletV3_DedustUranusMemeWalletV3Storage() -> anyhow::Result<()> {
    let body = fixture("schema_DedustUranusMemeWalletV3_DedustUranusMemeWalletV3Storage")?;
    let decoded: DedustUranusMemeWalletV3Storage = TLB::from_cell(&body)?;
    assert_eq!(*decoded.balance, 232901);
    assert_eq!(
        decoded.owner_address,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000c77")?
    );
    assert_eq!(
        decoded.meme_address,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000c78")?
    );
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustUranusMemeWalletV3_DedustUranusJettonTransfer() -> anyhow::Result<()> {
    let body = fixture("schema_DedustUranusMemeWalletV3_DedustUranusJettonTransfer")?;
    let decoded: crate::contracts::uranus::wallet_v3::msgs::DedustUranusMemeWalletV3JettonTransferMsg =
        TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 233193);
    assert_eq!(*decoded.amount, 233266);
    assert_eq!(
        decoded.destination,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000c7c")?
    );
    assert_eq!(
        decoded.response_destination,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000c7d"
        )?)
    );
    assert!(decoded.custom_payload.is_none());
    assert_eq!(*decoded.forward_ton_amount, 233558);
    let dispatched = DedustUranusMemeWalletV3IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustUranusMemeWalletV3IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustUranusMemeWalletV3_DedustUranusJettonInternalTransfer() -> anyhow::Result<()> {
    let body = fixture("schema_DedustUranusMemeWalletV3_DedustUranusJettonInternalTransfer")?;
    let decoded: crate::contracts::uranus::wallet_v3::msgs::DedustUranusMemeWalletV3JettonInternalTransferMsg =
        TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 233777);
    assert_eq!(*decoded.amount, 233850);
    assert_eq!(
        decoded.r#from,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000c84"
        )?)
    );
    assert_eq!(
        decoded.response_address,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000c85"
        )?)
    );
    assert_eq!(*decoded.forward_ton_amount, 234069);
    let dispatched = DedustUranusMemeWalletV3IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustUranusMemeWalletV3IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    let dispatched = DedustUranusMemeWalletV3OutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustUranusMemeWalletV3OutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustUranusMemeWalletV3_DedustUranusJettonBurn() -> anyhow::Result<()> {
    let body = fixture("schema_DedustUranusMemeWalletV3_DedustUranusJettonBurn")?;
    let decoded: crate::contracts::uranus::wallet_v3::msgs::DedustUranusMemeWalletV3JettonBurnMsg =
        TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 234288);
    assert_eq!(*decoded.amount, 234361);
    assert_eq!(
        decoded.response_destination,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000c8b"
        )?)
    );
    let dispatched = DedustUranusMemeWalletV3IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustUranusMemeWalletV3IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustUranusMemeWalletV3_DedustUranusSellTokensMessage() -> anyhow::Result<()> {
    let body = fixture("schema_DedustUranusMemeWalletV3_DedustUranusSellTokensMessage")?;
    let decoded: crate::contracts::uranus::wallet_v3::msgs::DedustUranusMemeWalletV3SellTokensMsg =
        TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 234726);
    assert_eq!(*decoded.amount, 234799);
    assert_eq!(*decoded.minimal_amount_out, 234872);
    assert_eq!(
        decoded.excesses_to,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000c92"
        )?)
    );
    assert!(decoded.partner_config.is_none());
    assert_eq!(
        decoded
            .referrer_config
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .referrer_id,
        U256::from_u64(235310)
    );
    assert_eq!(
        decoded
            .referrer_config
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .referrer_fee_bps,
        38775
    );
    let dispatched = DedustUranusMemeWalletV3IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustUranusMemeWalletV3IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustUranusMemeWalletV3_DedustUranusSellMessage() -> anyhow::Result<()> {
    let body = fixture("schema_DedustUranusMemeWalletV3_DedustUranusSellMessage")?;
    let decoded: crate::contracts::uranus::wallet_v3::msgs::DedustUranusMemeWalletV3SellMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 235529);
    assert_eq!(*decoded.amount, 235602);
    assert_eq!(*decoded.minimal_amount_out, 235675);
    assert_eq!(
        decoded.r#from,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000c9d")?
    );
    assert_eq!(
        decoded.excesses_to,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000c9e"
        )?)
    );
    assert!(decoded.partner_config.is_none());
    assert_eq!(
        decoded
            .referrer_config
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .referrer_id,
        U256::from_u64(236186)
    );
    assert_eq!(
        decoded
            .referrer_config
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .referrer_fee_bps,
        39651
    );
    let dispatched = DedustUranusMemeWalletV3OutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustUranusMemeWalletV3OutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustUranusMemeWalletV3_DedustUranusBurnNotification() -> anyhow::Result<()> {
    let body = fixture("schema_DedustUranusMemeWalletV3_DedustUranusBurnNotification")?;
    let decoded: crate::contracts::uranus::wallet_v3::msgs::DedustUranusMemeWalletV3BurnNotificationMsg =
        TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 236405);
    assert_eq!(*decoded.amount, 236478);
    assert_eq!(
        decoded.sender,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000ca8")?
    );
    assert_eq!(
        decoded.response_destination,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000ca9"
        )?)
    );
    let dispatched = DedustUranusMemeWalletV3OutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustUranusMemeWalletV3OutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustUranusMemeWalletV3_DedustUranusTransferNotification() -> anyhow::Result<()> {
    let body = fixture("schema_DedustUranusMemeWalletV3_DedustUranusTransferNotification")?;
    let decoded: crate::contracts::uranus::wallet_v3::msgs::DedustUranusMemeWalletV3TransferNotificationMsg =
        TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 236770);
    assert_eq!(*decoded.amount, 236843);
    assert_eq!(
        decoded.sender,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000cad")?
    );
    let dispatched = DedustUranusMemeWalletV3OutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustUranusMemeWalletV3OutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustUranusMemeWalletV3_DedustUranusExcesses() -> anyhow::Result<()> {
    let body = fixture("schema_DedustUranusMemeWalletV3_DedustUranusExcesses")?;
    let decoded: crate::contracts::uranus::wallet_v3::msgs::DedustUranusMemeWalletV3ExcessesMsg =
        TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 237135);
    let dispatched = DedustUranusMemeWalletV3OutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustUranusMemeWalletV3OutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustOperatorV2_DedustOperatorV2Forward() -> anyhow::Result<()> {
    let body = fixture("schema_DedustOperatorV2_DedustOperatorV2Forward")?;
    let decoded: DedustOperatorV2ForwardMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 237281);
    assert_eq!(
        decoded.destination,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000cb3")?
    );
    let dispatched = DedustOperatorV2IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustOperatorV2IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustOperatorV2_DedustOperatorV2SetUserAddress() -> anyhow::Result<()> {
    let body = fixture("schema_DedustOperatorV2_DedustOperatorV2SetUserAddress")?;
    let decoded: DedustOperatorV2SetUserAddressMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 237573);
    assert_eq!(
        decoded.user_address,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000cb7")?
    );
    let dispatched = DedustOperatorV2IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustOperatorV2IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustOperatorV2_DedustOperatorV2CollectFees() -> anyhow::Result<()> {
    let body = fixture("schema_DedustOperatorV2_DedustOperatorV2CollectFees")?;
    let decoded: DedustOperatorV2CollectFeesMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 237792);
    assert_eq!(*decoded.asset0_amount, 237865);
    assert_eq!(
        decoded.asset0_destination,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000cbb")?
    );
    assert_eq!(*decoded.asset1_amount, 238157);
    assert_eq!(
        decoded.asset1_destination,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000cbf"
        )?)
    );
    assert!(decoded.asset1_payload.is_none());
    let dispatched = DedustOperatorV2OutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustOperatorV2OutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustOperatorV2_DedustOperatorV2ConfigureStartTime() -> anyhow::Result<()> {
    let body = fixture("schema_DedustOperatorV2_DedustOperatorV2ConfigureStartTime")?;
    let decoded: DedustOperatorV2ConfigureStartTimeMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 238449);
    assert_eq!(decoded.start_at, 238522);
    let dispatched = DedustOperatorV2OutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustOperatorV2OutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustX1000WalletV1_DedustX1000WalletV1Storage() -> anyhow::Result<()> {
    let body = fixture("schema_DedustX1000WalletV1_DedustX1000WalletV1Storage")?;
    let decoded: DedustX1000WalletV1Storage = TLB::from_cell(&body)?;
    assert_eq!(decoded.seqno_offset, 238668);
    assert_eq!(
        decoded.seqno_bitmap.to_hex(),
        "000000000000000000000000000000000000000000000000000000000003a495"
    );
    assert_eq!(decoded.public_key, U256::from_u64(238814));
    assert!(decoded.revoked.is_empty());
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustX1000WalletV2_DedustX1000WalletV2Storage() -> anyhow::Result<()> {
    let body = fixture("schema_DedustX1000WalletV2_DedustX1000WalletV2Storage")?;
    let decoded: DedustX1000WalletV2Storage = TLB::from_cell(&body)?;
    assert_eq!(decoded.storage_version, 185);
    assert_eq!(decoded.counter0, 239106);
    assert_eq!(decoded.seqno, 239179);
    assert_eq!(decoded.public_key, U256::from_u64(239252));
    assert!(decoded.runtime_state.execution_entries.is_empty());
    assert!(decoded.runtime_state.pending_operations.is_empty());
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustX1000WalletV2_DedustX1000WalletV2SignedRequest() -> anyhow::Result<()> {
    let body = fixture("schema_DedustX1000WalletV2_DedustX1000WalletV2SignedRequest")?;
    let decoded: DedustX1000WalletV2SignedRequestMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.signature, U512::from_u64(239471));
    assert_eq!(decoded.seqno, 239544);
    assert_eq!(decoded.payload.ignored_word, 26);
    assert_eq!(decoded.payload.field16, 19);
    assert_eq!(decoded.payload.send_mode, 3);
    assert_eq!(decoded.payload.action.request_id, 987654321);
    assert_eq!(*decoded.payload.action.amount, 123456);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustX1000WalletV2_DedustX1000WalletV2InternalCallback() -> anyhow::Result<()> {
    let body = fixture("schema_DedustX1000WalletV2_DedustX1000WalletV2InternalCallback")?;
    let decoded: DedustX1000WalletV2InternalCallbackMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.opcode, 239690);
    assert_eq!(decoded.request_id, 239763);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustX1000WalletV2_DedustX1000WalletV2ExecuteStoredMessage() -> anyhow::Result<()> {
    let body = fixture("schema_DedustX1000WalletV2_DedustX1000WalletV2ExecuteStoredMessage")?;
    let decoded: DedustX1000WalletV2ExecuteStoredMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.request_id, 239982);
    let dispatched = DedustX1000WalletV2IncomingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustX1000WalletV2IncomingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustX1000WalletV2_DedustX1000WalletV2ExecutionRequested() -> anyhow::Result<()> {
    let body = fixture("schema_DedustX1000WalletV2_DedustX1000WalletV2ExecutionRequested")?;
    let decoded: DedustX1000WalletV2ExecutionRequestedMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.request_id, 240201);
    let dispatched = DedustX1000WalletV2OutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustX1000WalletV2OutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustX1000WalletV2_DedustX1000WalletV2JettonTransfer() -> anyhow::Result<()> {
    let body = fixture("schema_DedustX1000WalletV2_DedustX1000WalletV2JettonTransfer")?;
    let decoded: DedustX1000WalletV2JettonTransferMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 240347);
    assert_eq!(*decoded.amount, 240420);
    assert_eq!(
        decoded.destination,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000cde")?
    );
    assert_eq!(
        decoded.response_destination,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000cdf"
        )?)
    );
    assert_eq!(*decoded.forward_amount, 240785);
    let dispatched = DedustX1000WalletV2OutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustX1000WalletV2OutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustX1000WalletV2_DedustX1000GivenIn() -> anyhow::Result<()> {
    let body = fixture("schema_DedustX1000WalletV2_DedustX1000GivenIn")?;
    let decoded: DedustX1000GivenIn = TLB::from_cell(&body)?;

    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustX1000WalletV2_DedustX1000GivenOut() -> anyhow::Result<()> {
    let body = fixture("schema_DedustX1000WalletV2_DedustX1000GivenOut")?;
    let decoded: DedustX1000GivenOut = TLB::from_cell(&body)?;

    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustX1000WalletV2_DedustX1000SwapStep() -> anyhow::Result<()> {
    let body = fixture("schema_DedustX1000WalletV2_DedustX1000SwapStep")?;
    let decoded: DedustX1000SwapStep = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.pool_address,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000ce9")?
    );
    assert!(matches!(&decoded.kind, DedustX1000SwapKind::GivenIn(_)));
    assert_eq!(*decoded.limit, 241515);
    assert!(decoded.next.is_none());
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustX1000WalletV2_DedustX1000SwapParams() -> anyhow::Result<()> {
    let body = fixture("schema_DedustX1000WalletV2_DedustX1000SwapParams")?;
    let decoded: DedustX1000SwapParams = TLB::from_cell(&body)?;
    assert_eq!(decoded.deadline, 241734);
    assert!(decoded.recipient_address.is_none());
    assert_eq!(
        decoded.referral_address,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000cf1"
        )?)
    );
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustX1000WalletV2_DedustX1000DedustSwapPayload() -> anyhow::Result<()> {
    let body = fixture("schema_DedustX1000WalletV2_DedustX1000DedustSwapPayload")?;
    let decoded: DedustX1000DedustSwapPayloadMsg = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.step.pool_address,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000cf8")?
    );
    assert!(matches!(&decoded.step.kind, DedustX1000SwapKind::GivenOut(_)));
    assert_eq!(*decoded.step.limit, 242610);
    assert!(decoded.step.next.is_none());
    assert_eq!(decoded.params.deadline, 242902);
    assert_eq!(
        decoded.params.recipient_address,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000d00"
        )?)
    );
    assert_eq!(
        decoded.params.referral_address,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000d01"
        )?)
    );
    assert!(decoded.params.fulfill_payload.is_none());
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustX1000WalletV2_DedustX1000StonfiV2SwapBody() -> anyhow::Result<()> {
    let body = fixture("schema_DedustX1000WalletV2_DedustX1000StonfiV2SwapBody")?;
    let decoded: DedustX1000StonfiV2SwapBody = TLB::from_cell(&body)?;
    assert_eq!(*decoded.min_out, 243413);
    assert_eq!(
        decoded.receiver,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000d07"
        )?)
    );
    assert_eq!(*decoded.forward_gas, 243559);
    assert_eq!(*decoded.refund_forward_gas, 243778);
    assert_eq!(decoded.referral_fee, 47389);
    assert_eq!(
        decoded.referral_address,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000d0f"
        )?)
    );
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustX1000WalletV2_DedustX1000StonfiV2SwapPayload() -> anyhow::Result<()> {
    let body = fixture("schema_DedustX1000WalletV2_DedustX1000StonfiV2SwapPayload")?;
    let decoded: DedustX1000StonfiV2SwapPayloadMsg = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.token_wallet,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000d11")?
    );
    assert_eq!(
        decoded.refund_address,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000d12")?
    );
    assert_eq!(
        decoded.excess_address,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000d13")?
    );
    assert_eq!(decoded.deadline, 244435);
    assert_eq!(*decoded.swap_body.min_out, 244654);
    assert_eq!(
        decoded.swap_body.receiver,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000d18"
        )?)
    );
    assert_eq!(*decoded.swap_body.forward_gas, 244800);
    assert!(decoded.swap_body.custom_payload.is_none());
    assert_eq!(*decoded.swap_body.refund_forward_gas, 244946);
    assert_eq!(decoded.swap_body.referral_fee, 48557);
    assert_eq!(
        decoded.swap_body.referral_address,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000d1f"
        )?)
    );
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustX1000WalletV2_DedustX1000ToncoMultihop() -> anyhow::Result<()> {
    let body = fixture("schema_DedustX1000WalletV2_DedustX1000ToncoMultihop")?;
    let decoded: DedustX1000ToncoMultihop = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.target_address,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000d21")?
    );
    assert_eq!(*decoded.fulfill_forward_amount, 245457);
    assert!(decoded.fulfill_payload.is_none());
    assert_eq!(*decoded.reject_forward_amount, 245603);
    assert_eq!(
        decoded.excess_address,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000d27")?
    );
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustX1000WalletV2_DedustX1000ToncoSwapPayload() -> anyhow::Result<()> {
    let body = fixture("schema_DedustX1000WalletV2_DedustX1000ToncoSwapPayload")?;
    let decoded: DedustX1000ToncoSwapPayloadMsg = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.target_jetton_wallet,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000d29")?
    );
    assert_eq!(decoded.sqrt_price_limit_x96, U256::from_u64(246041));
    assert_eq!(*decoded.min_out, 246114);
    assert_eq!(
        decoded.owner_address,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000d2c")?
    );
    assert_eq!(
        decoded
            .multihop
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .target_address,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000d30")?
    );
    assert_eq!(
        *decoded
            .multihop
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .fulfill_forward_amount,
        246552
    );
    assert!(
        decoded
            .multihop
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .fulfill_payload
            .is_none()
    );
    assert_eq!(
        *decoded
            .multihop
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .reject_forward_amount,
        246698
    );
    assert!(
        decoded
            .multihop
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .reject_payload
            .is_none()
    );
    assert_eq!(
        decoded
            .multihop
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .excess_address,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000d35")?
    );
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustX1000WalletV2_DedustX1000CpmmNextStep() -> anyhow::Result<()> {
    let body = fixture("schema_DedustX1000WalletV2_DedustX1000CpmmNextStep")?;
    let decoded: DedustX1000CpmmNextStep = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.pool_address,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000d37")?
    );
    assert_eq!(*decoded.min_out, 247063);
    assert_eq!(decoded.deadline, 247136);
    assert_eq!(
        decoded
            .next
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .pool_address,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000d3d")?
    );
    assert_eq!(
        *decoded
            .next
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .min_out,
        247501
    );
    assert_eq!(
        decoded
            .next
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .deadline,
        247574
    );
    assert!(
        decoded
            .next
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .next
            .is_none()
    );
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustX1000WalletV2_DedustX1000CpmmPartnerConfig() -> anyhow::Result<()> {
    let body = fixture("schema_DedustX1000WalletV2_DedustX1000CpmmPartnerConfig")?;
    let decoded: DedustX1000CpmmPartnerConfig = TLB::from_cell(&body)?;
    assert_eq!(decoded.partner_id, U256::from_u64(247793));
    assert_eq!(decoded.fee_bps, 51258);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustX1000WalletV2_DedustX1000CpmmReferrerConfig() -> anyhow::Result<()> {
    let body = fixture("schema_DedustX1000WalletV2_DedustX1000CpmmReferrerConfig")?;
    let decoded: DedustX1000CpmmReferrerConfig = TLB::from_cell(&body)?;
    assert_eq!(decoded.referrer_id, U256::from_u64(248012));
    assert_eq!(decoded.fee_bps, 51477);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustX1000WalletV2_DedustX1000CpmmSwapPayload() -> anyhow::Result<()> {
    let body = fixture("schema_DedustX1000WalletV2_DedustX1000CpmmSwapPayload")?;
    let decoded: DedustX1000CpmmSwapPayloadMsg = TLB::from_cell(&body)?;
    assert_eq!(*decoded.min_out, 248231);
    assert_eq!(decoded.deadline, 248304);
    assert!(decoded.next.is_none());
    assert_eq!(
        decoded
            .partner
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .partner_id,
        U256::from_u64(248596)
    );
    assert_eq!(
        decoded
            .partner
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .fee_bps,
        52061
    );
    assert_eq!(
        decoded
            .referrer
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .referrer_id,
        U256::from_u64(248888)
    );
    assert_eq!(
        decoded
            .referrer
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .fee_bps,
        52353
    );
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustX1000WalletV2_DedustX1000CpmmPayoutOptions() -> anyhow::Result<()> {
    let body = fixture("schema_DedustX1000WalletV2_DedustX1000CpmmPayoutOptions")?;
    let decoded: DedustX1000CpmmPayoutOptions = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.destination,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000d54"
        )?)
    );
    assert_eq!(*decoded.extra_gas, 249180);
    assert!(decoded.payload.is_none());
    assert!(decoded.wrap_payload);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustX1000WalletV2_DedustX1000CpmmPayoutConfig() -> anyhow::Result<()> {
    let body = fixture("schema_DedustX1000WalletV2_DedustX1000CpmmPayoutConfig")?;
    let decoded: DedustX1000CpmmPayoutConfig = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.fulfill.destination,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000d5a"
        )?)
    );
    assert_eq!(*decoded.fulfill.extra_gas, 249618);
    assert!(decoded.fulfill.payload.is_none());
    assert!(decoded.fulfill.wrap_payload);
    assert!(decoded.reject.destination.is_none());
    assert_eq!(*decoded.reject.extra_gas, 249983);
    assert!(decoded.reject.wrap_payload);
    assert_eq!(
        decoded.excess_address,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000d64"
        )?)
    );
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustX1000WalletV2_DedustX1000CpmmPayJettonPayload() -> anyhow::Result<()> {
    let body = fixture("schema_DedustX1000WalletV2_DedustX1000CpmmPayJettonPayload")?;
    let decoded: DedustX1000CpmmPayJettonPayloadMsg = TLB::from_cell(&body)?;
    assert_eq!(*decoded.operation.min_out, 17);
    assert_eq!(decoded.operation.deadline, 1000);
    assert_eq!(
        decoded.payout.fulfill.destination,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000d69"
        )?)
    );
    assert_eq!(*decoded.payout.fulfill.extra_gas, 250713);
    assert!(decoded.payout.fulfill.payload.is_none());
    assert!(!decoded.payout.fulfill.wrap_payload);
    assert!(decoded.payout.reject.destination.is_none());
    assert_eq!(*decoded.payout.reject.extra_gas, 251078);
    assert!(decoded.payout.reject.payload.is_none());
    assert!(decoded.payout.reject.wrap_payload);
    assert_eq!(
        decoded.payout.excess_address,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000d72"
        )?)
    );
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustX1000WalletV2_DedustX1000BidaskDammReferral() -> anyhow::Result<()> {
    let body = fixture("schema_DedustX1000WalletV2_DedustX1000BidaskDammReferral")?;
    let decoded: DedustX1000BidaskDammReferral = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.referral_address,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000d74")?
    );
    assert_eq!(decoded.referral_fee, 54908);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustX1000WalletV2_DedustX1000BidaskDammSwapPayload() -> anyhow::Result<()> {
    let body = fixture("schema_DedustX1000WalletV2_DedustX1000BidaskDammSwapPayload")?;
    let decoded: DedustX1000BidaskDammSwapPayloadMsg = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.to_address,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000d77")?
    );
    assert_eq!(*decoded.slippage, 251735);
    assert_eq!(
        decoded.from_address,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000d79")?
    );
    assert_eq!(*decoded.exact_out, 251881);
    assert_eq!(
        decoded
            .referral
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .referral_address,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000d7e")?
    );
    assert_eq!(
        decoded
            .referral
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .referral_fee,
        55638
    );
    assert!(decoded.reject_payload.is_none());
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustX1000WalletV2_DedustX1000BidaskNormalSlippage() -> anyhow::Result<()> {
    let body = fixture("schema_DedustX1000WalletV2_DedustX1000BidaskNormalSlippage")?;
    let decoded: DedustX1000BidaskNormalSlippage = TLB::from_cell(&body)?;
    assert_eq!(*decoded.min_out, 252611);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustX1000WalletV2_DedustX1000BidaskPartialSlippage() -> anyhow::Result<()> {
    let body = fixture("schema_DedustX1000WalletV2_DedustX1000BidaskPartialSlippage")?;
    let decoded: DedustX1000BidaskPartialSlippage = TLB::from_cell(&body)?;
    assert_eq!(decoded.sqrt_price_limit, U256::from_u64(252757));
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustX1000WalletV2_DedustX1000BidaskAdditionalData() -> anyhow::Result<()> {
    let body = fixture("schema_DedustX1000WalletV2_DedustX1000BidaskAdditionalData")?;
    let decoded: DedustX1000BidaskAdditionalData = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.from_address,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000d88")?
    );
    assert_eq!(
        decoded.referral_address,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000d89"
        )?)
    );
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustX1000WalletV2_DedustX1000BidaskClmmSwapPayload() -> anyhow::Result<()> {
    let body = fixture("schema_DedustX1000WalletV2_DedustX1000BidaskClmmSwapPayload")?;
    let decoded: DedustX1000BidaskClmmSwapPayloadMsg = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.to_address,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000d8b")?
    );
    let variant_3468 = match &decoded.slippage {
        DedustX1000BidaskSlippage::DedustX1000BidaskNormalSlippage(value) => value,
        _ => anyhow::bail!("wrong DedustX1000BidaskSlippage variant"),
    };
    assert_eq!(*variant_3468.min_out, 253341);
    assert_eq!(*decoded.exact_out, 253414);
    assert_eq!(
        decoded
            .additional_data
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .from_address,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000d93")?
    );
    assert_eq!(
        decoded
            .additional_data
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .referral_address,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000d94"
        )?)
    );
    assert!(decoded.reject_payload.is_none());
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustX1000WalletV2_DedustX1000CoffeeSwapStep() -> anyhow::Result<()> {
    let body = fixture("schema_DedustX1000WalletV2_DedustX1000CoffeeSwapStep")?;
    let decoded: DedustX1000CoffeeSwapStep = TLB::from_cell(&body)?;
    assert_eq!(decoded.pool_address_hash, U256::from_u64(254144));
    assert_eq!(*decoded.min_out, 254217);
    assert!(decoded.next.is_none());
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustX1000WalletV2_DedustX1000CoffeeNotification() -> anyhow::Result<()> {
    let body = fixture("schema_DedustX1000WalletV2_DedustX1000CoffeeNotification")?;
    let decoded: DedustX1000CoffeeNotification = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.recipient.to_cell()?.hash()?,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000005")?
            .to_msg_address_int()
            .to_cell()?
            .hash()?
    );
    assert_eq!(*decoded.forward_amount, 254436);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustX1000WalletV2_DedustX1000CoffeeNotifications() -> anyhow::Result<()> {
    let body = fixture("schema_DedustX1000WalletV2_DedustX1000CoffeeNotifications")?;
    let decoded: DedustX1000CoffeeNotifications = TLB::from_cell(&body)?;
    assert_eq!(
        decoded
            .fulfill
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .recipient
            .to_cell()?
            .hash()?,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000005")?
            .to_msg_address_int()
            .to_cell()?
            .hash()?
    );
    assert_eq!(
        *decoded
            .fulfill
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .forward_amount,
        254874
    );
    assert_eq!(
        decoded
            .reject
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .recipient
            .to_cell()?
            .hash()?,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000005")?
            .to_msg_address_int()
            .to_cell()?
            .hash()?
    );
    assert_eq!(
        *decoded
            .reject
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .forward_amount,
        255239
    );
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustX1000WalletV2_DedustX1000CoffeeSwapParams() -> anyhow::Result<()> {
    let body = fixture("schema_DedustX1000WalletV2_DedustX1000CoffeeSwapParams")?;
    let decoded: DedustX1000CoffeeSwapParams = TLB::from_cell(&body)?;
    assert_eq!(decoded.deadline, 255458);
    assert_eq!(
        decoded.recipient,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000dac")?
    );
    assert!(decoded.referral.is_none());
    assert!(
        decoded
            .notifications
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .fulfill
            .is_none()
    );
    assert!(
        decoded
            .notifications
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .reject
            .is_none()
    );
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustX1000WalletV2_DedustX1000CoffeeSwapPayload() -> anyhow::Result<()> {
    let body = fixture("schema_DedustX1000WalletV2_DedustX1000CoffeeSwapPayload")?;
    let decoded: DedustX1000CoffeeSwapPayloadMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.step.pool_address_hash, U256::from_u64(256188));
    assert_eq!(*decoded.step.min_out, 256261);
    assert_eq!(
        decoded
            .step
            .next
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .pool_address_hash,
        U256::from_u64(256553)
    );
    assert_eq!(
        *decoded
            .step
            .next
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .min_out,
        256626
    );
    assert!(
        decoded
            .step
            .next
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .next
            .is_none()
    );
    assert_eq!(
        decoded
            .params
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .deadline,
        256991
    );
    assert_eq!(
        decoded
            .params
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .recipient,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000dc1")?
    );
    assert!(
        decoded
            .params
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .referral
            .is_none()
    );
    assert!(
        decoded
            .params
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .notifications
            .is_none()
    );
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustX1000WalletV2_DedustX1000MoonCallback() -> anyhow::Result<()> {
    let body = fixture("schema_DedustX1000WalletV2_DedustX1000MoonCallback")?;
    let decoded: DedustX1000MoonCallback = TLB::from_cell(&body)?;
    assert!(decoded.recipient.is_none());
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustX1000WalletV2_DedustX1000MoonSwapParams() -> anyhow::Result<()> {
    let body = fixture("schema_DedustX1000WalletV2_DedustX1000MoonSwapParams")?;
    let decoded: DedustX1000MoonSwapParams = TLB::from_cell(&body)?;
    assert_eq!(*decoded.min_out, 257721);
    assert_eq!(decoded.deadline, 257794);
    assert_eq!(
        decoded.excess_address,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000dcc"
        )?)
    );
    assert_eq!(
        decoded.referral_address,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000dcd"
        )?)
    );
    assert!(decoded.fulfill.is_none());
    assert_eq!(
        decoded
            .reject
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .recipient,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000dd2"
        )?)
    );
    assert!(
        decoded
            .reject
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .payload
            .is_none()
    );
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustX1000WalletV2_DedustX1000MoonSwapPayload() -> anyhow::Result<()> {
    let body = fixture("schema_DedustX1000WalletV2_DedustX1000MoonSwapPayload")?;
    let decoded: DedustX1000MoonSwapPayloadMsg = TLB::from_cell(&body)?;
    assert_eq!(*decoded.params.min_out, 258597);
    assert_eq!(decoded.params.deadline, 258670);
    assert_eq!(
        decoded.params.excess_address,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000dd8"
        )?)
    );
    assert_eq!(
        decoded.params.referral_address,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000dd9"
        )?)
    );
    assert!(decoded.params.fulfill.is_none());
    assert_eq!(
        decoded
            .params
            .reject
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .recipient,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000dde"
        )?)
    );
    assert!(
        decoded
            .params
            .reject
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .payload
            .is_none()
    );
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustX1000WalletV2_DedustX1000MoonSwapSucceeded() -> anyhow::Result<()> {
    let body = fixture("schema_DedustX1000WalletV2_DedustX1000MoonSwapSucceeded")?;
    let decoded: DedustX1000MoonSwapSucceededMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.request_id, 259400);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustX1000WalletV2_DedustX1000MoonSwapFailed() -> anyhow::Result<()> {
    let body = fixture("schema_DedustX1000WalletV2_DedustX1000MoonSwapFailed")?;
    let decoded: DedustX1000MoonSwapFailedMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.request_id, 259546);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustX1000WalletV2_DedustX1000MoonClmmCallback() -> anyhow::Result<()> {
    let body = fixture("schema_DedustX1000WalletV2_DedustX1000MoonClmmCallback")?;
    let decoded: DedustX1000MoonClmmCallback = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.recipient,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000de5")?
    );
    assert!(matches!(decoded.payload, DedustX1000WalletV2CallbackPayloadMsg::Explicit(None)));
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustX1000WalletV2_DedustX1000MoonClmmSwapPayload() -> anyhow::Result<()> {
    let body = fixture("schema_DedustX1000WalletV2_DedustX1000MoonClmmSwapPayload")?;
    let decoded: DedustX1000MoonClmmSwapPayloadMsg = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.excess_address,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000de8")?
    );
    assert_eq!(decoded.sqrt_price_limit_x128, U256::from_u64(259984));
    assert_eq!(
        decoded
            .fulfill
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .recipient,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000ded")?
    );
    assert!(matches!(
        decoded
            .fulfill
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .payload,
        DedustX1000WalletV2CallbackPayloadMsg::Explicit(None)
    ));
    assert!(decoded.reject.is_none());
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustX1000WalletV2_DedustX1000PayloadFa45056b() -> anyhow::Result<()> {
    let body = fixture("schema_DedustX1000WalletV2_DedustX1000PayloadFa45056b")?;
    let decoded: DedustX1000PayloadFa45056bMsg = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.address0.to_cell()?.hash()?,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000005")?
            .to_msg_address_int()
            .to_cell()?
            .hash()?
    );
    assert_eq!(*decoded.unknown0, 260568);
    assert!(decoded.unknown1.is_none());
    assert_eq!(
        (decoded.unknown2.as_ref().ok_or_else(|| anyhow::anyhow!("missing optional"))?)
            .to_cell()?
            .hash()?,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000df5")?
            .to_msg_address_int()
            .to_cell()?
            .hash()?
    );
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustX1000WalletV2_DedustX1000StonfiV1SwapPayload() -> anyhow::Result<()> {
    let body = fixture("schema_DedustX1000WalletV2_DedustX1000StonfiV1SwapPayload")?;
    let decoded: DedustX1000StonfiV1SwapPayloadMsg = TLB::from_cell(&body)?;
    assert_eq!(
        decoded.token_wallet,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000df7")?
    );
    assert_eq!(*decoded.min_out, 261079);
    assert_eq!(
        decoded.recipient,
        TonAddress::from_str("0:0000000000000000000000000000000000000000000000000000000000000df9")?
    );
    assert_eq!(
        decoded.referral_address,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000dfa"
        )?)
    );
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustX1000WalletV2_DedustX1000StonksPumpSell() -> anyhow::Result<()> {
    let body = fixture("schema_DedustX1000WalletV2_DedustX1000StonksPumpSell")?;
    let decoded: DedustX1000StonksPumpSellMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 261371);
    assert_eq!(*decoded.amount, 261444);
    assert!(decoded.proceeds_recipient.is_none());
    assert_eq!(*decoded.min_gram_out, 261590);
    let dispatched = DedustX1000WalletV2OutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustX1000WalletV2OutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustX1000WalletV2_DedustX1000UranusPartnerConfig() -> anyhow::Result<()> {
    let body = fixture("schema_DedustX1000WalletV2_DedustX1000UranusPartnerConfig")?;
    let decoded: DedustX1000UranusPartnerConfig = TLB::from_cell(&body)?;
    assert_eq!(decoded.partner_id, U256::from_u64(261736));
    assert_eq!(decoded.fee_bps, 65201);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustX1000WalletV2_DedustX1000UranusReferrerConfig() -> anyhow::Result<()> {
    let body = fixture("schema_DedustX1000WalletV2_DedustX1000UranusReferrerConfig")?;
    let decoded: DedustX1000UranusReferrerConfig = TLB::from_cell(&body)?;
    assert_eq!(decoded.referrer_id, U256::from_u64(261955));
    assert_eq!(decoded.fee_bps, 65420);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustX1000WalletV2_DedustX1000UranusSell() -> anyhow::Result<()> {
    let body = fixture("schema_DedustX1000WalletV2_DedustX1000UranusSell")?;
    let decoded: DedustX1000UranusSellMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 262174);
    assert_eq!(*decoded.amount, 262247);
    assert_eq!(*decoded.min_gram_out, 262320);
    assert!(decoded.excess_address.is_none());
    assert_eq!(
        decoded
            .partner
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .partner_id,
        U256::from_u64(262685)
    );
    assert_eq!(
        decoded
            .partner
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing optional"))?
            .fee_bps,
        614
    );
    assert!(decoded.referrer.is_none());
    let dispatched = DedustX1000WalletV2OutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustX1000WalletV2OutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustX1000WalletV2_DedustX1000JettonBurn() -> anyhow::Result<()> {
    let body = fixture("schema_DedustX1000WalletV2_DedustX1000JettonBurn")?;
    let decoded: DedustX1000JettonBurnMsg = TLB::from_cell(&body)?;
    assert_eq!(decoded.query_id, 262977);
    assert_eq!(*decoded.amount, 263050);
    assert_eq!(
        decoded.response_destination,
        Some(TonAddress::from_str(
            "0:0000000000000000000000000000000000000000000000000000000000000e14"
        )?)
    );
    let dispatched = DedustX1000WalletV2OutgoingMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustX1000WalletV2OutgoingMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustX1000WalletV2_DedustX1000WalletV2Event10937847() -> anyhow::Result<()> {
    let body = fixture("schema_DedustX1000WalletV2_DedustX1000WalletV2Event10937847")?;
    let decoded: DedustX1000WalletV2Event10937847Msg = TLB::from_cell(&body)?;
    assert_eq!(decoded.unknown0, 263415);
    assert_eq!(decoded.unknown1, 263488);
    let dispatched = DedustX1000WalletV2EventMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustX1000WalletV2EventMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

#[test]
fn test_schema_DedustX1000WalletV2_DedustX1000WalletV2EventBeaf9617() -> anyhow::Result<()> {
    let body = fixture("schema_DedustX1000WalletV2_DedustX1000WalletV2EventBeaf9617")?;
    let decoded: DedustX1000WalletV2EventBeaf9617Msg = TLB::from_cell(&body)?;
    assert_eq!(decoded.unknown0, 263634);
    let dispatched = DedustX1000WalletV2EventMsg::from_cell(&body)?;
    assert!(!matches!(dispatched, DedustX1000WalletV2EventMsg::Unknown(_)));
    assert_eq!(dispatched.to_cell()?, body);
    assert_eq!(decoded.to_cell()?.hash()?, body.hash()?);
    Ok(())
}

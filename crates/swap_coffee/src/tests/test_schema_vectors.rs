//! Independent synthetic wire vectors. These do not establish captured-code parity.

use crate::contracts::cross_dex::msgs::CoffeeCrossDexBidaskNativeTransferNotificationMsg;
use crate::contracts::cross_dex::msgs::CoffeeCrossDexCoffeeNotificationMsg;
use crate::contracts::cross_dex::msgs::CoffeeCrossDexDedustPayoutMsg;
use crate::contracts::cross_dex::msgs::CoffeeCrossDexExcessesMsg;
use crate::contracts::cross_dex::msgs::CoffeeCrossDexFailurePayloadMsg;
use crate::contracts::cross_dex::msgs::CoffeeCrossDexJettonRescueMsg;
use crate::contracts::cross_dex::msgs::CoffeeCrossDexJettonTransferMsg;
use crate::contracts::cross_dex::msgs::CoffeeCrossDexJettonTransferNotificationMsg;
use crate::contracts::cross_dex::msgs::CoffeeCrossDexNativeRescueMsg;
use crate::contracts::cross_dex::msgs::CoffeeCrossDexPtonTonTransferMsg;
use crate::contracts::cross_dex::msgs::CoffeeCrossDexResendJettonPayloadMsg;
use crate::contracts::cross_dex::msgs::CoffeeCrossDexResendMsg;
use crate::contracts::cross_dex::msgs::CoffeeCrossDexSetAdminMsg;
use crate::contracts::cross_dex::msgs::CoffeeCrossDexUpdateContractMsg;
use crate::contracts::cross_dex::msgs::CoffeeCrossDexWrappedPayloadMsg;
use crate::contracts::cross_dex::storage::CoffeeCrossDexDeploymentStorage;
use crate::contracts::cross_dex::storage::CoffeeCrossDexStorage;
use crate::contracts::cross_dex_bidask::msgs::CoffeeCrossDexBidaskBidaskNativeTransferNotificationMsg;
use crate::contracts::cross_dex_bidask::msgs::CoffeeCrossDexBidaskCoffeeNotificationMsg;
use crate::contracts::cross_dex_bidask::msgs::CoffeeCrossDexBidaskDedustPayoutMsg;
use crate::contracts::cross_dex_bidask::msgs::CoffeeCrossDexBidaskExcessesMsg;
use crate::contracts::cross_dex_bidask::msgs::CoffeeCrossDexBidaskFailurePayloadMsg;
use crate::contracts::cross_dex_bidask::msgs::CoffeeCrossDexBidaskJettonTransferMsg;
use crate::contracts::cross_dex_bidask::msgs::CoffeeCrossDexBidaskPtonTonTransferMsg;
use crate::contracts::cross_dex_bidask::msgs::CoffeeCrossDexBidaskResendJettonPayloadMsg;
use crate::contracts::cross_dex_bidask::msgs::CoffeeCrossDexBidaskResendMsg;
use crate::contracts::cross_dex_bidask::msgs::CoffeeCrossDexBidaskSetAdminMsg;
use crate::contracts::cross_dex_bidask::msgs::CoffeeCrossDexBidaskUpdateContractMsg;
use crate::contracts::cross_dex_bidask::msgs::CoffeeCrossDexBidaskWithdrawJettonsMsg;
use crate::contracts::cross_dex_bidask::msgs::CoffeeCrossDexBidaskWithdrawTonMsg;
use crate::contracts::cross_dex_bidask::storage::CoffeeCrossDexBidaskDeploymentStorage;
use crate::contracts::cross_dex_bidask::storage::CoffeeCrossDexBidaskStorage;
use crate::contracts::cross_dex_legacy::msgs::CoffeeCrossDexLegacyDedustPayoutMsg;
use crate::contracts::cross_dex_legacy::msgs::CoffeeCrossDexLegacyExcessesMsg;
use crate::contracts::cross_dex_legacy::msgs::CoffeeCrossDexLegacyFailurePayloadMsg;
use crate::contracts::cross_dex_legacy::msgs::CoffeeCrossDexLegacyJettonTransferMsg;
use crate::contracts::cross_dex_legacy::msgs::CoffeeCrossDexLegacyPtonTonTransferMsg;
use crate::contracts::cross_dex_legacy::msgs::CoffeeCrossDexLegacyResendJettonPayloadMsg;
use crate::contracts::cross_dex_legacy::msgs::CoffeeCrossDexLegacyResendMsg;
use crate::contracts::cross_dex_legacy::msgs::CoffeeCrossDexLegacySetAdminMsg;
use crate::contracts::cross_dex_legacy::msgs::CoffeeCrossDexLegacyUpdateContractMsg;
use crate::contracts::cross_dex_legacy::msgs::CoffeeCrossDexLegacyWithdrawJettonsMsg;
use crate::contracts::cross_dex_legacy::msgs::CoffeeCrossDexLegacyWithdrawTonMsg;
use crate::contracts::cross_dex_legacy::storage::CoffeeCrossDexLegacyDeploymentStorage;
use crate::contracts::cross_dex_legacy::storage::CoffeeCrossDexLegacyStorage;
use crate::contracts::factory::msgs::CoffeeFactoryActivateVaultInternalMsg;
use crate::contracts::factory::msgs::CoffeeFactoryActivateVaultMsg;
use crate::contracts::factory::msgs::CoffeeFactoryConstantProductAmm;
use crate::contracts::factory::msgs::CoffeeFactoryContractUpdate;
use crate::contracts::factory::msgs::CoffeeFactoryCreateLiquidityDepositoryInternalMsg;
use crate::contracts::factory::msgs::CoffeeFactoryCreateLiquidityDepositoryRequestMsg;
use crate::contracts::factory::msgs::CoffeeFactoryCreatePoolCreatorInternalMsg;
use crate::contracts::factory::msgs::CoffeeFactoryCreatePoolCreatorRequestMsg;
use crate::contracts::factory::msgs::CoffeeFactoryCreatePoolInternalMsg;
use crate::contracts::factory::msgs::CoffeeFactoryCreatePoolRequestMsg;
use crate::contracts::factory::msgs::CoffeeFactoryCreateVaultInternalMsg;
use crate::contracts::factory::msgs::CoffeeFactoryCreateVaultMsg;
use crate::contracts::factory::msgs::CoffeeFactoryCurveFiStableAmm;
use crate::contracts::factory::msgs::CoffeeFactoryDeployMsg;
use crate::contracts::factory::msgs::CoffeeFactoryDepositComplexCondition;
use crate::contracts::factory::msgs::CoffeeFactoryDepositLiquidityParams;
use crate::contracts::factory::msgs::CoffeeFactoryDepositLiquidityParamsTrimmed;
use crate::contracts::factory::msgs::CoffeeFactoryDepositLpQuantityCondition;
use crate::contracts::factory::msgs::CoffeeFactoryDepositNoCondition;
use crate::contracts::factory::msgs::CoffeeFactoryDepositReservesRatioCondition;
use crate::contracts::factory::msgs::CoffeeFactoryExcessesMsg;
use crate::contracts::factory::msgs::CoffeeFactoryExtraAsset;
use crate::contracts::factory::msgs::CoffeeFactoryJettonAsset;
use crate::contracts::factory::msgs::CoffeeFactoryLiquidityDepositoryContractType;
use crate::contracts::factory::msgs::CoffeeFactoryNativeAsset;
use crate::contracts::factory::msgs::CoffeeFactoryNotificationData;
use crate::contracts::factory::msgs::CoffeeFactoryNotificationDataSingle;
use crate::contracts::factory::msgs::CoffeeFactoryPoolContractType;
use crate::contracts::factory::msgs::CoffeeFactoryPoolCreationParams;
use crate::contracts::factory::msgs::CoffeeFactoryPoolCreatorContractType;
use crate::contracts::factory::msgs::CoffeeFactoryPoolCreatorProof;
use crate::contracts::factory::msgs::CoffeeFactoryPoolFeeAndStatusUpdate;
use crate::contracts::factory::msgs::CoffeeFactoryPoolFeeUpdate;
use crate::contracts::factory::msgs::CoffeeFactoryPoolNoUpdate;
use crate::contracts::factory::msgs::CoffeeFactoryPoolParams;
use crate::contracts::factory::msgs::CoffeeFactoryPoolStatusUpdate;
use crate::contracts::factory::msgs::CoffeeFactoryPrivatePoolCreationParams;
use crate::contracts::factory::msgs::CoffeeFactoryPublicPoolCreationParams;
use crate::contracts::factory::msgs::CoffeeFactoryUpdateAdminMsg;
use crate::contracts::factory::msgs::CoffeeFactoryUpdateCodeCellsMsg;
use crate::contracts::factory::msgs::CoffeeFactoryUpdateContractInternalMsg;
use crate::contracts::factory::msgs::CoffeeFactoryUpdateContractMsg;
use crate::contracts::factory::msgs::CoffeeFactoryUpdatePoolInternalMsg;
use crate::contracts::factory::msgs::CoffeeFactoryUpdatePoolMsg;
use crate::contracts::factory::msgs::CoffeeFactoryUpdateWithdrawerMsg;
use crate::contracts::factory::msgs::CoffeeFactoryVaultContractType;
use crate::contracts::factory::msgs::CoffeeFactoryVaultProof;
use crate::contracts::factory::msgs::CoffeeFactoryWithdrawInternalMsg;
use crate::contracts::factory::msgs::CoffeeFactoryWithdrawMsg;
use crate::contracts::factory::storage::CoffeeFactoryCodeCells;
use crate::contracts::factory::storage::CoffeeFactoryCodeDictionaries;
use crate::contracts::factory::storage::CoffeeFactoryStorage;
use crate::contracts::init::msgs::CoffeeInitDeployMsg;
use crate::contracts::init::storage::CoffeeInitStorage;
use crate::contracts::liquidity_depository::msgs::CoffeeLiquidityDepositoryContractUpdate;
use crate::contracts::liquidity_depository::msgs::CoffeeLiquidityDepositoryCreateLiquidityDepositoryInternalMsg;
use crate::contracts::liquidity_depository::msgs::CoffeeLiquidityDepositoryDeployMsg;
use crate::contracts::liquidity_depository::msgs::CoffeeLiquidityDepositoryDepositConditionComplex;
use crate::contracts::liquidity_depository::msgs::CoffeeLiquidityDepositoryDepositConditionLpQuantity;
use crate::contracts::liquidity_depository::msgs::CoffeeLiquidityDepositoryDepositConditionNone;
use crate::contracts::liquidity_depository::msgs::CoffeeLiquidityDepositoryDepositConditionReservesRatio;
use crate::contracts::liquidity_depository::msgs::CoffeeLiquidityDepositoryDepositLiquidityInternalMsg;
use crate::contracts::liquidity_depository::msgs::CoffeeLiquidityDepositoryDepositLiquidityParamsTrimmed;
use crate::contracts::liquidity_depository::msgs::CoffeeLiquidityDepositoryExcessesMsg;
use crate::contracts::liquidity_depository::msgs::CoffeeLiquidityDepositoryNotificationData;
use crate::contracts::liquidity_depository::msgs::CoffeeLiquidityDepositoryNotificationDataSingle;
use crate::contracts::liquidity_depository::msgs::CoffeeLiquidityDepositoryPayoutInternalMsg;
use crate::contracts::liquidity_depository::msgs::CoffeeLiquidityDepositoryUpdateContractInternalMsg;
use crate::contracts::liquidity_depository::msgs::CoffeeLiquidityDepositoryWithdrawDepositMsg;
use crate::contracts::liquidity_depository::storage::CoffeeLiquidityDepositoryAmmConstantProduct;
use crate::contracts::liquidity_depository::storage::CoffeeLiquidityDepositoryAmmCurveFiStable;
use crate::contracts::liquidity_depository::storage::CoffeeLiquidityDepositoryAssetExtra;
use crate::contracts::liquidity_depository::storage::CoffeeLiquidityDepositoryAssetJetton;
use crate::contracts::liquidity_depository::storage::CoffeeLiquidityDepositoryAssetNative;
use crate::contracts::liquidity_depository::storage::CoffeeLiquidityDepositoryInitData;
use crate::contracts::liquidity_depository::storage::CoffeeLiquidityDepositoryInitPoolParams;
use crate::contracts::liquidity_depository::storage::CoffeeLiquidityDepositoryStorage;
use crate::contracts::lp_wallet::msgs::CoffeeLpWalletBouncedKnownBodyMsg;
use crate::contracts::lp_wallet::msgs::CoffeeLpWalletBurnMsg;
use crate::contracts::lp_wallet::msgs::CoffeeLpWalletBurnNotificationMsg;
use crate::contracts::lp_wallet::msgs::CoffeeLpWalletExcessesMsg;
use crate::contracts::lp_wallet::msgs::CoffeeLpWalletInternalTransferMsg;
use crate::contracts::lp_wallet::msgs::CoffeeLpWalletTransferMsg;
use crate::contracts::lp_wallet::msgs::CoffeeLpWalletTransferNotificationMsg;
use crate::contracts::lp_wallet::storage::CoffeeLpWalletStorage;
use crate::contracts::mev_protector::msgs::CoffeeMevProtectFailedSwapPayloadMsg;
use crate::contracts::mev_protector::msgs::CoffeeMevProtectHoldFundsMsg;
use crate::contracts::mev_protector::msgs::CoffeeMevProtectorCoffeeNotificationMsg;
use crate::contracts::mev_protector::msgs::CoffeeMevProtectorDedustPayoutMsg;
use crate::contracts::mev_protector::msgs::CoffeeMevProtectorExcessesMsg;
use crate::contracts::mev_protector::msgs::CoffeeMevProtectorJettonRescueMsg;
use crate::contracts::mev_protector::msgs::CoffeeMevProtectorJettonTransferMsg;
use crate::contracts::mev_protector::msgs::CoffeeMevProtectorNativeRescueMsg;
use crate::contracts::mev_protector::msgs::CoffeeMevProtectorSendMessageAction;
use crate::contracts::mev_protector::msgs::CoffeeMevProtectorSetAdminMsg;
use crate::contracts::mev_protector::msgs::CoffeeMevProtectorSetPublicKeyMsg;
use crate::contracts::mev_protector::msgs::CoffeeMevProtectorSignedBatchMsg;
use crate::contracts::mev_protector::msgs::CoffeeMevProtectorUpdateContractMsg;
use crate::contracts::mev_protector::storage::CoffeeMevProtectorStorage;
use crate::contracts::mev_protector_legacy::msgs::CoffeeMevProtectorLegacyAdmin2bf71be6Msg;
use crate::contracts::mev_protector_legacy::msgs::CoffeeMevProtectorLegacyAdmin7b4b82feMsg;
use crate::contracts::mev_protector_legacy::msgs::CoffeeMevProtectorLegacyDedustPayoutMsg;
use crate::contracts::mev_protector_legacy::msgs::CoffeeMevProtectorLegacyExcessesMsg;
use crate::contracts::mev_protector_legacy::msgs::CoffeeMevProtectorLegacyFailedSwapPayloadMsg;
use crate::contracts::mev_protector_legacy::msgs::CoffeeMevProtectorLegacyHoldFundsMsg;
use crate::contracts::mev_protector_legacy::msgs::CoffeeMevProtectorLegacyJettonTransferMsg;
use crate::contracts::mev_protector_legacy::msgs::CoffeeMevProtectorLegacySendMessageAction;
use crate::contracts::mev_protector_legacy::msgs::CoffeeMevProtectorLegacySetAdminMsg;
use crate::contracts::mev_protector_legacy::msgs::CoffeeMevProtectorLegacySetPublicKeyMsg;
use crate::contracts::mev_protector_legacy::msgs::CoffeeMevProtectorLegacySignedBatchMsg;
use crate::contracts::mev_protector_legacy::msgs::CoffeeMevProtectorLegacyUpdateContractMsg;
use crate::contracts::mev_protector_legacy::storage::CoffeeMevProtectorLegacyStorage;
use crate::contracts::pool_constant_product::msgs::CoffeeConstantProductPoolCreatePoolInternalMsg;
use crate::contracts::pool_constant_product::msgs::CoffeeConstantProductPoolDeployMsg;
use crate::contracts::pool_constant_product::msgs::CoffeeConstantProductPoolDepositLiquidityFailedMsg;
use crate::contracts::pool_constant_product::msgs::CoffeeConstantProductPoolDepositLiquidityInternalMsg;
use crate::contracts::pool_constant_product::msgs::CoffeeConstantProductPoolDepositLiquiditySucceededMsg;
use crate::contracts::pool_constant_product::msgs::CoffeeConstantProductPoolExcessesMsg;
use crate::contracts::pool_constant_product::msgs::CoffeeConstantProductPoolJettonBurnNotificationMsg;
use crate::contracts::pool_constant_product::msgs::CoffeeConstantProductPoolLpWalletInternalTransferMsg;
use crate::contracts::pool_constant_product::msgs::CoffeeConstantProductPoolNotificationMsg;
use crate::contracts::pool_constant_product::msgs::CoffeeConstantProductPoolPayoutInternalMsg;
use crate::contracts::pool_constant_product::msgs::CoffeeConstantProductPoolProvideWalletAddressMsg;
use crate::contracts::pool_constant_product::msgs::CoffeeConstantProductPoolSwapFailedMsg;
use crate::contracts::pool_constant_product::msgs::CoffeeConstantProductPoolSwapInternalMsg;
use crate::contracts::pool_constant_product::msgs::CoffeeConstantProductPoolSwapInternalOutMsg;
use crate::contracts::pool_constant_product::msgs::CoffeeConstantProductPoolSwapSucceededMsg;
use crate::contracts::pool_constant_product::msgs::CoffeeConstantProductPoolTakeWalletAddressMsg;
use crate::contracts::pool_constant_product::msgs::CoffeeConstantProductPoolUpdateContractInternalMsg;
use crate::contracts::pool_constant_product::msgs::CoffeeConstantProductPoolUpdatePoolInternalMsg;
use crate::contracts::pool_constant_product::msgs::CoffeeConstantProductPoolWithdrawInternalMsg;
use crate::contracts::pool_constant_product::msgs::CoffeeConstantProductPoolWithdrawLiquiditySucceededMsg;
use crate::contracts::pool_constant_product::msgs::CoffeePoolContractUpdate;
use crate::contracts::pool_constant_product::msgs::CoffeePoolDepositConditionComplex;
use crate::contracts::pool_constant_product::msgs::CoffeePoolDepositConditionLpQuantity;
use crate::contracts::pool_constant_product::msgs::CoffeePoolDepositConditionNone;
use crate::contracts::pool_constant_product::msgs::CoffeePoolDepositConditionReservesRatio;
use crate::contracts::pool_constant_product::msgs::CoffeePoolDepositLiquidityParamsTrimmed;
use crate::contracts::pool_constant_product::msgs::CoffeePoolNotificationData;
use crate::contracts::pool_constant_product::msgs::CoffeePoolNotificationDataSingle;
use crate::contracts::pool_constant_product::msgs::CoffeePoolPreviousAssetHintAbsent;
use crate::contracts::pool_constant_product::msgs::CoffeePoolPreviousExtraAssetHint;
use crate::contracts::pool_constant_product::msgs::CoffeePoolPreviousJettonAssetHint;
use crate::contracts::pool_constant_product::msgs::CoffeePoolPreviousNativeAssetHint;
use crate::contracts::pool_constant_product::msgs::CoffeePoolSwapParams;
use crate::contracts::pool_constant_product::msgs::CoffeePoolSwapStepInternalParams;
use crate::contracts::pool_constant_product::msgs::CoffeePoolSwapStepParams;
use crate::contracts::pool_constant_product::msgs::CoffeePoolUpdateActive;
use crate::contracts::pool_constant_product::msgs::CoffeePoolUpdateFees;
use crate::contracts::pool_constant_product::msgs::CoffeePoolUpdateFeesAndActive;
use crate::contracts::pool_constant_product::msgs::CoffeePoolUpdateNoop;
use crate::contracts::pool_constant_product::msgs::CoffeePoolValuePair;
use crate::contracts::pool_constant_product::msgs::CoffeePoolWithdrawConditionAssetsQuantity;
use crate::contracts::pool_constant_product::msgs::CoffeePoolWithdrawConditionComplex;
use crate::contracts::pool_constant_product::msgs::CoffeePoolWithdrawConditionNone;
use crate::contracts::pool_constant_product::msgs::CoffeePoolWithdrawConditionReservesRatio;
use crate::contracts::pool_constant_product::msgs::CoffeePoolWithdrawLiquidityParams;
use crate::contracts::pool_constant_product::storage::CoffeeConstantProductPoolStorage;
use crate::contracts::pool_constant_product::storage::CoffeePoolExtraAsset;
use crate::contracts::pool_constant_product::storage::CoffeePoolJettonAsset;
use crate::contracts::pool_constant_product::storage::CoffeePoolNativeAsset;
use crate::contracts::pool_constant_product::storage::CoffeePoolVaultProofBody;
use crate::contracts::pool_creator::msgs::CoffeePoolCreatorContractUpdate;
use crate::contracts::pool_creator::msgs::CoffeePoolCreatorCreatePoolCreatorInternalMsg;
use crate::contracts::pool_creator::msgs::CoffeePoolCreatorCreatePoolRequestMsg;
use crate::contracts::pool_creator::msgs::CoffeePoolCreatorDeployMsg;
use crate::contracts::pool_creator::msgs::CoffeePoolCreatorExcessesMsg;
use crate::contracts::pool_creator::msgs::CoffeePoolCreatorNotificationDataSingle;
use crate::contracts::pool_creator::msgs::CoffeePoolCreatorPayoutInternalMsg;
use crate::contracts::pool_creator::msgs::CoffeePoolCreatorPoolCreationParams;
use crate::contracts::pool_creator::msgs::CoffeePoolCreatorPrivatePoolCreationParams;
use crate::contracts::pool_creator::msgs::CoffeePoolCreatorPublicPoolCreationParams;
use crate::contracts::pool_creator::msgs::CoffeePoolCreatorUpdateContractInternalMsg;
use crate::contracts::pool_creator::msgs::CoffeePoolCreatorWithdrawDepositMsg;
use crate::contracts::pool_creator::storage::CoffeePoolCreatorAmmConstantProduct;
use crate::contracts::pool_creator::storage::CoffeePoolCreatorAmmCurveFiStable;
use crate::contracts::pool_creator::storage::CoffeePoolCreatorAssetExtra;
use crate::contracts::pool_creator::storage::CoffeePoolCreatorAssetJetton;
use crate::contracts::pool_creator::storage::CoffeePoolCreatorAssetNative;
use crate::contracts::pool_creator::storage::CoffeePoolCreatorInitData;
use crate::contracts::pool_creator::storage::CoffeePoolCreatorInitPoolParams;
use crate::contracts::pool_creator::storage::CoffeePoolCreatorStorage;
use crate::contracts::pool_curve_fi_stable::msgs::CoffeeCurveFiStablePoolCreatePoolInternalMsg;
use crate::contracts::pool_curve_fi_stable::msgs::CoffeeCurveFiStablePoolDeployMsg;
use crate::contracts::pool_curve_fi_stable::msgs::CoffeeCurveFiStablePoolDepositLiquidityFailedMsg;
use crate::contracts::pool_curve_fi_stable::msgs::CoffeeCurveFiStablePoolDepositLiquidityInternalMsg;
use crate::contracts::pool_curve_fi_stable::msgs::CoffeeCurveFiStablePoolDepositLiquiditySucceededMsg;
use crate::contracts::pool_curve_fi_stable::msgs::CoffeeCurveFiStablePoolExcessesMsg;
use crate::contracts::pool_curve_fi_stable::msgs::CoffeeCurveFiStablePoolJettonBurnNotificationMsg;
use crate::contracts::pool_curve_fi_stable::msgs::CoffeeCurveFiStablePoolLpWalletInternalTransferMsg;
use crate::contracts::pool_curve_fi_stable::msgs::CoffeeCurveFiStablePoolNotificationMsg;
use crate::contracts::pool_curve_fi_stable::msgs::CoffeeCurveFiStablePoolPayoutInternalMsg;
use crate::contracts::pool_curve_fi_stable::msgs::CoffeeCurveFiStablePoolProvideWalletAddressMsg;
use crate::contracts::pool_curve_fi_stable::msgs::CoffeeCurveFiStablePoolSwapFailedMsg;
use crate::contracts::pool_curve_fi_stable::msgs::CoffeeCurveFiStablePoolSwapInternalMsg;
use crate::contracts::pool_curve_fi_stable::msgs::CoffeeCurveFiStablePoolSwapInternalOutMsg;
use crate::contracts::pool_curve_fi_stable::msgs::CoffeeCurveFiStablePoolSwapSucceededMsg;
use crate::contracts::pool_curve_fi_stable::msgs::CoffeeCurveFiStablePoolTakeWalletAddressMsg;
use crate::contracts::pool_curve_fi_stable::msgs::CoffeeCurveFiStablePoolUpdateContractInternalMsg;
use crate::contracts::pool_curve_fi_stable::msgs::CoffeeCurveFiStablePoolUpdatePoolInternalMsg;
use crate::contracts::pool_curve_fi_stable::msgs::CoffeeCurveFiStablePoolWithdrawInternalMsg;
use crate::contracts::pool_curve_fi_stable::msgs::CoffeeCurveFiStablePoolWithdrawLiquiditySucceededMsg;
use crate::contracts::pool_curve_fi_stable::storage::CoffeeCurveFiStablePoolAmmSettings;
use crate::contracts::pool_curve_fi_stable::storage::CoffeeCurveFiStablePoolStorage;
use crate::contracts::staking_item::msgs::CoffeeStakingItemExtendMsg;
use crate::contracts::staking_item::msgs::CoffeeStakingItemExtendNotificationMsg;
use crate::contracts::staking_item::msgs::CoffeeStakingItemInitMsg;
use crate::contracts::staking_item::msgs::CoffeeStakingItemUnlockMsg;
use crate::contracts::staking_item::msgs::CoffeeStakingItemUnlockNotificationMsg;
use crate::contracts::staking_item::storage::CoffeeStakingAssetData;
use crate::contracts::staking_item::storage::CoffeeStakingItemStorage;
use crate::contracts::staking_item::storage::CoffeeStakingPositionData;
use crate::contracts::staking_master::msgs::CoffeeStakingClaimRewardsMsg;
use crate::contracts::staking_master::msgs::CoffeeStakingDepositMsg;
use crate::contracts::staking_master::msgs::CoffeeStakingDistributorUpdatedMsg;
use crate::contracts::staking_master::msgs::CoffeeStakingForwardData;
use crate::contracts::staking_master::msgs::CoffeeStakingItemExtendedMsg;
use crate::contracts::staking_master::msgs::CoffeeStakingItemUnlockedMsg;
use crate::contracts::staking_master::msgs::CoffeeStakingJettonsUpdatedMsg;
use crate::contracts::staking_master::msgs::CoffeeStakingMasterInitMsg;
use crate::contracts::staking_master::msgs::CoffeeStakingMasterInitializedMsg;
use crate::contracts::staking_master::msgs::CoffeeStakingOwnerUpdatedMsg;
use crate::contracts::staking_master::msgs::CoffeeStakingPeriodsUpdatedMsg;
use crate::contracts::staking_master::msgs::CoffeeStakingRewardsClaimedMsg;
use crate::contracts::staking_master::msgs::CoffeeStakingRewardsUpdatedMsg;
use crate::contracts::staking_master::msgs::CoffeeStakingSetDistributorMsg;
use crate::contracts::staking_master::msgs::CoffeeStakingSetJettonsMsg;
use crate::contracts::staking_master::msgs::CoffeeStakingSetOwnerMsg;
use crate::contracts::staking_master::msgs::CoffeeStakingSetPeriodsMsg;
use crate::contracts::staking_master::msgs::CoffeeStakingUpdateContractMsg;
use crate::contracts::staking_master::msgs::CoffeeStakingUpdateRewardsMsg;
use crate::contracts::staking_master::storage::CoffeeStakingMasterState;
use crate::contracts::staking_master::storage::CoffeeStakingMasterStateRefs;
use crate::contracts::staking_master::storage::CoffeeStakingMasterStorage;
use crate::contracts::staking_vault::msgs::CoffeeStakingLockForwardPayloadMsg;
use crate::contracts::staking_vault::msgs::CoffeeStakingVaultDepositToMasterMsg;
use crate::contracts::staking_vault::msgs::CoffeeStakingVaultInitMsg;
use crate::contracts::staking_vault::msgs::CoffeeStakingVaultItemInitMsg;
use crate::contracts::staking_vault::msgs::CoffeeStakingVaultJettonRefundMsg;
use crate::contracts::staking_vault::msgs::CoffeeStakingVaultMasterInitMsg;
use crate::contracts::staking_vault::msgs::CoffeeStakingVaultSetJettonsMsg;
use crate::contracts::staking_vault::msgs::CoffeeStakingVaultSetOwnerMsg;
use crate::contracts::staking_vault::msgs::CoffeeStakingVaultSetPeriodsMsg;
use crate::contracts::staking_vault::msgs::CoffeeStakingVaultUpdateContractMsg;
use crate::contracts::staking_vault::storage::CoffeeStakingVaultStorage;
use crate::contracts::vault_extra::msgs::CoffeeCreatePoolExtraMsg;
use crate::contracts::vault_extra::msgs::CoffeeDepositLiquidityExtraMsg;
use crate::contracts::vault_extra::msgs::CoffeeSwapExtraMsg;
use crate::contracts::vault_extra::storage::CoffeeVaultExtraStorage;
use crate::contracts::vault_jetton::msgs::CoffeeActivateVaultInternalMsg;
use crate::contracts::vault_jetton::msgs::CoffeeCreatePoolJettonMsg;
use crate::contracts::vault_jetton::msgs::CoffeeCreateVaultInternalMsg;
use crate::contracts::vault_jetton::msgs::CoffeeDepositLiquidityJettonMsg;
use crate::contracts::vault_jetton::msgs::CoffeeJettonTransferMsg;
use crate::contracts::vault_jetton::msgs::CoffeeProvideWalletAddressMsg;
use crate::contracts::vault_jetton::msgs::CoffeeSwapJettonMsg;
use crate::contracts::vault_jetton::msgs::CoffeeTakeWalletAddressMsg;
use crate::contracts::vault_jetton::msgs::CoffeeTransferNotificationMsg;
use crate::contracts::vault_jetton::storage::CoffeeVaultJettonActiveStorage;
use crate::contracts::vault_jetton::storage::CoffeeVaultJettonInactiveStorage;
use crate::contracts::vault_native::msgs::CoffeeComplexDepositLiquidityCondition;
use crate::contracts::vault_native::msgs::CoffeeConstantProductAmm;
use crate::contracts::vault_native::msgs::CoffeeContractUpdate;
use crate::contracts::vault_native::msgs::CoffeeCreateLiquidityDepositoryRequestMsg;
use crate::contracts::vault_native::msgs::CoffeeCreatePoolCreatorRequestMsg;
use crate::contracts::vault_native::msgs::CoffeeCreatePoolNativeMsg;
use crate::contracts::vault_native::msgs::CoffeeCurveFiStableAmm;
use crate::contracts::vault_native::msgs::CoffeeDepositLiquidityNativeMsg;
use crate::contracts::vault_native::msgs::CoffeeDepositLiquidityParams;
use crate::contracts::vault_native::msgs::CoffeeDepositLiquidityParamsTrimmed;
use crate::contracts::vault_native::msgs::CoffeeExcessesMsg;
use crate::contracts::vault_native::msgs::CoffeeLpQuantityDepositLiquidityCondition;
use crate::contracts::vault_native::msgs::CoffeeNoDepositLiquidityCondition;
use crate::contracts::vault_native::msgs::CoffeeNoPreviousAssetHint;
use crate::contracts::vault_native::msgs::CoffeeNotificationData;
use crate::contracts::vault_native::msgs::CoffeeNotificationDataSingle;
use crate::contracts::vault_native::msgs::CoffeeNotificationMsg;
use crate::contracts::vault_native::msgs::CoffeePayoutInternalMsg;
use crate::contracts::vault_native::msgs::CoffeePayoutMsg;
use crate::contracts::vault_native::msgs::CoffeePoolCreationParams;
use crate::contracts::vault_native::msgs::CoffeePoolParams;
use crate::contracts::vault_native::msgs::CoffeePrivatePoolCreationParams;
use crate::contracts::vault_native::msgs::CoffeePublicPoolCreationParams;
use crate::contracts::vault_native::msgs::CoffeeReservesRatioDepositLiquidityCondition;
use crate::contracts::vault_native::msgs::CoffeeSomePreviousAssetHint;
use crate::contracts::vault_native::msgs::CoffeeSwapInternalMsg;
use crate::contracts::vault_native::msgs::CoffeeSwapNativeMsg;
use crate::contracts::vault_native::msgs::CoffeeSwapParams;
use crate::contracts::vault_native::msgs::CoffeeSwapStepInternalParams;
use crate::contracts::vault_native::msgs::CoffeeSwapStepParams;
use crate::contracts::vault_native::msgs::CoffeeUpdateContractInternalMsg;
use crate::contracts::vault_native::storage::CoffeeExtraAsset;
use crate::contracts::vault_native::storage::CoffeeJettonAsset;
use crate::contracts::vault_native::storage::CoffeeNativeAsset;
use crate::contracts::vault_native::storage::CoffeeVaultInitData;
use crate::contracts::vault_native::storage::CoffeeVaultNativeStorage;
use fastnum::U256;
use serde::Deserialize;
use std::collections::BTreeSet;
use ton::ton_core::cell::TonHash;
use ton::ton_core::types::tlb_core::TLBCoins;

use ton::ton_core::cell::TonCell;

use ton::ton_core::traits::tlb::TLB;
#[derive(Deserialize)]
struct CellVector {
    bits: String,
    refs: Vec<CellVector>,
}
#[derive(Deserialize)]
struct Vector {
    id: String,
    cell: CellVector,
}
#[derive(Deserialize)]
struct Vectors {
    vectors: Vec<Vector>,
}
fn build_cell(v: &CellVector) -> anyhow::Result<TonCell> {
    let mut b = TonCell::builder();
    for bit in v.bits.chars() {
        match bit {
            '0' => b.write_bit(false)?,
            '1' => b.write_bit(true)?,
            _ => anyhow::bail!("invalid fixture bit"),
        }
    }
    for child in &v.refs {
        b.write_ref(build_cell(child)?)?;
    }
    Ok(b.build()?)
}
fn with_trailing_bit(v: &TonCell) -> anyhow::Result<TonCell> {
    let mut b = TonCell::builder();
    b.write_cell(v)?;
    b.write_bit(true)?;
    Ok(b.build()?)
}
#[test]
fn test_independent_schema_wire_vectors() -> anyhow::Result<()> {
    let fixtures: Vectors = serde_json::from_str(include_str!("fixtures/wire-vectors.json"))?;
    anyhow::ensure!(fixtures.vectors.len() == 900, "wire corpus must cover all 900 cases");
    let mut seen = BTreeSet::new();
    for fixture in fixtures.vectors {
        anyhow::ensure!(seen.insert(fixture.id.clone()), "duplicate wire fixture: {}", fixture.id);
        let input = build_cell(&fixture.cell)?;
        match fixture.id.as_str() {
            "cross_dex/CoffeeCrossDexStorage/0" => {
                let decoded = CoffeeCrossDexStorage::from_cell(&input)?;
                assert!(decoded.admin_address.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexStorage::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex/CoffeeCrossDexStorage/1" => {
                let decoded = CoffeeCrossDexStorage::from_cell(&input)?;
                assert!(decoded.admin_address.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexStorage::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex/CoffeeCrossDexJettonTransferNotification/0" => {
                let decoded = CoffeeCrossDexJettonTransferNotificationMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert!(decoded.sender.is_none());
                assert_eq!(decoded.to_cell()?, input);
            },
            "cross_dex/CoffeeCrossDexJettonTransferNotification/1" => {
                let decoded = CoffeeCrossDexJettonTransferNotificationMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert!(decoded.sender.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexJettonTransferNotificationMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex/CoffeeCrossDexDedustPayout/0" => {
                let decoded = CoffeeCrossDexDedustPayoutMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert!(decoded.payload.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexDedustPayoutMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex/CoffeeCrossDexDedustPayout/1" => {
                let decoded = CoffeeCrossDexDedustPayoutMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert!(decoded.payload.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexDedustPayoutMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex/CoffeeCrossDexPtonTonTransfer/0" => {
                let decoded = CoffeeCrossDexPtonTonTransferMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.ton_amount, TLBCoins::new(37));
                assert!(decoded.refund_address.is_none());
                assert!(decoded.forward_payload.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexPtonTonTransferMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex/CoffeeCrossDexPtonTonTransfer/1" => {
                let decoded = CoffeeCrossDexPtonTonTransferMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.ton_amount, TLBCoins::new(37));
                assert!(decoded.refund_address.is_some());
                assert!(decoded.forward_payload.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexPtonTonTransferMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex/CoffeeCrossDexCoffeeNotification/0" => {
                let decoded = CoffeeCrossDexCoffeeNotificationMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexCoffeeNotificationMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex/CoffeeCrossDexCoffeeNotification/1" => {
                let decoded = CoffeeCrossDexCoffeeNotificationMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexCoffeeNotificationMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex/CoffeeCrossDexBidaskNativeTransferNotification/0" => {
                let decoded = CoffeeCrossDexBidaskNativeTransferNotificationMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.native_amount, TLBCoins::new(37));
                assert!(decoded.forward_payload.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    CoffeeCrossDexBidaskNativeTransferNotificationMsg::from_cell(&with_trailing_bit(&input)?).is_ok()
                );
            },
            "cross_dex/CoffeeCrossDexBidaskNativeTransferNotification/1" => {
                let decoded = CoffeeCrossDexBidaskNativeTransferNotificationMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.native_amount, TLBCoins::new(37));
                assert!(decoded.forward_payload.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    CoffeeCrossDexBidaskNativeTransferNotificationMsg::from_cell(&with_trailing_bit(&input)?).is_ok()
                );
            },
            "cross_dex/CoffeeCrossDexWrappedPayload/0" => {
                let decoded = CoffeeCrossDexWrappedPayloadMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert!(decoded.payload.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexWrappedPayloadMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex/CoffeeCrossDexWrappedPayload/1" => {
                let decoded = CoffeeCrossDexWrappedPayloadMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert!(decoded.payload.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexWrappedPayloadMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex/CoffeeCrossDexResend/0" => {
                let decoded = CoffeeCrossDexResendMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.next_recipient.workchain, -1);
                assert_eq!(decoded.next_recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.total_gas, TLBCoins::new(37));
                assert_eq!(decoded.next_gas, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexResendMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex/CoffeeCrossDexResend/1" => {
                let decoded = CoffeeCrossDexResendMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.next_recipient.workchain, -1);
                assert_eq!(decoded.next_recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.total_gas, TLBCoins::new(37));
                assert_eq!(decoded.next_gas, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexResendMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex/CoffeeCrossDexResendJettonPayload/0" => {
                let decoded = CoffeeCrossDexResendJettonPayloadMsg::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexResendJettonPayloadMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex/CoffeeCrossDexResendJettonPayload/1" => {
                let decoded = CoffeeCrossDexResendJettonPayloadMsg::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexResendJettonPayloadMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex/CoffeeCrossDexFailurePayload/0" => {
                let decoded = CoffeeCrossDexFailurePayloadMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexFailurePayloadMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex/CoffeeCrossDexFailurePayload/1" => {
                let decoded = CoffeeCrossDexFailurePayloadMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexFailurePayloadMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex/CoffeeCrossDexSetAdmin/0" => {
                let decoded = CoffeeCrossDexSetAdminMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert!(decoded.admin_address.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexSetAdminMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex/CoffeeCrossDexSetAdmin/1" => {
                let decoded = CoffeeCrossDexSetAdminMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert!(decoded.admin_address.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexSetAdminMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex/CoffeeCrossDexUpdateContract/0" => {
                let decoded = CoffeeCrossDexUpdateContractMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert!(decoded.code.is_none());
                assert!(decoded.data.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexUpdateContractMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex/CoffeeCrossDexUpdateContract/1" => {
                let decoded = CoffeeCrossDexUpdateContractMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert!(decoded.code.is_some());
                assert!(decoded.data.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexUpdateContractMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex/CoffeeCrossDexExcesses/0" => {
                let decoded = CoffeeCrossDexExcessesMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexExcessesMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex/CoffeeCrossDexExcesses/1" => {
                let decoded = CoffeeCrossDexExcessesMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexExcessesMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex/CoffeeCrossDexJettonTransfer/0" => {
                let decoded = CoffeeCrossDexJettonTransferMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.destination.workchain, -1);
                assert_eq!(decoded.destination.hash.as_slice()[31], 37);
                assert!(decoded.response_destination.is_none());
                assert!(decoded.custom_payload.is_none());
                assert_eq!(decoded.forward_ton_amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
            },
            "cross_dex/CoffeeCrossDexJettonTransfer/1" => {
                let decoded = CoffeeCrossDexJettonTransferMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.destination.workchain, -1);
                assert_eq!(decoded.destination.hash.as_slice()[31], 37);
                assert!(decoded.response_destination.is_some());
                assert!(decoded.custom_payload.is_some());
                assert_eq!(decoded.forward_ton_amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexJettonTransferMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex/CoffeeCrossDexDeploymentStorage/0" => {
                let decoded = CoffeeCrossDexDeploymentStorage::from_cell(&input)?;
                assert!(decoded.admin_address.is_none());
                assert_eq!(decoded.deployment_discriminator, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexDeploymentStorage::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex/CoffeeCrossDexDeploymentStorage/1" => {
                let decoded = CoffeeCrossDexDeploymentStorage::from_cell(&input)?;
                assert!(decoded.admin_address.is_some());
                assert_eq!(decoded.deployment_discriminator, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexDeploymentStorage::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex/CoffeeCrossDexNativeRescue/0" => {
                let decoded = CoffeeCrossDexNativeRescueMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexNativeRescueMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex/CoffeeCrossDexNativeRescue/1" => {
                let decoded = CoffeeCrossDexNativeRescueMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexNativeRescueMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex/CoffeeCrossDexJettonRescue/0" => {
                let decoded = CoffeeCrossDexJettonRescueMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.jetton_wallet.workchain, -1);
                assert_eq!(decoded.jetton_wallet.hash.as_slice()[31], 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexJettonRescueMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex/CoffeeCrossDexJettonRescue/1" => {
                let decoded = CoffeeCrossDexJettonRescueMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.jetton_wallet.workchain, -1);
                assert_eq!(decoded.jetton_wallet.hash.as_slice()[31], 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexJettonRescueMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex_bidask/CoffeeCrossDexBidaskStorage/0" => {
                let decoded = CoffeeCrossDexBidaskStorage::from_cell(&input)?;
                assert!(decoded.admin_address.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexBidaskStorage::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex_bidask/CoffeeCrossDexBidaskStorage/1" => {
                let decoded = CoffeeCrossDexBidaskStorage::from_cell(&input)?;
                assert!(decoded.admin_address.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexBidaskStorage::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex_bidask/CoffeeCrossDexBidaskDeploymentStorage/0" => {
                let decoded = CoffeeCrossDexBidaskDeploymentStorage::from_cell(&input)?;
                assert!(decoded.admin_address.is_none());
                assert_eq!(decoded.deployment_discriminator, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexBidaskDeploymentStorage::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex_bidask/CoffeeCrossDexBidaskDeploymentStorage/1" => {
                let decoded = CoffeeCrossDexBidaskDeploymentStorage::from_cell(&input)?;
                assert!(decoded.admin_address.is_some());
                assert_eq!(decoded.deployment_discriminator, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexBidaskDeploymentStorage::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex_bidask/CoffeeCrossDexBidaskJettonTransferNotification/0" => {
                let decoded = CoffeeCrossDexJettonTransferNotificationMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert!(decoded.sender.is_none());
                assert_eq!(decoded.to_cell()?, input);
            },
            "cross_dex_bidask/CoffeeCrossDexBidaskJettonTransferNotification/1" => {
                let decoded = CoffeeCrossDexJettonTransferNotificationMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert!(decoded.sender.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexJettonTransferNotificationMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex_bidask/CoffeeCrossDexBidaskDedustPayout/0" => {
                let decoded = CoffeeCrossDexBidaskDedustPayoutMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert!(decoded.payload.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexBidaskDedustPayoutMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex_bidask/CoffeeCrossDexBidaskDedustPayout/1" => {
                let decoded = CoffeeCrossDexBidaskDedustPayoutMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert!(decoded.payload.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexBidaskDedustPayoutMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex_bidask/CoffeeCrossDexBidaskPtonTonTransfer/0" => {
                let decoded = CoffeeCrossDexBidaskPtonTonTransferMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.ton_amount, TLBCoins::new(37));
                assert!(decoded.refund_address.is_none());
                assert!(decoded.forward_payload.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexBidaskPtonTonTransferMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex_bidask/CoffeeCrossDexBidaskPtonTonTransfer/1" => {
                let decoded = CoffeeCrossDexBidaskPtonTonTransferMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.ton_amount, TLBCoins::new(37));
                assert!(decoded.refund_address.is_some());
                assert!(decoded.forward_payload.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexBidaskPtonTonTransferMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex_bidask/CoffeeCrossDexBidaskCoffeeNotification/0" => {
                let decoded = CoffeeCrossDexBidaskCoffeeNotificationMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexBidaskCoffeeNotificationMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex_bidask/CoffeeCrossDexBidaskCoffeeNotification/1" => {
                let decoded = CoffeeCrossDexBidaskCoffeeNotificationMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexBidaskCoffeeNotificationMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex_bidask/CoffeeCrossDexBidaskBidaskNativeTransferNotification/0" => {
                let decoded = CoffeeCrossDexBidaskBidaskNativeTransferNotificationMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.native_amount, TLBCoins::new(37));
                assert!(decoded.forward_payload.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    CoffeeCrossDexBidaskBidaskNativeTransferNotificationMsg::from_cell(&with_trailing_bit(&input)?)
                        .is_ok()
                );
            },
            "cross_dex_bidask/CoffeeCrossDexBidaskBidaskNativeTransferNotification/1" => {
                let decoded = CoffeeCrossDexBidaskBidaskNativeTransferNotificationMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.native_amount, TLBCoins::new(37));
                assert!(decoded.forward_payload.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    CoffeeCrossDexBidaskBidaskNativeTransferNotificationMsg::from_cell(&with_trailing_bit(&input)?)
                        .is_ok()
                );
            },
            "cross_dex_bidask/CoffeeCrossDexBidaskResend/0" => {
                let decoded = CoffeeCrossDexBidaskResendMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.next_recipient.workchain, -1);
                assert_eq!(decoded.next_recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.total_gas, TLBCoins::new(37));
                assert_eq!(decoded.next_gas, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexBidaskResendMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex_bidask/CoffeeCrossDexBidaskResend/1" => {
                let decoded = CoffeeCrossDexBidaskResendMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.next_recipient.workchain, -1);
                assert_eq!(decoded.next_recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.total_gas, TLBCoins::new(37));
                assert_eq!(decoded.next_gas, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexBidaskResendMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex_bidask/CoffeeCrossDexBidaskResendJettonPayload/0" => {
                let decoded = CoffeeCrossDexBidaskResendJettonPayloadMsg::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexBidaskResendJettonPayloadMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex_bidask/CoffeeCrossDexBidaskResendJettonPayload/1" => {
                let decoded = CoffeeCrossDexBidaskResendJettonPayloadMsg::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexBidaskResendJettonPayloadMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex_bidask/CoffeeCrossDexBidaskFailurePayload/0" => {
                let decoded = CoffeeCrossDexBidaskFailurePayloadMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexBidaskFailurePayloadMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex_bidask/CoffeeCrossDexBidaskFailurePayload/1" => {
                let decoded = CoffeeCrossDexBidaskFailurePayloadMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexBidaskFailurePayloadMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex_bidask/CoffeeCrossDexBidaskSetAdmin/0" => {
                let decoded = CoffeeCrossDexBidaskSetAdminMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert!(decoded.admin_address.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexBidaskSetAdminMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex_bidask/CoffeeCrossDexBidaskSetAdmin/1" => {
                let decoded = CoffeeCrossDexBidaskSetAdminMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert!(decoded.admin_address.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexBidaskSetAdminMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex_bidask/CoffeeCrossDexBidaskUpdateContract/0" => {
                let decoded = CoffeeCrossDexBidaskUpdateContractMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert!(decoded.code.is_none());
                assert!(decoded.data.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexBidaskUpdateContractMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex_bidask/CoffeeCrossDexBidaskUpdateContract/1" => {
                let decoded = CoffeeCrossDexBidaskUpdateContractMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert!(decoded.code.is_some());
                assert!(decoded.data.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexBidaskUpdateContractMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex_bidask/CoffeeCrossDexBidaskWithdrawTon/0" => {
                let decoded = CoffeeCrossDexBidaskWithdrawTonMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexBidaskWithdrawTonMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex_bidask/CoffeeCrossDexBidaskWithdrawTon/1" => {
                let decoded = CoffeeCrossDexBidaskWithdrawTonMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexBidaskWithdrawTonMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex_bidask/CoffeeCrossDexBidaskWithdrawJettons/0" => {
                let decoded = CoffeeCrossDexBidaskWithdrawJettonsMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.jetton_wallet.workchain, -1);
                assert_eq!(decoded.jetton_wallet.hash.as_slice()[31], 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexBidaskWithdrawJettonsMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex_bidask/CoffeeCrossDexBidaskWithdrawJettons/1" => {
                let decoded = CoffeeCrossDexBidaskWithdrawJettonsMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.jetton_wallet.workchain, -1);
                assert_eq!(decoded.jetton_wallet.hash.as_slice()[31], 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexBidaskWithdrawJettonsMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex_bidask/CoffeeCrossDexBidaskExcesses/0" => {
                let decoded = CoffeeCrossDexBidaskExcessesMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexBidaskExcessesMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex_bidask/CoffeeCrossDexBidaskExcesses/1" => {
                let decoded = CoffeeCrossDexBidaskExcessesMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexBidaskExcessesMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex_bidask/CoffeeCrossDexBidaskJettonTransfer/0" => {
                let decoded = CoffeeCrossDexBidaskJettonTransferMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert!(decoded.destination.is_none());
                assert!(decoded.response_destination.is_none());
                assert!(decoded.custom_payload.is_none());
                assert_eq!(decoded.forward_ton_amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
            },
            "cross_dex_bidask/CoffeeCrossDexBidaskJettonTransfer/1" => {
                let decoded = CoffeeCrossDexBidaskJettonTransferMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert!(decoded.destination.is_some());
                assert!(decoded.response_destination.is_some());
                assert!(decoded.custom_payload.is_some());
                assert_eq!(decoded.forward_ton_amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexBidaskJettonTransferMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex_legacy/CoffeeCrossDexLegacyStorage/0" => {
                let decoded = CoffeeCrossDexLegacyStorage::from_cell(&input)?;
                assert!(decoded.admin_address.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexLegacyStorage::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex_legacy/CoffeeCrossDexLegacyStorage/1" => {
                let decoded = CoffeeCrossDexLegacyStorage::from_cell(&input)?;
                assert!(decoded.admin_address.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexLegacyStorage::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex_legacy/CoffeeCrossDexLegacyDeploymentStorage/0" => {
                let decoded = CoffeeCrossDexLegacyDeploymentStorage::from_cell(&input)?;
                assert!(decoded.admin_address.is_none());
                assert_eq!(decoded.deployment_discriminator, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexLegacyDeploymentStorage::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex_legacy/CoffeeCrossDexLegacyDeploymentStorage/1" => {
                let decoded = CoffeeCrossDexLegacyDeploymentStorage::from_cell(&input)?;
                assert!(decoded.admin_address.is_some());
                assert_eq!(decoded.deployment_discriminator, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexLegacyDeploymentStorage::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex_legacy/CoffeeCrossDexLegacyJettonTransferNotification/0" => {
                let decoded = CoffeeCrossDexJettonTransferNotificationMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert!(decoded.sender.is_none());
                assert_eq!(decoded.to_cell()?, input);
            },
            "cross_dex_legacy/CoffeeCrossDexLegacyJettonTransferNotification/1" => {
                let decoded = CoffeeCrossDexJettonTransferNotificationMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert!(decoded.sender.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexJettonTransferNotificationMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex_legacy/CoffeeCrossDexLegacyDedustPayout/0" => {
                let decoded = CoffeeCrossDexLegacyDedustPayoutMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert!(decoded.payload.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexLegacyDedustPayoutMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex_legacy/CoffeeCrossDexLegacyDedustPayout/1" => {
                let decoded = CoffeeCrossDexLegacyDedustPayoutMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert!(decoded.payload.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexLegacyDedustPayoutMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex_legacy/CoffeeCrossDexLegacyPtonTonTransfer/0" => {
                let decoded = CoffeeCrossDexLegacyPtonTonTransferMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.ton_amount, TLBCoins::new(37));
                assert!(decoded.refund_address.is_none());
                assert!(decoded.forward_payload.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexLegacyPtonTonTransferMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex_legacy/CoffeeCrossDexLegacyPtonTonTransfer/1" => {
                let decoded = CoffeeCrossDexLegacyPtonTonTransferMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.ton_amount, TLBCoins::new(37));
                assert!(decoded.refund_address.is_some());
                assert!(decoded.forward_payload.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexLegacyPtonTonTransferMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex_legacy/CoffeeCrossDexLegacyResend/0" => {
                let decoded = CoffeeCrossDexLegacyResendMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.next_recipient.workchain, -1);
                assert_eq!(decoded.next_recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.total_gas, TLBCoins::new(37));
                assert_eq!(decoded.next_gas, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexLegacyResendMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex_legacy/CoffeeCrossDexLegacyResend/1" => {
                let decoded = CoffeeCrossDexLegacyResendMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.next_recipient.workchain, -1);
                assert_eq!(decoded.next_recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.total_gas, TLBCoins::new(37));
                assert_eq!(decoded.next_gas, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexLegacyResendMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex_legacy/CoffeeCrossDexLegacyResendJettonPayload/0" => {
                let decoded = CoffeeCrossDexLegacyResendJettonPayloadMsg::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexLegacyResendJettonPayloadMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex_legacy/CoffeeCrossDexLegacyResendJettonPayload/1" => {
                let decoded = CoffeeCrossDexLegacyResendJettonPayloadMsg::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexLegacyResendJettonPayloadMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex_legacy/CoffeeCrossDexLegacyFailurePayload/0" => {
                let decoded = CoffeeCrossDexLegacyFailurePayloadMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexLegacyFailurePayloadMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex_legacy/CoffeeCrossDexLegacyFailurePayload/1" => {
                let decoded = CoffeeCrossDexLegacyFailurePayloadMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexLegacyFailurePayloadMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex_legacy/CoffeeCrossDexLegacySetAdmin/0" => {
                let decoded = CoffeeCrossDexLegacySetAdminMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert!(decoded.admin_address.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexLegacySetAdminMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex_legacy/CoffeeCrossDexLegacySetAdmin/1" => {
                let decoded = CoffeeCrossDexLegacySetAdminMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert!(decoded.admin_address.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexLegacySetAdminMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex_legacy/CoffeeCrossDexLegacyUpdateContract/0" => {
                let decoded = CoffeeCrossDexLegacyUpdateContractMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert!(decoded.code.is_none());
                assert!(decoded.data.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexLegacyUpdateContractMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex_legacy/CoffeeCrossDexLegacyUpdateContract/1" => {
                let decoded = CoffeeCrossDexLegacyUpdateContractMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert!(decoded.code.is_some());
                assert!(decoded.data.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexLegacyUpdateContractMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex_legacy/CoffeeCrossDexLegacyWithdrawTon/0" => {
                let decoded = CoffeeCrossDexLegacyWithdrawTonMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexLegacyWithdrawTonMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex_legacy/CoffeeCrossDexLegacyWithdrawTon/1" => {
                let decoded = CoffeeCrossDexLegacyWithdrawTonMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexLegacyWithdrawTonMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex_legacy/CoffeeCrossDexLegacyWithdrawJettons/0" => {
                let decoded = CoffeeCrossDexLegacyWithdrawJettonsMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.jetton_wallet.workchain, -1);
                assert_eq!(decoded.jetton_wallet.hash.as_slice()[31], 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexLegacyWithdrawJettonsMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex_legacy/CoffeeCrossDexLegacyWithdrawJettons/1" => {
                let decoded = CoffeeCrossDexLegacyWithdrawJettonsMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.jetton_wallet.workchain, -1);
                assert_eq!(decoded.jetton_wallet.hash.as_slice()[31], 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexLegacyWithdrawJettonsMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex_legacy/CoffeeCrossDexLegacyExcesses/0" => {
                let decoded = CoffeeCrossDexLegacyExcessesMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexLegacyExcessesMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex_legacy/CoffeeCrossDexLegacyExcesses/1" => {
                let decoded = CoffeeCrossDexLegacyExcessesMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexLegacyExcessesMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "cross_dex_legacy/CoffeeCrossDexLegacyJettonTransfer/0" => {
                let decoded = CoffeeCrossDexLegacyJettonTransferMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert!(decoded.destination.is_none());
                assert!(decoded.response_destination.is_none());
                assert!(decoded.custom_payload.is_none());
                assert_eq!(decoded.forward_ton_amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
            },
            "cross_dex_legacy/CoffeeCrossDexLegacyJettonTransfer/1" => {
                let decoded = CoffeeCrossDexLegacyJettonTransferMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert!(decoded.destination.is_some());
                assert!(decoded.response_destination.is_some());
                assert!(decoded.custom_payload.is_some());
                assert_eq!(decoded.forward_ton_amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexLegacyJettonTransferMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryStorage/0" => {
                let decoded = CoffeeFactoryStorage::from_cell(&input)?;
                assert!(decoded.admin_address.is_none());
                assert!(decoded.withdrawer_address.is_none());
                assert_eq!(CoffeeFactoryStorage::from_cell(&decoded.to_cell()?)?, decoded);
            },
            "factory/CoffeeFactoryStorage/1" => {
                let decoded = CoffeeFactoryStorage::from_cell(&input)?;
                assert!(decoded.admin_address.is_some());
                assert!(decoded.withdrawer_address.is_some());
                assert_eq!(CoffeeFactoryStorage::from_cell(&decoded.to_cell()?)?, decoded);
            },
            "factory/CoffeeFactoryCodeCells/0" => {
                let decoded = CoffeeFactoryCodeCells::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryCodeCells::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryCodeCells/1" => {
                let decoded = CoffeeFactoryCodeCells::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryCodeCells::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryCodeDictionaries/0" => {
                let decoded = CoffeeFactoryCodeDictionaries::from_cell(&input)?;
                assert_eq!(decoded.vault_code_by_asset_type.len(), 0);
                assert_eq!(decoded.pool_code_by_amm.len(), 0);
                assert_eq!(CoffeeFactoryCodeDictionaries::from_cell(&decoded.to_cell()?)?, decoded);
                assert!(CoffeeFactoryCodeDictionaries::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryCodeDictionaries/1" => {
                let decoded = CoffeeFactoryCodeDictionaries::from_cell(&input)?;
                assert_eq!(decoded.vault_code_by_asset_type.len(), 1);
                assert!(decoded.vault_code_by_asset_type.contains_key(&0));
                assert_eq!(decoded.pool_code_by_amm.len(), 1);
                assert!(decoded.pool_code_by_amm.contains_key(&0));
                assert_eq!(CoffeeFactoryCodeDictionaries::from_cell(&decoded.to_cell()?)?, decoded);
                assert!(CoffeeFactoryCodeDictionaries::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryNativeAsset/0" => {
                let decoded = CoffeeFactoryNativeAsset::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryNativeAsset::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryNativeAsset/1" => {
                let decoded = CoffeeFactoryNativeAsset::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryNativeAsset::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryJettonAsset/0" => {
                let decoded = CoffeeFactoryJettonAsset::from_cell(&input)?;
                assert_eq!(decoded.chain, 37);
                assert_eq!(decoded.hash, U256::from(37u8));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryJettonAsset::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryJettonAsset/1" => {
                let decoded = CoffeeFactoryJettonAsset::from_cell(&input)?;
                assert_eq!(decoded.chain, 37);
                assert_eq!(decoded.hash, U256::from(37u8));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryJettonAsset::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryExtraAsset/0" => {
                let decoded = CoffeeFactoryExtraAsset::from_cell(&input)?;
                assert_eq!(decoded.id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryExtraAsset::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryExtraAsset/1" => {
                let decoded = CoffeeFactoryExtraAsset::from_cell(&input)?;
                assert_eq!(decoded.id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryExtraAsset::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryConstantProductAmm/0" => {
                let decoded = CoffeeFactoryConstantProductAmm::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryConstantProductAmm::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryConstantProductAmm/1" => {
                let decoded = CoffeeFactoryConstantProductAmm::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryConstantProductAmm::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryCurveFiStableAmm/0" => {
                let decoded = CoffeeFactoryCurveFiStableAmm::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryCurveFiStableAmm::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryCurveFiStableAmm/1" => {
                let decoded = CoffeeFactoryCurveFiStableAmm::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryCurveFiStableAmm::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryPoolParams/0" => {
                let decoded = CoffeeFactoryPoolParams::from_cell(&input)?;
                assert!(decoded.amm_settings.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryPoolParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryPoolParams/1" => {
                let decoded = CoffeeFactoryPoolParams::from_cell(&input)?;
                assert!(decoded.amm_settings.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryPoolParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryPublicPoolCreationParams/0" => {
                let decoded = CoffeeFactoryPublicPoolCreationParams::from_cell(&input)?;
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.use_recipient_on_failure, -1);
                assert!(decoded.notification_data.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryPublicPoolCreationParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryPublicPoolCreationParams/1" => {
                let decoded = CoffeeFactoryPublicPoolCreationParams::from_cell(&input)?;
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.use_recipient_on_failure, -1);
                assert!(decoded.notification_data.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryPublicPoolCreationParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryPrivatePoolCreationParams/0" => {
                let decoded = CoffeeFactoryPrivatePoolCreationParams::from_cell(&input)?;
                assert_eq!(decoded.is_active, 1);
                assert!(decoded.extra_settings.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryPrivatePoolCreationParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryPrivatePoolCreationParams/1" => {
                let decoded = CoffeeFactoryPrivatePoolCreationParams::from_cell(&input)?;
                assert_eq!(decoded.is_active, 1);
                assert!(decoded.extra_settings.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryPrivatePoolCreationParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryPoolCreationParams/0" => {
                let decoded = CoffeeFactoryPoolCreationParams::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryPoolCreationParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryPoolCreationParams/1" => {
                let decoded = CoffeeFactoryPoolCreationParams::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryPoolCreationParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryNotificationDataSingle/0" => {
                let decoded = CoffeeFactoryNotificationDataSingle::from_cell(&input)?;
                assert_eq!(decoded.receiver.workchain, -1);
                assert_eq!(decoded.receiver.hash.as_slice()[31], 37);
                assert_eq!(decoded.fwd_gas, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryNotificationDataSingle::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryNotificationDataSingle/1" => {
                let decoded = CoffeeFactoryNotificationDataSingle::from_cell(&input)?;
                assert_eq!(decoded.receiver.workchain, -1);
                assert_eq!(decoded.receiver.hash.as_slice()[31], 37);
                assert_eq!(decoded.fwd_gas, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryNotificationDataSingle::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryNotificationData/0" => {
                let decoded = CoffeeFactoryNotificationData::from_cell(&input)?;
                assert!(decoded.on_success.is_none());
                assert!(decoded.on_failure.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryNotificationData::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryNotificationData/1" => {
                let decoded = CoffeeFactoryNotificationData::from_cell(&input)?;
                assert!(decoded.on_success.is_some());
                assert!(decoded.on_failure.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryNotificationData::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryDepositNoCondition/0" => {
                let decoded = CoffeeFactoryDepositNoCondition::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryDepositNoCondition::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryDepositNoCondition/1" => {
                let decoded = CoffeeFactoryDepositNoCondition::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryDepositNoCondition::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryDepositLpQuantityCondition/0" => {
                let decoded = CoffeeFactoryDepositLpQuantityCondition::from_cell(&input)?;
                assert_eq!(decoded.min_lp_amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryDepositLpQuantityCondition::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryDepositLpQuantityCondition/1" => {
                let decoded = CoffeeFactoryDepositLpQuantityCondition::from_cell(&input)?;
                assert_eq!(decoded.min_lp_amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryDepositLpQuantityCondition::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryDepositReservesRatioCondition/0" => {
                let decoded = CoffeeFactoryDepositReservesRatioCondition::from_cell(&input)?;
                assert_eq!(decoded.denominator, 37);
                assert_eq!(decoded.min_nominator, 37);
                assert_eq!(decoded.max_nominator, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryDepositReservesRatioCondition::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryDepositReservesRatioCondition/1" => {
                let decoded = CoffeeFactoryDepositReservesRatioCondition::from_cell(&input)?;
                assert_eq!(decoded.denominator, 37);
                assert_eq!(decoded.min_nominator, 37);
                assert_eq!(decoded.max_nominator, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryDepositReservesRatioCondition::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryDepositComplexCondition/0" => {
                let decoded = CoffeeFactoryDepositComplexCondition::from_cell(&input)?;
                assert_eq!(decoded.min_lp_amount, TLBCoins::new(37));
                assert_eq!(decoded.denominator, 37);
                assert_eq!(decoded.min_nominator, 37);
                assert_eq!(decoded.max_nominator, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryDepositComplexCondition::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryDepositComplexCondition/1" => {
                let decoded = CoffeeFactoryDepositComplexCondition::from_cell(&input)?;
                assert_eq!(decoded.min_lp_amount, TLBCoins::new(37));
                assert_eq!(decoded.denominator, 37);
                assert_eq!(decoded.min_nominator, 37);
                assert_eq!(decoded.max_nominator, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryDepositComplexCondition::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryDepositLiquidityParamsTrimmed/0" => {
                let decoded = CoffeeFactoryDepositLiquidityParamsTrimmed::from_cell(&input)?;
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.use_recipient_on_failure, -1);
                assert!(decoded.referral.is_none());
                assert_eq!(decoded.deadline, 37);
                assert!(decoded.extra_settings.is_none());
                assert!(decoded.notification_data.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryDepositLiquidityParamsTrimmed::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryDepositLiquidityParamsTrimmed/1" => {
                let decoded = CoffeeFactoryDepositLiquidityParamsTrimmed::from_cell(&input)?;
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.use_recipient_on_failure, -1);
                assert!(decoded.referral.is_some());
                assert_eq!(decoded.deadline, 37);
                assert!(decoded.extra_settings.is_some());
                assert!(decoded.notification_data.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryDepositLiquidityParamsTrimmed::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryDepositLiquidityParams/0" => {
                let decoded = CoffeeFactoryDepositLiquidityParams::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryDepositLiquidityParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryDepositLiquidityParams/1" => {
                let decoded = CoffeeFactoryDepositLiquidityParams::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryDepositLiquidityParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryContractUpdate/0" => {
                let decoded = CoffeeFactoryContractUpdate::from_cell(&input)?;
                assert!(decoded.code.is_none());
                assert!(decoded.data.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryContractUpdate::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryContractUpdate/1" => {
                let decoded = CoffeeFactoryContractUpdate::from_cell(&input)?;
                assert!(decoded.code.is_some());
                assert!(decoded.data.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryContractUpdate::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryPoolNoUpdate/0" => {
                let decoded = CoffeeFactoryPoolNoUpdate::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryPoolNoUpdate::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryPoolNoUpdate/1" => {
                let decoded = CoffeeFactoryPoolNoUpdate::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryPoolNoUpdate::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryPoolFeeUpdate/0" => {
                let decoded = CoffeeFactoryPoolFeeUpdate::from_cell(&input)?;
                assert_eq!(decoded.protocol_fee, 37);
                assert_eq!(decoded.lp_fee, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryPoolFeeUpdate::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryPoolFeeUpdate/1" => {
                let decoded = CoffeeFactoryPoolFeeUpdate::from_cell(&input)?;
                assert_eq!(decoded.protocol_fee, 37);
                assert_eq!(decoded.lp_fee, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryPoolFeeUpdate::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryPoolStatusUpdate/0" => {
                let decoded = CoffeeFactoryPoolStatusUpdate::from_cell(&input)?;
                assert_eq!(decoded.is_active, 1);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryPoolStatusUpdate::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryPoolStatusUpdate/1" => {
                let decoded = CoffeeFactoryPoolStatusUpdate::from_cell(&input)?;
                assert_eq!(decoded.is_active, 1);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryPoolStatusUpdate::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryPoolFeeAndStatusUpdate/0" => {
                let decoded = CoffeeFactoryPoolFeeAndStatusUpdate::from_cell(&input)?;
                assert_eq!(decoded.protocol_fee, 37);
                assert_eq!(decoded.lp_fee, 37);
                assert_eq!(decoded.is_active, 1);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryPoolFeeAndStatusUpdate::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryPoolFeeAndStatusUpdate/1" => {
                let decoded = CoffeeFactoryPoolFeeAndStatusUpdate::from_cell(&input)?;
                assert_eq!(decoded.protocol_fee, 37);
                assert_eq!(decoded.lp_fee, 37);
                assert_eq!(decoded.is_active, 1);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryPoolFeeAndStatusUpdate::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryVaultContractType/0" => {
                let decoded = CoffeeFactoryVaultContractType::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryVaultContractType::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryVaultContractType/1" => {
                let decoded = CoffeeFactoryVaultContractType::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryVaultContractType::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryPoolContractType/0" => {
                let decoded = CoffeeFactoryPoolContractType::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryPoolContractType::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryPoolContractType/1" => {
                let decoded = CoffeeFactoryPoolContractType::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryPoolContractType::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryLiquidityDepositoryContractType/0" => {
                let decoded = CoffeeFactoryLiquidityDepositoryContractType::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryLiquidityDepositoryContractType::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryLiquidityDepositoryContractType/1" => {
                let decoded = CoffeeFactoryLiquidityDepositoryContractType::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryLiquidityDepositoryContractType::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryPoolCreatorContractType/0" => {
                let decoded = CoffeeFactoryPoolCreatorContractType::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryPoolCreatorContractType::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryPoolCreatorContractType/1" => {
                let decoded = CoffeeFactoryPoolCreatorContractType::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryPoolCreatorContractType::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryVaultProof/0" => {
                let decoded = CoffeeFactoryVaultProof::from_cell(&input)?;
                assert_eq!(decoded.factory_address.workchain, -1);
                assert_eq!(decoded.factory_address.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryVaultProof::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryVaultProof/1" => {
                let decoded = CoffeeFactoryVaultProof::from_cell(&input)?;
                assert_eq!(decoded.factory_address.workchain, -1);
                assert_eq!(decoded.factory_address.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryVaultProof::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryPoolCreatorProof/0" => {
                let decoded = CoffeeFactoryPoolCreatorProof::from_cell(&input)?;
                assert_eq!(decoded.factory_address.workchain, -1);
                assert_eq!(decoded.factory_address.hash.as_slice()[31], 37);
                assert_eq!(decoded.owner.workchain, -1);
                assert_eq!(decoded.owner.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryPoolCreatorProof::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryPoolCreatorProof/1" => {
                let decoded = CoffeeFactoryPoolCreatorProof::from_cell(&input)?;
                assert_eq!(decoded.factory_address.workchain, -1);
                assert_eq!(decoded.factory_address.hash.as_slice()[31], 37);
                assert_eq!(decoded.owner.workchain, -1);
                assert_eq!(decoded.owner.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryPoolCreatorProof::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryCreatePoolCreatorRequest/0" => {
                let decoded = CoffeeFactoryCreatePoolCreatorRequestMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.sender.workchain, -1);
                assert_eq!(decoded.sender.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryCreatePoolCreatorRequestMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryCreatePoolCreatorRequest/1" => {
                let decoded = CoffeeFactoryCreatePoolCreatorRequestMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.sender.workchain, -1);
                assert_eq!(decoded.sender.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryCreatePoolCreatorRequestMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryCreateLiquidityDepositoryRequest/0" => {
                let decoded = CoffeeFactoryCreateLiquidityDepositoryRequestMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.sender.workchain, -1);
                assert_eq!(decoded.sender.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    CoffeeFactoryCreateLiquidityDepositoryRequestMsg::from_cell(&with_trailing_bit(&input)?).is_ok()
                );
            },
            "factory/CoffeeFactoryCreateLiquidityDepositoryRequest/1" => {
                let decoded = CoffeeFactoryCreateLiquidityDepositoryRequestMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.sender.workchain, -1);
                assert_eq!(decoded.sender.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    CoffeeFactoryCreateLiquidityDepositoryRequestMsg::from_cell(&with_trailing_bit(&input)?).is_ok()
                );
            },
            "factory/CoffeeFactoryCreatePoolRequest/0" => {
                let decoded = CoffeeFactoryCreatePoolRequestMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount1, TLBCoins::new(37));
                assert_eq!(decoded.amount2, TLBCoins::new(37));
                assert_eq!(decoded.tx_initiator.workchain, -1);
                assert_eq!(decoded.tx_initiator.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryCreatePoolRequestMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryCreatePoolRequest/1" => {
                let decoded = CoffeeFactoryCreatePoolRequestMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount1, TLBCoins::new(37));
                assert_eq!(decoded.amount2, TLBCoins::new(37));
                assert_eq!(decoded.tx_initiator.workchain, -1);
                assert_eq!(decoded.tx_initiator.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryCreatePoolRequestMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryCreateVault/0" => {
                let decoded = CoffeeFactoryCreateVaultMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryCreateVaultMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryCreateVault/1" => {
                let decoded = CoffeeFactoryCreateVaultMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryCreateVaultMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryUpdateAdmin/0" => {
                let decoded = CoffeeFactoryUpdateAdminMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert!(decoded.address.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryUpdateAdminMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryUpdateAdmin/1" => {
                let decoded = CoffeeFactoryUpdateAdminMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert!(decoded.address.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryUpdateAdminMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryUpdateWithdrawer/0" => {
                let decoded = CoffeeFactoryUpdateWithdrawerMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert!(decoded.address.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryUpdateWithdrawerMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryUpdateWithdrawer/1" => {
                let decoded = CoffeeFactoryUpdateWithdrawerMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert!(decoded.address.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryUpdateWithdrawerMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryUpdateCodeCells/0" => {
                let decoded = CoffeeFactoryUpdateCodeCellsMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(CoffeeFactoryUpdateCodeCellsMsg::from_cell(&decoded.to_cell()?)?, decoded);
                assert!(CoffeeFactoryUpdateCodeCellsMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryUpdateCodeCells/1" => {
                let decoded = CoffeeFactoryUpdateCodeCellsMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(CoffeeFactoryUpdateCodeCellsMsg::from_cell(&decoded.to_cell()?)?, decoded);
                assert!(CoffeeFactoryUpdateCodeCellsMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryUpdateContract/0" => {
                let decoded = CoffeeFactoryUpdateContractMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert!(decoded.contract_address.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryUpdateContractMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryUpdateContract/1" => {
                let decoded = CoffeeFactoryUpdateContractMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert!(decoded.contract_address.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryUpdateContractMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryUpdatePool/0" => {
                let decoded = CoffeeFactoryUpdatePoolMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryUpdatePoolMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryUpdatePool/1" => {
                let decoded = CoffeeFactoryUpdatePoolMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryUpdatePoolMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryActivateVault/0" => {
                let decoded = CoffeeFactoryActivateVaultMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.wallet.workchain, -1);
                assert_eq!(decoded.wallet.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryActivateVaultMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryActivateVault/1" => {
                let decoded = CoffeeFactoryActivateVaultMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.wallet.workchain, -1);
                assert_eq!(decoded.wallet.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryActivateVaultMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryWithdraw/0" => {
                let decoded = CoffeeFactoryWithdrawMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.pool_address.workchain, -1);
                assert_eq!(decoded.pool_address.hash.as_slice()[31], 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.receiver.workchain, -1);
                assert_eq!(decoded.receiver.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryWithdrawMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryWithdraw/1" => {
                let decoded = CoffeeFactoryWithdrawMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.pool_address.workchain, -1);
                assert_eq!(decoded.pool_address.hash.as_slice()[31], 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.receiver.workchain, -1);
                assert_eq!(decoded.receiver.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryWithdrawMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryDeploy/0" => {
                let decoded = CoffeeFactoryDeployMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert!(decoded.action.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryDeployMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryDeploy/1" => {
                let decoded = CoffeeFactoryDeployMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert!(decoded.action.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryDeployMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryCreatePoolCreatorInternal/0" => {
                let decoded = CoffeeFactoryCreatePoolCreatorInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryCreatePoolCreatorInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryCreatePoolCreatorInternal/1" => {
                let decoded = CoffeeFactoryCreatePoolCreatorInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryCreatePoolCreatorInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryCreateLiquidityDepositoryInternal/0" => {
                let decoded = CoffeeFactoryCreateLiquidityDepositoryInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    CoffeeFactoryCreateLiquidityDepositoryInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok()
                );
            },
            "factory/CoffeeFactoryCreateLiquidityDepositoryInternal/1" => {
                let decoded = CoffeeFactoryCreateLiquidityDepositoryInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    CoffeeFactoryCreateLiquidityDepositoryInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok()
                );
            },
            "factory/CoffeeFactoryCreatePoolInternal/0" => {
                let decoded = CoffeeFactoryCreatePoolInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount1, TLBCoins::new(37));
                assert_eq!(decoded.amount2, TLBCoins::new(37));
                assert_eq!(decoded.tx_initiator.workchain, -1);
                assert_eq!(decoded.tx_initiator.hash.as_slice()[31], 37);
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.use_recipient_on_failure, -1);
                assert!(decoded.extra_settings.is_none());
                assert!(decoded.notification_data.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryCreatePoolInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryCreatePoolInternal/1" => {
                let decoded = CoffeeFactoryCreatePoolInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount1, TLBCoins::new(37));
                assert_eq!(decoded.amount2, TLBCoins::new(37));
                assert_eq!(decoded.tx_initiator.workchain, -1);
                assert_eq!(decoded.tx_initiator.hash.as_slice()[31], 37);
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.use_recipient_on_failure, -1);
                assert!(decoded.extra_settings.is_some());
                assert!(decoded.notification_data.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryCreatePoolInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryCreateVaultInternal/0" => {
                let decoded = CoffeeFactoryCreateVaultInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryCreateVaultInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryCreateVaultInternal/1" => {
                let decoded = CoffeeFactoryCreateVaultInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryCreateVaultInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryUpdatePoolInternal/0" => {
                let decoded = CoffeeFactoryUpdatePoolInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.excesses_receiver.workchain, -1);
                assert_eq!(decoded.excesses_receiver.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryUpdatePoolInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryUpdatePoolInternal/1" => {
                let decoded = CoffeeFactoryUpdatePoolInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.excesses_receiver.workchain, -1);
                assert_eq!(decoded.excesses_receiver.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryUpdatePoolInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryActivateVaultInternal/0" => {
                let decoded = CoffeeFactoryActivateVaultInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.wallet.workchain, -1);
                assert_eq!(decoded.wallet.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryActivateVaultInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryActivateVaultInternal/1" => {
                let decoded = CoffeeFactoryActivateVaultInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.wallet.workchain, -1);
                assert_eq!(decoded.wallet.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryActivateVaultInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryWithdrawInternal/0" => {
                let decoded = CoffeeFactoryWithdrawInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.receiver.workchain, -1);
                assert_eq!(decoded.receiver.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryWithdrawInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryWithdrawInternal/1" => {
                let decoded = CoffeeFactoryWithdrawInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.receiver.workchain, -1);
                assert_eq!(decoded.receiver.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryWithdrawInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryUpdateContractInternal/0" => {
                let decoded = CoffeeFactoryUpdateContractInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.excesses_receiver.workchain, -1);
                assert_eq!(decoded.excesses_receiver.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryUpdateContractInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryUpdateContractInternal/1" => {
                let decoded = CoffeeFactoryUpdateContractInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.excesses_receiver.workchain, -1);
                assert_eq!(decoded.excesses_receiver.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryUpdateContractInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryExcesses/0" => {
                let decoded = CoffeeFactoryExcessesMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryExcessesMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "factory/CoffeeFactoryExcesses/1" => {
                let decoded = CoffeeFactoryExcessesMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeFactoryExcessesMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "init/CoffeeInitStorage/0" => {
                let decoded = CoffeeInitStorage::from_cell(&input)?;
                assert_eq!(decoded.factory_address.workchain, -1);
                assert_eq!(decoded.factory_address.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
            },
            "init/CoffeeInitStorage/1" => {
                let decoded = CoffeeInitStorage::from_cell(&input)?;
                assert_eq!(decoded.factory_address.workchain, -1);
                assert_eq!(decoded.factory_address.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
            },
            "init/CoffeeInitDeploy/0" => {
                let decoded = CoffeeInitDeployMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert!(decoded.action.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeInitDeployMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "init/CoffeeInitDeploy/1" => {
                let decoded = CoffeeInitDeployMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert!(decoded.action.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeInitDeployMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "liquidity_depository/CoffeeLiquidityDepositoryStorage/0" => {
                let decoded = CoffeeLiquidityDepositoryStorage::from_cell(&input)?;
                assert_eq!(decoded.amount1, TLBCoins::new(37));
                assert_eq!(decoded.amount2, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeLiquidityDepositoryStorage::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "liquidity_depository/CoffeeLiquidityDepositoryStorage/1" => {
                let decoded = CoffeeLiquidityDepositoryStorage::from_cell(&input)?;
                assert_eq!(decoded.amount1, TLBCoins::new(37));
                assert_eq!(decoded.amount2, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeLiquidityDepositoryStorage::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "liquidity_depository/CoffeeLiquidityDepositoryInitData/0" => {
                let decoded = CoffeeLiquidityDepositoryInitData::from_cell(&input)?;
                assert_eq!(decoded.factory_address.workchain, -1);
                assert_eq!(decoded.factory_address.hash.as_slice()[31], 37);
                assert_eq!(decoded.contract_type, 3);
                assert_eq!(decoded.owner_address.workchain, -1);
                assert_eq!(decoded.owner_address.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeLiquidityDepositoryInitData::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "liquidity_depository/CoffeeLiquidityDepositoryInitData/1" => {
                let decoded = CoffeeLiquidityDepositoryInitData::from_cell(&input)?;
                assert_eq!(decoded.factory_address.workchain, -1);
                assert_eq!(decoded.factory_address.hash.as_slice()[31], 37);
                assert_eq!(decoded.contract_type, 3);
                assert_eq!(decoded.owner_address.workchain, -1);
                assert_eq!(decoded.owner_address.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeLiquidityDepositoryInitData::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "liquidity_depository/CoffeeLiquidityDepositoryInitPoolParams/0" => {
                let decoded = CoffeeLiquidityDepositoryInitPoolParams::from_cell(&input)?;
                assert!(decoded.amm_settings.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeLiquidityDepositoryInitPoolParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "liquidity_depository/CoffeeLiquidityDepositoryInitPoolParams/1" => {
                let decoded = CoffeeLiquidityDepositoryInitPoolParams::from_cell(&input)?;
                assert!(decoded.amm_settings.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeLiquidityDepositoryInitPoolParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "liquidity_depository/CoffeeLiquidityDepositoryAssetNative/0" => {
                let decoded = CoffeeLiquidityDepositoryAssetNative::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeLiquidityDepositoryAssetNative::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "liquidity_depository/CoffeeLiquidityDepositoryAssetNative/1" => {
                let decoded = CoffeeLiquidityDepositoryAssetNative::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeLiquidityDepositoryAssetNative::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "liquidity_depository/CoffeeLiquidityDepositoryAssetJetton/0" => {
                let decoded = CoffeeLiquidityDepositoryAssetJetton::from_cell(&input)?;
                assert_eq!(decoded.chain, 37);
                assert_eq!(decoded.hash, U256::from(37u8));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeLiquidityDepositoryAssetJetton::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "liquidity_depository/CoffeeLiquidityDepositoryAssetJetton/1" => {
                let decoded = CoffeeLiquidityDepositoryAssetJetton::from_cell(&input)?;
                assert_eq!(decoded.chain, 37);
                assert_eq!(decoded.hash, U256::from(37u8));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeLiquidityDepositoryAssetJetton::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "liquidity_depository/CoffeeLiquidityDepositoryAssetExtra/0" => {
                let decoded = CoffeeLiquidityDepositoryAssetExtra::from_cell(&input)?;
                assert_eq!(decoded.id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeLiquidityDepositoryAssetExtra::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "liquidity_depository/CoffeeLiquidityDepositoryAssetExtra/1" => {
                let decoded = CoffeeLiquidityDepositoryAssetExtra::from_cell(&input)?;
                assert_eq!(decoded.id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeLiquidityDepositoryAssetExtra::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "liquidity_depository/CoffeeLiquidityDepositoryAmmConstantProduct/0" => {
                let decoded = CoffeeLiquidityDepositoryAmmConstantProduct::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeLiquidityDepositoryAmmConstantProduct::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "liquidity_depository/CoffeeLiquidityDepositoryAmmConstantProduct/1" => {
                let decoded = CoffeeLiquidityDepositoryAmmConstantProduct::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeLiquidityDepositoryAmmConstantProduct::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "liquidity_depository/CoffeeLiquidityDepositoryAmmCurveFiStable/0" => {
                let decoded = CoffeeLiquidityDepositoryAmmCurveFiStable::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeLiquidityDepositoryAmmCurveFiStable::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "liquidity_depository/CoffeeLiquidityDepositoryAmmCurveFiStable/1" => {
                let decoded = CoffeeLiquidityDepositoryAmmCurveFiStable::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeLiquidityDepositoryAmmCurveFiStable::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "liquidity_depository/CoffeeLiquidityDepositoryDepositConditionNone/0" => {
                let decoded = CoffeeLiquidityDepositoryDepositConditionNone::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeLiquidityDepositoryDepositConditionNone::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "liquidity_depository/CoffeeLiquidityDepositoryDepositConditionNone/1" => {
                let decoded = CoffeeLiquidityDepositoryDepositConditionNone::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeLiquidityDepositoryDepositConditionNone::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "liquidity_depository/CoffeeLiquidityDepositoryDepositConditionLpQuantity/0" => {
                let decoded = CoffeeLiquidityDepositoryDepositConditionLpQuantity::from_cell(&input)?;
                assert_eq!(decoded.min_lp_amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    CoffeeLiquidityDepositoryDepositConditionLpQuantity::from_cell(&with_trailing_bit(&input)?).is_ok()
                );
            },
            "liquidity_depository/CoffeeLiquidityDepositoryDepositConditionLpQuantity/1" => {
                let decoded = CoffeeLiquidityDepositoryDepositConditionLpQuantity::from_cell(&input)?;
                assert_eq!(decoded.min_lp_amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    CoffeeLiquidityDepositoryDepositConditionLpQuantity::from_cell(&with_trailing_bit(&input)?).is_ok()
                );
            },
            "liquidity_depository/CoffeeLiquidityDepositoryDepositConditionReservesRatio/0" => {
                let decoded = CoffeeLiquidityDepositoryDepositConditionReservesRatio::from_cell(&input)?;
                assert_eq!(decoded.denominator, 37);
                assert_eq!(decoded.min_nominator, 37);
                assert_eq!(decoded.max_nominator, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    CoffeeLiquidityDepositoryDepositConditionReservesRatio::from_cell(&with_trailing_bit(&input)?)
                        .is_ok()
                );
            },
            "liquidity_depository/CoffeeLiquidityDepositoryDepositConditionReservesRatio/1" => {
                let decoded = CoffeeLiquidityDepositoryDepositConditionReservesRatio::from_cell(&input)?;
                assert_eq!(decoded.denominator, 37);
                assert_eq!(decoded.min_nominator, 37);
                assert_eq!(decoded.max_nominator, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    CoffeeLiquidityDepositoryDepositConditionReservesRatio::from_cell(&with_trailing_bit(&input)?)
                        .is_ok()
                );
            },
            "liquidity_depository/CoffeeLiquidityDepositoryDepositConditionComplex/0" => {
                let decoded = CoffeeLiquidityDepositoryDepositConditionComplex::from_cell(&input)?;
                assert_eq!(decoded.min_lp_amount, TLBCoins::new(37));
                assert_eq!(decoded.denominator, 37);
                assert_eq!(decoded.min_nominator, 37);
                assert_eq!(decoded.max_nominator, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    CoffeeLiquidityDepositoryDepositConditionComplex::from_cell(&with_trailing_bit(&input)?).is_ok()
                );
            },
            "liquidity_depository/CoffeeLiquidityDepositoryDepositConditionComplex/1" => {
                let decoded = CoffeeLiquidityDepositoryDepositConditionComplex::from_cell(&input)?;
                assert_eq!(decoded.min_lp_amount, TLBCoins::new(37));
                assert_eq!(decoded.denominator, 37);
                assert_eq!(decoded.min_nominator, 37);
                assert_eq!(decoded.max_nominator, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    CoffeeLiquidityDepositoryDepositConditionComplex::from_cell(&with_trailing_bit(&input)?).is_ok()
                );
            },
            "liquidity_depository/CoffeeLiquidityDepositoryNotificationDataSingle/0" => {
                let decoded = CoffeeLiquidityDepositoryNotificationDataSingle::from_cell(&input)?;
                assert!(decoded.receiver.is_none());
                assert_eq!(decoded.fwd_gas, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    CoffeeLiquidityDepositoryNotificationDataSingle::from_cell(&with_trailing_bit(&input)?).is_ok()
                );
            },
            "liquidity_depository/CoffeeLiquidityDepositoryNotificationDataSingle/1" => {
                let decoded = CoffeeLiquidityDepositoryNotificationDataSingle::from_cell(&input)?;
                assert!(decoded.receiver.is_some());
                assert_eq!(decoded.fwd_gas, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    CoffeeLiquidityDepositoryNotificationDataSingle::from_cell(&with_trailing_bit(&input)?).is_ok()
                );
            },
            "liquidity_depository/CoffeeLiquidityDepositoryNotificationData/0" => {
                let decoded = CoffeeLiquidityDepositoryNotificationData::from_cell(&input)?;
                assert!(decoded.on_success.is_none());
                assert!(decoded.on_failure.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeLiquidityDepositoryNotificationData::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "liquidity_depository/CoffeeLiquidityDepositoryNotificationData/1" => {
                let decoded = CoffeeLiquidityDepositoryNotificationData::from_cell(&input)?;
                assert!(decoded.on_success.is_some());
                assert!(decoded.on_failure.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeLiquidityDepositoryNotificationData::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "liquidity_depository/CoffeeLiquidityDepositoryDepositLiquidityParamsTrimmed/0" => {
                let decoded = CoffeeLiquidityDepositoryDepositLiquidityParamsTrimmed::from_cell(&input)?;
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.use_recipient_on_failure, -1);
                assert_eq!(decoded.referral.workchain, -1);
                assert_eq!(decoded.referral.hash.as_slice()[31], 37);
                assert_eq!(decoded.deadline, 37);
                assert!(decoded.extra_settings.is_none());
                assert!(decoded.notification_data.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    CoffeeLiquidityDepositoryDepositLiquidityParamsTrimmed::from_cell(&with_trailing_bit(&input)?)
                        .is_ok()
                );
            },
            "liquidity_depository/CoffeeLiquidityDepositoryDepositLiquidityParamsTrimmed/1" => {
                let decoded = CoffeeLiquidityDepositoryDepositLiquidityParamsTrimmed::from_cell(&input)?;
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.use_recipient_on_failure, -1);
                assert_eq!(decoded.referral.workchain, -1);
                assert_eq!(decoded.referral.hash.as_slice()[31], 37);
                assert_eq!(decoded.deadline, 37);
                assert!(decoded.extra_settings.is_some());
                assert!(decoded.notification_data.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    CoffeeLiquidityDepositoryDepositLiquidityParamsTrimmed::from_cell(&with_trailing_bit(&input)?)
                        .is_ok()
                );
            },
            "liquidity_depository/CoffeeLiquidityDepositoryContractUpdate/0" => {
                let decoded = CoffeeLiquidityDepositoryContractUpdate::from_cell(&input)?;
                assert!(decoded.code.is_none());
                assert!(decoded.data.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeLiquidityDepositoryContractUpdate::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "liquidity_depository/CoffeeLiquidityDepositoryContractUpdate/1" => {
                let decoded = CoffeeLiquidityDepositoryContractUpdate::from_cell(&input)?;
                assert!(decoded.code.is_some());
                assert!(decoded.data.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeLiquidityDepositoryContractUpdate::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "liquidity_depository/CoffeeLiquidityDepositoryDeploy/0" => {
                let decoded = CoffeeLiquidityDepositoryDeployMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert!(decoded.action.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeLiquidityDepositoryDeployMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "liquidity_depository/CoffeeLiquidityDepositoryDeploy/1" => {
                let decoded = CoffeeLiquidityDepositoryDeployMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert!(decoded.action.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeLiquidityDepositoryDeployMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "liquidity_depository/CoffeeLiquidityDepositoryCreateLiquidityDepositoryInternal/0" => {
                let decoded = CoffeeLiquidityDepositoryCreateLiquidityDepositoryInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    CoffeeLiquidityDepositoryCreateLiquidityDepositoryInternalMsg::from_cell(&with_trailing_bit(
                        &input
                    )?)
                    .is_ok()
                );
            },
            "liquidity_depository/CoffeeLiquidityDepositoryCreateLiquidityDepositoryInternal/1" => {
                let decoded = CoffeeLiquidityDepositoryCreateLiquidityDepositoryInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    CoffeeLiquidityDepositoryCreateLiquidityDepositoryInternalMsg::from_cell(&with_trailing_bit(
                        &input
                    )?)
                    .is_ok()
                );
            },
            "liquidity_depository/CoffeeLiquidityDepositoryWithdrawDeposit/0" => {
                let decoded = CoffeeLiquidityDepositoryWithdrawDepositMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeLiquidityDepositoryWithdrawDepositMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "liquidity_depository/CoffeeLiquidityDepositoryWithdrawDeposit/1" => {
                let decoded = CoffeeLiquidityDepositoryWithdrawDepositMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeLiquidityDepositoryWithdrawDepositMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "liquidity_depository/CoffeeLiquidityDepositoryUpdateContractInternal/0" => {
                let decoded = CoffeeLiquidityDepositoryUpdateContractInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.excesses_receiver.workchain, -1);
                assert_eq!(decoded.excesses_receiver.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    CoffeeLiquidityDepositoryUpdateContractInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok()
                );
            },
            "liquidity_depository/CoffeeLiquidityDepositoryUpdateContractInternal/1" => {
                let decoded = CoffeeLiquidityDepositoryUpdateContractInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.excesses_receiver.workchain, -1);
                assert_eq!(decoded.excesses_receiver.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    CoffeeLiquidityDepositoryUpdateContractInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok()
                );
            },
            "liquidity_depository/CoffeeLiquidityDepositoryDepositLiquidityInternal/0" => {
                let decoded = CoffeeLiquidityDepositoryDepositLiquidityInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount1, TLBCoins::new(37));
                assert_eq!(decoded.amount2, TLBCoins::new(37));
                assert_eq!(decoded.tx_initiator.workchain, -1);
                assert_eq!(decoded.tx_initiator.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    CoffeeLiquidityDepositoryDepositLiquidityInternalMsg::from_cell(&with_trailing_bit(&input)?)
                        .is_ok()
                );
            },
            "liquidity_depository/CoffeeLiquidityDepositoryDepositLiquidityInternal/1" => {
                let decoded = CoffeeLiquidityDepositoryDepositLiquidityInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount1, TLBCoins::new(37));
                assert_eq!(decoded.amount2, TLBCoins::new(37));
                assert_eq!(decoded.tx_initiator.workchain, -1);
                assert_eq!(decoded.tx_initiator.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    CoffeeLiquidityDepositoryDepositLiquidityInternalMsg::from_cell(&with_trailing_bit(&input)?)
                        .is_ok()
                );
            },
            "liquidity_depository/CoffeeLiquidityDepositoryPayoutInternal/0" => {
                let decoded = CoffeeLiquidityDepositoryPayoutInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert!(decoded.notification_data.is_none());
                assert!(decoded.proof.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeLiquidityDepositoryPayoutInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "liquidity_depository/CoffeeLiquidityDepositoryPayoutInternal/1" => {
                let decoded = CoffeeLiquidityDepositoryPayoutInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert!(decoded.notification_data.is_some());
                assert!(decoded.proof.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeLiquidityDepositoryPayoutInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "liquidity_depository/CoffeeLiquidityDepositoryExcesses/0" => {
                let decoded = CoffeeLiquidityDepositoryExcessesMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeLiquidityDepositoryExcessesMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "liquidity_depository/CoffeeLiquidityDepositoryExcesses/1" => {
                let decoded = CoffeeLiquidityDepositoryExcessesMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeLiquidityDepositoryExcessesMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "lp_wallet/JettonWalletCoffeeLpStorage/0" => {
                let decoded = CoffeeLpWalletStorage::from_cell(&input)?;
                assert_eq!(decoded.balance, TLBCoins::new(37));
                assert_eq!(decoded.owner_address.workchain, -1);
                assert_eq!(decoded.owner_address.hash.as_slice()[31], 37);
                assert_eq!(decoded.jetton_master_address.workchain, -1);
                assert_eq!(decoded.jetton_master_address.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeLpWalletStorage::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "lp_wallet/JettonWalletCoffeeLpStorage/1" => {
                let decoded = CoffeeLpWalletStorage::from_cell(&input)?;
                assert_eq!(decoded.balance, TLBCoins::new(37));
                assert_eq!(decoded.owner_address.workchain, -1);
                assert_eq!(decoded.owner_address.hash.as_slice()[31], 37);
                assert_eq!(decoded.jetton_master_address.workchain, -1);
                assert_eq!(decoded.jetton_master_address.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeLpWalletStorage::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "lp_wallet/JettonWalletCoffeeLpTransfer/0" => {
                let decoded = CoffeeLpWalletTransferMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.destination.workchain, -1);
                assert_eq!(decoded.destination.hash.as_slice()[31], 37);
                assert!(decoded.response_address.is_none());
                assert!(decoded.custom_payload.is_none());
                assert_eq!(decoded.forward_ton_amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
            },
            "lp_wallet/JettonWalletCoffeeLpTransfer/1" => {
                let decoded = CoffeeLpWalletTransferMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.destination.workchain, -1);
                assert_eq!(decoded.destination.hash.as_slice()[31], 37);
                assert!(decoded.response_address.is_some());
                assert!(decoded.custom_payload.is_some());
                assert_eq!(decoded.forward_ton_amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeLpWalletTransferMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "lp_wallet/JettonWalletCoffeeLpInternalTransfer/0" => {
                let decoded = CoffeeLpWalletInternalTransferMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert!(decoded.from_address.is_none());
                assert!(decoded.response_address.is_none());
                assert_eq!(decoded.forward_ton_amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
            },
            "lp_wallet/JettonWalletCoffeeLpInternalTransfer/1" => {
                let decoded = CoffeeLpWalletInternalTransferMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert!(decoded.from_address.is_some());
                assert!(decoded.response_address.is_some());
                assert_eq!(decoded.forward_ton_amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeLpWalletInternalTransferMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "lp_wallet/JettonWalletCoffeeLpBurn/0" => {
                let decoded = CoffeeLpWalletBurnMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert!(decoded.response_address.is_none());
                assert!(decoded.custom_payload.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeLpWalletBurnMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "lp_wallet/JettonWalletCoffeeLpBurn/1" => {
                let decoded = CoffeeLpWalletBurnMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert!(decoded.response_address.is_some());
                assert!(decoded.custom_payload.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeLpWalletBurnMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "lp_wallet/JettonWalletCoffeeLpTransferNotification/0" => {
                let decoded = CoffeeLpWalletTransferNotificationMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert!(decoded.from_address.is_none());
                assert_eq!(decoded.to_cell()?, input);
            },
            "lp_wallet/JettonWalletCoffeeLpTransferNotification/1" => {
                let decoded = CoffeeLpWalletTransferNotificationMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert!(decoded.from_address.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeLpWalletTransferNotificationMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "lp_wallet/JettonWalletCoffeeLpExcesses/0" => {
                let decoded = CoffeeLpWalletExcessesMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeLpWalletExcessesMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "lp_wallet/JettonWalletCoffeeLpExcesses/1" => {
                let decoded = CoffeeLpWalletExcessesMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeLpWalletExcessesMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "lp_wallet/JettonWalletCoffeeLpBurnNotification/0" => {
                let decoded = CoffeeLpWalletBurnNotificationMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.from_address.workchain, -1);
                assert_eq!(decoded.from_address.hash.as_slice()[31], 37);
                assert!(decoded.response_address.is_none());
                assert!(decoded.custom_payload.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeLpWalletBurnNotificationMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "lp_wallet/JettonWalletCoffeeLpBurnNotification/1" => {
                let decoded = CoffeeLpWalletBurnNotificationMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.from_address.workchain, -1);
                assert_eq!(decoded.from_address.hash.as_slice()[31], 37);
                assert!(decoded.response_address.is_some());
                assert!(decoded.custom_payload.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeLpWalletBurnNotificationMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "lp_wallet/JettonWalletCoffeeLpBouncedKnownBody/0" => {
                let decoded = CoffeeLpWalletBouncedKnownBodyMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
            },
            "lp_wallet/JettonWalletCoffeeLpBouncedKnownBody/1" => {
                let decoded = CoffeeLpWalletBouncedKnownBodyMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
            },
            "mev_protector/CoffeeMevProtectorStorage/0" => {
                let decoded = CoffeeMevProtectorStorage::from_cell(&input)?;
                assert!(decoded.admin_address.is_none());
                assert_eq!(decoded.config, 37);
                assert_eq!(decoded.processed_until, 37);
                assert_eq!(decoded.public_key, U256::from(37u8));
                assert_eq!(decoded.processed_queries.len(), 0);
                assert_eq!(CoffeeMevProtectorStorage::from_cell(&decoded.to_cell()?)?, decoded);
                assert!(CoffeeMevProtectorStorage::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "mev_protector/CoffeeMevProtectorStorage/1" => {
                let decoded = CoffeeMevProtectorStorage::from_cell(&input)?;
                assert!(decoded.admin_address.is_some());
                assert_eq!(decoded.config, 37);
                assert_eq!(decoded.processed_until, 37);
                assert_eq!(decoded.public_key, U256::from(37u8));
                assert_eq!(decoded.processed_queries.len(), 1);
                assert!(decoded.processed_queries.contains_key(&0));
                assert_eq!(CoffeeMevProtectorStorage::from_cell(&decoded.to_cell()?)?, decoded);
                assert!(CoffeeMevProtectorStorage::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "mev_protector/CoffeeMevProtectorJettonTransferNotification/0" => {
                let decoded = CoffeeCrossDexJettonTransferNotificationMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert!(decoded.sender.is_none());
                assert_eq!(decoded.to_cell()?, input);
            },
            "mev_protector/CoffeeMevProtectorJettonTransferNotification/1" => {
                let decoded = CoffeeCrossDexJettonTransferNotificationMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert!(decoded.sender.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexJettonTransferNotificationMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "mev_protector/CoffeeMevProtectHoldFunds/0" => {
                let decoded = CoffeeMevProtectHoldFundsMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeMevProtectHoldFundsMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "mev_protector/CoffeeMevProtectHoldFunds/1" => {
                let decoded = CoffeeMevProtectHoldFundsMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeMevProtectHoldFundsMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "mev_protector/CoffeeMevProtectFailedSwapPayload/0" => {
                let decoded = CoffeeMevProtectFailedSwapPayloadMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeMevProtectFailedSwapPayloadMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "mev_protector/CoffeeMevProtectFailedSwapPayload/1" => {
                let decoded = CoffeeMevProtectFailedSwapPayloadMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeMevProtectFailedSwapPayloadMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "mev_protector/CoffeeMevProtectorDedustPayout/0" => {
                let decoded = CoffeeMevProtectorDedustPayoutMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert!(decoded.payload.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeMevProtectorDedustPayoutMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "mev_protector/CoffeeMevProtectorDedustPayout/1" => {
                let decoded = CoffeeMevProtectorDedustPayoutMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert!(decoded.payload.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeMevProtectorDedustPayoutMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "mev_protector/CoffeeMevProtectorCoffeeNotification/0" => {
                let decoded = CoffeeMevProtectorCoffeeNotificationMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeMevProtectorCoffeeNotificationMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "mev_protector/CoffeeMevProtectorCoffeeNotification/1" => {
                let decoded = CoffeeMevProtectorCoffeeNotificationMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeMevProtectorCoffeeNotificationMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "mev_protector/CoffeeMevProtectorSetAdmin/0" => {
                let decoded = CoffeeMevProtectorSetAdminMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert!(decoded.admin_address.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeMevProtectorSetAdminMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "mev_protector/CoffeeMevProtectorSetAdmin/1" => {
                let decoded = CoffeeMevProtectorSetAdminMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert!(decoded.admin_address.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeMevProtectorSetAdminMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "mev_protector/CoffeeMevProtectorSetPublicKey/0" => {
                let decoded = CoffeeMevProtectorSetPublicKeyMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.public_key, U256::from(37u8));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeMevProtectorSetPublicKeyMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "mev_protector/CoffeeMevProtectorSetPublicKey/1" => {
                let decoded = CoffeeMevProtectorSetPublicKeyMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.public_key, U256::from(37u8));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeMevProtectorSetPublicKeyMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "mev_protector/CoffeeMevProtectorUpdateContract/0" => {
                let decoded = CoffeeMevProtectorUpdateContractMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert!(decoded.code.is_none());
                assert!(decoded.data.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeMevProtectorUpdateContractMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "mev_protector/CoffeeMevProtectorUpdateContract/1" => {
                let decoded = CoffeeMevProtectorUpdateContractMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert!(decoded.code.is_some());
                assert!(decoded.data.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeMevProtectorUpdateContractMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "mev_protector/CoffeeMevProtectorExcesses/0" => {
                let decoded = CoffeeMevProtectorExcessesMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeMevProtectorExcessesMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "mev_protector/CoffeeMevProtectorExcesses/1" => {
                let decoded = CoffeeMevProtectorExcessesMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeMevProtectorExcessesMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "mev_protector/CoffeeMevProtectorJettonTransfer/0" => {
                let decoded = CoffeeMevProtectorJettonTransferMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.destination.workchain, -1);
                assert_eq!(decoded.destination.hash.as_slice()[31], 37);
                assert!(decoded.response_destination.is_none());
                assert!(decoded.custom_payload.is_none());
                assert_eq!(decoded.forward_ton_amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
            },
            "mev_protector/CoffeeMevProtectorJettonTransfer/1" => {
                let decoded = CoffeeMevProtectorJettonTransferMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.destination.workchain, -1);
                assert_eq!(decoded.destination.hash.as_slice()[31], 37);
                assert!(decoded.response_destination.is_some());
                assert!(decoded.custom_payload.is_some());
                assert_eq!(decoded.forward_ton_amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeMevProtectorJettonTransferMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "mev_protector/CoffeeMevProtectorSendMessageAction/0" => {
                let decoded = CoffeeMevProtectorSendMessageAction::from_cell(&input)?;
                assert_eq!(decoded.mode, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeMevProtectorSendMessageAction::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "mev_protector/CoffeeMevProtectorSendMessageAction/1" => {
                let decoded = CoffeeMevProtectorSendMessageAction::from_cell(&input)?;
                assert_eq!(decoded.mode, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeMevProtectorSendMessageAction::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "mev_protector/CoffeeMevProtectorSignedBatch/0" => {
                let decoded = CoffeeMevProtectorSignedBatchMsg::from_cell(&input)?;
                assert_eq!(decoded.subwallet_id, 37);
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.payload.len(), 0);
                assert_eq!(CoffeeMevProtectorSignedBatchMsg::from_cell(&decoded.to_cell()?)?, decoded);
                assert!(CoffeeMevProtectorSignedBatchMsg::from_cell(&with_trailing_bit(&input)?).is_err());
            },
            "mev_protector/CoffeeMevProtectorSignedBatch/1" => {
                let decoded = CoffeeMevProtectorSignedBatchMsg::from_cell(&input)?;
                assert_eq!(decoded.subwallet_id, 37);
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.payload.len(), 1);
                assert!(decoded.payload.contains_key(&0));
                assert_eq!(CoffeeMevProtectorSignedBatchMsg::from_cell(&decoded.to_cell()?)?, decoded);
                assert!(CoffeeMevProtectorSignedBatchMsg::from_cell(&with_trailing_bit(&input)?).is_err());
            },
            "mev_protector/CoffeeMevProtectorNativeRescue/0" => {
                let decoded = CoffeeMevProtectorNativeRescueMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeMevProtectorNativeRescueMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "mev_protector/CoffeeMevProtectorNativeRescue/1" => {
                let decoded = CoffeeMevProtectorNativeRescueMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeMevProtectorNativeRescueMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "mev_protector/CoffeeMevProtectorJettonRescue/0" => {
                let decoded = CoffeeMevProtectorJettonRescueMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.jetton_wallet.workchain, -1);
                assert_eq!(decoded.jetton_wallet.hash.as_slice()[31], 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeMevProtectorJettonRescueMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "mev_protector/CoffeeMevProtectorJettonRescue/1" => {
                let decoded = CoffeeMevProtectorJettonRescueMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.jetton_wallet.workchain, -1);
                assert_eq!(decoded.jetton_wallet.hash.as_slice()[31], 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeMevProtectorJettonRescueMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "mev_protector_legacy/CoffeeMevProtectorLegacyStorage/0" => {
                let decoded = CoffeeMevProtectorLegacyStorage::from_cell(&input)?;
                assert!(decoded.admin_address.is_none());
                assert_eq!(decoded.subwallet_id, 37);
                assert_eq!(decoded.processed_until, 37);
                assert_eq!(decoded.public_key, U256::from(37u8));
                assert_eq!(decoded.processed_queries.len(), 0);
                assert_eq!(CoffeeMevProtectorLegacyStorage::from_cell(&decoded.to_cell()?)?, decoded);
                assert!(CoffeeMevProtectorLegacyStorage::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "mev_protector_legacy/CoffeeMevProtectorLegacyStorage/1" => {
                let decoded = CoffeeMevProtectorLegacyStorage::from_cell(&input)?;
                assert!(decoded.admin_address.is_some());
                assert_eq!(decoded.subwallet_id, 37);
                assert_eq!(decoded.processed_until, 37);
                assert_eq!(decoded.public_key, U256::from(37u8));
                assert_eq!(decoded.processed_queries.len(), 1);
                assert!(decoded.processed_queries.contains_key(&0));
                assert_eq!(CoffeeMevProtectorLegacyStorage::from_cell(&decoded.to_cell()?)?, decoded);
                assert!(CoffeeMevProtectorLegacyStorage::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "mev_protector_legacy/CoffeeMevProtectorLegacyJettonTransferNotification/0" => {
                let decoded = CoffeeCrossDexJettonTransferNotificationMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert!(decoded.sender.is_none());
                assert_eq!(decoded.to_cell()?, input);
            },
            "mev_protector_legacy/CoffeeMevProtectorLegacyJettonTransferNotification/1" => {
                let decoded = CoffeeCrossDexJettonTransferNotificationMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert!(decoded.sender.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexJettonTransferNotificationMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "mev_protector_legacy/CoffeeMevProtectorLegacyHoldFunds/0" => {
                let decoded = CoffeeMevProtectorLegacyHoldFundsMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeMevProtectorLegacyHoldFundsMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "mev_protector_legacy/CoffeeMevProtectorLegacyHoldFunds/1" => {
                let decoded = CoffeeMevProtectorLegacyHoldFundsMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeMevProtectorLegacyHoldFundsMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "mev_protector_legacy/CoffeeMevProtectorLegacyFailedSwapPayload/0" => {
                let decoded = CoffeeMevProtectorLegacyFailedSwapPayloadMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeMevProtectorLegacyFailedSwapPayloadMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "mev_protector_legacy/CoffeeMevProtectorLegacyFailedSwapPayload/1" => {
                let decoded = CoffeeMevProtectorLegacyFailedSwapPayloadMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeMevProtectorLegacyFailedSwapPayloadMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "mev_protector_legacy/CoffeeMevProtectorLegacyDedustPayout/0" => {
                let decoded = CoffeeMevProtectorLegacyDedustPayoutMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert!(decoded.payload.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeMevProtectorLegacyDedustPayoutMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "mev_protector_legacy/CoffeeMevProtectorLegacyDedustPayout/1" => {
                let decoded = CoffeeMevProtectorLegacyDedustPayoutMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert!(decoded.payload.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeMevProtectorLegacyDedustPayoutMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "mev_protector_legacy/CoffeeMevProtectorLegacySetAdmin/0" => {
                let decoded = CoffeeMevProtectorLegacySetAdminMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert!(decoded.admin_address.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeMevProtectorLegacySetAdminMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "mev_protector_legacy/CoffeeMevProtectorLegacySetAdmin/1" => {
                let decoded = CoffeeMevProtectorLegacySetAdminMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert!(decoded.admin_address.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeMevProtectorLegacySetAdminMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "mev_protector_legacy/CoffeeMevProtectorLegacySetPublicKey/0" => {
                let decoded = CoffeeMevProtectorLegacySetPublicKeyMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.public_key, U256::from(37u8));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeMevProtectorLegacySetPublicKeyMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "mev_protector_legacy/CoffeeMevProtectorLegacySetPublicKey/1" => {
                let decoded = CoffeeMevProtectorLegacySetPublicKeyMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.public_key, U256::from(37u8));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeMevProtectorLegacySetPublicKeyMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "mev_protector_legacy/CoffeeMevProtectorLegacyUpdateContract/0" => {
                let decoded = CoffeeMevProtectorLegacyUpdateContractMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert!(decoded.code.is_none());
                assert!(decoded.data.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeMevProtectorLegacyUpdateContractMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "mev_protector_legacy/CoffeeMevProtectorLegacyUpdateContract/1" => {
                let decoded = CoffeeMevProtectorLegacyUpdateContractMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert!(decoded.code.is_some());
                assert!(decoded.data.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeMevProtectorLegacyUpdateContractMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "mev_protector_legacy/CoffeeMevProtectorLegacyAdmin2bf71be6/0" => {
                let decoded = CoffeeMevProtectorLegacyAdmin2bf71be6Msg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.destination.workchain, -1);
                assert_eq!(decoded.destination.hash.as_slice()[31], 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeMevProtectorLegacyAdmin2bf71be6Msg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "mev_protector_legacy/CoffeeMevProtectorLegacyAdmin2bf71be6/1" => {
                let decoded = CoffeeMevProtectorLegacyAdmin2bf71be6Msg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.destination.workchain, -1);
                assert_eq!(decoded.destination.hash.as_slice()[31], 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeMevProtectorLegacyAdmin2bf71be6Msg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "mev_protector_legacy/CoffeeMevProtectorLegacyAdmin7b4b82fe/0" => {
                let decoded = CoffeeMevProtectorLegacyAdmin7b4b82feMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.jetton_wallet.workchain, -1);
                assert_eq!(decoded.jetton_wallet.hash.as_slice()[31], 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeMevProtectorLegacyAdmin7b4b82feMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "mev_protector_legacy/CoffeeMevProtectorLegacyAdmin7b4b82fe/1" => {
                let decoded = CoffeeMevProtectorLegacyAdmin7b4b82feMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.jetton_wallet.workchain, -1);
                assert_eq!(decoded.jetton_wallet.hash.as_slice()[31], 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeMevProtectorLegacyAdmin7b4b82feMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "mev_protector_legacy/CoffeeMevProtectorLegacyExcesses/0" => {
                let decoded = CoffeeMevProtectorLegacyExcessesMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeMevProtectorLegacyExcessesMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "mev_protector_legacy/CoffeeMevProtectorLegacyExcesses/1" => {
                let decoded = CoffeeMevProtectorLegacyExcessesMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeMevProtectorLegacyExcessesMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "mev_protector_legacy/CoffeeMevProtectorLegacyJettonTransfer/0" => {
                let decoded = CoffeeMevProtectorLegacyJettonTransferMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.destination.workchain, -1);
                assert_eq!(decoded.destination.hash.as_slice()[31], 37);
                assert!(decoded.response_destination.is_none());
                assert!(decoded.custom_payload.is_none());
                assert_eq!(decoded.forward_ton_amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
            },
            "mev_protector_legacy/CoffeeMevProtectorLegacyJettonTransfer/1" => {
                let decoded = CoffeeMevProtectorLegacyJettonTransferMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.destination.workchain, -1);
                assert_eq!(decoded.destination.hash.as_slice()[31], 37);
                assert!(decoded.response_destination.is_some());
                assert!(decoded.custom_payload.is_some());
                assert_eq!(decoded.forward_ton_amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeMevProtectorLegacyJettonTransferMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "mev_protector_legacy/CoffeeMevProtectorLegacySendMessageAction/0" => {
                let decoded = CoffeeMevProtectorLegacySendMessageAction::from_cell(&input)?;
                assert_eq!(decoded.mode, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeMevProtectorLegacySendMessageAction::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "mev_protector_legacy/CoffeeMevProtectorLegacySendMessageAction/1" => {
                let decoded = CoffeeMevProtectorLegacySendMessageAction::from_cell(&input)?;
                assert_eq!(decoded.mode, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeMevProtectorLegacySendMessageAction::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "mev_protector_legacy/CoffeeMevProtectorLegacySignedBatch/0" => {
                let decoded = CoffeeMevProtectorLegacySignedBatchMsg::from_cell(&input)?;
                assert_eq!(decoded.subwallet_id, 37);
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.payload.len(), 0);
                assert_eq!(CoffeeMevProtectorLegacySignedBatchMsg::from_cell(&decoded.to_cell()?)?, decoded);
                assert!(CoffeeMevProtectorLegacySignedBatchMsg::from_cell(&with_trailing_bit(&input)?).is_err());
            },
            "mev_protector_legacy/CoffeeMevProtectorLegacySignedBatch/1" => {
                let decoded = CoffeeMevProtectorLegacySignedBatchMsg::from_cell(&input)?;
                assert_eq!(decoded.subwallet_id, 37);
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.payload.len(), 1);
                assert!(decoded.payload.contains_key(&0));
                assert_eq!(CoffeeMevProtectorLegacySignedBatchMsg::from_cell(&decoded.to_cell()?)?, decoded);
                assert!(CoffeeMevProtectorLegacySignedBatchMsg::from_cell(&with_trailing_bit(&input)?).is_err());
            },
            "pool_constant_product/CoffeePoolConstantProductStorage/0" => {
                let decoded = CoffeeConstantProductPoolStorage::from_cell(&input)?;
                assert!(decoded.is_active);
                assert_eq!(decoded.reserve1, TLBCoins::new(37));
                assert_eq!(decoded.reserve2, TLBCoins::new(37));
                assert_eq!(decoded.total_supply, TLBCoins::new(37));
                assert_eq!(decoded.protocol_fee, 37);
                assert_eq!(decoded.lp_fee, 37);
                assert_eq!(decoded.protocol_fee_collected_asset1, TLBCoins::new(37));
                assert_eq!(decoded.protocol_fee_collected_asset2, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeConstantProductPoolStorage::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolConstantProductStorage/1" => {
                let decoded = CoffeeConstantProductPoolStorage::from_cell(&input)?;
                assert!(decoded.is_active);
                assert_eq!(decoded.reserve1, TLBCoins::new(37));
                assert_eq!(decoded.reserve2, TLBCoins::new(37));
                assert_eq!(decoded.total_supply, TLBCoins::new(37));
                assert_eq!(decoded.protocol_fee, 37);
                assert_eq!(decoded.lp_fee, 37);
                assert_eq!(decoded.protocol_fee_collected_asset1, TLBCoins::new(37));
                assert_eq!(decoded.protocol_fee_collected_asset2, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeConstantProductPoolStorage::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolNativeAsset/0" => {
                let decoded = CoffeePoolNativeAsset::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolNativeAsset::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolNativeAsset/1" => {
                let decoded = CoffeePoolNativeAsset::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolNativeAsset::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolJettonAsset/0" => {
                let decoded = CoffeePoolJettonAsset::from_cell(&input)?;
                assert_eq!(decoded.chain, 37);
                assert_eq!(decoded.hash, U256::from(37u8));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolJettonAsset::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolJettonAsset/1" => {
                let decoded = CoffeePoolJettonAsset::from_cell(&input)?;
                assert_eq!(decoded.chain, 37);
                assert_eq!(decoded.hash, U256::from(37u8));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolJettonAsset::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolExtraAsset/0" => {
                let decoded = CoffeePoolExtraAsset::from_cell(&input)?;
                assert_eq!(decoded.id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolExtraAsset::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolExtraAsset/1" => {
                let decoded = CoffeePoolExtraAsset::from_cell(&input)?;
                assert_eq!(decoded.id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolExtraAsset::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolPreviousAssetHintAbsent/0" => {
                let decoded = CoffeePoolPreviousAssetHintAbsent::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolPreviousAssetHintAbsent::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolPreviousAssetHintAbsent/1" => {
                let decoded = CoffeePoolPreviousAssetHintAbsent::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolPreviousAssetHintAbsent::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolPreviousNativeAssetHint/0" => {
                let decoded = CoffeePoolPreviousNativeAssetHint::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolPreviousNativeAssetHint::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolPreviousNativeAssetHint/1" => {
                let decoded = CoffeePoolPreviousNativeAssetHint::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolPreviousNativeAssetHint::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolPreviousJettonAssetHint/0" => {
                let decoded = CoffeePoolPreviousJettonAssetHint::from_cell(&input)?;
                assert_eq!(decoded.chain, 37);
                assert_eq!(decoded.hash, U256::from(37u8));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolPreviousJettonAssetHint::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolPreviousJettonAssetHint/1" => {
                let decoded = CoffeePoolPreviousJettonAssetHint::from_cell(&input)?;
                assert_eq!(decoded.chain, 37);
                assert_eq!(decoded.hash, U256::from(37u8));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolPreviousJettonAssetHint::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolPreviousExtraAssetHint/0" => {
                let decoded = CoffeePoolPreviousExtraAssetHint::from_cell(&input)?;
                assert_eq!(decoded.id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolPreviousExtraAssetHint::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolPreviousExtraAssetHint/1" => {
                let decoded = CoffeePoolPreviousExtraAssetHint::from_cell(&input)?;
                assert_eq!(decoded.id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolPreviousExtraAssetHint::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolDoubleAssetParams/0" => {
                let decoded =
                    crate::contracts::pool_constant_product::storage::CoffeeConstantProductPoolDoubleAssetParams::from_cell(&input)?;
                assert_eq!(decoded.amm, 7);
                assert!(decoded.amm_settings.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    crate::contracts::pool_constant_product::storage::CoffeeConstantProductPoolDoubleAssetParams::from_cell(
                        &with_trailing_bit(&input)?
                    )
                    .is_ok()
                );
            },
            "pool_constant_product/CoffeePoolDoubleAssetParams/1" => {
                let decoded =
                    crate::contracts::pool_constant_product::storage::CoffeeConstantProductPoolDoubleAssetParams::from_cell(&input)?;
                assert_eq!(decoded.amm, 7);
                assert!(decoded.amm_settings.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    crate::contracts::pool_constant_product::storage::CoffeeConstantProductPoolDoubleAssetParams::from_cell(
                        &with_trailing_bit(&input)?
                    )
                    .is_ok()
                );
            },
            "pool_constant_product/CoffeePoolVaultProofBody/0" => {
                let decoded = CoffeePoolVaultProofBody::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolVaultProofBody::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolVaultProofBody/1" => {
                let decoded = CoffeePoolVaultProofBody::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolVaultProofBody::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolPoolProofBody/0" => {
                let decoded =
                    crate::contracts::pool_constant_product::storage::CoffeeConstantProductPoolSourcePoolProofBody::from_cell(&input)?;
                assert_eq!(decoded.amm, 7);
                assert!(decoded.amm_settings.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    crate::contracts::pool_constant_product::storage::CoffeeConstantProductPoolSourcePoolProofBody::from_cell(
                        &with_trailing_bit(&input)?
                    )
                    .is_ok()
                );
            },
            "pool_constant_product/CoffeePoolPoolProofBody/1" => {
                let decoded =
                    crate::contracts::pool_constant_product::storage::CoffeeConstantProductPoolSourcePoolProofBody::from_cell(&input)?;
                assert_eq!(decoded.amm, 7);
                assert!(decoded.amm_settings.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    crate::contracts::pool_constant_product::storage::CoffeeConstantProductPoolSourcePoolProofBody::from_cell(
                        &with_trailing_bit(&input)?
                    )
                    .is_ok()
                );
            },
            "pool_constant_product/CoffeePoolLiquidityDepositoryProofBody/0" => {
                let decoded = crate::contracts::pool_constant_product::storage::CoffeeConstantProductPoolLiquidityDepositoryProofBody::from_cell(&input)?;
                assert_eq!(decoded.owner.workchain, -1);
                assert_eq!(decoded.owner.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    crate::contracts::pool_constant_product::storage::CoffeeConstantProductPoolLiquidityDepositoryProofBody::from_cell(&with_trailing_bit(
                        &input
                    )?)
                    .is_ok()
                );
            },
            "pool_constant_product/CoffeePoolLiquidityDepositoryProofBody/1" => {
                let decoded = crate::contracts::pool_constant_product::storage::CoffeeConstantProductPoolLiquidityDepositoryProofBody::from_cell(&input)?;
                assert_eq!(decoded.owner.workchain, -1);
                assert_eq!(decoded.owner.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    crate::contracts::pool_constant_product::storage::CoffeeConstantProductPoolLiquidityDepositoryProofBody::from_cell(&with_trailing_bit(
                        &input
                    )?)
                    .is_ok()
                );
            },
            "pool_constant_product/CoffeePoolCreatorProofBody/0" => {
                let decoded =
                    crate::contracts::pool_constant_product::storage::CoffeeConstantProductPoolCreatorProofBody::from_cell(&input)?;
                assert_eq!(decoded.owner.workchain, -1);
                assert_eq!(decoded.owner.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    crate::contracts::pool_constant_product::storage::CoffeeConstantProductPoolCreatorProofBody::from_cell(
                        &with_trailing_bit(&input)?
                    )
                    .is_ok()
                );
            },
            "pool_constant_product/CoffeePoolCreatorProofBody/1" => {
                let decoded =
                    crate::contracts::pool_constant_product::storage::CoffeeConstantProductPoolCreatorProofBody::from_cell(&input)?;
                assert_eq!(decoded.owner.workchain, -1);
                assert_eq!(decoded.owner.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    crate::contracts::pool_constant_product::storage::CoffeeConstantProductPoolCreatorProofBody::from_cell(
                        &with_trailing_bit(&input)?
                    )
                    .is_ok()
                );
            },
            "pool_constant_product/CoffeePoolProof/0" => {
                let decoded =
                    crate::contracts::pool_constant_product::storage::CoffeeConstantProductPoolProof::from_cell(
                        &input,
                    )?;
                assert_eq!(decoded.factory_address.workchain, -1);
                assert_eq!(decoded.factory_address.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    crate::contracts::pool_constant_product::storage::CoffeeConstantProductPoolProof::from_cell(
                        &with_trailing_bit(&input)?
                    )
                    .is_ok()
                );
            },
            "pool_constant_product/CoffeePoolProof/1" => {
                let decoded =
                    crate::contracts::pool_constant_product::storage::CoffeeConstantProductPoolProof::from_cell(
                        &input,
                    )?;
                assert_eq!(decoded.factory_address.workchain, -1);
                assert_eq!(decoded.factory_address.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    crate::contracts::pool_constant_product::storage::CoffeeConstantProductPoolProof::from_cell(
                        &with_trailing_bit(&input)?
                    )
                    .is_ok()
                );
            },
            "pool_constant_product/CoffeePoolNotificationDataSingle/0" => {
                let decoded = CoffeePoolNotificationDataSingle::from_cell(&input)?;
                assert!(decoded.receiver.is_none());
                assert_eq!(decoded.fwd_gas, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolNotificationDataSingle::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolNotificationDataSingle/1" => {
                let decoded = CoffeePoolNotificationDataSingle::from_cell(&input)?;
                assert!(decoded.receiver.is_some());
                assert_eq!(decoded.fwd_gas, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolNotificationDataSingle::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolNotificationData/0" => {
                let decoded = CoffeePoolNotificationData::from_cell(&input)?;
                assert!(decoded.on_success.is_none());
                assert!(decoded.on_failure.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolNotificationData::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolNotificationData/1" => {
                let decoded = CoffeePoolNotificationData::from_cell(&input)?;
                assert!(decoded.on_success.is_some());
                assert!(decoded.on_failure.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolNotificationData::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolSwapParams/0" => {
                let decoded = CoffeePoolSwapParams::from_cell(&input)?;
                assert_eq!(decoded.deadline, 37);
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert!(decoded.referral.is_none());
                assert!(decoded.notification_data.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolSwapParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolSwapParams/1" => {
                let decoded = CoffeePoolSwapParams::from_cell(&input)?;
                assert_eq!(decoded.deadline, 37);
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert!(decoded.referral.is_some());
                assert!(decoded.notification_data.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolSwapParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolSwapStepParams/0" => {
                let decoded = CoffeePoolSwapStepParams::from_cell(&input)?;
                assert_eq!(decoded.pool_address_hash, U256::from(37u8));
                assert_eq!(decoded.min_output_amount, TLBCoins::new(37));
                assert!(decoded.next.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolSwapStepParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolSwapStepParams/1" => {
                let decoded = CoffeePoolSwapStepParams::from_cell(&input)?;
                assert_eq!(decoded.pool_address_hash, U256::from(37u8));
                assert_eq!(decoded.min_output_amount, TLBCoins::new(37));
                assert!(decoded.next.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolSwapStepParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolSwapStepInternalParams/0" => {
                let decoded = CoffeePoolSwapStepInternalParams::from_cell(&input)?;
                assert_eq!(decoded.previous_amount, TLBCoins::new(37));
                assert_eq!(decoded.min_output_amount, TLBCoins::new(37));
                assert!(decoded.next.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolSwapStepInternalParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolSwapStepInternalParams/1" => {
                let decoded = CoffeePoolSwapStepInternalParams::from_cell(&input)?;
                assert_eq!(decoded.previous_amount, TLBCoins::new(37));
                assert_eq!(decoded.min_output_amount, TLBCoins::new(37));
                assert!(decoded.next.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolSwapStepInternalParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolDepositConditionNone/0" => {
                let decoded = CoffeePoolDepositConditionNone::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolDepositConditionNone::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolDepositConditionNone/1" => {
                let decoded = CoffeePoolDepositConditionNone::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolDepositConditionNone::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolDepositConditionLpQuantity/0" => {
                let decoded = CoffeePoolDepositConditionLpQuantity::from_cell(&input)?;
                assert_eq!(decoded.min_lp_amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolDepositConditionLpQuantity::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolDepositConditionLpQuantity/1" => {
                let decoded = CoffeePoolDepositConditionLpQuantity::from_cell(&input)?;
                assert_eq!(decoded.min_lp_amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolDepositConditionLpQuantity::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolDepositConditionReservesRatio/0" => {
                let decoded = CoffeePoolDepositConditionReservesRatio::from_cell(&input)?;
                assert_eq!(decoded.denominator, 37);
                assert_eq!(decoded.min_nominator, 37);
                assert_eq!(decoded.max_nominator, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolDepositConditionReservesRatio::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolDepositConditionReservesRatio/1" => {
                let decoded = CoffeePoolDepositConditionReservesRatio::from_cell(&input)?;
                assert_eq!(decoded.denominator, 37);
                assert_eq!(decoded.min_nominator, 37);
                assert_eq!(decoded.max_nominator, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolDepositConditionReservesRatio::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolDepositConditionComplex/0" => {
                let decoded = CoffeePoolDepositConditionComplex::from_cell(&input)?;
                assert_eq!(decoded.min_lp_amount, TLBCoins::new(37));
                assert_eq!(decoded.denominator, 37);
                assert_eq!(decoded.min_nominator, 37);
                assert_eq!(decoded.max_nominator, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolDepositConditionComplex::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolDepositConditionComplex/1" => {
                let decoded = CoffeePoolDepositConditionComplex::from_cell(&input)?;
                assert_eq!(decoded.min_lp_amount, TLBCoins::new(37));
                assert_eq!(decoded.denominator, 37);
                assert_eq!(decoded.min_nominator, 37);
                assert_eq!(decoded.max_nominator, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolDepositConditionComplex::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolDepositLiquidityParamsTrimmed/0" => {
                let decoded = CoffeePoolDepositLiquidityParamsTrimmed::from_cell(&input)?;
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.use_recipient_on_failure, -1);
                assert!(decoded.referral.is_none());
                assert_eq!(decoded.deadline, 37);
                assert!(decoded.extra_settings.is_none());
                assert!(decoded.notification_data.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolDepositLiquidityParamsTrimmed::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolDepositLiquidityParamsTrimmed/1" => {
                let decoded = CoffeePoolDepositLiquidityParamsTrimmed::from_cell(&input)?;
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.use_recipient_on_failure, -1);
                assert!(decoded.referral.is_some());
                assert_eq!(decoded.deadline, 37);
                assert!(decoded.extra_settings.is_some());
                assert!(decoded.notification_data.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolDepositLiquidityParamsTrimmed::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolWithdrawConditionNone/0" => {
                let decoded = CoffeePoolWithdrawConditionNone::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolWithdrawConditionNone::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolWithdrawConditionNone/1" => {
                let decoded = CoffeePoolWithdrawConditionNone::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolWithdrawConditionNone::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolWithdrawConditionAssetsQuantity/0" => {
                let decoded = CoffeePoolWithdrawConditionAssetsQuantity::from_cell(&input)?;
                assert_eq!(decoded.min_first_amount, TLBCoins::new(37));
                assert_eq!(decoded.min_second_amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolWithdrawConditionAssetsQuantity::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolWithdrawConditionAssetsQuantity/1" => {
                let decoded = CoffeePoolWithdrawConditionAssetsQuantity::from_cell(&input)?;
                assert_eq!(decoded.min_first_amount, TLBCoins::new(37));
                assert_eq!(decoded.min_second_amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolWithdrawConditionAssetsQuantity::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolWithdrawConditionReservesRatio/0" => {
                let decoded = CoffeePoolWithdrawConditionReservesRatio::from_cell(&input)?;
                assert_eq!(decoded.denominator, 37);
                assert_eq!(decoded.min_nominator, 37);
                assert_eq!(decoded.max_nominator, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolWithdrawConditionReservesRatio::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolWithdrawConditionReservesRatio/1" => {
                let decoded = CoffeePoolWithdrawConditionReservesRatio::from_cell(&input)?;
                assert_eq!(decoded.denominator, 37);
                assert_eq!(decoded.min_nominator, 37);
                assert_eq!(decoded.max_nominator, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolWithdrawConditionReservesRatio::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolWithdrawConditionComplex/0" => {
                let decoded = CoffeePoolWithdrawConditionComplex::from_cell(&input)?;
                assert_eq!(decoded.min_first_amount, TLBCoins::new(37));
                assert_eq!(decoded.min_second_amount, TLBCoins::new(37));
                assert_eq!(decoded.denominator, 37);
                assert_eq!(decoded.min_nominator, 37);
                assert_eq!(decoded.max_nominator, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolWithdrawConditionComplex::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolWithdrawConditionComplex/1" => {
                let decoded = CoffeePoolWithdrawConditionComplex::from_cell(&input)?;
                assert_eq!(decoded.min_first_amount, TLBCoins::new(37));
                assert_eq!(decoded.min_second_amount, TLBCoins::new(37));
                assert_eq!(decoded.denominator, 37);
                assert_eq!(decoded.min_nominator, 37);
                assert_eq!(decoded.max_nominator, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolWithdrawConditionComplex::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolWithdrawLiquidityParams/0" => {
                let decoded = CoffeePoolWithdrawLiquidityParams::from_cell(&input)?;
                assert_eq!(decoded.use_recipient_on_failure, -1);
                assert_eq!(decoded.deadline, 37);
                assert!(decoded.extra_settings.is_none());
                assert!(decoded.on_success.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolWithdrawLiquidityParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolWithdrawLiquidityParams/1" => {
                let decoded = CoffeePoolWithdrawLiquidityParams::from_cell(&input)?;
                assert_eq!(decoded.use_recipient_on_failure, -1);
                assert_eq!(decoded.deadline, 37);
                assert!(decoded.extra_settings.is_some());
                assert!(decoded.on_success.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolWithdrawLiquidityParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolContractUpdate/0" => {
                let decoded = CoffeePoolContractUpdate::from_cell(&input)?;
                assert!(decoded.code.is_none());
                assert!(decoded.data.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolContractUpdate::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolContractUpdate/1" => {
                let decoded = CoffeePoolContractUpdate::from_cell(&input)?;
                assert!(decoded.code.is_some());
                assert!(decoded.data.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolContractUpdate::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolUpdateNoop/0" => {
                let decoded = CoffeePoolUpdateNoop::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolUpdateNoop::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolUpdateNoop/1" => {
                let decoded = CoffeePoolUpdateNoop::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolUpdateNoop::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolUpdateFees/0" => {
                let decoded = CoffeePoolUpdateFees::from_cell(&input)?;
                assert_eq!(decoded.protocol_fee, 37);
                assert_eq!(decoded.lp_fee, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolUpdateFees::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolUpdateFees/1" => {
                let decoded = CoffeePoolUpdateFees::from_cell(&input)?;
                assert_eq!(decoded.protocol_fee, 37);
                assert_eq!(decoded.lp_fee, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolUpdateFees::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolUpdateActive/0" => {
                let decoded = CoffeePoolUpdateActive::from_cell(&input)?;
                assert!(decoded.is_active);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolUpdateActive::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolUpdateActive/1" => {
                let decoded = CoffeePoolUpdateActive::from_cell(&input)?;
                assert!(decoded.is_active);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolUpdateActive::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolUpdateFeesAndActive/0" => {
                let decoded = CoffeePoolUpdateFeesAndActive::from_cell(&input)?;
                assert_eq!(decoded.protocol_fee, 37);
                assert_eq!(decoded.lp_fee, 37);
                assert!(decoded.is_active);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolUpdateFeesAndActive::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolUpdateFeesAndActive/1" => {
                let decoded = CoffeePoolUpdateFeesAndActive::from_cell(&input)?;
                assert_eq!(decoded.protocol_fee, 37);
                assert_eq!(decoded.lp_fee, 37);
                assert!(decoded.is_active);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolUpdateFeesAndActive::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolValuePair/0" => {
                let decoded = CoffeePoolValuePair::from_cell(&input)?;
                assert_eq!(decoded.first, TLBCoins::new(37));
                assert_eq!(decoded.second, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolValuePair::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolValuePair/1" => {
                let decoded = CoffeePoolValuePair::from_cell(&input)?;
                assert_eq!(decoded.first, TLBCoins::new(37));
                assert_eq!(decoded.second, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolValuePair::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolConstantProductSwapInternal/0" => {
                let decoded = CoffeeConstantProductPoolSwapInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeConstantProductPoolSwapInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolConstantProductSwapInternal/1" => {
                let decoded = CoffeeConstantProductPoolSwapInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeConstantProductPoolSwapInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolConstantProductDepositLiquidityInternal/0" => {
                let decoded = CoffeeConstantProductPoolDepositLiquidityInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount1, TLBCoins::new(37));
                assert_eq!(decoded.amount2, TLBCoins::new(37));
                assert_eq!(decoded.tx_initiator.workchain, -1);
                assert_eq!(decoded.tx_initiator.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    CoffeeConstantProductPoolDepositLiquidityInternalMsg::from_cell(&with_trailing_bit(&input)?)
                        .is_ok()
                );
            },
            "pool_constant_product/CoffeePoolConstantProductDepositLiquidityInternal/1" => {
                let decoded = CoffeeConstantProductPoolDepositLiquidityInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount1, TLBCoins::new(37));
                assert_eq!(decoded.amount2, TLBCoins::new(37));
                assert_eq!(decoded.tx_initiator.workchain, -1);
                assert_eq!(decoded.tx_initiator.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    CoffeeConstantProductPoolDepositLiquidityInternalMsg::from_cell(&with_trailing_bit(&input)?)
                        .is_ok()
                );
            },
            "pool_constant_product/CoffeePoolConstantProductJettonBurnNotification/0" => {
                let decoded = CoffeeConstantProductPoolJettonBurnNotificationMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.from_address.workchain, -1);
                assert_eq!(decoded.from_address.hash.as_slice()[31], 37);
                assert!(decoded.response_address.is_none());
                assert!(decoded.custom_payload.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    CoffeeConstantProductPoolJettonBurnNotificationMsg::from_cell(&with_trailing_bit(&input)?).is_ok()
                );
            },
            "pool_constant_product/CoffeePoolConstantProductJettonBurnNotification/1" => {
                let decoded = CoffeeConstantProductPoolJettonBurnNotificationMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.from_address.workchain, -1);
                assert_eq!(decoded.from_address.hash.as_slice()[31], 37);
                assert!(decoded.response_address.is_some());
                assert!(decoded.custom_payload.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    CoffeeConstantProductPoolJettonBurnNotificationMsg::from_cell(&with_trailing_bit(&input)?).is_ok()
                );
            },
            "pool_constant_product/CoffeePoolConstantProductProvideWalletAddress/0" => {
                let decoded = CoffeeConstantProductPoolProvideWalletAddressMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.owner_address.workchain, -1);
                assert_eq!(decoded.owner_address.hash.as_slice()[31], 37);
                assert!(decoded.include_address);
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    CoffeeConstantProductPoolProvideWalletAddressMsg::from_cell(&with_trailing_bit(&input)?).is_ok()
                );
            },
            "pool_constant_product/CoffeePoolConstantProductProvideWalletAddress/1" => {
                let decoded = CoffeeConstantProductPoolProvideWalletAddressMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.owner_address.workchain, -1);
                assert_eq!(decoded.owner_address.hash.as_slice()[31], 37);
                assert!(decoded.include_address);
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    CoffeeConstantProductPoolProvideWalletAddressMsg::from_cell(&with_trailing_bit(&input)?).is_ok()
                );
            },
            "pool_constant_product/CoffeePoolConstantProductWithdrawInternal/0" => {
                let decoded = CoffeeConstantProductPoolWithdrawInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.receiver.workchain, -1);
                assert_eq!(decoded.receiver.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeConstantProductPoolWithdrawInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolConstantProductWithdrawInternal/1" => {
                let decoded = CoffeeConstantProductPoolWithdrawInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.receiver.workchain, -1);
                assert_eq!(decoded.receiver.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeConstantProductPoolWithdrawInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolConstantProductUpdatePoolInternal/0" => {
                let decoded = CoffeeConstantProductPoolUpdatePoolInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert!(decoded.excesses_receiver.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeConstantProductPoolUpdatePoolInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolConstantProductUpdatePoolInternal/1" => {
                let decoded = CoffeeConstantProductPoolUpdatePoolInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert!(decoded.excesses_receiver.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeConstantProductPoolUpdatePoolInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolConstantProductUpdateContractInternal/0" => {
                let decoded = CoffeeConstantProductPoolUpdateContractInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert!(decoded.excesses_receiver.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    CoffeeConstantProductPoolUpdateContractInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok()
                );
            },
            "pool_constant_product/CoffeePoolConstantProductUpdateContractInternal/1" => {
                let decoded = CoffeeConstantProductPoolUpdateContractInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert!(decoded.excesses_receiver.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    CoffeeConstantProductPoolUpdateContractInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok()
                );
            },
            "pool_constant_product/CoffeePoolConstantProductCreatePoolInternal/0" => {
                let decoded = CoffeeConstantProductPoolCreatePoolInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount1, TLBCoins::new(37));
                assert_eq!(decoded.amount2, TLBCoins::new(37));
                assert_eq!(decoded.tx_initiator.workchain, -1);
                assert_eq!(decoded.tx_initiator.hash.as_slice()[31], 37);
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.use_recipient_on_failure, -1);
                assert!(decoded.extra_settings.is_none());
                assert!(decoded.notification_data.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeConstantProductPoolCreatePoolInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolConstantProductCreatePoolInternal/1" => {
                let decoded = CoffeeConstantProductPoolCreatePoolInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount1, TLBCoins::new(37));
                assert_eq!(decoded.amount2, TLBCoins::new(37));
                assert_eq!(decoded.tx_initiator.workchain, -1);
                assert_eq!(decoded.tx_initiator.hash.as_slice()[31], 37);
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.use_recipient_on_failure, -1);
                assert!(decoded.extra_settings.is_some());
                assert!(decoded.notification_data.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeConstantProductPoolCreatePoolInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolConstantProductDeploy/0" => {
                let decoded = CoffeeConstantProductPoolDeployMsg::from_cell(&input)?;
                assert!(decoded.action.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeConstantProductPoolDeployMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolConstantProductDeploy/1" => {
                let decoded = CoffeeConstantProductPoolDeployMsg::from_cell(&input)?;
                assert!(decoded.action.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeConstantProductPoolDeployMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolConstantProductPayoutInternal/0" => {
                let decoded = CoffeeConstantProductPoolPayoutInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert!(decoded.notification_data.is_none());
                assert!(decoded.proof.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeConstantProductPoolPayoutInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolConstantProductPayoutInternal/1" => {
                let decoded = CoffeeConstantProductPoolPayoutInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert!(decoded.notification_data.is_some());
                assert!(decoded.proof.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeConstantProductPoolPayoutInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolConstantProductSwapInternalOut/0" => {
                let decoded = CoffeeConstantProductPoolSwapInternalOutMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeConstantProductPoolSwapInternalOutMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolConstantProductSwapInternalOut/1" => {
                let decoded = CoffeeConstantProductPoolSwapInternalOutMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeConstantProductPoolSwapInternalOutMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolConstantProductNotification/0" => {
                let decoded = CoffeeConstantProductPoolNotificationMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeConstantProductPoolNotificationMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolConstantProductNotification/1" => {
                let decoded = CoffeeConstantProductPoolNotificationMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeConstantProductPoolNotificationMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolConstantProductSwapSucceeded/0" => {
                let decoded = CoffeeConstantProductPoolSwapSucceededMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.input_amount, TLBCoins::new(37));
                assert_eq!(decoded.output_amount, TLBCoins::new(37));
                assert_eq!(decoded.protocol_fee, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeConstantProductPoolSwapSucceededMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolConstantProductSwapSucceeded/1" => {
                let decoded = CoffeeConstantProductPoolSwapSucceededMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.input_amount, TLBCoins::new(37));
                assert_eq!(decoded.output_amount, TLBCoins::new(37));
                assert_eq!(decoded.protocol_fee, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeConstantProductPoolSwapSucceededMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolConstantProductSwapFailed/0" => {
                let decoded = CoffeeConstantProductPoolSwapFailedMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.input_amount, TLBCoins::new(37));
                assert!(decoded.reserves.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeConstantProductPoolSwapFailedMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolConstantProductSwapFailed/1" => {
                let decoded = CoffeeConstantProductPoolSwapFailedMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.input_amount, TLBCoins::new(37));
                assert!(decoded.reserves.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeConstantProductPoolSwapFailedMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolConstantProductDepositLiquiditySucceeded/0" => {
                let decoded = CoffeeConstantProductPoolDepositLiquiditySucceededMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount1, TLBCoins::new(37));
                assert_eq!(decoded.amount2, TLBCoins::new(37));
                assert_eq!(decoded.lp_amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    CoffeeConstantProductPoolDepositLiquiditySucceededMsg::from_cell(&with_trailing_bit(&input)?)
                        .is_ok()
                );
            },
            "pool_constant_product/CoffeePoolConstantProductDepositLiquiditySucceeded/1" => {
                let decoded = CoffeeConstantProductPoolDepositLiquiditySucceededMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount1, TLBCoins::new(37));
                assert_eq!(decoded.amount2, TLBCoins::new(37));
                assert_eq!(decoded.lp_amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    CoffeeConstantProductPoolDepositLiquiditySucceededMsg::from_cell(&with_trailing_bit(&input)?)
                        .is_ok()
                );
            },
            "pool_constant_product/CoffeePoolConstantProductDepositLiquidityFailed/0" => {
                let decoded = CoffeeConstantProductPoolDepositLiquidityFailedMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount1, TLBCoins::new(37));
                assert_eq!(decoded.amount2, TLBCoins::new(37));
                assert_eq!(decoded.min_lp_amount, TLBCoins::new(37));
                assert_eq!(decoded.total_supply, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    CoffeeConstantProductPoolDepositLiquidityFailedMsg::from_cell(&with_trailing_bit(&input)?).is_ok()
                );
            },
            "pool_constant_product/CoffeePoolConstantProductDepositLiquidityFailed/1" => {
                let decoded = CoffeeConstantProductPoolDepositLiquidityFailedMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount1, TLBCoins::new(37));
                assert_eq!(decoded.amount2, TLBCoins::new(37));
                assert_eq!(decoded.min_lp_amount, TLBCoins::new(37));
                assert_eq!(decoded.total_supply, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    CoffeeConstantProductPoolDepositLiquidityFailedMsg::from_cell(&with_trailing_bit(&input)?).is_ok()
                );
            },
            "pool_constant_product/CoffeePoolConstantProductWithdrawLiquiditySucceeded/0" => {
                let decoded = CoffeeConstantProductPoolWithdrawLiquiditySucceededMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount1, TLBCoins::new(37));
                assert_eq!(decoded.amount2, TLBCoins::new(37));
                assert_eq!(decoded.lp_amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    CoffeeConstantProductPoolWithdrawLiquiditySucceededMsg::from_cell(&with_trailing_bit(&input)?)
                        .is_ok()
                );
            },
            "pool_constant_product/CoffeePoolConstantProductWithdrawLiquiditySucceeded/1" => {
                let decoded = CoffeeConstantProductPoolWithdrawLiquiditySucceededMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount1, TLBCoins::new(37));
                assert_eq!(decoded.amount2, TLBCoins::new(37));
                assert_eq!(decoded.lp_amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    CoffeeConstantProductPoolWithdrawLiquiditySucceededMsg::from_cell(&with_trailing_bit(&input)?)
                        .is_ok()
                );
            },
            "pool_constant_product/CoffeePoolConstantProductTakeWalletAddress/0" => {
                let decoded = CoffeeConstantProductPoolTakeWalletAddressMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.wallet_address.workchain, -1);
                assert_eq!(decoded.wallet_address.hash.as_slice()[31], 37);
                assert!(decoded.owner_address.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeConstantProductPoolTakeWalletAddressMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolConstantProductTakeWalletAddress/1" => {
                let decoded = CoffeeConstantProductPoolTakeWalletAddressMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.wallet_address.workchain, -1);
                assert_eq!(decoded.wallet_address.hash.as_slice()[31], 37);
                assert!(decoded.owner_address.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeConstantProductPoolTakeWalletAddressMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolConstantProductExcesses/0" => {
                let decoded = CoffeeConstantProductPoolExcessesMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeConstantProductPoolExcessesMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolConstantProductExcesses/1" => {
                let decoded = CoffeeConstantProductPoolExcessesMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeConstantProductPoolExcessesMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_constant_product/CoffeePoolConstantProductLpWalletInternalTransfer/0" => {
                let decoded = CoffeeConstantProductPoolLpWalletInternalTransferMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert!(decoded.from_address.is_none());
                assert_eq!(decoded.response_address.workchain, -1);
                assert_eq!(decoded.response_address.hash.as_slice()[31], 37);
                assert_eq!(decoded.forward_ton_amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
            },
            "pool_constant_product/CoffeePoolConstantProductLpWalletInternalTransfer/1" => {
                let decoded = CoffeeConstantProductPoolLpWalletInternalTransferMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert!(decoded.from_address.is_some());
                assert_eq!(decoded.response_address.workchain, -1);
                assert_eq!(decoded.response_address.hash.as_slice()[31], 37);
                assert_eq!(decoded.forward_ton_amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    CoffeeConstantProductPoolLpWalletInternalTransferMsg::from_cell(&with_trailing_bit(&input)?)
                        .is_ok()
                );
            },
            "pool_creator/CoffeePoolCreatorStorage/0" => {
                let decoded = CoffeePoolCreatorStorage::from_cell(&input)?;
                assert_eq!(decoded.amount1, TLBCoins::new(37));
                assert_eq!(decoded.amount2, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolCreatorStorage::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_creator/CoffeePoolCreatorStorage/1" => {
                let decoded = CoffeePoolCreatorStorage::from_cell(&input)?;
                assert_eq!(decoded.amount1, TLBCoins::new(37));
                assert_eq!(decoded.amount2, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolCreatorStorage::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_creator/CoffeePoolCreatorInitData/0" => {
                let decoded = CoffeePoolCreatorInitData::from_cell(&input)?;
                assert_eq!(decoded.factory_address.workchain, -1);
                assert_eq!(decoded.factory_address.hash.as_slice()[31], 37);
                assert_eq!(decoded.contract_type, 3);
                assert_eq!(decoded.owner_address.workchain, -1);
                assert_eq!(decoded.owner_address.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolCreatorInitData::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_creator/CoffeePoolCreatorInitData/1" => {
                let decoded = CoffeePoolCreatorInitData::from_cell(&input)?;
                assert_eq!(decoded.factory_address.workchain, -1);
                assert_eq!(decoded.factory_address.hash.as_slice()[31], 37);
                assert_eq!(decoded.contract_type, 3);
                assert_eq!(decoded.owner_address.workchain, -1);
                assert_eq!(decoded.owner_address.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolCreatorInitData::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_creator/CoffeePoolCreatorInitPoolParams/0" => {
                let decoded = CoffeePoolCreatorInitPoolParams::from_cell(&input)?;
                assert!(decoded.amm_settings.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolCreatorInitPoolParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_creator/CoffeePoolCreatorInitPoolParams/1" => {
                let decoded = CoffeePoolCreatorInitPoolParams::from_cell(&input)?;
                assert!(decoded.amm_settings.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolCreatorInitPoolParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_creator/CoffeePoolCreatorAssetNative/0" => {
                let decoded = CoffeePoolCreatorAssetNative::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolCreatorAssetNative::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_creator/CoffeePoolCreatorAssetNative/1" => {
                let decoded = CoffeePoolCreatorAssetNative::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolCreatorAssetNative::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_creator/CoffeePoolCreatorAssetJetton/0" => {
                let decoded = CoffeePoolCreatorAssetJetton::from_cell(&input)?;
                assert_eq!(decoded.chain, 37);
                assert_eq!(decoded.hash, U256::from(37u8));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolCreatorAssetJetton::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_creator/CoffeePoolCreatorAssetJetton/1" => {
                let decoded = CoffeePoolCreatorAssetJetton::from_cell(&input)?;
                assert_eq!(decoded.chain, 37);
                assert_eq!(decoded.hash, U256::from(37u8));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolCreatorAssetJetton::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_creator/CoffeePoolCreatorAssetExtra/0" => {
                let decoded = CoffeePoolCreatorAssetExtra::from_cell(&input)?;
                assert_eq!(decoded.id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolCreatorAssetExtra::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_creator/CoffeePoolCreatorAssetExtra/1" => {
                let decoded = CoffeePoolCreatorAssetExtra::from_cell(&input)?;
                assert_eq!(decoded.id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolCreatorAssetExtra::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_creator/CoffeePoolCreatorAmmConstantProduct/0" => {
                let decoded = CoffeePoolCreatorAmmConstantProduct::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolCreatorAmmConstantProduct::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_creator/CoffeePoolCreatorAmmConstantProduct/1" => {
                let decoded = CoffeePoolCreatorAmmConstantProduct::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolCreatorAmmConstantProduct::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_creator/CoffeePoolCreatorAmmCurveFiStable/0" => {
                let decoded = CoffeePoolCreatorAmmCurveFiStable::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolCreatorAmmCurveFiStable::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_creator/CoffeePoolCreatorAmmCurveFiStable/1" => {
                let decoded = CoffeePoolCreatorAmmCurveFiStable::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolCreatorAmmCurveFiStable::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_creator/CoffeePoolCreatorNotificationDataSingle/0" => {
                let decoded = CoffeePoolCreatorNotificationDataSingle::from_cell(&input)?;
                assert!(decoded.receiver.is_none());
                assert_eq!(decoded.fwd_gas, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolCreatorNotificationDataSingle::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_creator/CoffeePoolCreatorNotificationDataSingle/1" => {
                let decoded = CoffeePoolCreatorNotificationDataSingle::from_cell(&input)?;
                assert!(decoded.receiver.is_some());
                assert_eq!(decoded.fwd_gas, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolCreatorNotificationDataSingle::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_creator/CoffeePoolCreatorPublicPoolCreationParams/0" => {
                let decoded = CoffeePoolCreatorPublicPoolCreationParams::from_cell(&input)?;
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.use_recipient_on_failure, -1);
                assert!(decoded.notification_data.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolCreatorPublicPoolCreationParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_creator/CoffeePoolCreatorPublicPoolCreationParams/1" => {
                let decoded = CoffeePoolCreatorPublicPoolCreationParams::from_cell(&input)?;
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.use_recipient_on_failure, -1);
                assert!(decoded.notification_data.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolCreatorPublicPoolCreationParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_creator/CoffeePoolCreatorPrivatePoolCreationParams/0" => {
                let decoded = CoffeePoolCreatorPrivatePoolCreationParams::from_cell(&input)?;
                assert_eq!(decoded.is_active, 1);
                assert!(decoded.extra_settings.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolCreatorPrivatePoolCreationParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_creator/CoffeePoolCreatorPrivatePoolCreationParams/1" => {
                let decoded = CoffeePoolCreatorPrivatePoolCreationParams::from_cell(&input)?;
                assert_eq!(decoded.is_active, 1);
                assert!(decoded.extra_settings.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolCreatorPrivatePoolCreationParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_creator/CoffeePoolCreatorPoolCreationParams/0" => {
                let decoded = CoffeePoolCreatorPoolCreationParams::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolCreatorPoolCreationParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_creator/CoffeePoolCreatorPoolCreationParams/1" => {
                let decoded = CoffeePoolCreatorPoolCreationParams::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolCreatorPoolCreationParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_creator/CoffeePoolCreatorContractUpdate/0" => {
                let decoded = CoffeePoolCreatorContractUpdate::from_cell(&input)?;
                assert!(decoded.code.is_none());
                assert!(decoded.data.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolCreatorContractUpdate::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_creator/CoffeePoolCreatorContractUpdate/1" => {
                let decoded = CoffeePoolCreatorContractUpdate::from_cell(&input)?;
                assert!(decoded.code.is_some());
                assert!(decoded.data.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolCreatorContractUpdate::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_creator/CoffeePoolCreatorDeploy/0" => {
                let decoded = CoffeePoolCreatorDeployMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert!(decoded.action.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolCreatorDeployMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_creator/CoffeePoolCreatorDeploy/1" => {
                let decoded = CoffeePoolCreatorDeployMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert!(decoded.action.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolCreatorDeployMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_creator/CoffeePoolCreatorCreatePoolCreatorInternal/0" => {
                let decoded = CoffeePoolCreatorCreatePoolCreatorInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolCreatorCreatePoolCreatorInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_creator/CoffeePoolCreatorCreatePoolCreatorInternal/1" => {
                let decoded = CoffeePoolCreatorCreatePoolCreatorInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolCreatorCreatePoolCreatorInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_creator/CoffeePoolCreatorWithdrawDeposit/0" => {
                let decoded = CoffeePoolCreatorWithdrawDepositMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolCreatorWithdrawDepositMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_creator/CoffeePoolCreatorWithdrawDeposit/1" => {
                let decoded = CoffeePoolCreatorWithdrawDepositMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolCreatorWithdrawDepositMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_creator/CoffeePoolCreatorUpdateContractInternal/0" => {
                let decoded = CoffeePoolCreatorUpdateContractInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.excesses_receiver.workchain, -1);
                assert_eq!(decoded.excesses_receiver.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolCreatorUpdateContractInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_creator/CoffeePoolCreatorUpdateContractInternal/1" => {
                let decoded = CoffeePoolCreatorUpdateContractInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.excesses_receiver.workchain, -1);
                assert_eq!(decoded.excesses_receiver.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolCreatorUpdateContractInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_creator/CoffeePoolCreatorCreatePoolRequest/0" => {
                let decoded = CoffeePoolCreatorCreatePoolRequestMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount1, TLBCoins::new(37));
                assert_eq!(decoded.amount2, TLBCoins::new(37));
                assert_eq!(decoded.tx_initiator.workchain, -1);
                assert_eq!(decoded.tx_initiator.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolCreatorCreatePoolRequestMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_creator/CoffeePoolCreatorCreatePoolRequest/1" => {
                let decoded = CoffeePoolCreatorCreatePoolRequestMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount1, TLBCoins::new(37));
                assert_eq!(decoded.amount2, TLBCoins::new(37));
                assert_eq!(decoded.tx_initiator.workchain, -1);
                assert_eq!(decoded.tx_initiator.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolCreatorCreatePoolRequestMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_creator/CoffeePoolCreatorPayoutInternal/0" => {
                let decoded = CoffeePoolCreatorPayoutInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert!(decoded.notification_data.is_none());
                assert!(decoded.proof.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolCreatorPayoutInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_creator/CoffeePoolCreatorPayoutInternal/1" => {
                let decoded = CoffeePoolCreatorPayoutInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert!(decoded.notification_data.is_some());
                assert!(decoded.proof.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolCreatorPayoutInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_creator/CoffeePoolCreatorExcesses/0" => {
                let decoded = CoffeePoolCreatorExcessesMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolCreatorExcessesMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_creator/CoffeePoolCreatorExcesses/1" => {
                let decoded = CoffeePoolCreatorExcessesMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolCreatorExcessesMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolCurveFiStableStorage/0" => {
                let decoded = CoffeeCurveFiStablePoolStorage::from_cell(&input)?;
                assert!(decoded.is_active);
                assert_eq!(decoded.reserve1, TLBCoins::new(37));
                assert_eq!(decoded.reserve2, TLBCoins::new(37));
                assert_eq!(decoded.total_supply, TLBCoins::new(37));
                assert_eq!(decoded.protocol_fee, 37);
                assert_eq!(decoded.lp_fee, 37);
                assert_eq!(decoded.protocol_fee_collected_asset1, TLBCoins::new(37));
                assert_eq!(decoded.protocol_fee_collected_asset2, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCurveFiStablePoolStorage::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolCurveFiStableStorage/1" => {
                let decoded = CoffeeCurveFiStablePoolStorage::from_cell(&input)?;
                assert!(decoded.is_active);
                assert_eq!(decoded.reserve1, TLBCoins::new(37));
                assert_eq!(decoded.reserve2, TLBCoins::new(37));
                assert_eq!(decoded.total_supply, TLBCoins::new(37));
                assert_eq!(decoded.protocol_fee, 37);
                assert_eq!(decoded.lp_fee, 37);
                assert_eq!(decoded.protocol_fee_collected_asset1, TLBCoins::new(37));
                assert_eq!(decoded.protocol_fee_collected_asset2, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCurveFiStablePoolStorage::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolNativeAsset/0" => {
                let decoded = CoffeePoolNativeAsset::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolNativeAsset::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolNativeAsset/1" => {
                let decoded = CoffeePoolNativeAsset::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolNativeAsset::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolJettonAsset/0" => {
                let decoded = CoffeePoolJettonAsset::from_cell(&input)?;
                assert_eq!(decoded.chain, 37);
                assert_eq!(decoded.hash, U256::from(37u8));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolJettonAsset::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolJettonAsset/1" => {
                let decoded = CoffeePoolJettonAsset::from_cell(&input)?;
                assert_eq!(decoded.chain, 37);
                assert_eq!(decoded.hash, U256::from(37u8));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolJettonAsset::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolExtraAsset/0" => {
                let decoded = CoffeePoolExtraAsset::from_cell(&input)?;
                assert_eq!(decoded.id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolExtraAsset::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolExtraAsset/1" => {
                let decoded = CoffeePoolExtraAsset::from_cell(&input)?;
                assert_eq!(decoded.id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolExtraAsset::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolCurveFiStableAmmSettings/0" => {
                let decoded = CoffeeCurveFiStablePoolAmmSettings::from_cell(&input)?;
                assert_eq!(decoded.amplification, 37);
                assert_eq!(decoded.asset1_weight, TLBCoins::new(37));
                assert_eq!(decoded.asset2_weight, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCurveFiStablePoolAmmSettings::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolCurveFiStableAmmSettings/1" => {
                let decoded = CoffeeCurveFiStablePoolAmmSettings::from_cell(&input)?;
                assert_eq!(decoded.amplification, 37);
                assert_eq!(decoded.asset1_weight, TLBCoins::new(37));
                assert_eq!(decoded.asset2_weight, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCurveFiStablePoolAmmSettings::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolPreviousAssetHintAbsent/0" => {
                let decoded = CoffeePoolPreviousAssetHintAbsent::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolPreviousAssetHintAbsent::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolPreviousAssetHintAbsent/1" => {
                let decoded = CoffeePoolPreviousAssetHintAbsent::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolPreviousAssetHintAbsent::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolPreviousNativeAssetHint/0" => {
                let decoded = CoffeePoolPreviousNativeAssetHint::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolPreviousNativeAssetHint::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolPreviousNativeAssetHint/1" => {
                let decoded = CoffeePoolPreviousNativeAssetHint::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolPreviousNativeAssetHint::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolPreviousJettonAssetHint/0" => {
                let decoded = CoffeePoolPreviousJettonAssetHint::from_cell(&input)?;
                assert_eq!(decoded.chain, 37);
                assert_eq!(decoded.hash, U256::from(37u8));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolPreviousJettonAssetHint::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolPreviousJettonAssetHint/1" => {
                let decoded = CoffeePoolPreviousJettonAssetHint::from_cell(&input)?;
                assert_eq!(decoded.chain, 37);
                assert_eq!(decoded.hash, U256::from(37u8));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolPreviousJettonAssetHint::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolPreviousExtraAssetHint/0" => {
                let decoded = CoffeePoolPreviousExtraAssetHint::from_cell(&input)?;
                assert_eq!(decoded.id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolPreviousExtraAssetHint::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolPreviousExtraAssetHint/1" => {
                let decoded = CoffeePoolPreviousExtraAssetHint::from_cell(&input)?;
                assert_eq!(decoded.id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolPreviousExtraAssetHint::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolDoubleAssetParams/0" => {
                let decoded =
                    crate::contracts::pool_curve_fi_stable::storage::CoffeeCurveFiStablePoolDoubleAssetParams::from_cell(&input)?;
                assert_eq!(decoded.amm, 7);
                assert!(decoded.amm_settings.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    crate::contracts::pool_curve_fi_stable::storage::CoffeeCurveFiStablePoolDoubleAssetParams::from_cell(
                        &with_trailing_bit(&input)?
                    )
                    .is_ok()
                );
            },
            "pool_curve_fi_stable/CoffeePoolDoubleAssetParams/1" => {
                let decoded =
                    crate::contracts::pool_curve_fi_stable::storage::CoffeeCurveFiStablePoolDoubleAssetParams::from_cell(&input)?;
                assert_eq!(decoded.amm, 7);
                assert!(decoded.amm_settings.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    crate::contracts::pool_curve_fi_stable::storage::CoffeeCurveFiStablePoolDoubleAssetParams::from_cell(
                        &with_trailing_bit(&input)?
                    )
                    .is_ok()
                );
            },
            "pool_curve_fi_stable/CoffeePoolVaultProofBody/0" => {
                let decoded = CoffeePoolVaultProofBody::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolVaultProofBody::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolVaultProofBody/1" => {
                let decoded = CoffeePoolVaultProofBody::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolVaultProofBody::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolPoolProofBody/0" => {
                let decoded =
                    crate::contracts::pool_curve_fi_stable::storage::CoffeeCurveFiStablePoolSourcePoolProofBody::from_cell(&input)?;
                assert_eq!(decoded.amm, 7);
                assert!(decoded.amm_settings.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    crate::contracts::pool_curve_fi_stable::storage::CoffeeCurveFiStablePoolSourcePoolProofBody::from_cell(
                        &with_trailing_bit(&input)?
                    )
                    .is_ok()
                );
            },
            "pool_curve_fi_stable/CoffeePoolPoolProofBody/1" => {
                let decoded =
                    crate::contracts::pool_curve_fi_stable::storage::CoffeeCurveFiStablePoolSourcePoolProofBody::from_cell(&input)?;
                assert_eq!(decoded.amm, 7);
                assert!(decoded.amm_settings.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    crate::contracts::pool_curve_fi_stable::storage::CoffeeCurveFiStablePoolSourcePoolProofBody::from_cell(
                        &with_trailing_bit(&input)?
                    )
                    .is_ok()
                );
            },
            "pool_curve_fi_stable/CoffeePoolLiquidityDepositoryProofBody/0" => {
                let decoded =
                    crate::contracts::pool_curve_fi_stable::storage::CoffeeCurveFiStablePoolLiquidityDepositoryProofBody::from_cell(
                        &input,
                    )?;
                assert_eq!(decoded.owner.workchain, -1);
                assert_eq!(decoded.owner.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    crate::contracts::pool_curve_fi_stable::storage::CoffeeCurveFiStablePoolLiquidityDepositoryProofBody::from_cell(
                        &with_trailing_bit(&input)?
                    )
                    .is_ok()
                );
            },
            "pool_curve_fi_stable/CoffeePoolLiquidityDepositoryProofBody/1" => {
                let decoded =
                    crate::contracts::pool_curve_fi_stable::storage::CoffeeCurveFiStablePoolLiquidityDepositoryProofBody::from_cell(
                        &input,
                    )?;
                assert_eq!(decoded.owner.workchain, -1);
                assert_eq!(decoded.owner.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    crate::contracts::pool_curve_fi_stable::storage::CoffeeCurveFiStablePoolLiquidityDepositoryProofBody::from_cell(
                        &with_trailing_bit(&input)?
                    )
                    .is_ok()
                );
            },
            "pool_curve_fi_stable/CoffeePoolCreatorProofBody/0" => {
                let decoded =
                    crate::contracts::pool_curve_fi_stable::storage::CoffeeCurveFiStablePoolCreatorProofBody::from_cell(&input)?;
                assert_eq!(decoded.owner.workchain, -1);
                assert_eq!(decoded.owner.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    crate::contracts::pool_curve_fi_stable::storage::CoffeeCurveFiStablePoolCreatorProofBody::from_cell(
                        &with_trailing_bit(&input)?
                    )
                    .is_ok()
                );
            },
            "pool_curve_fi_stable/CoffeePoolCreatorProofBody/1" => {
                let decoded =
                    crate::contracts::pool_curve_fi_stable::storage::CoffeeCurveFiStablePoolCreatorProofBody::from_cell(&input)?;
                assert_eq!(decoded.owner.workchain, -1);
                assert_eq!(decoded.owner.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    crate::contracts::pool_curve_fi_stable::storage::CoffeeCurveFiStablePoolCreatorProofBody::from_cell(
                        &with_trailing_bit(&input)?
                    )
                    .is_ok()
                );
            },
            "pool_curve_fi_stable/CoffeePoolProof/0" => {
                let decoded =
                    crate::contracts::pool_curve_fi_stable::storage::CoffeeCurveFiStablePoolProof::from_cell(&input)?;
                assert_eq!(decoded.factory_address.workchain, -1);
                assert_eq!(decoded.factory_address.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    crate::contracts::pool_curve_fi_stable::storage::CoffeeCurveFiStablePoolProof::from_cell(
                        &with_trailing_bit(&input)?
                    )
                    .is_ok()
                );
            },
            "pool_curve_fi_stable/CoffeePoolProof/1" => {
                let decoded =
                    crate::contracts::pool_curve_fi_stable::storage::CoffeeCurveFiStablePoolProof::from_cell(&input)?;
                assert_eq!(decoded.factory_address.workchain, -1);
                assert_eq!(decoded.factory_address.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    crate::contracts::pool_curve_fi_stable::storage::CoffeeCurveFiStablePoolProof::from_cell(
                        &with_trailing_bit(&input)?
                    )
                    .is_ok()
                );
            },
            "pool_curve_fi_stable/CoffeePoolNotificationDataSingle/0" => {
                let decoded = CoffeePoolNotificationDataSingle::from_cell(&input)?;
                assert!(decoded.receiver.is_none());
                assert_eq!(decoded.fwd_gas, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolNotificationDataSingle::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolNotificationDataSingle/1" => {
                let decoded = CoffeePoolNotificationDataSingle::from_cell(&input)?;
                assert!(decoded.receiver.is_some());
                assert_eq!(decoded.fwd_gas, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolNotificationDataSingle::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolNotificationData/0" => {
                let decoded = CoffeePoolNotificationData::from_cell(&input)?;
                assert!(decoded.on_success.is_none());
                assert!(decoded.on_failure.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolNotificationData::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolNotificationData/1" => {
                let decoded = CoffeePoolNotificationData::from_cell(&input)?;
                assert!(decoded.on_success.is_some());
                assert!(decoded.on_failure.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolNotificationData::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolSwapParams/0" => {
                let decoded = CoffeePoolSwapParams::from_cell(&input)?;
                assert_eq!(decoded.deadline, 37);
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert!(decoded.referral.is_none());
                assert!(decoded.notification_data.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolSwapParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolSwapParams/1" => {
                let decoded = CoffeePoolSwapParams::from_cell(&input)?;
                assert_eq!(decoded.deadline, 37);
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert!(decoded.referral.is_some());
                assert!(decoded.notification_data.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolSwapParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolSwapStepParams/0" => {
                let decoded = CoffeePoolSwapStepParams::from_cell(&input)?;
                assert_eq!(decoded.pool_address_hash, U256::from(37u8));
                assert_eq!(decoded.min_output_amount, TLBCoins::new(37));
                assert!(decoded.next.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolSwapStepParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolSwapStepParams/1" => {
                let decoded = CoffeePoolSwapStepParams::from_cell(&input)?;
                assert_eq!(decoded.pool_address_hash, U256::from(37u8));
                assert_eq!(decoded.min_output_amount, TLBCoins::new(37));
                assert!(decoded.next.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolSwapStepParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolSwapStepInternalParams/0" => {
                let decoded = CoffeePoolSwapStepInternalParams::from_cell(&input)?;
                assert_eq!(decoded.previous_amount, TLBCoins::new(37));
                assert_eq!(decoded.min_output_amount, TLBCoins::new(37));
                assert!(decoded.next.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolSwapStepInternalParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolSwapStepInternalParams/1" => {
                let decoded = CoffeePoolSwapStepInternalParams::from_cell(&input)?;
                assert_eq!(decoded.previous_amount, TLBCoins::new(37));
                assert_eq!(decoded.min_output_amount, TLBCoins::new(37));
                assert!(decoded.next.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolSwapStepInternalParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolDepositConditionNone/0" => {
                let decoded = CoffeePoolDepositConditionNone::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolDepositConditionNone::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolDepositConditionNone/1" => {
                let decoded = CoffeePoolDepositConditionNone::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolDepositConditionNone::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolDepositConditionLpQuantity/0" => {
                let decoded = CoffeePoolDepositConditionLpQuantity::from_cell(&input)?;
                assert_eq!(decoded.min_lp_amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolDepositConditionLpQuantity::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolDepositConditionLpQuantity/1" => {
                let decoded = CoffeePoolDepositConditionLpQuantity::from_cell(&input)?;
                assert_eq!(decoded.min_lp_amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolDepositConditionLpQuantity::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolDepositConditionReservesRatio/0" => {
                let decoded = CoffeePoolDepositConditionReservesRatio::from_cell(&input)?;
                assert_eq!(decoded.denominator, 37);
                assert_eq!(decoded.min_nominator, 37);
                assert_eq!(decoded.max_nominator, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolDepositConditionReservesRatio::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolDepositConditionReservesRatio/1" => {
                let decoded = CoffeePoolDepositConditionReservesRatio::from_cell(&input)?;
                assert_eq!(decoded.denominator, 37);
                assert_eq!(decoded.min_nominator, 37);
                assert_eq!(decoded.max_nominator, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolDepositConditionReservesRatio::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolDepositConditionComplex/0" => {
                let decoded = CoffeePoolDepositConditionComplex::from_cell(&input)?;
                assert_eq!(decoded.min_lp_amount, TLBCoins::new(37));
                assert_eq!(decoded.denominator, 37);
                assert_eq!(decoded.min_nominator, 37);
                assert_eq!(decoded.max_nominator, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolDepositConditionComplex::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolDepositConditionComplex/1" => {
                let decoded = CoffeePoolDepositConditionComplex::from_cell(&input)?;
                assert_eq!(decoded.min_lp_amount, TLBCoins::new(37));
                assert_eq!(decoded.denominator, 37);
                assert_eq!(decoded.min_nominator, 37);
                assert_eq!(decoded.max_nominator, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolDepositConditionComplex::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolDepositLiquidityParamsTrimmed/0" => {
                let decoded = CoffeePoolDepositLiquidityParamsTrimmed::from_cell(&input)?;
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.use_recipient_on_failure, -1);
                assert!(decoded.referral.is_none());
                assert_eq!(decoded.deadline, 37);
                assert!(decoded.extra_settings.is_none());
                assert!(decoded.notification_data.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolDepositLiquidityParamsTrimmed::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolDepositLiquidityParamsTrimmed/1" => {
                let decoded = CoffeePoolDepositLiquidityParamsTrimmed::from_cell(&input)?;
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.use_recipient_on_failure, -1);
                assert!(decoded.referral.is_some());
                assert_eq!(decoded.deadline, 37);
                assert!(decoded.extra_settings.is_some());
                assert!(decoded.notification_data.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolDepositLiquidityParamsTrimmed::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolWithdrawConditionNone/0" => {
                let decoded = CoffeePoolWithdrawConditionNone::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolWithdrawConditionNone::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolWithdrawConditionNone/1" => {
                let decoded = CoffeePoolWithdrawConditionNone::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolWithdrawConditionNone::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolWithdrawConditionAssetsQuantity/0" => {
                let decoded = CoffeePoolWithdrawConditionAssetsQuantity::from_cell(&input)?;
                assert_eq!(decoded.min_first_amount, TLBCoins::new(37));
                assert_eq!(decoded.min_second_amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolWithdrawConditionAssetsQuantity::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolWithdrawConditionAssetsQuantity/1" => {
                let decoded = CoffeePoolWithdrawConditionAssetsQuantity::from_cell(&input)?;
                assert_eq!(decoded.min_first_amount, TLBCoins::new(37));
                assert_eq!(decoded.min_second_amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolWithdrawConditionAssetsQuantity::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolWithdrawConditionReservesRatio/0" => {
                let decoded = CoffeePoolWithdrawConditionReservesRatio::from_cell(&input)?;
                assert_eq!(decoded.denominator, 37);
                assert_eq!(decoded.min_nominator, 37);
                assert_eq!(decoded.max_nominator, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolWithdrawConditionReservesRatio::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolWithdrawConditionReservesRatio/1" => {
                let decoded = CoffeePoolWithdrawConditionReservesRatio::from_cell(&input)?;
                assert_eq!(decoded.denominator, 37);
                assert_eq!(decoded.min_nominator, 37);
                assert_eq!(decoded.max_nominator, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolWithdrawConditionReservesRatio::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolWithdrawConditionComplex/0" => {
                let decoded = CoffeePoolWithdrawConditionComplex::from_cell(&input)?;
                assert_eq!(decoded.min_first_amount, TLBCoins::new(37));
                assert_eq!(decoded.min_second_amount, TLBCoins::new(37));
                assert_eq!(decoded.denominator, 37);
                assert_eq!(decoded.min_nominator, 37);
                assert_eq!(decoded.max_nominator, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolWithdrawConditionComplex::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolWithdrawConditionComplex/1" => {
                let decoded = CoffeePoolWithdrawConditionComplex::from_cell(&input)?;
                assert_eq!(decoded.min_first_amount, TLBCoins::new(37));
                assert_eq!(decoded.min_second_amount, TLBCoins::new(37));
                assert_eq!(decoded.denominator, 37);
                assert_eq!(decoded.min_nominator, 37);
                assert_eq!(decoded.max_nominator, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolWithdrawConditionComplex::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolWithdrawLiquidityParams/0" => {
                let decoded = CoffeePoolWithdrawLiquidityParams::from_cell(&input)?;
                assert_eq!(decoded.use_recipient_on_failure, -1);
                assert_eq!(decoded.deadline, 37);
                assert!(decoded.extra_settings.is_none());
                assert!(decoded.on_success.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolWithdrawLiquidityParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolWithdrawLiquidityParams/1" => {
                let decoded = CoffeePoolWithdrawLiquidityParams::from_cell(&input)?;
                assert_eq!(decoded.use_recipient_on_failure, -1);
                assert_eq!(decoded.deadline, 37);
                assert!(decoded.extra_settings.is_some());
                assert!(decoded.on_success.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolWithdrawLiquidityParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolContractUpdate/0" => {
                let decoded = CoffeePoolContractUpdate::from_cell(&input)?;
                assert!(decoded.code.is_none());
                assert!(decoded.data.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolContractUpdate::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolContractUpdate/1" => {
                let decoded = CoffeePoolContractUpdate::from_cell(&input)?;
                assert!(decoded.code.is_some());
                assert!(decoded.data.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolContractUpdate::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolUpdateNoop/0" => {
                let decoded = CoffeePoolUpdateNoop::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolUpdateNoop::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolUpdateNoop/1" => {
                let decoded = CoffeePoolUpdateNoop::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolUpdateNoop::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolUpdateFees/0" => {
                let decoded = CoffeePoolUpdateFees::from_cell(&input)?;
                assert_eq!(decoded.protocol_fee, 37);
                assert_eq!(decoded.lp_fee, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolUpdateFees::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolUpdateFees/1" => {
                let decoded = CoffeePoolUpdateFees::from_cell(&input)?;
                assert_eq!(decoded.protocol_fee, 37);
                assert_eq!(decoded.lp_fee, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolUpdateFees::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolUpdateActive/0" => {
                let decoded = CoffeePoolUpdateActive::from_cell(&input)?;
                assert!(decoded.is_active);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolUpdateActive::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolUpdateActive/1" => {
                let decoded = CoffeePoolUpdateActive::from_cell(&input)?;
                assert!(decoded.is_active);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolUpdateActive::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolUpdateFeesAndActive/0" => {
                let decoded = CoffeePoolUpdateFeesAndActive::from_cell(&input)?;
                assert_eq!(decoded.protocol_fee, 37);
                assert_eq!(decoded.lp_fee, 37);
                assert!(decoded.is_active);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolUpdateFeesAndActive::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolUpdateFeesAndActive/1" => {
                let decoded = CoffeePoolUpdateFeesAndActive::from_cell(&input)?;
                assert_eq!(decoded.protocol_fee, 37);
                assert_eq!(decoded.lp_fee, 37);
                assert!(decoded.is_active);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolUpdateFeesAndActive::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolValuePair/0" => {
                let decoded = CoffeePoolValuePair::from_cell(&input)?;
                assert_eq!(decoded.first, TLBCoins::new(37));
                assert_eq!(decoded.second, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolValuePair::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolValuePair/1" => {
                let decoded = CoffeePoolValuePair::from_cell(&input)?;
                assert_eq!(decoded.first, TLBCoins::new(37));
                assert_eq!(decoded.second, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolValuePair::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolCurveFiStableSwapInternal/0" => {
                let decoded = CoffeeCurveFiStablePoolSwapInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCurveFiStablePoolSwapInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolCurveFiStableSwapInternal/1" => {
                let decoded = CoffeeCurveFiStablePoolSwapInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCurveFiStablePoolSwapInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolCurveFiStableDepositLiquidityInternal/0" => {
                let decoded = CoffeeCurveFiStablePoolDepositLiquidityInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount1, TLBCoins::new(37));
                assert_eq!(decoded.amount2, TLBCoins::new(37));
                assert_eq!(decoded.tx_initiator.workchain, -1);
                assert_eq!(decoded.tx_initiator.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    CoffeeCurveFiStablePoolDepositLiquidityInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok()
                );
            },
            "pool_curve_fi_stable/CoffeePoolCurveFiStableDepositLiquidityInternal/1" => {
                let decoded = CoffeeCurveFiStablePoolDepositLiquidityInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount1, TLBCoins::new(37));
                assert_eq!(decoded.amount2, TLBCoins::new(37));
                assert_eq!(decoded.tx_initiator.workchain, -1);
                assert_eq!(decoded.tx_initiator.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    CoffeeCurveFiStablePoolDepositLiquidityInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok()
                );
            },
            "pool_curve_fi_stable/CoffeePoolCurveFiStableJettonBurnNotification/0" => {
                let decoded = CoffeeCurveFiStablePoolJettonBurnNotificationMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.from_address.workchain, -1);
                assert_eq!(decoded.from_address.hash.as_slice()[31], 37);
                assert!(decoded.response_address.is_none());
                assert!(decoded.custom_payload.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    CoffeeCurveFiStablePoolJettonBurnNotificationMsg::from_cell(&with_trailing_bit(&input)?).is_ok()
                );
            },
            "pool_curve_fi_stable/CoffeePoolCurveFiStableJettonBurnNotification/1" => {
                let decoded = CoffeeCurveFiStablePoolJettonBurnNotificationMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.from_address.workchain, -1);
                assert_eq!(decoded.from_address.hash.as_slice()[31], 37);
                assert!(decoded.response_address.is_some());
                assert!(decoded.custom_payload.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    CoffeeCurveFiStablePoolJettonBurnNotificationMsg::from_cell(&with_trailing_bit(&input)?).is_ok()
                );
            },
            "pool_curve_fi_stable/CoffeePoolCurveFiStableProvideWalletAddress/0" => {
                let decoded = CoffeeCurveFiStablePoolProvideWalletAddressMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.owner_address.workchain, -1);
                assert_eq!(decoded.owner_address.hash.as_slice()[31], 37);
                assert!(decoded.include_address);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCurveFiStablePoolProvideWalletAddressMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolCurveFiStableProvideWalletAddress/1" => {
                let decoded = CoffeeCurveFiStablePoolProvideWalletAddressMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.owner_address.workchain, -1);
                assert_eq!(decoded.owner_address.hash.as_slice()[31], 37);
                assert!(decoded.include_address);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCurveFiStablePoolProvideWalletAddressMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolCurveFiStableWithdrawInternal/0" => {
                let decoded = CoffeeCurveFiStablePoolWithdrawInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.receiver.workchain, -1);
                assert_eq!(decoded.receiver.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCurveFiStablePoolWithdrawInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolCurveFiStableWithdrawInternal/1" => {
                let decoded = CoffeeCurveFiStablePoolWithdrawInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.receiver.workchain, -1);
                assert_eq!(decoded.receiver.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCurveFiStablePoolWithdrawInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolCurveFiStableUpdatePoolInternal/0" => {
                let decoded = CoffeeCurveFiStablePoolUpdatePoolInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert!(decoded.excesses_receiver.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCurveFiStablePoolUpdatePoolInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolCurveFiStableUpdatePoolInternal/1" => {
                let decoded = CoffeeCurveFiStablePoolUpdatePoolInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert!(decoded.excesses_receiver.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCurveFiStablePoolUpdatePoolInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolCurveFiStableUpdateContractInternal/0" => {
                let decoded = CoffeeCurveFiStablePoolUpdateContractInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert!(decoded.excesses_receiver.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    CoffeeCurveFiStablePoolUpdateContractInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok()
                );
            },
            "pool_curve_fi_stable/CoffeePoolCurveFiStableUpdateContractInternal/1" => {
                let decoded = CoffeeCurveFiStablePoolUpdateContractInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert!(decoded.excesses_receiver.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    CoffeeCurveFiStablePoolUpdateContractInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok()
                );
            },
            "pool_curve_fi_stable/CoffeePoolCurveFiStableCreatePoolInternal/0" => {
                let decoded = CoffeeCurveFiStablePoolCreatePoolInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount1, TLBCoins::new(37));
                assert_eq!(decoded.amount2, TLBCoins::new(37));
                assert_eq!(decoded.tx_initiator.workchain, -1);
                assert_eq!(decoded.tx_initiator.hash.as_slice()[31], 37);
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.use_recipient_on_failure, -1);
                assert!(decoded.extra_settings.is_none());
                assert!(decoded.notification_data.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCurveFiStablePoolCreatePoolInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolCurveFiStableCreatePoolInternal/1" => {
                let decoded = CoffeeCurveFiStablePoolCreatePoolInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount1, TLBCoins::new(37));
                assert_eq!(decoded.amount2, TLBCoins::new(37));
                assert_eq!(decoded.tx_initiator.workchain, -1);
                assert_eq!(decoded.tx_initiator.hash.as_slice()[31], 37);
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.use_recipient_on_failure, -1);
                assert!(decoded.extra_settings.is_some());
                assert!(decoded.notification_data.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCurveFiStablePoolCreatePoolInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolCurveFiStableDeploy/0" => {
                let decoded = CoffeeCurveFiStablePoolDeployMsg::from_cell(&input)?;
                assert!(decoded.action.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCurveFiStablePoolDeployMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolCurveFiStableDeploy/1" => {
                let decoded = CoffeeCurveFiStablePoolDeployMsg::from_cell(&input)?;
                assert!(decoded.action.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCurveFiStablePoolDeployMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolCurveFiStablePayoutInternal/0" => {
                let decoded = CoffeeCurveFiStablePoolPayoutInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert!(decoded.notification_data.is_none());
                assert!(decoded.proof.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCurveFiStablePoolPayoutInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolCurveFiStablePayoutInternal/1" => {
                let decoded = CoffeeCurveFiStablePoolPayoutInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert!(decoded.notification_data.is_some());
                assert!(decoded.proof.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCurveFiStablePoolPayoutInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolCurveFiStableSwapInternalOut/0" => {
                let decoded = CoffeeCurveFiStablePoolSwapInternalOutMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCurveFiStablePoolSwapInternalOutMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolCurveFiStableSwapInternalOut/1" => {
                let decoded = CoffeeCurveFiStablePoolSwapInternalOutMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCurveFiStablePoolSwapInternalOutMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolCurveFiStableNotification/0" => {
                let decoded = CoffeeCurveFiStablePoolNotificationMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCurveFiStablePoolNotificationMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolCurveFiStableNotification/1" => {
                let decoded = CoffeeCurveFiStablePoolNotificationMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCurveFiStablePoolNotificationMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolCurveFiStableSwapSucceeded/0" => {
                let decoded = CoffeeCurveFiStablePoolSwapSucceededMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.input_amount, TLBCoins::new(37));
                assert_eq!(decoded.output_amount, TLBCoins::new(37));
                assert_eq!(decoded.protocol_fee, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCurveFiStablePoolSwapSucceededMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolCurveFiStableSwapSucceeded/1" => {
                let decoded = CoffeeCurveFiStablePoolSwapSucceededMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.input_amount, TLBCoins::new(37));
                assert_eq!(decoded.output_amount, TLBCoins::new(37));
                assert_eq!(decoded.protocol_fee, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCurveFiStablePoolSwapSucceededMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolCurveFiStableSwapFailed/0" => {
                let decoded = CoffeeCurveFiStablePoolSwapFailedMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.input_amount, TLBCoins::new(37));
                assert!(decoded.reserves.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCurveFiStablePoolSwapFailedMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolCurveFiStableSwapFailed/1" => {
                let decoded = CoffeeCurveFiStablePoolSwapFailedMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.input_amount, TLBCoins::new(37));
                assert!(decoded.reserves.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCurveFiStablePoolSwapFailedMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolCurveFiStableDepositLiquiditySucceeded/0" => {
                let decoded = CoffeeCurveFiStablePoolDepositLiquiditySucceededMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount1, TLBCoins::new(37));
                assert_eq!(decoded.amount2, TLBCoins::new(37));
                assert_eq!(decoded.lp_amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    CoffeeCurveFiStablePoolDepositLiquiditySucceededMsg::from_cell(&with_trailing_bit(&input)?).is_ok()
                );
            },
            "pool_curve_fi_stable/CoffeePoolCurveFiStableDepositLiquiditySucceeded/1" => {
                let decoded = CoffeeCurveFiStablePoolDepositLiquiditySucceededMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount1, TLBCoins::new(37));
                assert_eq!(decoded.amount2, TLBCoins::new(37));
                assert_eq!(decoded.lp_amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    CoffeeCurveFiStablePoolDepositLiquiditySucceededMsg::from_cell(&with_trailing_bit(&input)?).is_ok()
                );
            },
            "pool_curve_fi_stable/CoffeePoolCurveFiStableDepositLiquidityFailed/0" => {
                let decoded = CoffeeCurveFiStablePoolDepositLiquidityFailedMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount1, TLBCoins::new(37));
                assert_eq!(decoded.amount2, TLBCoins::new(37));
                assert_eq!(decoded.min_lp_amount, TLBCoins::new(37));
                assert_eq!(decoded.total_supply, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    CoffeeCurveFiStablePoolDepositLiquidityFailedMsg::from_cell(&with_trailing_bit(&input)?).is_ok()
                );
            },
            "pool_curve_fi_stable/CoffeePoolCurveFiStableDepositLiquidityFailed/1" => {
                let decoded = CoffeeCurveFiStablePoolDepositLiquidityFailedMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount1, TLBCoins::new(37));
                assert_eq!(decoded.amount2, TLBCoins::new(37));
                assert_eq!(decoded.min_lp_amount, TLBCoins::new(37));
                assert_eq!(decoded.total_supply, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    CoffeeCurveFiStablePoolDepositLiquidityFailedMsg::from_cell(&with_trailing_bit(&input)?).is_ok()
                );
            },
            "pool_curve_fi_stable/CoffeePoolCurveFiStableWithdrawLiquiditySucceeded/0" => {
                let decoded = CoffeeCurveFiStablePoolWithdrawLiquiditySucceededMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount1, TLBCoins::new(37));
                assert_eq!(decoded.amount2, TLBCoins::new(37));
                assert_eq!(decoded.lp_amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    CoffeeCurveFiStablePoolWithdrawLiquiditySucceededMsg::from_cell(&with_trailing_bit(&input)?)
                        .is_ok()
                );
            },
            "pool_curve_fi_stable/CoffeePoolCurveFiStableWithdrawLiquiditySucceeded/1" => {
                let decoded = CoffeeCurveFiStablePoolWithdrawLiquiditySucceededMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount1, TLBCoins::new(37));
                assert_eq!(decoded.amount2, TLBCoins::new(37));
                assert_eq!(decoded.lp_amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    CoffeeCurveFiStablePoolWithdrawLiquiditySucceededMsg::from_cell(&with_trailing_bit(&input)?)
                        .is_ok()
                );
            },
            "pool_curve_fi_stable/CoffeePoolCurveFiStableTakeWalletAddress/0" => {
                let decoded = CoffeeCurveFiStablePoolTakeWalletAddressMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.wallet_address.workchain, -1);
                assert_eq!(decoded.wallet_address.hash.as_slice()[31], 37);
                assert!(decoded.owner_address.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCurveFiStablePoolTakeWalletAddressMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolCurveFiStableTakeWalletAddress/1" => {
                let decoded = CoffeeCurveFiStablePoolTakeWalletAddressMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.wallet_address.workchain, -1);
                assert_eq!(decoded.wallet_address.hash.as_slice()[31], 37);
                assert!(decoded.owner_address.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCurveFiStablePoolTakeWalletAddressMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolCurveFiStableExcesses/0" => {
                let decoded = CoffeeCurveFiStablePoolExcessesMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCurveFiStablePoolExcessesMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolCurveFiStableExcesses/1" => {
                let decoded = CoffeeCurveFiStablePoolExcessesMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCurveFiStablePoolExcessesMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "pool_curve_fi_stable/CoffeePoolCurveFiStableLpWalletInternalTransfer/0" => {
                let decoded = CoffeeCurveFiStablePoolLpWalletInternalTransferMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert!(decoded.from_address.is_none());
                assert_eq!(decoded.response_address.workchain, -1);
                assert_eq!(decoded.response_address.hash.as_slice()[31], 37);
                assert_eq!(decoded.forward_ton_amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
            },
            "pool_curve_fi_stable/CoffeePoolCurveFiStableLpWalletInternalTransfer/1" => {
                let decoded = CoffeeCurveFiStablePoolLpWalletInternalTransferMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert!(decoded.from_address.is_some());
                assert_eq!(decoded.response_address.workchain, -1);
                assert_eq!(decoded.response_address.hash.as_slice()[31], 37);
                assert_eq!(decoded.forward_ton_amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(
                    CoffeeCurveFiStablePoolLpWalletInternalTransferMsg::from_cell(&with_trailing_bit(&input)?).is_ok()
                );
            },
            "staking_item/CoffeeStakingItemStorage/0" => {
                let decoded = CoffeeStakingItemStorage::from_cell(&input)?;
                assert!(decoded.initialized);
                assert_eq!(decoded.nft_id, 37);
                assert_eq!(decoded.master.workchain, -1);
                assert_eq!(decoded.master.hash.as_slice()[31], 37);
                assert_eq!(decoded.owner.workchain, -1);
                assert_eq!(decoded.owner.hash.as_slice()[31], 37);
                assert_eq!(decoded.periods.len(), 0);
                assert_eq!(CoffeeStakingItemStorage::from_cell(&decoded.to_cell()?)?, decoded);
                assert!(CoffeeStakingItemStorage::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_item/CoffeeStakingItemStorage/1" => {
                let decoded = CoffeeStakingItemStorage::from_cell(&input)?;
                assert!(decoded.initialized);
                assert_eq!(decoded.nft_id, 37);
                assert_eq!(decoded.master.workchain, -1);
                assert_eq!(decoded.master.hash.as_slice()[31], 37);
                assert_eq!(decoded.owner.workchain, -1);
                assert_eq!(decoded.owner.hash.as_slice()[31], 37);
                assert_eq!(decoded.periods.len(), 1);
                assert!(decoded.periods.contains_key(&0));
                assert_eq!(CoffeeStakingItemStorage::from_cell(&decoded.to_cell()?)?, decoded);
                assert!(CoffeeStakingItemStorage::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_item/CoffeeStakingAssetData/0" => {
                let decoded = CoffeeStakingAssetData::from_cell(&input)?;
                assert!(decoded.wallet.is_none());
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeStakingAssetData::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_item/CoffeeStakingAssetData/1" => {
                let decoded = CoffeeStakingAssetData::from_cell(&input)?;
                assert!(decoded.wallet.is_some());
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeStakingAssetData::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_item/CoffeeStakingPositionData/0" => {
                let decoded = CoffeeStakingPositionData::from_cell(&input)?;
                assert_eq!(decoded.user_points, TLBCoins::new(37));
                assert_eq!(decoded.additional_points, TLBCoins::new(37));
                assert_eq!(decoded.start_timestamp, 37);
                assert_eq!(decoded.end_timestamp, 37);
                assert_eq!(decoded.period_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeStakingPositionData::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_item/CoffeeStakingPositionData/1" => {
                let decoded = CoffeeStakingPositionData::from_cell(&input)?;
                assert_eq!(decoded.user_points, TLBCoins::new(37));
                assert_eq!(decoded.additional_points, TLBCoins::new(37));
                assert_eq!(decoded.start_timestamp, 37);
                assert_eq!(decoded.end_timestamp, 37);
                assert_eq!(decoded.period_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeStakingPositionData::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_item/CoffeeStakingItemInit/0" => {
                let decoded = CoffeeStakingItemInitMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.owner.workchain, -1);
                assert_eq!(decoded.owner.hash.as_slice()[31], 37);
                assert_eq!(decoded.periods.len(), 0);
                assert_eq!(CoffeeStakingItemInitMsg::from_cell(&decoded.to_cell()?)?, decoded);
                assert!(CoffeeStakingItemInitMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_item/CoffeeStakingItemInit/1" => {
                let decoded = CoffeeStakingItemInitMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.owner.workchain, -1);
                assert_eq!(decoded.owner.hash.as_slice()[31], 37);
                assert_eq!(decoded.periods.len(), 1);
                assert!(decoded.periods.contains_key(&0));
                assert_eq!(CoffeeStakingItemInitMsg::from_cell(&decoded.to_cell()?)?, decoded);
                assert!(CoffeeStakingItemInitMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_item/CoffeeStakingItemUnlock/0" => {
                let decoded = CoffeeStakingItemUnlockMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeStakingItemUnlockMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_item/CoffeeStakingItemUnlock/1" => {
                let decoded = CoffeeStakingItemUnlockMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeStakingItemUnlockMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_item/CoffeeStakingItemExtend/0" => {
                let decoded = CoffeeStakingItemExtendMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.period_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeStakingItemExtendMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_item/CoffeeStakingItemExtend/1" => {
                let decoded = CoffeeStakingItemExtendMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.period_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeStakingItemExtendMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_item/CoffeeStakingItemUnlockNotification/0" => {
                let decoded = CoffeeStakingItemUnlockNotificationMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.nft_id, 37);
                assert_eq!(decoded.owner.workchain, -1);
                assert_eq!(decoded.owner.hash.as_slice()[31], 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeStakingItemUnlockNotificationMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_item/CoffeeStakingItemUnlockNotification/1" => {
                let decoded = CoffeeStakingItemUnlockNotificationMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.nft_id, 37);
                assert_eq!(decoded.owner.workchain, -1);
                assert_eq!(decoded.owner.hash.as_slice()[31], 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeStakingItemUnlockNotificationMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_item/CoffeeStakingItemExtendNotification/0" => {
                let decoded = CoffeeStakingItemExtendNotificationMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.nft_id, 37);
                assert_eq!(decoded.owner.workchain, -1);
                assert_eq!(decoded.owner.hash.as_slice()[31], 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.end_timestamp, 37);
                assert_eq!(decoded.period_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeStakingItemExtendNotificationMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_item/CoffeeStakingItemExtendNotification/1" => {
                let decoded = CoffeeStakingItemExtendNotificationMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.nft_id, 37);
                assert_eq!(decoded.owner.workchain, -1);
                assert_eq!(decoded.owner.hash.as_slice()[31], 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.end_timestamp, 37);
                assert_eq!(decoded.period_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeStakingItemExtendNotificationMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_master/CoffeeStakingMasterStorage/0" => {
                let decoded = CoffeeStakingMasterStorage::from_cell(&input)?;
                assert!(decoded.initialized);
                assert!(decoded.owner.is_none());
                assert!(decoded.distributor.is_none());
                assert_eq!(decoded.vault.workchain, -1);
                assert_eq!(decoded.vault.hash.as_slice()[31], 37);
                assert_eq!(CoffeeStakingMasterStorage::from_cell(&decoded.to_cell()?)?, decoded);
                assert!(CoffeeStakingMasterStorage::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_master/CoffeeStakingMasterStorage/1" => {
                let decoded = CoffeeStakingMasterStorage::from_cell(&input)?;
                assert!(decoded.initialized);
                assert!(decoded.owner.is_some());
                assert!(decoded.distributor.is_some());
                assert_eq!(decoded.vault.workchain, -1);
                assert_eq!(decoded.vault.hash.as_slice()[31], 37);
                assert_eq!(CoffeeStakingMasterStorage::from_cell(&decoded.to_cell()?)?, decoded);
                assert!(CoffeeStakingMasterStorage::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_master/CoffeeStakingMasterStateRefs/0" => {
                let decoded = CoffeeStakingMasterStateRefs::from_cell(&input)?;
                assert_eq!(decoded.rewards.len(), 0);
                assert_eq!(decoded.jettons.len(), 0);
                assert_eq!(decoded.periods.len(), 0);
                assert_eq!(CoffeeStakingMasterStateRefs::from_cell(&decoded.to_cell()?)?, decoded);
                assert!(CoffeeStakingMasterStateRefs::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_master/CoffeeStakingMasterStateRefs/1" => {
                let decoded = CoffeeStakingMasterStateRefs::from_cell(&input)?;
                assert_eq!(decoded.rewards.len(), 1);
                assert!(decoded.rewards.contains_key(&TonHash::ZERO));
                assert_eq!(decoded.jettons.len(), 1);
                assert!(decoded.jettons.contains_key(&TonHash::ZERO));
                assert_eq!(decoded.periods.len(), 1);
                assert!(decoded.periods.contains_key(&0));
                assert_eq!(CoffeeStakingMasterStateRefs::from_cell(&decoded.to_cell()?)?, decoded);
                assert!(CoffeeStakingMasterStateRefs::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_master/CoffeeStakingMasterState/0" => {
                let decoded = CoffeeStakingMasterState::from_cell(&input)?;
                assert_eq!(decoded.total_points, TLBCoins::new(37));
                assert_eq!(decoded.next_item_index, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeStakingMasterState::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_master/CoffeeStakingMasterState/1" => {
                let decoded = CoffeeStakingMasterState::from_cell(&input)?;
                assert_eq!(decoded.total_points, TLBCoins::new(37));
                assert_eq!(decoded.next_item_index, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeStakingMasterState::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_master/CoffeeStakingAssetData/0" => {
                let decoded = CoffeeStakingAssetData::from_cell(&input)?;
                assert!(decoded.wallet.is_none());
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeStakingAssetData::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_master/CoffeeStakingAssetData/1" => {
                let decoded = CoffeeStakingAssetData::from_cell(&input)?;
                assert!(decoded.wallet.is_some());
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeStakingAssetData::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_master/CoffeeStakingPositionData/0" => {
                let decoded = CoffeeStakingPositionData::from_cell(&input)?;
                assert_eq!(decoded.user_points, TLBCoins::new(37));
                assert_eq!(decoded.additional_points, TLBCoins::new(37));
                assert_eq!(decoded.start_timestamp, 37);
                assert_eq!(decoded.end_timestamp, 37);
                assert_eq!(decoded.period_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeStakingPositionData::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_master/CoffeeStakingPositionData/1" => {
                let decoded = CoffeeStakingPositionData::from_cell(&input)?;
                assert_eq!(decoded.user_points, TLBCoins::new(37));
                assert_eq!(decoded.additional_points, TLBCoins::new(37));
                assert_eq!(decoded.start_timestamp, 37);
                assert_eq!(decoded.end_timestamp, 37);
                assert_eq!(decoded.period_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeStakingPositionData::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_master/CoffeeStakingForwardData/0" => {
                let decoded = CoffeeStakingForwardData::from_cell(&input)?;
                assert_eq!(decoded.gas, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeStakingForwardData::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_master/CoffeeStakingForwardData/1" => {
                let decoded = CoffeeStakingForwardData::from_cell(&input)?;
                assert_eq!(decoded.gas, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeStakingForwardData::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_master/CoffeeStakingMasterInit/0" => {
                let decoded = CoffeeStakingMasterInitMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.owner.workchain, -1);
                assert_eq!(decoded.owner.hash.as_slice()[31], 37);
                assert_eq!(decoded.distributor.workchain, -1);
                assert_eq!(decoded.distributor.hash.as_slice()[31], 37);
                assert_eq!(decoded.jettons.len(), 0);
                assert_eq!(decoded.periods.len(), 0);
                assert_eq!(CoffeeStakingMasterInitMsg::from_cell(&decoded.to_cell()?)?, decoded);
                assert!(CoffeeStakingMasterInitMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_master/CoffeeStakingMasterInit/1" => {
                let decoded = CoffeeStakingMasterInitMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.owner.workchain, -1);
                assert_eq!(decoded.owner.hash.as_slice()[31], 37);
                assert_eq!(decoded.distributor.workchain, -1);
                assert_eq!(decoded.distributor.hash.as_slice()[31], 37);
                assert_eq!(decoded.jettons.len(), 1);
                assert!(decoded.jettons.contains_key(&TonHash::ZERO));
                assert_eq!(decoded.periods.len(), 1);
                assert!(decoded.periods.contains_key(&0));
                assert_eq!(CoffeeStakingMasterInitMsg::from_cell(&decoded.to_cell()?)?, decoded);
                assert!(CoffeeStakingMasterInitMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_master/CoffeeStakingUpdateRewards/0" => {
                let decoded = CoffeeStakingUpdateRewardsMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.jetton_wallet.workchain, -1);
                assert_eq!(decoded.jetton_wallet.hash.as_slice()[31], 37);
                assert_eq!(decoded.jetton_amount, TLBCoins::new(37));
                assert_eq!(decoded.duration, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeStakingUpdateRewardsMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_master/CoffeeStakingUpdateRewards/1" => {
                let decoded = CoffeeStakingUpdateRewardsMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.jetton_wallet.workchain, -1);
                assert_eq!(decoded.jetton_wallet.hash.as_slice()[31], 37);
                assert_eq!(decoded.jetton_amount, TLBCoins::new(37));
                assert_eq!(decoded.duration, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeStakingUpdateRewardsMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_master/CoffeeStakingDeposit/0" => {
                let decoded = CoffeeStakingDepositMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.sender.workchain, -1);
                assert_eq!(decoded.sender.hash.as_slice()[31], 37);
                assert_eq!(decoded.jetton_amount, TLBCoins::new(37));
                assert_eq!(decoded.from_user.workchain, -1);
                assert_eq!(decoded.from_user.hash.as_slice()[31], 37);
                assert_eq!(decoded.period_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeStakingDepositMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_master/CoffeeStakingDeposit/1" => {
                let decoded = CoffeeStakingDepositMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.sender.workchain, -1);
                assert_eq!(decoded.sender.hash.as_slice()[31], 37);
                assert_eq!(decoded.jetton_amount, TLBCoins::new(37));
                assert_eq!(decoded.from_user.workchain, -1);
                assert_eq!(decoded.from_user.hash.as_slice()[31], 37);
                assert_eq!(decoded.period_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeStakingDepositMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_master/CoffeeStakingClaimRewards/0" => {
                let decoded = CoffeeStakingClaimRewardsMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.jetton_wallet.workchain, -1);
                assert_eq!(decoded.jetton_wallet.hash.as_slice()[31], 37);
                assert_eq!(decoded.jetton_amount, TLBCoins::new(37));
                assert_eq!(decoded.receiver.workchain, -1);
                assert_eq!(decoded.receiver.hash.as_slice()[31], 37);
                assert!(decoded.payload.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeStakingClaimRewardsMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_master/CoffeeStakingClaimRewards/1" => {
                let decoded = CoffeeStakingClaimRewardsMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.jetton_wallet.workchain, -1);
                assert_eq!(decoded.jetton_wallet.hash.as_slice()[31], 37);
                assert_eq!(decoded.jetton_amount, TLBCoins::new(37));
                assert_eq!(decoded.receiver.workchain, -1);
                assert_eq!(decoded.receiver.hash.as_slice()[31], 37);
                assert!(decoded.payload.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeStakingClaimRewardsMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_master/CoffeeStakingSetDistributor/0" => {
                let decoded = CoffeeStakingSetDistributorMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert!(decoded.distributor.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeStakingSetDistributorMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_master/CoffeeStakingSetDistributor/1" => {
                let decoded = CoffeeStakingSetDistributorMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert!(decoded.distributor.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeStakingSetDistributorMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_master/CoffeeStakingSetOwner/0" => {
                let decoded = CoffeeStakingSetOwnerMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert!(decoded.owner.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeStakingSetOwnerMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_master/CoffeeStakingSetOwner/1" => {
                let decoded = CoffeeStakingSetOwnerMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert!(decoded.owner.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeStakingSetOwnerMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_master/CoffeeStakingSetJettons/0" => {
                let decoded = CoffeeStakingSetJettonsMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.jettons.len(), 0);
                assert_eq!(CoffeeStakingSetJettonsMsg::from_cell(&decoded.to_cell()?)?, decoded);
                assert!(CoffeeStakingSetJettonsMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_master/CoffeeStakingSetJettons/1" => {
                let decoded = CoffeeStakingSetJettonsMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.jettons.len(), 1);
                assert!(decoded.jettons.contains_key(&TonHash::ZERO));
                assert_eq!(CoffeeStakingSetJettonsMsg::from_cell(&decoded.to_cell()?)?, decoded);
                assert!(CoffeeStakingSetJettonsMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_master/CoffeeStakingSetPeriods/0" => {
                let decoded = CoffeeStakingSetPeriodsMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.periods.len(), 0);
                assert_eq!(CoffeeStakingSetPeriodsMsg::from_cell(&decoded.to_cell()?)?, decoded);
                assert!(CoffeeStakingSetPeriodsMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_master/CoffeeStakingSetPeriods/1" => {
                let decoded = CoffeeStakingSetPeriodsMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.periods.len(), 1);
                assert!(decoded.periods.contains_key(&0));
                assert_eq!(CoffeeStakingSetPeriodsMsg::from_cell(&decoded.to_cell()?)?, decoded);
                assert!(CoffeeStakingSetPeriodsMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_master/CoffeeStakingUpdateContract/0" => {
                let decoded = CoffeeStakingUpdateContractMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert!(decoded.new_data.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeStakingUpdateContractMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_master/CoffeeStakingUpdateContract/1" => {
                let decoded = CoffeeStakingUpdateContractMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert!(decoded.new_data.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeStakingUpdateContractMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_master/CoffeeStakingItemExtended/0" => {
                let decoded = CoffeeStakingItemExtendedMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.nft_id, 37);
                assert_eq!(decoded.owner.workchain, -1);
                assert_eq!(decoded.owner.hash.as_slice()[31], 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.end_timestamp, 37);
                assert_eq!(decoded.period_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeStakingItemExtendedMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_master/CoffeeStakingItemExtended/1" => {
                let decoded = CoffeeStakingItemExtendedMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.nft_id, 37);
                assert_eq!(decoded.owner.workchain, -1);
                assert_eq!(decoded.owner.hash.as_slice()[31], 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.end_timestamp, 37);
                assert_eq!(decoded.period_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeStakingItemExtendedMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_master/CoffeeStakingItemUnlocked/0" => {
                let decoded = CoffeeStakingItemUnlockedMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.nft_id, 37);
                assert_eq!(decoded.owner.workchain, -1);
                assert_eq!(decoded.owner.hash.as_slice()[31], 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeStakingItemUnlockedMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_master/CoffeeStakingItemUnlocked/1" => {
                let decoded = CoffeeStakingItemUnlockedMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.nft_id, 37);
                assert_eq!(decoded.owner.workchain, -1);
                assert_eq!(decoded.owner.hash.as_slice()[31], 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeStakingItemUnlockedMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_master/CoffeeStakingMasterInitialized/0" => {
                let decoded = CoffeeStakingMasterInitializedMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeStakingMasterInitializedMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_master/CoffeeStakingMasterInitialized/1" => {
                let decoded = CoffeeStakingMasterInitializedMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeStakingMasterInitializedMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_master/CoffeeStakingRewardsUpdated/0" => {
                let decoded = CoffeeStakingRewardsUpdatedMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.jetton_wallet.workchain, -1);
                assert_eq!(decoded.jetton_wallet.hash.as_slice()[31], 37);
                assert_eq!(decoded.duration, 37);
                assert_eq!(decoded.finish_at, 37);
                assert_eq!(decoded.rewards_rate, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeStakingRewardsUpdatedMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_master/CoffeeStakingRewardsUpdated/1" => {
                let decoded = CoffeeStakingRewardsUpdatedMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.jetton_wallet.workchain, -1);
                assert_eq!(decoded.jetton_wallet.hash.as_slice()[31], 37);
                assert_eq!(decoded.duration, 37);
                assert_eq!(decoded.finish_at, 37);
                assert_eq!(decoded.rewards_rate, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeStakingRewardsUpdatedMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_master/CoffeeStakingRewardsClaimed/0" => {
                let decoded = CoffeeStakingRewardsClaimedMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.jetton_wallet.workchain, -1);
                assert_eq!(decoded.jetton_wallet.hash.as_slice()[31], 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.receiver.workchain, -1);
                assert_eq!(decoded.receiver.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeStakingRewardsClaimedMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_master/CoffeeStakingRewardsClaimed/1" => {
                let decoded = CoffeeStakingRewardsClaimedMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.jetton_wallet.workchain, -1);
                assert_eq!(decoded.jetton_wallet.hash.as_slice()[31], 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.receiver.workchain, -1);
                assert_eq!(decoded.receiver.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeStakingRewardsClaimedMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_master/CoffeeStakingDistributorUpdated/0" => {
                let decoded = CoffeeStakingDistributorUpdatedMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeStakingDistributorUpdatedMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_master/CoffeeStakingDistributorUpdated/1" => {
                let decoded = CoffeeStakingDistributorUpdatedMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeStakingDistributorUpdatedMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_master/CoffeeStakingOwnerUpdated/0" => {
                let decoded = CoffeeStakingOwnerUpdatedMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeStakingOwnerUpdatedMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_master/CoffeeStakingOwnerUpdated/1" => {
                let decoded = CoffeeStakingOwnerUpdatedMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeStakingOwnerUpdatedMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_master/CoffeeStakingJettonsUpdated/0" => {
                let decoded = CoffeeStakingJettonsUpdatedMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.jettons.len(), 0);
                assert_eq!(CoffeeStakingJettonsUpdatedMsg::from_cell(&decoded.to_cell()?)?, decoded);
                assert!(CoffeeStakingJettonsUpdatedMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_master/CoffeeStakingJettonsUpdated/1" => {
                let decoded = CoffeeStakingJettonsUpdatedMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.jettons.len(), 1);
                assert!(decoded.jettons.contains_key(&TonHash::ZERO));
                assert_eq!(CoffeeStakingJettonsUpdatedMsg::from_cell(&decoded.to_cell()?)?, decoded);
                assert!(CoffeeStakingJettonsUpdatedMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_master/CoffeeStakingPeriodsUpdated/0" => {
                let decoded = CoffeeStakingPeriodsUpdatedMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.periods.len(), 0);
                assert_eq!(CoffeeStakingPeriodsUpdatedMsg::from_cell(&decoded.to_cell()?)?, decoded);
                assert!(CoffeeStakingPeriodsUpdatedMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_master/CoffeeStakingPeriodsUpdated/1" => {
                let decoded = CoffeeStakingPeriodsUpdatedMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.periods.len(), 1);
                assert!(decoded.periods.contains_key(&0));
                assert_eq!(CoffeeStakingPeriodsUpdatedMsg::from_cell(&decoded.to_cell()?)?, decoded);
                assert!(CoffeeStakingPeriodsUpdatedMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_vault/CoffeeStakingVaultStorage/0" => {
                let decoded = CoffeeStakingVaultStorage::from_cell(&input)?;
                assert!(decoded.initialized);
                assert!(decoded.owner.is_none());
                assert_eq!(decoded.jettons.len(), 0);
                assert_eq!(decoded.periods.len(), 0);
                assert_eq!(CoffeeStakingVaultStorage::from_cell(&decoded.to_cell()?)?, decoded);
                assert!(CoffeeStakingVaultStorage::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_vault/CoffeeStakingVaultStorage/1" => {
                let decoded = CoffeeStakingVaultStorage::from_cell(&input)?;
                assert!(decoded.initialized);
                assert!(decoded.owner.is_some());
                assert_eq!(decoded.jettons.len(), 1);
                assert!(decoded.jettons.contains_key(&TonHash::ZERO));
                assert_eq!(decoded.periods.len(), 1);
                assert!(decoded.periods.contains_key(&0));
                assert_eq!(CoffeeStakingVaultStorage::from_cell(&decoded.to_cell()?)?, decoded);
                assert!(CoffeeStakingVaultStorage::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_vault/CoffeeStakingLockForwardPayload/0" => {
                let decoded = CoffeeStakingLockForwardPayloadMsg::from_cell(&input)?;
                assert_eq!(decoded.period_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeStakingLockForwardPayloadMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_vault/CoffeeStakingLockForwardPayload/1" => {
                let decoded = CoffeeStakingLockForwardPayloadMsg::from_cell(&input)?;
                assert_eq!(decoded.period_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeStakingLockForwardPayloadMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_vault/CoffeeStakingVaultInit/0" => {
                let decoded = CoffeeStakingVaultInitMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.owner.workchain, -1);
                assert_eq!(decoded.owner.hash.as_slice()[31], 37);
                assert_eq!(decoded.jettons.len(), 0);
                assert_eq!(decoded.periods.len(), 0);
                assert_eq!(CoffeeStakingVaultInitMsg::from_cell(&decoded.to_cell()?)?, decoded);
                assert!(CoffeeStakingVaultInitMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_vault/CoffeeStakingVaultInit/1" => {
                let decoded = CoffeeStakingVaultInitMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.owner.workchain, -1);
                assert_eq!(decoded.owner.hash.as_slice()[31], 37);
                assert_eq!(decoded.jettons.len(), 1);
                assert!(decoded.jettons.contains_key(&TonHash::ZERO));
                assert_eq!(decoded.periods.len(), 1);
                assert!(decoded.periods.contains_key(&0));
                assert_eq!(CoffeeStakingVaultInitMsg::from_cell(&decoded.to_cell()?)?, decoded);
                assert!(CoffeeStakingVaultInitMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_vault/CoffeeStakingVaultTransferNotification/0" => {
                let decoded = CoffeeCrossDexJettonTransferNotificationMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert!(decoded.sender.is_none());
                assert_eq!(decoded.to_cell()?, input);
            },
            "staking_vault/CoffeeStakingVaultTransferNotification/1" => {
                let decoded = CoffeeCrossDexJettonTransferNotificationMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert!(decoded.sender.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCrossDexJettonTransferNotificationMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_vault/CoffeeStakingVaultSetOwner/0" => {
                let decoded = CoffeeStakingVaultSetOwnerMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert!(decoded.owner.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeStakingVaultSetOwnerMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_vault/CoffeeStakingVaultSetOwner/1" => {
                let decoded = CoffeeStakingVaultSetOwnerMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert!(decoded.owner.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeStakingVaultSetOwnerMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_vault/CoffeeStakingVaultSetJettons/0" => {
                let decoded = CoffeeStakingVaultSetJettonsMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.jettons.len(), 0);
                assert_eq!(CoffeeStakingVaultSetJettonsMsg::from_cell(&decoded.to_cell()?)?, decoded);
                assert!(CoffeeStakingVaultSetJettonsMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_vault/CoffeeStakingVaultSetJettons/1" => {
                let decoded = CoffeeStakingVaultSetJettonsMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.jettons.len(), 1);
                assert!(decoded.jettons.contains_key(&TonHash::ZERO));
                assert_eq!(CoffeeStakingVaultSetJettonsMsg::from_cell(&decoded.to_cell()?)?, decoded);
                assert!(CoffeeStakingVaultSetJettonsMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_vault/CoffeeStakingVaultSetPeriods/0" => {
                let decoded = CoffeeStakingVaultSetPeriodsMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.periods.len(), 0);
                assert_eq!(CoffeeStakingVaultSetPeriodsMsg::from_cell(&decoded.to_cell()?)?, decoded);
                assert!(CoffeeStakingVaultSetPeriodsMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_vault/CoffeeStakingVaultSetPeriods/1" => {
                let decoded = CoffeeStakingVaultSetPeriodsMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.periods.len(), 1);
                assert!(decoded.periods.contains_key(&0));
                assert_eq!(CoffeeStakingVaultSetPeriodsMsg::from_cell(&decoded.to_cell()?)?, decoded);
                assert!(CoffeeStakingVaultSetPeriodsMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_vault/CoffeeStakingVaultUpdateContract/0" => {
                let decoded = CoffeeStakingVaultUpdateContractMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert!(decoded.new_data.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeStakingVaultUpdateContractMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_vault/CoffeeStakingVaultUpdateContract/1" => {
                let decoded = CoffeeStakingVaultUpdateContractMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert!(decoded.new_data.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeStakingVaultUpdateContractMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_vault/CoffeeStakingVaultMasterInit/0" => {
                let decoded = CoffeeStakingVaultMasterInitMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.owner.workchain, -1);
                assert_eq!(decoded.owner.hash.as_slice()[31], 37);
                assert_eq!(decoded.distributor.workchain, -1);
                assert_eq!(decoded.distributor.hash.as_slice()[31], 37);
                assert_eq!(decoded.jettons.len(), 0);
                assert_eq!(decoded.periods.len(), 0);
                assert_eq!(CoffeeStakingVaultMasterInitMsg::from_cell(&decoded.to_cell()?)?, decoded);
                assert!(CoffeeStakingVaultMasterInitMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_vault/CoffeeStakingVaultMasterInit/1" => {
                let decoded = CoffeeStakingVaultMasterInitMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.owner.workchain, -1);
                assert_eq!(decoded.owner.hash.as_slice()[31], 37);
                assert_eq!(decoded.distributor.workchain, -1);
                assert_eq!(decoded.distributor.hash.as_slice()[31], 37);
                assert_eq!(decoded.jettons.len(), 1);
                assert!(decoded.jettons.contains_key(&TonHash::ZERO));
                assert_eq!(decoded.periods.len(), 1);
                assert!(decoded.periods.contains_key(&0));
                assert_eq!(CoffeeStakingVaultMasterInitMsg::from_cell(&decoded.to_cell()?)?, decoded);
                assert!(CoffeeStakingVaultMasterInitMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_vault/CoffeeStakingVaultDepositToMaster/0" => {
                let decoded = CoffeeStakingVaultDepositToMasterMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.sender.workchain, -1);
                assert_eq!(decoded.sender.hash.as_slice()[31], 37);
                assert_eq!(decoded.jetton_amount, TLBCoins::new(37));
                assert_eq!(decoded.from_user.workchain, -1);
                assert_eq!(decoded.from_user.hash.as_slice()[31], 37);
                assert_eq!(decoded.period_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeStakingVaultDepositToMasterMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_vault/CoffeeStakingVaultDepositToMaster/1" => {
                let decoded = CoffeeStakingVaultDepositToMasterMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.sender.workchain, -1);
                assert_eq!(decoded.sender.hash.as_slice()[31], 37);
                assert_eq!(decoded.jetton_amount, TLBCoins::new(37));
                assert_eq!(decoded.from_user.workchain, -1);
                assert_eq!(decoded.from_user.hash.as_slice()[31], 37);
                assert_eq!(decoded.period_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeStakingVaultDepositToMasterMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_vault/CoffeeStakingVaultItemInit/0" => {
                let decoded = CoffeeStakingVaultItemInitMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.owner.workchain, -1);
                assert_eq!(decoded.owner.hash.as_slice()[31], 37);
                assert_eq!(decoded.periods.len(), 0);
                assert_eq!(CoffeeStakingVaultItemInitMsg::from_cell(&decoded.to_cell()?)?, decoded);
                assert!(CoffeeStakingVaultItemInitMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_vault/CoffeeStakingVaultItemInit/1" => {
                let decoded = CoffeeStakingVaultItemInitMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.owner.workchain, -1);
                assert_eq!(decoded.owner.hash.as_slice()[31], 37);
                assert_eq!(decoded.periods.len(), 1);
                assert!(decoded.periods.contains_key(&0));
                assert_eq!(CoffeeStakingVaultItemInitMsg::from_cell(&decoded.to_cell()?)?, decoded);
                assert!(CoffeeStakingVaultItemInitMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "staking_vault/CoffeeStakingVaultJettonRefund/0" => {
                let decoded = CoffeeStakingVaultJettonRefundMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.destination.workchain, -1);
                assert_eq!(decoded.destination.hash.as_slice()[31], 37);
                assert_eq!(decoded.response_destination.workchain, -1);
                assert_eq!(decoded.response_destination.hash.as_slice()[31], 37);
                assert!(decoded.custom_payload.is_none());
                assert_eq!(decoded.forward_ton_amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
            },
            "staking_vault/CoffeeStakingVaultJettonRefund/1" => {
                let decoded = CoffeeStakingVaultJettonRefundMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.destination.workchain, -1);
                assert_eq!(decoded.destination.hash.as_slice()[31], 37);
                assert_eq!(decoded.response_destination.workchain, -1);
                assert_eq!(decoded.response_destination.hash.as_slice()[31], 37);
                assert!(decoded.custom_payload.is_some());
                assert_eq!(decoded.forward_ton_amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeStakingVaultJettonRefundMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_extra/CoffeeVaultExtraStorage/0" => {
                let decoded = CoffeeVaultExtraStorage::from_cell(&input)?;
                assert_eq!(decoded.factory_initialization_marker, None);
                assert_eq!(decoded.to_cell()?, input);
            },
            "vault_extra/CoffeeVaultExtraStorage/1" => {
                let decoded = CoffeeVaultExtraStorage::from_cell(&input)?;
                assert_eq!(decoded.factory_initialization_marker, Some(true));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeVaultExtraStorage::from_cell(&with_trailing_bit(&input)?).is_err());
            },
            "vault_extra/CoffeeVaultInitData/0" => {
                let decoded = CoffeeVaultInitData::from_cell(&input)?;
                assert_eq!(decoded.factory_address.workchain, -1);
                assert_eq!(decoded.factory_address.hash.as_slice()[31], 37);
                assert_eq!(decoded.contract_type, 3);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeVaultInitData::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_extra/CoffeeVaultInitData/1" => {
                let decoded = CoffeeVaultInitData::from_cell(&input)?;
                assert_eq!(decoded.factory_address.workchain, -1);
                assert_eq!(decoded.factory_address.hash.as_slice()[31], 37);
                assert_eq!(decoded.contract_type, 3);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeVaultInitData::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_extra/CoffeeNativeAsset/0" => {
                let decoded = CoffeeNativeAsset::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeNativeAsset::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_extra/CoffeeNativeAsset/1" => {
                let decoded = CoffeeNativeAsset::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeNativeAsset::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_extra/CoffeeJettonAsset/0" => {
                let decoded = CoffeeJettonAsset::from_cell(&input)?;
                assert_eq!(decoded.chain, 37);
                assert_eq!(decoded.hash, U256::from(37u8));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeJettonAsset::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_extra/CoffeeJettonAsset/1" => {
                let decoded = CoffeeJettonAsset::from_cell(&input)?;
                assert_eq!(decoded.chain, 37);
                assert_eq!(decoded.hash, U256::from(37u8));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeJettonAsset::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_extra/CoffeeExtraAsset/0" => {
                let decoded = CoffeeExtraAsset::from_cell(&input)?;
                assert_eq!(decoded.id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeExtraAsset::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_extra/CoffeeExtraAsset/1" => {
                let decoded = CoffeeExtraAsset::from_cell(&input)?;
                assert_eq!(decoded.id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeExtraAsset::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_extra/CoffeeNoPreviousAssetHint/0" => {
                let decoded = CoffeeNoPreviousAssetHint::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeNoPreviousAssetHint::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_extra/CoffeeNoPreviousAssetHint/1" => {
                let decoded = CoffeeNoPreviousAssetHint::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeNoPreviousAssetHint::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_extra/CoffeeSomePreviousAssetHint/0" => {
                let decoded = CoffeeSomePreviousAssetHint::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeSomePreviousAssetHint::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_extra/CoffeeSomePreviousAssetHint/1" => {
                let decoded = CoffeeSomePreviousAssetHint::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeSomePreviousAssetHint::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_extra/CoffeeConstantProductAmm/0" => {
                let decoded = CoffeeConstantProductAmm::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeConstantProductAmm::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_extra/CoffeeConstantProductAmm/1" => {
                let decoded = CoffeeConstantProductAmm::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeConstantProductAmm::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_extra/CoffeeCurveFiStableAmm/0" => {
                let decoded = CoffeeCurveFiStableAmm::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCurveFiStableAmm::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_extra/CoffeeCurveFiStableAmm/1" => {
                let decoded = CoffeeCurveFiStableAmm::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCurveFiStableAmm::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_extra/CoffeeNoDepositLiquidityCondition/0" => {
                let decoded = CoffeeNoDepositLiquidityCondition::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeNoDepositLiquidityCondition::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_extra/CoffeeNoDepositLiquidityCondition/1" => {
                let decoded = CoffeeNoDepositLiquidityCondition::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeNoDepositLiquidityCondition::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_extra/CoffeeLpQuantityDepositLiquidityCondition/0" => {
                let decoded = CoffeeLpQuantityDepositLiquidityCondition::from_cell(&input)?;
                assert_eq!(decoded.min_lp_amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeLpQuantityDepositLiquidityCondition::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_extra/CoffeeLpQuantityDepositLiquidityCondition/1" => {
                let decoded = CoffeeLpQuantityDepositLiquidityCondition::from_cell(&input)?;
                assert_eq!(decoded.min_lp_amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeLpQuantityDepositLiquidityCondition::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_extra/CoffeeReservesRatioDepositLiquidityCondition/0" => {
                let decoded = CoffeeReservesRatioDepositLiquidityCondition::from_cell(&input)?;
                assert_eq!(decoded.denominator, 37);
                assert_eq!(decoded.min_nominator, 37);
                assert_eq!(decoded.max_nominator, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeReservesRatioDepositLiquidityCondition::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_extra/CoffeeReservesRatioDepositLiquidityCondition/1" => {
                let decoded = CoffeeReservesRatioDepositLiquidityCondition::from_cell(&input)?;
                assert_eq!(decoded.denominator, 37);
                assert_eq!(decoded.min_nominator, 37);
                assert_eq!(decoded.max_nominator, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeReservesRatioDepositLiquidityCondition::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_extra/CoffeeComplexDepositLiquidityCondition/0" => {
                let decoded = CoffeeComplexDepositLiquidityCondition::from_cell(&input)?;
                assert_eq!(decoded.min_lp_amount, TLBCoins::new(37));
                assert_eq!(decoded.denominator, 37);
                assert_eq!(decoded.min_nominator, 37);
                assert_eq!(decoded.max_nominator, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeComplexDepositLiquidityCondition::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_extra/CoffeeComplexDepositLiquidityCondition/1" => {
                let decoded = CoffeeComplexDepositLiquidityCondition::from_cell(&input)?;
                assert_eq!(decoded.min_lp_amount, TLBCoins::new(37));
                assert_eq!(decoded.denominator, 37);
                assert_eq!(decoded.min_nominator, 37);
                assert_eq!(decoded.max_nominator, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeComplexDepositLiquidityCondition::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_extra/CoffeePoolParams/0" => {
                let decoded = CoffeePoolParams::from_cell(&input)?;
                assert!(decoded.amm_settings.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_extra/CoffeePoolParams/1" => {
                let decoded = CoffeePoolParams::from_cell(&input)?;
                assert!(decoded.amm_settings.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_extra/CoffeePublicPoolCreationParams/0" => {
                let decoded = CoffeePublicPoolCreationParams::from_cell(&input)?;
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.use_recipient_on_failure, -1);
                assert!(decoded.notification_data.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePublicPoolCreationParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_extra/CoffeePublicPoolCreationParams/1" => {
                let decoded = CoffeePublicPoolCreationParams::from_cell(&input)?;
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.use_recipient_on_failure, -1);
                assert!(decoded.notification_data.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePublicPoolCreationParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_extra/CoffeePrivatePoolCreationParams/0" => {
                let decoded = CoffeePrivatePoolCreationParams::from_cell(&input)?;
                assert_eq!(decoded.is_active, 1);
                assert!(decoded.extra_settings.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePrivatePoolCreationParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_extra/CoffeePrivatePoolCreationParams/1" => {
                let decoded = CoffeePrivatePoolCreationParams::from_cell(&input)?;
                assert_eq!(decoded.is_active, 1);
                assert!(decoded.extra_settings.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePrivatePoolCreationParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_extra/CoffeePoolCreationParams/0" => {
                let decoded = CoffeePoolCreationParams::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolCreationParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_extra/CoffeePoolCreationParams/1" => {
                let decoded = CoffeePoolCreationParams::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolCreationParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_extra/CoffeeNotificationDataSingle/0" => {
                let decoded = CoffeeNotificationDataSingle::from_cell(&input)?;
                assert!(decoded.receiver.is_none());
                assert_eq!(decoded.fwd_gas, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeNotificationDataSingle::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_extra/CoffeeNotificationDataSingle/1" => {
                let decoded = CoffeeNotificationDataSingle::from_cell(&input)?;
                assert!(decoded.receiver.is_some());
                assert_eq!(decoded.fwd_gas, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeNotificationDataSingle::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_extra/CoffeeNotificationData/0" => {
                let decoded = CoffeeNotificationData::from_cell(&input)?;
                assert!(decoded.on_success.is_none());
                assert!(decoded.on_failure.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeNotificationData::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_extra/CoffeeNotificationData/1" => {
                let decoded = CoffeeNotificationData::from_cell(&input)?;
                assert!(decoded.on_success.is_some());
                assert!(decoded.on_failure.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeNotificationData::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_extra/CoffeeSwapParams/0" => {
                let decoded = CoffeeSwapParams::from_cell(&input)?;
                assert_eq!(decoded.deadline, 37);
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert!(decoded.referral.is_none());
                assert!(decoded.notification_data.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeSwapParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_extra/CoffeeSwapParams/1" => {
                let decoded = CoffeeSwapParams::from_cell(&input)?;
                assert_eq!(decoded.deadline, 37);
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert!(decoded.referral.is_some());
                assert!(decoded.notification_data.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeSwapParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_extra/CoffeeSwapStepParams/0" => {
                let decoded = CoffeeSwapStepParams::from_cell(&input)?;
                assert_eq!(decoded.pool_address_hash, U256::from(37u8));
                assert_eq!(decoded.min_output_amount, TLBCoins::new(37));
                assert!(decoded.next.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeSwapStepParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_extra/CoffeeSwapStepParams/1" => {
                let decoded = CoffeeSwapStepParams::from_cell(&input)?;
                assert_eq!(decoded.pool_address_hash, U256::from(37u8));
                assert_eq!(decoded.min_output_amount, TLBCoins::new(37));
                assert!(decoded.next.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeSwapStepParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_extra/CoffeeSwapStepInternalParams/0" => {
                let decoded = CoffeeSwapStepInternalParams::from_cell(&input)?;
                assert_eq!(decoded.previous_amount, TLBCoins::new(37));
                assert_eq!(decoded.min_output_amount, TLBCoins::new(37));
                assert!(decoded.next.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeSwapStepInternalParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_extra/CoffeeSwapStepInternalParams/1" => {
                let decoded = CoffeeSwapStepInternalParams::from_cell(&input)?;
                assert_eq!(decoded.previous_amount, TLBCoins::new(37));
                assert_eq!(decoded.min_output_amount, TLBCoins::new(37));
                assert!(decoded.next.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeSwapStepInternalParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_extra/CoffeeDepositLiquidityParamsTrimmed/0" => {
                let decoded = CoffeeDepositLiquidityParamsTrimmed::from_cell(&input)?;
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.use_recipient_on_failure, -1);
                assert_eq!(decoded.referral.workchain, -1);
                assert_eq!(decoded.referral.hash.as_slice()[31], 37);
                assert_eq!(decoded.deadline, 37);
                assert!(decoded.extra_settings.is_none());
                assert!(decoded.notification_data.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeDepositLiquidityParamsTrimmed::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_extra/CoffeeDepositLiquidityParamsTrimmed/1" => {
                let decoded = CoffeeDepositLiquidityParamsTrimmed::from_cell(&input)?;
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.use_recipient_on_failure, -1);
                assert_eq!(decoded.referral.workchain, -1);
                assert_eq!(decoded.referral.hash.as_slice()[31], 37);
                assert_eq!(decoded.deadline, 37);
                assert!(decoded.extra_settings.is_some());
                assert!(decoded.notification_data.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeDepositLiquidityParamsTrimmed::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_extra/CoffeeDepositLiquidityParams/0" => {
                let decoded = CoffeeDepositLiquidityParams::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeDepositLiquidityParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_extra/CoffeeDepositLiquidityParams/1" => {
                let decoded = CoffeeDepositLiquidityParams::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeDepositLiquidityParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_extra/CoffeeContractUpdate/0" => {
                let decoded = CoffeeContractUpdate::from_cell(&input)?;
                assert!(decoded.code.is_none());
                assert!(decoded.data.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeContractUpdate::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_extra/CoffeeContractUpdate/1" => {
                let decoded = CoffeeContractUpdate::from_cell(&input)?;
                assert!(decoded.code.is_some());
                assert!(decoded.data.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeContractUpdate::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_extra/CoffeePayoutInternal/0" => {
                let decoded = CoffeePayoutInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert!(decoded.notification_data.is_none());
                assert!(decoded.proof.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePayoutInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_extra/CoffeePayoutInternal/1" => {
                let decoded = CoffeePayoutInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert!(decoded.notification_data.is_some());
                assert!(decoded.proof.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePayoutInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_extra/CoffeeSwapExtra/0" => {
                let decoded = CoffeeSwapExtraMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeSwapExtraMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_extra/CoffeeSwapExtra/1" => {
                let decoded = CoffeeSwapExtraMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeSwapExtraMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_extra/CoffeeDepositLiquidityExtra/0" => {
                let decoded = CoffeeDepositLiquidityExtraMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeDepositLiquidityExtraMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_extra/CoffeeDepositLiquidityExtra/1" => {
                let decoded = CoffeeDepositLiquidityExtraMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeDepositLiquidityExtraMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_extra/CoffeeCreatePoolExtra/0" => {
                let decoded = CoffeeCreatePoolExtraMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCreatePoolExtraMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_extra/CoffeeCreatePoolExtra/1" => {
                let decoded = CoffeeCreatePoolExtraMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCreatePoolExtraMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_extra/CoffeeUpdateContractInternal/0" => {
                let decoded = CoffeeUpdateContractInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.excesses_receiver.workchain, -1);
                assert_eq!(decoded.excesses_receiver.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeUpdateContractInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_extra/CoffeeUpdateContractInternal/1" => {
                let decoded = CoffeeUpdateContractInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.excesses_receiver.workchain, -1);
                assert_eq!(decoded.excesses_receiver.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeUpdateContractInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_extra/CoffeeNotification/0" => {
                let decoded = CoffeeNotificationMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeNotificationMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_extra/CoffeeNotification/1" => {
                let decoded = CoffeeNotificationMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeNotificationMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_extra/CoffeePayout/0" => {
                let decoded = CoffeePayoutMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePayoutMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_extra/CoffeePayout/1" => {
                let decoded = CoffeePayoutMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePayoutMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_extra/CoffeeSwapInternal/0" => {
                let decoded = CoffeeSwapInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeSwapInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_extra/CoffeeSwapInternal/1" => {
                let decoded = CoffeeSwapInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeSwapInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_extra/CoffeeCreateLiquidityDepositoryRequest/0" => {
                let decoded = CoffeeCreateLiquidityDepositoryRequestMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.sender.workchain, -1);
                assert_eq!(decoded.sender.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCreateLiquidityDepositoryRequestMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_extra/CoffeeCreateLiquidityDepositoryRequest/1" => {
                let decoded = CoffeeCreateLiquidityDepositoryRequestMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.sender.workchain, -1);
                assert_eq!(decoded.sender.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCreateLiquidityDepositoryRequestMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_extra/CoffeeCreatePoolCreatorRequest/0" => {
                let decoded = CoffeeCreatePoolCreatorRequestMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.sender.workchain, -1);
                assert_eq!(decoded.sender.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCreatePoolCreatorRequestMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_extra/CoffeeCreatePoolCreatorRequest/1" => {
                let decoded = CoffeeCreatePoolCreatorRequestMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.sender.workchain, -1);
                assert_eq!(decoded.sender.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCreatePoolCreatorRequestMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_extra/CoffeeExcesses/0" => {
                let decoded = CoffeeExcessesMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeExcessesMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_extra/CoffeeExcesses/1" => {
                let decoded = CoffeeExcessesMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeExcessesMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeeVaultJettonActiveStorage/0" => {
                let decoded = CoffeeVaultJettonActiveStorage::from_cell(&input)?;
                assert!(decoded.is_active);
                assert_eq!(decoded.jetton_wallet_address.workchain, -1);
                assert_eq!(decoded.jetton_wallet_address.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeVaultJettonActiveStorage::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeeVaultJettonActiveStorage/1" => {
                let decoded = CoffeeVaultJettonActiveStorage::from_cell(&input)?;
                assert!(decoded.is_active);
                assert_eq!(decoded.jetton_wallet_address.workchain, -1);
                assert_eq!(decoded.jetton_wallet_address.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeVaultJettonActiveStorage::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeeVaultJettonInactiveStorage/0" => {
                let decoded = CoffeeVaultJettonInactiveStorage::from_cell(&input)?;
                assert!(!decoded.is_active);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeVaultJettonInactiveStorage::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeeVaultJettonInactiveStorage/1" => {
                let decoded = CoffeeVaultJettonInactiveStorage::from_cell(&input)?;
                assert!(!decoded.is_active);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeVaultJettonInactiveStorage::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeeVaultInitData/0" => {
                let decoded = CoffeeVaultInitData::from_cell(&input)?;
                assert_eq!(decoded.factory_address.workchain, -1);
                assert_eq!(decoded.factory_address.hash.as_slice()[31], 37);
                assert_eq!(decoded.contract_type, 3);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeVaultInitData::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeeVaultInitData/1" => {
                let decoded = CoffeeVaultInitData::from_cell(&input)?;
                assert_eq!(decoded.factory_address.workchain, -1);
                assert_eq!(decoded.factory_address.hash.as_slice()[31], 37);
                assert_eq!(decoded.contract_type, 3);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeVaultInitData::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeeNativeAsset/0" => {
                let decoded = CoffeeNativeAsset::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeNativeAsset::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeeNativeAsset/1" => {
                let decoded = CoffeeNativeAsset::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeNativeAsset::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeeJettonAsset/0" => {
                let decoded = CoffeeJettonAsset::from_cell(&input)?;
                assert_eq!(decoded.chain, 37);
                assert_eq!(decoded.hash, U256::from(37u8));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeJettonAsset::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeeJettonAsset/1" => {
                let decoded = CoffeeJettonAsset::from_cell(&input)?;
                assert_eq!(decoded.chain, 37);
                assert_eq!(decoded.hash, U256::from(37u8));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeJettonAsset::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeeExtraAsset/0" => {
                let decoded = CoffeeExtraAsset::from_cell(&input)?;
                assert_eq!(decoded.id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeExtraAsset::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeeExtraAsset/1" => {
                let decoded = CoffeeExtraAsset::from_cell(&input)?;
                assert_eq!(decoded.id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeExtraAsset::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeeNoPreviousAssetHint/0" => {
                let decoded = CoffeeNoPreviousAssetHint::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeNoPreviousAssetHint::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeeNoPreviousAssetHint/1" => {
                let decoded = CoffeeNoPreviousAssetHint::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeNoPreviousAssetHint::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeeSomePreviousAssetHint/0" => {
                let decoded = CoffeeSomePreviousAssetHint::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeSomePreviousAssetHint::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeeSomePreviousAssetHint/1" => {
                let decoded = CoffeeSomePreviousAssetHint::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeSomePreviousAssetHint::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeeConstantProductAmm/0" => {
                let decoded = CoffeeConstantProductAmm::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeConstantProductAmm::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeeConstantProductAmm/1" => {
                let decoded = CoffeeConstantProductAmm::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeConstantProductAmm::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeeCurveFiStableAmm/0" => {
                let decoded = CoffeeCurveFiStableAmm::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCurveFiStableAmm::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeeCurveFiStableAmm/1" => {
                let decoded = CoffeeCurveFiStableAmm::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCurveFiStableAmm::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeeNoDepositLiquidityCondition/0" => {
                let decoded = CoffeeNoDepositLiquidityCondition::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeNoDepositLiquidityCondition::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeeNoDepositLiquidityCondition/1" => {
                let decoded = CoffeeNoDepositLiquidityCondition::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeNoDepositLiquidityCondition::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeeLpQuantityDepositLiquidityCondition/0" => {
                let decoded = CoffeeLpQuantityDepositLiquidityCondition::from_cell(&input)?;
                assert_eq!(decoded.min_lp_amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeLpQuantityDepositLiquidityCondition::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeeLpQuantityDepositLiquidityCondition/1" => {
                let decoded = CoffeeLpQuantityDepositLiquidityCondition::from_cell(&input)?;
                assert_eq!(decoded.min_lp_amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeLpQuantityDepositLiquidityCondition::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeeReservesRatioDepositLiquidityCondition/0" => {
                let decoded = CoffeeReservesRatioDepositLiquidityCondition::from_cell(&input)?;
                assert_eq!(decoded.denominator, 37);
                assert_eq!(decoded.min_nominator, 37);
                assert_eq!(decoded.max_nominator, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeReservesRatioDepositLiquidityCondition::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeeReservesRatioDepositLiquidityCondition/1" => {
                let decoded = CoffeeReservesRatioDepositLiquidityCondition::from_cell(&input)?;
                assert_eq!(decoded.denominator, 37);
                assert_eq!(decoded.min_nominator, 37);
                assert_eq!(decoded.max_nominator, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeReservesRatioDepositLiquidityCondition::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeeComplexDepositLiquidityCondition/0" => {
                let decoded = CoffeeComplexDepositLiquidityCondition::from_cell(&input)?;
                assert_eq!(decoded.min_lp_amount, TLBCoins::new(37));
                assert_eq!(decoded.denominator, 37);
                assert_eq!(decoded.min_nominator, 37);
                assert_eq!(decoded.max_nominator, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeComplexDepositLiquidityCondition::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeeComplexDepositLiquidityCondition/1" => {
                let decoded = CoffeeComplexDepositLiquidityCondition::from_cell(&input)?;
                assert_eq!(decoded.min_lp_amount, TLBCoins::new(37));
                assert_eq!(decoded.denominator, 37);
                assert_eq!(decoded.min_nominator, 37);
                assert_eq!(decoded.max_nominator, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeComplexDepositLiquidityCondition::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeePoolParams/0" => {
                let decoded = CoffeePoolParams::from_cell(&input)?;
                assert!(decoded.amm_settings.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeePoolParams/1" => {
                let decoded = CoffeePoolParams::from_cell(&input)?;
                assert!(decoded.amm_settings.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeePublicPoolCreationParams/0" => {
                let decoded = CoffeePublicPoolCreationParams::from_cell(&input)?;
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.use_recipient_on_failure, -1);
                assert!(decoded.notification_data.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePublicPoolCreationParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeePublicPoolCreationParams/1" => {
                let decoded = CoffeePublicPoolCreationParams::from_cell(&input)?;
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.use_recipient_on_failure, -1);
                assert!(decoded.notification_data.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePublicPoolCreationParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeePrivatePoolCreationParams/0" => {
                let decoded = CoffeePrivatePoolCreationParams::from_cell(&input)?;
                assert_eq!(decoded.is_active, 1);
                assert!(decoded.extra_settings.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePrivatePoolCreationParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeePrivatePoolCreationParams/1" => {
                let decoded = CoffeePrivatePoolCreationParams::from_cell(&input)?;
                assert_eq!(decoded.is_active, 1);
                assert!(decoded.extra_settings.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePrivatePoolCreationParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeePoolCreationParams/0" => {
                let decoded = CoffeePoolCreationParams::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolCreationParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeePoolCreationParams/1" => {
                let decoded = CoffeePoolCreationParams::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolCreationParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeeNotificationDataSingle/0" => {
                let decoded = CoffeeNotificationDataSingle::from_cell(&input)?;
                assert!(decoded.receiver.is_none());
                assert_eq!(decoded.fwd_gas, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeNotificationDataSingle::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeeNotificationDataSingle/1" => {
                let decoded = CoffeeNotificationDataSingle::from_cell(&input)?;
                assert!(decoded.receiver.is_some());
                assert_eq!(decoded.fwd_gas, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeNotificationDataSingle::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeeNotificationData/0" => {
                let decoded = CoffeeNotificationData::from_cell(&input)?;
                assert!(decoded.on_success.is_none());
                assert!(decoded.on_failure.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeNotificationData::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeeNotificationData/1" => {
                let decoded = CoffeeNotificationData::from_cell(&input)?;
                assert!(decoded.on_success.is_some());
                assert!(decoded.on_failure.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeNotificationData::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeeSwapParams/0" => {
                let decoded = CoffeeSwapParams::from_cell(&input)?;
                assert_eq!(decoded.deadline, 37);
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert!(decoded.referral.is_none());
                assert!(decoded.notification_data.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeSwapParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeeSwapParams/1" => {
                let decoded = CoffeeSwapParams::from_cell(&input)?;
                assert_eq!(decoded.deadline, 37);
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert!(decoded.referral.is_some());
                assert!(decoded.notification_data.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeSwapParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeeSwapStepParams/0" => {
                let decoded = CoffeeSwapStepParams::from_cell(&input)?;
                assert_eq!(decoded.pool_address_hash, U256::from(37u8));
                assert_eq!(decoded.min_output_amount, TLBCoins::new(37));
                assert!(decoded.next.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeSwapStepParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeeSwapStepParams/1" => {
                let decoded = CoffeeSwapStepParams::from_cell(&input)?;
                assert_eq!(decoded.pool_address_hash, U256::from(37u8));
                assert_eq!(decoded.min_output_amount, TLBCoins::new(37));
                assert!(decoded.next.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeSwapStepParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeeSwapStepInternalParams/0" => {
                let decoded = CoffeeSwapStepInternalParams::from_cell(&input)?;
                assert_eq!(decoded.previous_amount, TLBCoins::new(37));
                assert_eq!(decoded.min_output_amount, TLBCoins::new(37));
                assert!(decoded.next.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeSwapStepInternalParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeeSwapStepInternalParams/1" => {
                let decoded = CoffeeSwapStepInternalParams::from_cell(&input)?;
                assert_eq!(decoded.previous_amount, TLBCoins::new(37));
                assert_eq!(decoded.min_output_amount, TLBCoins::new(37));
                assert!(decoded.next.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeSwapStepInternalParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeeDepositLiquidityParamsTrimmed/0" => {
                let decoded = CoffeeDepositLiquidityParamsTrimmed::from_cell(&input)?;
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.use_recipient_on_failure, -1);
                assert_eq!(decoded.referral.workchain, -1);
                assert_eq!(decoded.referral.hash.as_slice()[31], 37);
                assert_eq!(decoded.deadline, 37);
                assert!(decoded.extra_settings.is_none());
                assert!(decoded.notification_data.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeDepositLiquidityParamsTrimmed::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeeDepositLiquidityParamsTrimmed/1" => {
                let decoded = CoffeeDepositLiquidityParamsTrimmed::from_cell(&input)?;
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.use_recipient_on_failure, -1);
                assert_eq!(decoded.referral.workchain, -1);
                assert_eq!(decoded.referral.hash.as_slice()[31], 37);
                assert_eq!(decoded.deadline, 37);
                assert!(decoded.extra_settings.is_some());
                assert!(decoded.notification_data.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeDepositLiquidityParamsTrimmed::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeeDepositLiquidityParams/0" => {
                let decoded = CoffeeDepositLiquidityParams::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeDepositLiquidityParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeeDepositLiquidityParams/1" => {
                let decoded = CoffeeDepositLiquidityParams::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeDepositLiquidityParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeeContractUpdate/0" => {
                let decoded = CoffeeContractUpdate::from_cell(&input)?;
                assert!(decoded.code.is_none());
                assert!(decoded.data.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeContractUpdate::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeeContractUpdate/1" => {
                let decoded = CoffeeContractUpdate::from_cell(&input)?;
                assert!(decoded.code.is_some());
                assert!(decoded.data.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeContractUpdate::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeeTransferNotification/0" => {
                let decoded = CoffeeTransferNotificationMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert!(decoded.sender.is_none());
                assert_eq!(decoded.forward_payload_selector, None);
                assert_eq!(decoded.to_cell()?, input);
            },
            "vault_jetton/CoffeeTransferNotification/1" => {
                let decoded = CoffeeTransferNotificationMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert!(decoded.sender.is_some());
                assert_eq!(decoded.forward_payload_selector, Some(true));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeTransferNotificationMsg::from_cell(&with_trailing_bit(&input)?).is_err());
            },
            "vault_jetton/CoffeeSwapJetton/0" => {
                let decoded = CoffeeSwapJettonMsg::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeSwapJettonMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeeSwapJetton/1" => {
                let decoded = CoffeeSwapJettonMsg::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeSwapJettonMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeeDepositLiquidityJetton/0" => {
                let decoded = CoffeeDepositLiquidityJettonMsg::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeDepositLiquidityJettonMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeeDepositLiquidityJetton/1" => {
                let decoded = CoffeeDepositLiquidityJettonMsg::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeDepositLiquidityJettonMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeeCreatePoolJetton/0" => {
                let decoded = CoffeeCreatePoolJettonMsg::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCreatePoolJettonMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeeCreatePoolJetton/1" => {
                let decoded = CoffeeCreatePoolJettonMsg::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCreatePoolJettonMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeePayoutInternal/0" => {
                let decoded = CoffeePayoutInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert!(decoded.notification_data.is_none());
                assert!(decoded.proof.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePayoutInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeePayoutInternal/1" => {
                let decoded = CoffeePayoutInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert!(decoded.notification_data.is_some());
                assert!(decoded.proof.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePayoutInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeeCreateVaultInternal/0" => {
                let decoded = CoffeeCreateVaultInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCreateVaultInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeeCreateVaultInternal/1" => {
                let decoded = CoffeeCreateVaultInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCreateVaultInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeeTakeWalletAddress/0" => {
                let decoded = CoffeeTakeWalletAddressMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.wallet_address.workchain, -1);
                assert_eq!(decoded.wallet_address.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeTakeWalletAddressMsg::from_cell(&with_trailing_bit(&input)?).is_err());
            },
            "vault_jetton/CoffeeTakeWalletAddress/1" => {
                let decoded = CoffeeTakeWalletAddressMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.wallet_address.workchain, -1);
                assert_eq!(decoded.wallet_address.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeTakeWalletAddressMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeeActivateVaultInternal/0" => {
                let decoded = CoffeeActivateVaultInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.wallet.workchain, -1);
                assert_eq!(decoded.wallet.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeActivateVaultInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeeActivateVaultInternal/1" => {
                let decoded = CoffeeActivateVaultInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.wallet.workchain, -1);
                assert_eq!(decoded.wallet.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeActivateVaultInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeeUpdateContractInternal/0" => {
                let decoded = CoffeeUpdateContractInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.excesses_receiver.workchain, -1);
                assert_eq!(decoded.excesses_receiver.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeUpdateContractInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeeUpdateContractInternal/1" => {
                let decoded = CoffeeUpdateContractInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.excesses_receiver.workchain, -1);
                assert_eq!(decoded.excesses_receiver.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeUpdateContractInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeeNotification/0" => {
                let decoded = CoffeeNotificationMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeNotificationMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeeNotification/1" => {
                let decoded = CoffeeNotificationMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeNotificationMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeeSwapInternal/0" => {
                let decoded = CoffeeSwapInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeSwapInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeeSwapInternal/1" => {
                let decoded = CoffeeSwapInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeSwapInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeeCreateLiquidityDepositoryRequest/0" => {
                let decoded = CoffeeCreateLiquidityDepositoryRequestMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.sender.workchain, -1);
                assert_eq!(decoded.sender.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCreateLiquidityDepositoryRequestMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeeCreateLiquidityDepositoryRequest/1" => {
                let decoded = CoffeeCreateLiquidityDepositoryRequestMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.sender.workchain, -1);
                assert_eq!(decoded.sender.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCreateLiquidityDepositoryRequestMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeeCreatePoolCreatorRequest/0" => {
                let decoded = CoffeeCreatePoolCreatorRequestMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.sender.workchain, -1);
                assert_eq!(decoded.sender.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCreatePoolCreatorRequestMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeeCreatePoolCreatorRequest/1" => {
                let decoded = CoffeeCreatePoolCreatorRequestMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.sender.workchain, -1);
                assert_eq!(decoded.sender.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCreatePoolCreatorRequestMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeeJettonTransfer/0" => {
                let decoded = CoffeeJettonTransferMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.destination.workchain, -1);
                assert_eq!(decoded.destination.hash.as_slice()[31], 37);
                assert_eq!(decoded.response_destination.workchain, -1);
                assert_eq!(decoded.response_destination.hash.as_slice()[31], 37);
                assert!(decoded.custom_payload.is_none());
                assert_eq!(decoded.forward_ton_amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
            },
            "vault_jetton/CoffeeJettonTransfer/1" => {
                let decoded = CoffeeJettonTransferMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.destination.workchain, -1);
                assert_eq!(decoded.destination.hash.as_slice()[31], 37);
                assert_eq!(decoded.response_destination.workchain, -1);
                assert_eq!(decoded.response_destination.hash.as_slice()[31], 37);
                assert!(decoded.custom_payload.is_some());
                assert_eq!(decoded.forward_ton_amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeJettonTransferMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeeProvideWalletAddress/0" => {
                let decoded = CoffeeProvideWalletAddressMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.owner_address.workchain, -1);
                assert_eq!(decoded.owner_address.hash.as_slice()[31], 37);
                assert!(decoded.include_address);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeProvideWalletAddressMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeeProvideWalletAddress/1" => {
                let decoded = CoffeeProvideWalletAddressMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.owner_address.workchain, -1);
                assert_eq!(decoded.owner_address.hash.as_slice()[31], 37);
                assert!(decoded.include_address);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeProvideWalletAddressMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeeExcesses/0" => {
                let decoded = CoffeeExcessesMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeExcessesMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_jetton/CoffeeExcesses/1" => {
                let decoded = CoffeeExcessesMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeExcessesMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_native/CoffeeVaultNativeStorage/0" => {
                let decoded = CoffeeVaultNativeStorage::from_cell(&input)?;
                assert_eq!(decoded.factory_initialization_marker, None);
                assert_eq!(decoded.to_cell()?, input);
            },
            "vault_native/CoffeeVaultNativeStorage/1" => {
                let decoded = CoffeeVaultNativeStorage::from_cell(&input)?;
                assert_eq!(decoded.factory_initialization_marker, Some(true));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeVaultNativeStorage::from_cell(&with_trailing_bit(&input)?).is_err());
            },
            "vault_native/CoffeeVaultInitData/0" => {
                let decoded = CoffeeVaultInitData::from_cell(&input)?;
                assert_eq!(decoded.factory_address.workchain, -1);
                assert_eq!(decoded.factory_address.hash.as_slice()[31], 37);
                assert_eq!(decoded.contract_type, 3);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeVaultInitData::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_native/CoffeeVaultInitData/1" => {
                let decoded = CoffeeVaultInitData::from_cell(&input)?;
                assert_eq!(decoded.factory_address.workchain, -1);
                assert_eq!(decoded.factory_address.hash.as_slice()[31], 37);
                assert_eq!(decoded.contract_type, 3);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeVaultInitData::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_native/CoffeeNativeAsset/0" => {
                let decoded = CoffeeNativeAsset::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeNativeAsset::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_native/CoffeeNativeAsset/1" => {
                let decoded = CoffeeNativeAsset::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeNativeAsset::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_native/CoffeeJettonAsset/0" => {
                let decoded = CoffeeJettonAsset::from_cell(&input)?;
                assert_eq!(decoded.chain, 37);
                assert_eq!(decoded.hash, U256::from(37u8));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeJettonAsset::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_native/CoffeeJettonAsset/1" => {
                let decoded = CoffeeJettonAsset::from_cell(&input)?;
                assert_eq!(decoded.chain, 37);
                assert_eq!(decoded.hash, U256::from(37u8));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeJettonAsset::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_native/CoffeeExtraAsset/0" => {
                let decoded = CoffeeExtraAsset::from_cell(&input)?;
                assert_eq!(decoded.id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeExtraAsset::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_native/CoffeeExtraAsset/1" => {
                let decoded = CoffeeExtraAsset::from_cell(&input)?;
                assert_eq!(decoded.id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeExtraAsset::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_native/CoffeeNoPreviousAssetHint/0" => {
                let decoded = CoffeeNoPreviousAssetHint::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeNoPreviousAssetHint::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_native/CoffeeNoPreviousAssetHint/1" => {
                let decoded = CoffeeNoPreviousAssetHint::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeNoPreviousAssetHint::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_native/CoffeeSomePreviousAssetHint/0" => {
                let decoded = CoffeeSomePreviousAssetHint::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeSomePreviousAssetHint::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_native/CoffeeSomePreviousAssetHint/1" => {
                let decoded = CoffeeSomePreviousAssetHint::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeSomePreviousAssetHint::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_native/CoffeeConstantProductAmm/0" => {
                let decoded = CoffeeConstantProductAmm::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeConstantProductAmm::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_native/CoffeeConstantProductAmm/1" => {
                let decoded = CoffeeConstantProductAmm::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeConstantProductAmm::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_native/CoffeeCurveFiStableAmm/0" => {
                let decoded = CoffeeCurveFiStableAmm::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCurveFiStableAmm::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_native/CoffeeCurveFiStableAmm/1" => {
                let decoded = CoffeeCurveFiStableAmm::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCurveFiStableAmm::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_native/CoffeeNoDepositLiquidityCondition/0" => {
                let decoded = CoffeeNoDepositLiquidityCondition::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeNoDepositLiquidityCondition::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_native/CoffeeNoDepositLiquidityCondition/1" => {
                let decoded = CoffeeNoDepositLiquidityCondition::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeNoDepositLiquidityCondition::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_native/CoffeeLpQuantityDepositLiquidityCondition/0" => {
                let decoded = CoffeeLpQuantityDepositLiquidityCondition::from_cell(&input)?;
                assert_eq!(decoded.min_lp_amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeLpQuantityDepositLiquidityCondition::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_native/CoffeeLpQuantityDepositLiquidityCondition/1" => {
                let decoded = CoffeeLpQuantityDepositLiquidityCondition::from_cell(&input)?;
                assert_eq!(decoded.min_lp_amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeLpQuantityDepositLiquidityCondition::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_native/CoffeeReservesRatioDepositLiquidityCondition/0" => {
                let decoded = CoffeeReservesRatioDepositLiquidityCondition::from_cell(&input)?;
                assert_eq!(decoded.denominator, 37);
                assert_eq!(decoded.min_nominator, 37);
                assert_eq!(decoded.max_nominator, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeReservesRatioDepositLiquidityCondition::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_native/CoffeeReservesRatioDepositLiquidityCondition/1" => {
                let decoded = CoffeeReservesRatioDepositLiquidityCondition::from_cell(&input)?;
                assert_eq!(decoded.denominator, 37);
                assert_eq!(decoded.min_nominator, 37);
                assert_eq!(decoded.max_nominator, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeReservesRatioDepositLiquidityCondition::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_native/CoffeeComplexDepositLiquidityCondition/0" => {
                let decoded = CoffeeComplexDepositLiquidityCondition::from_cell(&input)?;
                assert_eq!(decoded.min_lp_amount, TLBCoins::new(37));
                assert_eq!(decoded.denominator, 37);
                assert_eq!(decoded.min_nominator, 37);
                assert_eq!(decoded.max_nominator, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeComplexDepositLiquidityCondition::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_native/CoffeeComplexDepositLiquidityCondition/1" => {
                let decoded = CoffeeComplexDepositLiquidityCondition::from_cell(&input)?;
                assert_eq!(decoded.min_lp_amount, TLBCoins::new(37));
                assert_eq!(decoded.denominator, 37);
                assert_eq!(decoded.min_nominator, 37);
                assert_eq!(decoded.max_nominator, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeComplexDepositLiquidityCondition::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_native/CoffeePoolParams/0" => {
                let decoded = CoffeePoolParams::from_cell(&input)?;
                assert!(decoded.amm_settings.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_native/CoffeePoolParams/1" => {
                let decoded = CoffeePoolParams::from_cell(&input)?;
                assert!(decoded.amm_settings.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_native/CoffeePublicPoolCreationParams/0" => {
                let decoded = CoffeePublicPoolCreationParams::from_cell(&input)?;
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.use_recipient_on_failure, -1);
                assert!(decoded.notification_data.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePublicPoolCreationParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_native/CoffeePublicPoolCreationParams/1" => {
                let decoded = CoffeePublicPoolCreationParams::from_cell(&input)?;
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.use_recipient_on_failure, -1);
                assert!(decoded.notification_data.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePublicPoolCreationParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_native/CoffeePrivatePoolCreationParams/0" => {
                let decoded = CoffeePrivatePoolCreationParams::from_cell(&input)?;
                assert_eq!(decoded.is_active, 1);
                assert!(decoded.extra_settings.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePrivatePoolCreationParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_native/CoffeePrivatePoolCreationParams/1" => {
                let decoded = CoffeePrivatePoolCreationParams::from_cell(&input)?;
                assert_eq!(decoded.is_active, 1);
                assert!(decoded.extra_settings.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePrivatePoolCreationParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_native/CoffeePoolCreationParams/0" => {
                let decoded = CoffeePoolCreationParams::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolCreationParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_native/CoffeePoolCreationParams/1" => {
                let decoded = CoffeePoolCreationParams::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePoolCreationParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_native/CoffeeNotificationDataSingle/0" => {
                let decoded = CoffeeNotificationDataSingle::from_cell(&input)?;
                assert!(decoded.receiver.is_none());
                assert_eq!(decoded.fwd_gas, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeNotificationDataSingle::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_native/CoffeeNotificationDataSingle/1" => {
                let decoded = CoffeeNotificationDataSingle::from_cell(&input)?;
                assert!(decoded.receiver.is_some());
                assert_eq!(decoded.fwd_gas, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeNotificationDataSingle::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_native/CoffeeNotificationData/0" => {
                let decoded = CoffeeNotificationData::from_cell(&input)?;
                assert!(decoded.on_success.is_none());
                assert!(decoded.on_failure.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeNotificationData::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_native/CoffeeNotificationData/1" => {
                let decoded = CoffeeNotificationData::from_cell(&input)?;
                assert!(decoded.on_success.is_some());
                assert!(decoded.on_failure.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeNotificationData::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_native/CoffeeSwapParams/0" => {
                let decoded = CoffeeSwapParams::from_cell(&input)?;
                assert_eq!(decoded.deadline, 37);
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert!(decoded.referral.is_none());
                assert!(decoded.notification_data.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeSwapParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_native/CoffeeSwapParams/1" => {
                let decoded = CoffeeSwapParams::from_cell(&input)?;
                assert_eq!(decoded.deadline, 37);
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert!(decoded.referral.is_some());
                assert!(decoded.notification_data.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeSwapParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_native/CoffeeSwapStepParams/0" => {
                let decoded = CoffeeSwapStepParams::from_cell(&input)?;
                assert_eq!(decoded.pool_address_hash, U256::from(37u8));
                assert_eq!(decoded.min_output_amount, TLBCoins::new(37));
                assert!(decoded.next.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeSwapStepParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_native/CoffeeSwapStepParams/1" => {
                let decoded = CoffeeSwapStepParams::from_cell(&input)?;
                assert_eq!(decoded.pool_address_hash, U256::from(37u8));
                assert_eq!(decoded.min_output_amount, TLBCoins::new(37));
                assert!(decoded.next.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeSwapStepParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_native/CoffeeSwapStepInternalParams/0" => {
                let decoded = CoffeeSwapStepInternalParams::from_cell(&input)?;
                assert_eq!(decoded.previous_amount, TLBCoins::new(37));
                assert_eq!(decoded.min_output_amount, TLBCoins::new(37));
                assert!(decoded.next.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeSwapStepInternalParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_native/CoffeeSwapStepInternalParams/1" => {
                let decoded = CoffeeSwapStepInternalParams::from_cell(&input)?;
                assert_eq!(decoded.previous_amount, TLBCoins::new(37));
                assert_eq!(decoded.min_output_amount, TLBCoins::new(37));
                assert!(decoded.next.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeSwapStepInternalParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_native/CoffeeDepositLiquidityParamsTrimmed/0" => {
                let decoded = CoffeeDepositLiquidityParamsTrimmed::from_cell(&input)?;
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.use_recipient_on_failure, -1);
                assert_eq!(decoded.referral.workchain, -1);
                assert_eq!(decoded.referral.hash.as_slice()[31], 37);
                assert_eq!(decoded.deadline, 37);
                assert!(decoded.extra_settings.is_none());
                assert!(decoded.notification_data.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeDepositLiquidityParamsTrimmed::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_native/CoffeeDepositLiquidityParamsTrimmed/1" => {
                let decoded = CoffeeDepositLiquidityParamsTrimmed::from_cell(&input)?;
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.use_recipient_on_failure, -1);
                assert_eq!(decoded.referral.workchain, -1);
                assert_eq!(decoded.referral.hash.as_slice()[31], 37);
                assert_eq!(decoded.deadline, 37);
                assert!(decoded.extra_settings.is_some());
                assert!(decoded.notification_data.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeDepositLiquidityParamsTrimmed::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_native/CoffeeDepositLiquidityParams/0" => {
                let decoded = CoffeeDepositLiquidityParams::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeDepositLiquidityParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_native/CoffeeDepositLiquidityParams/1" => {
                let decoded = CoffeeDepositLiquidityParams::from_cell(&input)?;
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeDepositLiquidityParams::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_native/CoffeeContractUpdate/0" => {
                let decoded = CoffeeContractUpdate::from_cell(&input)?;
                assert!(decoded.code.is_none());
                assert!(decoded.data.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeContractUpdate::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_native/CoffeeContractUpdate/1" => {
                let decoded = CoffeeContractUpdate::from_cell(&input)?;
                assert!(decoded.code.is_some());
                assert!(decoded.data.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeContractUpdate::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_native/CoffeePayoutInternal/0" => {
                let decoded = CoffeePayoutInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert!(decoded.notification_data.is_none());
                assert!(decoded.proof.is_none());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePayoutInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_native/CoffeePayoutInternal/1" => {
                let decoded = CoffeePayoutInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.recipient.workchain, -1);
                assert_eq!(decoded.recipient.hash.as_slice()[31], 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert!(decoded.notification_data.is_some());
                assert!(decoded.proof.is_some());
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePayoutInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_native/CoffeeSwapNative/0" => {
                let decoded = CoffeeSwapNativeMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeSwapNativeMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_native/CoffeeSwapNative/1" => {
                let decoded = CoffeeSwapNativeMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeSwapNativeMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_native/CoffeeDepositLiquidityNative/0" => {
                let decoded = CoffeeDepositLiquidityNativeMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.input_amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeDepositLiquidityNativeMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_native/CoffeeDepositLiquidityNative/1" => {
                let decoded = CoffeeDepositLiquidityNativeMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.input_amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeDepositLiquidityNativeMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_native/CoffeeCreatePoolNative/0" => {
                let decoded = CoffeeCreatePoolNativeMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCreatePoolNativeMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_native/CoffeeCreatePoolNative/1" => {
                let decoded = CoffeeCreatePoolNativeMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCreatePoolNativeMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_native/CoffeeUpdateContractInternal/0" => {
                let decoded = CoffeeUpdateContractInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.excesses_receiver.workchain, -1);
                assert_eq!(decoded.excesses_receiver.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeUpdateContractInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_native/CoffeeUpdateContractInternal/1" => {
                let decoded = CoffeeUpdateContractInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.excesses_receiver.workchain, -1);
                assert_eq!(decoded.excesses_receiver.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeUpdateContractInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_native/CoffeeNotification/0" => {
                let decoded = CoffeeNotificationMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeNotificationMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_native/CoffeeNotification/1" => {
                let decoded = CoffeeNotificationMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeNotificationMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_native/CoffeePayout/0" => {
                let decoded = CoffeePayoutMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePayoutMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_native/CoffeePayout/1" => {
                let decoded = CoffeePayoutMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeePayoutMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_native/CoffeeSwapInternal/0" => {
                let decoded = CoffeeSwapInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeSwapInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_native/CoffeeSwapInternal/1" => {
                let decoded = CoffeeSwapInternalMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeSwapInternalMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_native/CoffeeCreateLiquidityDepositoryRequest/0" => {
                let decoded = CoffeeCreateLiquidityDepositoryRequestMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.sender.workchain, -1);
                assert_eq!(decoded.sender.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCreateLiquidityDepositoryRequestMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_native/CoffeeCreateLiquidityDepositoryRequest/1" => {
                let decoded = CoffeeCreateLiquidityDepositoryRequestMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.sender.workchain, -1);
                assert_eq!(decoded.sender.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCreateLiquidityDepositoryRequestMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_native/CoffeeCreatePoolCreatorRequest/0" => {
                let decoded = CoffeeCreatePoolCreatorRequestMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.sender.workchain, -1);
                assert_eq!(decoded.sender.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCreatePoolCreatorRequestMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_native/CoffeeCreatePoolCreatorRequest/1" => {
                let decoded = CoffeeCreatePoolCreatorRequestMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.amount, TLBCoins::new(37));
                assert_eq!(decoded.sender.workchain, -1);
                assert_eq!(decoded.sender.hash.as_slice()[31], 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeCreatePoolCreatorRequestMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_native/CoffeeExcesses/0" => {
                let decoded = CoffeeExcessesMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeExcessesMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            "vault_native/CoffeeExcesses/1" => {
                let decoded = CoffeeExcessesMsg::from_cell(&input)?;
                assert_eq!(decoded.query_id, 37);
                assert_eq!(decoded.to_cell()?, input);
                assert!(CoffeeExcessesMsg::from_cell(&with_trailing_bit(&input)?).is_ok());
            },
            _ => anyhow::bail!("unhandled fixture {}", fixture.id),
        }
    }
    Ok(())
}
pub(super) fn cell_for(id: &str) -> anyhow::Result<TonCell> {
    let fixtures: Vectors = serde_json::from_str(include_str!("fixtures/wire-vectors.json"))?;
    let fixture = fixtures
        .vectors
        .into_iter()
        .find(|x| x.id == id)
        .ok_or_else(|| anyhow::anyhow!("missing fixture {id}"))?;
    build_cell(&fixture.cell)
}

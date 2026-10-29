#!/usr/bin/env python3
"""Generate explicit Rust records from pinned MIT TON ABI catalog, not protocol math.

Usage: python3 tools/reference/dedust/generate_bindings.py CORPUS
Output is confined to crates/dedust and tools/reference/dedust in this snapshot.
Every generated operation remains unverified until independent fixture evidence.
"""
import json, pathlib, re, sys, hashlib
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from rust_imports import shorten_type_paths
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
from type_names import rename_source, rename_metadata

ROOT=pathlib.Path(__file__).resolve().parents[3]
MODS={
'DedustFactoryV1':'classic/factory_v1','DedustFactoryV2':'classic/factory_v2',
'DedustPoolV1':'classic/pool_v1','DedustPoolV2':'classic/pool_v2',
'DedustVaultNativeV1':'classic/vault_native_v1','DedustVaultNativeV2':'classic/vault_native_v2',
'DedustVaultJettonV1':'classic/vault_jetton_v1','DedustVaultJettonV2':'classic/vault_jetton_v2',
'DedustLiquidityDepositV1':'classic/liquidity_deposit_v1','DedustLiquidityDepositV2':'classic/liquidity_deposit_v2',
'DedustV2Cpmm':'cpmm_v2/pool','DedustUranusFactoryV3':'uranus/factory_v3',
'DedustUranusMemeV2':'uranus/meme_v2','DedustUranusMemeV3':'uranus/meme_v3',
'DedustUranusMemeWalletV2':'uranus/wallet_v2','DedustUranusMemeWalletV3':'uranus/wallet_v3','DedustOperatorV2':'operator/v2',
'DedustX1000WalletV1':'custom_wallet/v1','DedustX1000WalletV2':'custom_wallet/v2'}
# V2 vaults use the same pool identity and swap continuation wire records.
SHARED_VAULT_RECORDS = {
 'DedustVolatilePool', 'DedustStablePool', 'DedustPoolType', 'DedustPoolParams',
 'DedustGivenIn', 'DedustGivenOut', 'DedustSwapKind', 'DedustSwapStepParams',
 'DedustSwapStep', 'DedustSwapParams',
}
def snake(s):
 s=re.sub(r'([A-Z]+)([A-Z][a-z])',r'\1_\2',s);s=re.sub(r'([a-z0-9])([A-Z])',r'\1_\2',s).lower()
 return 'r#'+s if s in {'type','ref','in','from','self','match','return','loop'} else s

def doc(s):return '\n'.join('/// '+line for line in (s or '').splitlines())+'\n' if s else ''
def number(n,signed=False):
 return ('i' if signed else 'u')+str(next(x for x in [8,16,32,64,128] if x>=n)) if n<=128 else ('I512' if signed else ('U256' if n<=256 else 'U512'))
def generated_paths(root=ROOT):
 paths={root/'crates/dedust/src/contracts'/f'{mod}/storage.rs' for cn,mod in MODS.items() if cn!='DedustX1000WalletV2'}
 paths.update(root/'crates/dedust/src/contracts'/mod/'msgs.rs' for cn,mod in MODS.items() if cn not in ('DedustX1000WalletV1','DedustX1000WalletV2','DedustV2Cpmm'))
 paths.update(root/'crates/dedust/src/contracts'/f'{group}.rs' for group in {mod.split('/')[0] for mod in MODS.values()})
 paths.add(root/'tools/reference/dedust/bindings-generation.json')
 return paths

def write_generated(path,text,root=ROOT):
 if path not in generated_paths(root):
  raise ValueError('Refusing to overwrite handwritten path: '+str(path))
 path.parent.mkdir(parents=True,exist_ok=True)
 path.write_text(shorten_type_paths(text) if path.suffix == ".rs" else text)

IMPORTS={
'DedustClassicAsset':'crate::contracts::classic::asset::DedustClassicAsset',
'VersionedCode':'crate::contracts::classic::common::method_results::VersionedCode',
'I512':'fastnum::I512','U256':'fastnum::U256','U512':'fastnum::U512','HashMap':'std::collections::HashMap',
'TonCell':'ton::ton_core::cell::TonCell','TonHash':'ton::ton_core::cell::TonHash','CellParser':'ton::ton_core::cell::CellParser','CellBuilder':'ton::ton_core::cell::CellBuilder','TonCoreError':'ton::ton_core::errors::TonCoreError',
'TLBPrefix':'ton::ton_core::traits::tlb::TLBPrefix','TLB':'ton::ton_core::TLB','TonAddress':'ton::ton_core::types::TonAddress',
'SnakeData':'ton::contracts::tep::snake_data::SnakeData',
'FromTVMStack':'ton::block_tlb::FromTVMStack','ToTVMStack':'ton::block_tlb::ToTVMStack','TVMStack':'ton::block_tlb::TVMStack','TonResult':'ton::errors::TonResult'}
for k in ['TLBCoins','TLBRef','MsgAddress','MsgAddressInt','VarLenBytes']:IMPORTS[k]='ton::ton_core::types::tlb_core::'+k
for k in ['ConstLen','InternalAddress','NullableAddress']:IMPORTS[k]='ton::ton_core::types::tlb_core::adapters::'+k
for k in ['TLBHashMapE','DictKeyAdapterTonHash','DictKeyAdapterUint','DictValAdapterTLB','DictValAdapterNum']:IMPORTS[k]='ton::tlb_adapters::'+k
for k,m in [('MaybeRefAddress','maybe_ref_address'),('MaybeRefNullableAddress','maybe_ref_nullable_address'),('MaybeAddress','maybe_address')]:IMPORTS[k]='tonabis_core::adapters::'+m+'::'+k
# Defaults include trait imports implied by calls/derives; same-name derives and
# traits need both namespaces, just as TON's documented usage does.
def grouped_imports(paths):
 tree={}
 for path in sorted(set(paths)):
  node=tree
  for part in path.split('::'):node=node.setdefault(part,{})
  node['']=None
 def item(name,child):
  return name if child=={'':None} else name+'::'+render(child)
 def render(node):
  items=(['self'] if '' in node else [])+[item(name,child) for name,child in node.items() if name]
  return items[0] if len(items)==1 else '{'+','.join(items)+'}'
 return ''.join('use '+item(name,child)+';\n' for name,child in tree.items())



def imports(body):
 syntax=re.sub(r'(?m)^\s*//[^\n]*','',body)
 names=set(re.findall(r'\b\w+\b',syntax));result=[]
 for name,path in IMPORTS.items():
  if name not in names:continue
  if name == 'TLB' and not re.search(r'#\[derive\([^\n]*\bTLB\b', syntax):continue
  # Fully qualified occurrences do not need an import.
  if not re.search(r'(?<![\w:])'+re.escape(name)+r'\b',syntax):continue
  result.append('use '+path+';\n')
 if re.search(r'impl TLB\b', syntax) or re.search(r'\.(?:to_cell|write|read)\(|::from_cell\(',syntax):result.append('use ton::ton_core::traits::tlb::TLB;\n')
 if 'bits_len =' in syntax and re.search(r'#\[tlb\(bits_len',syntax) and not any('adapters::ConstLen;' in x for x in result):result.append('use '+IMPORTS['ConstLen']+';\n')
 return grouped_imports([line.removeprefix('use ').removesuffix(';\n') for line in result])

DEFAULT_WIDTHS = {**{k+str(n):n for k in ('u','i') for n in (8,16,32,64,128)}, 'U256':256, 'U512':512, 'I512':512}
FIELD_NOTES = {
    ('DedustFactoryV1', 'DedustFactoryV1Storage', 'canAcceptAfter'):
        'Unix timestamp after which `accept_ownership` can complete.',
    ('DedustFactoryV1', 'DedustFactoryV1Config', 'vaultCodeByAssetType'):
        'Bounded uint4 dictionary of inline version16/ref-code entries, confirmed by getter129291 DICTUGET4.',
    ('DedustFactoryV2', 'DedustFactoryV2Storage', 'canAcceptAfter'):
        'Unix timestamp after which `accept_ownership` can complete.',
    ('DedustFactoryV2', 'DedustFactoryV2Storage', 'operator'):
        'Optional trailing Operator configuration. Captured V2 loader accepts the older root with no trailing fields; absence remains distinct from version0/null.',
    ('DedustFactoryV2', 'DedustFactoryV2Config', 'vaultCodeByAssetType'):
        'Bounded uint4 dictionary of inline version16/ref-code entries, confirmed by getter129291 DICTUGET4.',
    ('DedustPoolV1', 'DedustPoolV1SwapParams', 'deadline'):
        'Unix timestamp deadline, or zero when disabled.',
    ('DedustPoolV2', 'DedustPoolV2Storage', 'normalizationFactor0'):
        'Stored stable normalization factor0; source numeric getter111021 and staged swap math, not a cumulative price.',
    ('DedustPoolV2', 'DedustPoolV2Storage', 'normalizationFactor1'):
        'Stored stable normalization factor1; source numeric getter111021 and staged swap math, not a cumulative price.',
    ('DedustPoolV2', 'DedustPoolV2SwapParams', 'deadline'):
        'Unix timestamp deadline, or zero when disabled.',
    ('DedustPoolV2', 'DedustPoolV2QuoteResponse', 'ignoredContext'):
        'LDDICT/ENDS followed by DROP: the optional referenced cell is not inspected. Preserve its bytes without inventing a context ABI.',
    ('DedustVaultNativeV2', 'DedustSwapParams', 'deadline'):
        'Unix timestamp deadline, or zero when disabled.',
    ('DedustVaultJettonV1', 'DedustVaultJettonV1SwapParams', 'deadline'):
        'Unix timestamp deadline, or zero when disabled.',
    ('DedustVaultJettonV1', 'DedustVaultJettonV1TransferNotification', 'forwardPayload'):
        'Typed generation-specific swap/deposit dispatcher with exact Either inline/reference form and unknown-byte preservation.',
    ('DedustVaultJettonV1', 'DedustVaultJettonV1BouncedProvideWalletAddress', 'prefix'):
        'Source skips32bits without marker validation; bounced flag remains a separate envelope property.',
    ('DedustVaultJettonV2', 'DedustSwapParams', 'deadline'):
        'Unix timestamp deadline, or zero when disabled.',
    ('DedustVaultJettonV2', 'DedustVaultJettonV2TransferNotification', 'forwardPayload'):
        'Typed generation-specific swap/deposit dispatcher with exact Either inline/reference form and unknown-byte preservation.',
    ('DedustVaultJettonV2', 'DedustVaultJettonV2BouncedProvideWalletAddress', 'prefix'):
        'Source skips32bits without marker validation; bounced flag remains a separate envelope property.',
    ('DedustLiquidityDepositV1', 'DedustLiquidityDepositInstall', 'extraParams'):
        'Fixed initialized-program arguments: minLP, targets, fulfillment/rejection refs, then source-ignored suffix. See classic_initializers.test.tolk.',
    ('DedustLiquidityDepositV2', 'DedustLiquidityDepositInstall', 'extraParams'):
        'Fixed initialized-program arguments: minLP, targets, fulfillment/rejection refs, then source-ignored suffix. See classic_initializers.test.tolk.',
    ('DedustV2Cpmm', 'DedustV2CpmmSwapStep', 'deadline'):
        'Unix timestamp after which this step must not execute.',
    ('DedustV2Cpmm', 'DedustV2CpmmActivationTime', 'activeSince'):
        'Unix timestamp from which the operation is active.',
    ('DedustV2Cpmm', 'DedustV2CpmmPayoutOptions', 'extraGas'):
        'Extra gas attached to the downstream notification.',
    ('DedustV2Cpmm', 'DedustV2CpmmSwapPayload', 'deadline'):
        'Unix timestamp after which the swap must not execute.',
    ('DedustV2Cpmm', 'DedustV2CpmmPayJetton', 'paymentPayload'):
        'Referenced finite swap/deposit/reward operation. Known malformed bodies fail; unknown bytes remain preserved.',
    ('DedustV2Cpmm', 'DedustV2CpmmPayNative', 'paymentPayload'):
        'Referenced finite swap/deposit/reward operation. Known malformed bodies fail; unknown bytes remain preserved.',
    ('DedustV2Cpmm', 'DedustV2CpmmTransferNotification', 'forwardPayload'):
        'Typed Either inline/reference CBC33949 payment frame; caller payloads inside its payout config remain opaque.',
    ('DedustV2Cpmm', 'DedustV2CpmmJoinLiquidity', 'config'):
        'Fixed deposit configuration loaded by the pool/child path, not caller-defined payload. See position/deposit captured-code replay.',
    ('DedustV2Cpmm', 'DedustV2CpmmRefundDeposit', 'config'):
        'Fixed deposit configuration loaded by the pool/child path, not caller-defined payload. See position/deposit captured-code replay.',
    ('DedustUranusMemeWalletV2', 'DedustUranusJettonBurn', 'ignoredSuffix'):
        'Captured V2 branch ignores this suffix; preserve all bits/refs losslessly.',
    ('DedustUranusMemeWalletV2', 'DedustUranusSellTokensMessage', 'ignoredSuffix'):
        'Captured V2 branch ignores this suffix; preserve all bits/refs losslessly.',
    ('DedustX1000WalletV1', 'DedustX1000WalletV1Storage', 'seqnoOffset'):
        'Base sequence number for the active 256-message replay window.',
    ('DedustX1000WalletV2', 'DedustX1000WalletV2Storage', 'seqno'):
        'Sequence number consumed and incremented by accepted signed requests.',
    ('DedustX1000WalletV2', 'DedustX1000WalletV2Storage', 'runtimeState'):
        'Typed runtime dictionary envelopes and source-selected continuation contexts; see the independent branch replay evidence.',
    ('DedustX1000WalletV2', 'DedustX1000WalletV2Storage', 'implementationCode'):
        'Executable implementation dispatch code invoked with BLESS/CALLXARGS, preserved as code. Not application state or a substitute for a missing record.',
    ('DedustX1000WalletV2', 'DedustX1000SwapParams', 'deadline'):
        'Unix deadline, or zero when disabled.',
    ('DedustX1000WalletV2', 'DedustX1000StonfiV2SwapBody', 'minOut'):
        'Minimum accepted output amount in token base units.',
    ('DedustX1000WalletV2', 'DedustX1000StonfiV2SwapBody', 'forwardGas'):
        'Nanograms forwarded with a success notification.',
    ('DedustX1000WalletV2', 'DedustX1000StonfiV2SwapBody', 'refundForwardGas'):
        'Nanograms forwarded with a refund notification.',
    ('DedustX1000WalletV2', 'DedustX1000StonfiV2SwapBody', 'referralFee'):
        'Referral fee in STON.fi fee units.',
    ('DedustX1000WalletV2', 'DedustX1000ToncoMultihop', 'fulfillForwardAmount'):
        'Nanograms forwarded after success.',
    ('DedustX1000WalletV2', 'DedustX1000ToncoMultihop', 'rejectForwardAmount'):
        'Nanograms forwarded after rejection.',
    ('DedustX1000WalletV2', 'DedustX1000CpmmNextStep', 'deadline'):
        'Unix deadline for the next step.',
    ('DedustX1000WalletV2', 'DedustX1000CpmmPayoutOptions', 'extraGas'):
        'Additional downstream gas in nanograms.',
    ('DedustX1000WalletV2', 'DedustX1000BidaskDammReferral', 'referralFee'):
        'Referral fee in Bidask fee units.',
    ('DedustX1000WalletV2', 'DedustX1000BidaskPartialSlippage', 'sqrtPriceLimit'):
        'Q256 square-root price boundary.',
    ('DedustX1000WalletV2', 'DedustX1000BidaskAdditionalData', 'referralAddress'):
        'Optional referral; captured builder7 explicitly writes addr_none. Preserve null versus standard zero.',
    ('DedustX1000WalletV2', 'DedustX1000CoffeeNotification', 'forwardAmount'):
        'Nanograms forwarded to the callback.',
    ('DedustX1000WalletV2', 'DedustX1000CoffeeSwapParams', 'deadline'):
        'Unix deadline.',
    ('DedustX1000WalletV2', 'DedustX1000MoonSwapParams', 'deadline'):
        'Unix deadline.',
    ('DedustX1000WalletV2', 'DedustX1000MoonClmmCallback', 'payload'):
        'Preserve recipient-only compact form emitted by captured builder7 and explicitly marked optional payload. Peer acceptance of compact form is not assumed.',
    ('DedustX1000WalletV2', 'DedustX1000WalletV2SignedRequest', 'payload'):
        'Fixed typed signed envelope and action; unresolved scalar meanings remain explicitly positional.',
    ('DedustX1000WalletV2', 'DedustX1000WalletV2ExecuteStoredMessage', 'ignoredSuffix'):
        'Source controller branch ignores all remaining bits/refs after request id; retained byte-exact, not an unresolved callback record.',
    ('DedustX1000WalletV2', 'DedustX1000WalletV2JettonTransfer', 'forwardAmount'):
        'GRAM amount forwarded with the recipient notification, in nanograms.',
    ('DedustX1000WalletV2', 'DedustX1000StonfiV2SwapPayload', 'deadline'):
        'Unix deadline for execution.',
    ('DedustX1000WalletV2', 'DedustX1000CpmmSwapPayload', 'deadline'):
        'Unix deadline encoded in 40 bits.',
    ('DedustPoolV2', 'DedustPoolV2Config', 'quoteProviderData'):
        'Optional fixed provider pair, not an arbitrary dictionary; selector66722.',
    ('DedustUranusMemeV3', 'DedustUranusMemeV3Storage', 'partnerFee'):
        'Coins120 accumulated partner fee, reconciled against captured getter83180.',
    ('DedustFactoryV1', 'DedustFactoryV1AcceptOwnership', 'tail'):
        'Body tail ignored by this opcode handler.',
    ('DedustFactoryV1', 'DedustFactoryV1CancelOwnershipTransfer', 'tail'):
        'Body tail ignored by this opcode handler.',
    ('DedustFactoryV1', 'DedustFactoryV1ResetGas', 'tail'):
        'Body tail ignored by this opcode handler.',
    ('DedustPoolV1', 'DedustPoolV1Storage', 'tradeFeeNumerator'):
        'Trade fee numerator returned by `get_trade_fee`; denominator is fixed at 10000.',
    ('DedustPoolV1', 'DedustPoolV1Storage', 'asset0ProtocolFee'):
        'Accumulated protocol fee in asset0 units.',
    ('DedustPoolV1', 'DedustPoolV1Storage', 'asset1ProtocolFee'):
        'Accumulated protocol fee in asset1 units.',
    ('DedustPoolV2', 'DedustPoolV2Storage', 'tradeFeeNumerator'):
        'Trade fee numerator returned by `get_trade_fee`; denominator is fixed at 10000.',
    ('DedustPoolV2', 'DedustPoolV2Storage', 'asset0ProtocolFee'):
        'Accumulated protocol fee in asset0 units.',
    ('DedustPoolV2', 'DedustPoolV2Storage', 'asset1ProtocolFee'):
        'Accumulated protocol fee in asset1 units.',
    ('DedustPoolV2', 'DedustPoolV2ConfigureTradeFee', 'tradeFee'):
        'New trade-fee numerator; the denominator is fixed at 10000.',
    ('DedustVaultJettonV1', 'DedustVaultJettonV1TransferNotification', 'amount'):
        'Received Jetton amount in base units.',
    ('DedustVaultJettonV1', 'DedustVaultJettonV1DestroyNonReady', 'tail'):
        'Body tail ignored by the v1 handler.',
    ('DedustVaultJettonV1', 'DedustVaultJettonV1JettonTransfer', 'customPayload'):
        'Optional custom payload; payout paths store `null`.',
    ('DedustVaultJettonV1', 'DedustVaultJettonV1BouncedProvideWalletAddress', 'tail'):
        'Remaining bounced body bits/refs are ignored.',
    ('DedustVaultJettonV2', 'DedustVaultJettonV2TransferNotification', 'amount'):
        'Received Jetton amount in base units.',
    ('DedustVaultJettonV2', 'DedustVaultJettonV2JettonTransfer', 'customPayload'):
        'Optional custom payload; payout paths store `null`.',
    ('DedustVaultJettonV2', 'DedustVaultJettonV2ProvideWalletAddress', 'queryId'):
        'Query id copied into `take_wallet_address`; live code sends zero.',
    ('DedustLiquidityDepositV1', 'DedustDepositAsset', 'amount'):
        'Deposited amount in basic asset units.',
    ('DedustLiquidityDepositV2', 'DedustDepositAsset', 'amount'):
        'Deposited amount in basic asset units.',
    ('DedustV2Cpmm', 'DedustV2CpmmPartnerConfig', 'partnerFeeBPS'):
        'Partner fee in basis points.',
    ('DedustV2Cpmm', 'DedustV2CpmmReferrerConfig', 'referrerFeeBPS'):
        'Referrer fee in basis points.',
    ('DedustV2Cpmm', 'DedustV2CpmmPoolConfig', 'baseFeeBPS'):
        'Base swap fee in basis points.',
    ('DedustV2Cpmm', 'DedustV2CpmmPoolConfig', 'creatorFeeBPS'):
        'Creator fee in basis points.',
    ('DedustV2Cpmm', 'DedustV2CpmmSwapFees', 'isX'):
        'Whether fees are denominated in asset X.',
    ('DedustUranusFactoryV3', 'DedustUranusPartnerConfig', 'partnerFeeBps'):
        'Partner fee share in basis points.',
    ('DedustUranusFactoryV3', 'DedustUranusReferrerConfig', 'referrerFeeBps'):
        'Referrer fee share in basis points.',
    ('DedustUranusFactoryV3', 'DedustUranusDeployCustomizedMeme', 'partnerFeeBps'):
        'Partner share of the bonding-curve base fee in basis points.',
    ('DedustUranusFactoryV3', 'DedustUranusDeployCustomizedMeme', 'poolPartnerFeeBps'):
        'Partner share of the migrated CPMM pool base fee in basis points.',
    ('DedustUranusMemeV2', 'DedustUranusMemeV2PartnerConfig', 'partnerFeeBps'):
        'Partner fee share in basis points.',
    ('DedustUranusMemeV2', 'DedustUranusMemeV2ReferrerConfig', 'referrerFeeBps'):
        'Referrer fee share in basis points.',
    ('DedustUranusMemeV2', 'DedustUranusMemeV2Storage', 'baseFeeBps'):
        'Bonding-curve base trading fee in basis points.',
    ('DedustUranusMemeV3', 'DedustUranusPartnerConfig', 'partnerFeeBps'):
        'Partner fee share in basis points.',
    ('DedustUranusMemeV3', 'DedustUranusReferrerConfig', 'referrerFeeBps'):
        'Referrer fee share in basis points.',
    ('DedustUranusMemeV3', 'DedustUranusMemeV3Storage', 'baseFeeBps'):
        'Bonding-curve base trading fee in basis points.',
    ('DedustUranusMemeWalletV2', 'DedustUranusPartnerConfig', 'partnerFeeBps'):
        'Partner fee share in basis points.',
    ('DedustUranusMemeWalletV2', 'DedustUranusReferrerConfig', 'referrerFeeBps'):
        'Referrer fee share in basis points.',
    ('DedustUranusMemeWalletV3', 'DedustUranusPartnerConfig', 'partnerFeeBps'):
        'Partner fee share in basis points.',
    ('DedustUranusMemeWalletV3', 'DedustUranusReferrerConfig', 'referrerFeeBps'):
        'Referrer fee share in basis points.',
    ('DedustX1000WalletV2', 'DedustX1000SwapStep', 'kind'):
        'Exact-input or exact-output mode.',
    ('DedustX1000WalletV2', 'DedustX1000CpmmPartnerConfig', 'feeBps'):
        'Partner fee in basis points.',
    ('DedustX1000WalletV2', 'DedustX1000CpmmReferrerConfig', 'feeBps'):
        'Referrer fee in basis points.',
    ('DedustX1000WalletV2', 'DedustX1000UranusPartnerConfig', 'feeBps'):
        'Partner fee in basis points.',
    ('DedustX1000WalletV2', 'DedustX1000UranusReferrerConfig', 'feeBps'):
        'Referrer fee in basis points.',
    ('DedustX1000WalletV2', 'DedustX1000WalletV2SignedRequest', 'signature'):
        'Ed25519 signature checked against the public key in storage.',
    ('DedustX1000WalletV2', 'DedustX1000WalletV2JettonTransfer', 'amount'):
        'Jetton quantity in token base units.',
    ('DedustX1000WalletV2', 'DedustX1000BidaskDammSwapPayload', 'exactOut'):
        'Exact-output target, or zero for exact-input mode.',
    ('DedustX1000WalletV2', 'DedustX1000BidaskClmmSwapPayload', 'exactOut'):
        'Exact-output target, or zero for exact-input mode.',
    ('DedustX1000WalletV2', 'DedustX1000StonksPumpSell', 'amount'):
        'Jetton quantity sold in token base units.',
    ('DedustX1000WalletV2', 'DedustX1000UranusSell', 'amount'):
        'Token quantity sold in base units.',
    ('DedustX1000WalletV2', 'DedustX1000JettonBurn', 'amount'):
        'Jetton quantity burned in token base units.',
}
GETTER_COINS_FIELDS = {
    ('custom_wallet/v2', 83648): ('value',),
    ('classic/liquidity_deposit_v1', 84232): ('asset0_target_balance', 'asset1_target_balance'),
    ('classic/liquidity_deposit_v1', 85296): ('min_lp_amount',),
    ('classic/liquidity_deposit_v1', 87878): ('asset0_balance', 'asset1_balance'),
    ('classic/liquidity_deposit_v2', 84232): ('asset0_target_balance', 'asset1_target_balance'),
    ('classic/liquidity_deposit_v2', 85296): ('min_lp_amount',),
    ('classic/liquidity_deposit_v2', 87878): ('asset0_balance', 'asset1_balance'),
    ('classic/lp_wallet', 97026): ('balance',),
    ('classic/pool_v1', 65971): ('reserve0', 'reserve1'),
    ('classic/pool_v1', 106029): ('total_supply',),
    ('classic/pool_v1', 112792): ('asset0_protocol_fee', 'asset1_protocol_fee'),
    ('classic/pool_v2', 65971): ('reserve0', 'reserve1'),
    ('classic/pool_v2', 106029): ('total_supply',),
    ('classic/pool_v2', 112792): ('asset0_protocol_fee', 'asset1_protocol_fee'),
    ('classic/vault_native_v2', 109867): ('native_balance',),
    ('cpmm_v2/deposit', 70330): ('amount_x', 'amount_y', 'minimal_liquidity', 'target_x', 'target_y'),
    ('cpmm_v2/pool', 81689): ('creator_fee_x', 'creator_fee_y', 'liquidity', 'protocol_fee_x', 'protocol_fee_y', 'reserve_x', 'reserve_y'),
    ('cpmm_v2/position', 89720): ('locked_liquidity', 'total_liquidity', 'x_pending', 'y_pending'),
    ('uranus/meme_v2', 101289): ('alpha', 'beta', 'creator_fee', 'current_supply', 'on_sell_supply', 'raised_funds'),
    ('uranus/meme_v2', 106029): ('total_supply',),
    ('uranus/meme_v3', 83180): ('partner_fee',),
    ('uranus/meme_v3', 101289): ('alpha', 'beta', 'creator_fee', 'current_supply', 'on_sell_supply', 'raised_funds'),
    ('uranus/meme_v3', 106029): ('total_supply',),
    ('uranus/meme_v3', 121862): ('alpha', 'beta', 'bonding_curve_supply', 'current_supply', 'liquidity_supply', 'max_supply', 'migration_fee', 'raised_funds', 'raising_funds'),
    ('uranus/wallet_v2', 97026): ('balance',),
    ('uranus/wallet_v3', 97026): ('balance',),
}

GETTER_NATIVE_FIELDS = {
    ('custom_wallet/v2', 85143): 'u32',
    ('custom_wallet/v2', 91459): 'u32',
    ('custom_wallet/v2', 95507): 'u8',
    ('custom_wallet/v2', 97027): 'u32',
    ('custom_wallet/v2', 97376): 'u8',
}

GETTER_U256_FIELDS = {
    ('cpmm_v2/pool', 81689): ('x_fee_per_liquidity', 'y_fee_per_liquidity'),
    ('cpmm_v2/position', 89720): ('x_checkpoint', 'y_checkpoint'),
    ('custom_wallet/v2', 89828): ('value',),
}

GETTER_NAMES = {79554: 'get_pool_code',
 120195: 'get_liquidity_deposit_code',
 129291: 'get_vault_code',
 78248: 'get_operator_code',
 126188: 'get_operator_address',
 79987: 'get_operator_data',
 106049: 'get_decimals',
 66722: 'get_quote_providers',
 111021: 'get_scaling_factors',
 112861: 'get_start_at',
 109505: 'estimate_gas_one_arg',
 117300: 'estimate_gas_two_args',
 109867: 'get_tracked_native_balance',
 112421: 'get_liquidity_amounts',
 83648: 'get_balance',
 85802: 'is_pending_operations_empty',
 89828: 'get_implementation_code_hash',
 91459: 'get_stored_seqno',
 94056: 'is_execution_entries_empty',
 97027: 'get_counter',
 97376: 'get_pending_operation_presence',
 98121: 'is_reserved_values_empty',
 118999: 'has_no_pending_operations'}

def handwritten_apis(records):
 # Canonical API metadata for contracts whose schemas are entirely handwritten.
 # Wire evidence stays in the pinned fixtures and inventory, not this map.
 def getter(module,selector,name,result,fields,parameters=()):
  return {'name':name,'rust_name':name,'selector':selector,'module':module,'owner':'handwritten',
   'result':result,'result_module':module+'/method_results','return_type':result,
   'parameters':[{'name':n,'rust_type':t,'wire_kind':'int'} for n,t in parameters],
   'fields':[{'name':n,'rust_type':t} for n,t in fields]}
 def record(contract,module,wrapper,getters):
  return {'contract':contract,'module':module,'wrapper':wrapper,'owner':'handwritten',
   'schema_sha256':None,'api_path':f'crates/dedust/src/contracts/{module}.rs',
   'generated_paths':[],'symbols':[],'getters':getters,'handwritten_children':['storage','msgs','method_results']}
 pool=next(row for row in records if row['contract']=='DedustPoolV2')
 managed=[dict(g,module='classic/operator_pool') for g in pool['getters']]
 managed.append({'name':'get_method_112861','rust_name':'get_start_at','selector':112861,
  'module':'classic/operator_pool','owner':'handwritten','result':None,'result_module':None,
  'return_type':'u32','parameters':[],'fields':[{'name':'active_since','rust_type':'u32'}]})
 module='classic/lp_wallet'
 wallet=record('DedustLpWallet',module,'DedustClassicLpWalletContract',[
  getter(module,97026,'get_wallet_data','WalletData',[
   ('balance','Coins'),('owner','MsgAddress'),('pool','MsgAddress'),('wallet_code','TonCell')])])
 module='cpmm_v2/position'
 position=record('DedustV2Position',module,'DedustV2CpmmPositionContract',[
  getter(module,89720,'get_position_data','PositionData',[
   ('pool','MsgAddress'),('owner','MsgAddress'),('total_liquidity','Coins'),('locked_liquidity','Coins'),
   ('rewards','HashMap<u8, crate::contracts::cpmm_v2::position::storage::DedustV2CpmmPositionReward>'),('x_pending','Coins'),('y_pending','Coins'),
   ('x_checkpoint','U256'),('y_checkpoint','U256')]),
  getter(module,80110,'get_available_fees','AvailableFees',[('x_fee','I512'),('y_fee','I512')],
   [('x_checkpoint','I512'),('y_checkpoint','I512')])])
 module='cpmm_v2/deposit'
 deposit=record('DedustV2Deposit',module,'DedustV2CpmmDepositContract',[
  getter(module,70330,'get_deposit_data','DepositData',[
   ('pool','MsgAddress'),('fulfill_destination','MsgAddress'),('reject_destination','MsgAddress'),
   ('target_x','Coins'),('target_y','Coins'),('locked_liquidity_share','u16'),
   ('minimal_liquidity','Coins'),('amount_x','Coins'),('amount_y','Coins')])])
 operator=record('DedustOperatorPool','classic/operator_pool','DedustClassicOperatorPoolContract',managed)
 operator['handwritten_children']=['storage','msgs']
 return [wallet,operator,position,deposit]

def main(corpus):
 CORPUS=pathlib.Path(corpus); ABI=CORPUS/'ton-blockchain-abis/build/abi'
 existing=json.loads((ROOT/'tools/reference/dedust/bindings-generation.json').read_text())
 pinned={row['contract']:row['schema_sha256'] for row in existing}
 for contract in MODS:
  source='DedustUranusMemeWalletV3' if contract=='DedustUranusMemeWalletV2' else contract
  if hashlib.sha256((ABI/(source+'.json')).read_bytes()).hexdigest()!=pinned[contract]:
   raise ValueError('Pinned ABI input hash mismatch: '+contract)
 records=[]
 for cn,mod in MODS.items():
  schema_cn='DedustUranusMemeWalletV3' if cn=='DedustUranusMemeWalletV2' else cn
  path=ABI/(schema_cn+'.json'); d=json.loads(path.read_text())
  # Retain this rejected layout in the pinned corpus, not the supported Rust API.
  excluded_declarations = {}
  if cn == 'DedustVaultNativeV2':
   excluded_declarations['DedustVaultNativeV2InlineDepositLiquidityCandidate'] = 'Reference-only inline candidate; the supported deposit message uses referenced DedustDepositLiquidityParams.'
   d['declarations'] = [decl for decl in d['declarations'] if decl['name'] not in excluded_declarations]
  if cn=='DedustUranusMemeWalletV2':
   d=json.loads(json.dumps(d).replace('MemeWalletV3','MemeWalletV2'))
  if cn=='DedustX1000WalletV2':
   for getter in d['get_methods']:
    if getter['tvm_method_id']==97376:getter['description']='Numeric getter97376 returns1 when the first runtime dictionary contains the supplied uint64 key, otherwise0; DICTUGET/IFNOTJMP source order corrects reversed catalog prose.'
    if getter['tvm_method_id']==89828:getter['description']='Numeric getter89828 returns the executable implementation-code cell hash (HASHCU), not an application-state hash.'
   for decl in d['declarations']:
    if 'description'in decl:decl['description']=decl['description'].replace('application-defined','source-defined').replace('Application-defined','Source-defined')
    for field in decl.get('fields',[]):
     if 'description'in field:field['description']=field['description'].replace('application-defined','source-defined').replace('Application-defined','Source-defined')
   for getter in d['get_methods']:
    if 'description'in getter:getter['description']=getter['description'].replace('application-defined','source-defined')
  ts=d['unique_types']
  if cn=='DedustUranusMemeWalletV2':
   for t in ts:
    if t['kind']=='address':t['kind']='addressAny'
   remaining=next(i for i,t in enumerate(ts) if t['kind']=='remaining')
   for decl in d['declarations']:
    if decl['name'] in ['DedustUranusJettonBurn','DedustUranusSellTokensMessage']:
     decl['fields'].append({'name':'ignoredSuffix','ty_idx':remaining,'description':'Captured V2 branch ignores this suffix; preserve all bits/refs losslessly.'})

  if cn in ['DedustFactoryV1','DedustFactoryV2']:
   key=len(ts);ts.append({'kind':'uintN','n':4})
   val=len(ts);ts.append({'kind':'StructRef','struct_name':'crate::contracts::classic::common::method_results::VersionedCode'})
   mapping=len(ts);ts.append({'kind':'mapKV','key_ty_idx':key,'value_ty_idx':val})
   for decl in d['declarations']:
    for field in decl.get('fields',[]):
     if field['name']=='vaultCodeByAssetType':field['ty_idx']=mapping;field['description']='Bounded uint4 dictionary of inline version16/ref-code entries, confirmed by getter129291 DICTUGET4.'

  if cn=='DedustFactoryV2':
   operator=len(ts);ts.append({'kind':'trailingStruct','rust_type':'crate::contracts::classic::common::OperatorCode'})
   for decl in d['declarations']:
    if decl['name']=='DedustFactoryV2Storage':
     decl['fields']=[f for f in decl['fields']if f['name']not in ['operatorCodeVersion','operatorCode']]+[{'name':'operator','ty_idx':operator,'description':'Optional trailing Operator configuration. Captured V2 loader accepts the older root with no trailing fields; absence remains distinct from version0/null.'}]

  # Executable-evidence override: catalog v3Tail is reconciled against library
  # a63e4a80... get_partner_data (83180), migration serialization and saved BOC.
  if cn=='DedustUranusMemeV3':
   def type_index(kind,**attrs):
    return next(i for i,t in enumerate(ts) if t['kind']==kind and all(t.get(k)==v for k,v in attrs.items()))
   partner_index=len(ts);ts.append({'kind':'StructRef','struct_name':'DedustUranusMigrationConfig'})
   partner_ref=len(ts);ts.append({'kind':'cellOf','inner_ty_idx':partner_index})
   coin_index=type_index('coins');u16_index=type_index('uintN',n=16);address_index=type_index('address')
   for decl in d['declarations']:
    if decl['name']=='DedustUranusMemeV3Storage':
     decl['fields']=decl['fields'][:-1]+[
      {'name':'partnerFee','ty_idx':coin_index,'description':'Accumulated partner fee; confirmed by captured library getter 83180.'},
      {'name':'partnerFeeBps','ty_idx':u16_index,'description':'Partner share of bonding-curve fees.'},
      {'name':'migrationConfig','ty_idx':partner_ref,'description':'Complete typed migration configuration reconciled from code and BOC.'}]
   d['declarations'].append({'kind':'struct','name':'DedustUranusMigrationConfig','ty_idx':partner_index,
    'description':'Captured v3 migration configuration; see uranus_tail.test.tolk and library a63e4a80.',
    'fields':[{'name':'partnerAddress','ty_idx':address_index},{'name':'poolPartnerFeeBps','ty_idx':u16_index},{'name':'poolBaseFeeBps','ty_idx':u16_index},{'name':'poolLiquidityOwnerAddress','ty_idx':address_index}]})
  if cn=='DedustUranusMemeWalletV3':
   either=len(ts);ts.append({'kind':'StructRef','struct_name':'ton::ton_core::types::tlb_core::TLBEitherRef<TonCell>'})
   for decl in d['declarations']:
    for field in decl.get('fields',[]):
     if field['name']=='forwardPayload':field['ty_idx']=either
   optional=next(i for i,t in enumerate(ts) if t['kind']=='addressOpt')
   for decl in d['declarations']:
    if decl['name']=='DedustUranusJettonInternalTransfer':
     for field in decl['fields']:
      if field['name']=='from':field['ty_idx']=optional
  if cn in ['DedustVaultJettonV1','DedustVaultJettonV2']:
   u32_index=next(i for i,t in enumerate(ts)if t['kind']=='uintN'and t['n']==32)
   remaining_index=next(i for i,t in enumerate(ts)if t['kind']=='remaining')
   name=cn+'BouncedProvideWalletAddress'
   if cn.endswith('V2'):
    idx=len(ts);ts.append({'kind':'StructRef','struct_name':name})
    d['declarations'].append({'kind':'struct','name':name,'ty_idx':idx,'fields':[{'name':'originalOpcode','ty_idx':u32_index},{'name':'tail','ty_idx':remaining_index}]})
   for decl in d['declarations']:
    if decl['name']==name:
     decl.pop('prefix',None)
     decl['message_role']='bounce'
     decl['fields'].insert(0,{'name':'prefix','ty_idx':u32_index,'description':'Source skips32bits without marker validation; bounced flag remains a separate envelope property.'})
     decl['description']='Discovery bounce prefix. Only original opcode2c76b973 destroys/refunds the vault; other opcodes are ignored before storage access. Remaining bits/refs are ignored.'
  if cn=='DedustPoolV2':
   address=next(i for i,t in enumerate(ts)if t['kind']=='addressOpt')
   providers=len(ts);ts.append({'kind':'StructRef','struct_name':'DedustPoolV2QuoteProviders'})
   providers_ref=len(ts);ts.append({'kind':'cellOf','inner_ty_idx':providers})
   providers_optional=len(ts);ts.append({'kind':'nullable','inner_ty_idx':providers_ref})
   d['declarations'].append({'kind':'struct','name':'DedustPoolV2QuoteProviders','ty_idx':providers,'description':'Fixed optional provider pair loaded by numeric getter66722 and quote configuration paths; not an arbitrary dictionary.','fields':[{'name':'provider0','ty_idx':address},{'name':'provider1','ty_idx':address}]})
   for decl in d['declarations']:
    if decl['name']=='DedustPoolV2Config':
     for field in decl['fields']:
      if field['name']=='quoteProviderData':field['ty_idx']=providers_optional;field['description']='Optional fixed provider address pair, independently reconciled against numeric getter66722.'
    if decl['name']=='DedustPoolV2Storage':
     for field in decl['fields']:
      if field['name']in ['cumulativePrice0','cumulativePrice1']:
       side=field['name'][-1];field['name']='normalizationFactor'+side;field['description']='Stored stable normalization factor'+side+'; source numeric getter111021 and staged swap math, not a cumulative price.'
  if cn in ['DedustLiquidityDepositV1','DedustLiquidityDepositV2']:
   arguments=len(ts);ts.append({'kind':'StructRef','struct_name':'crate::contracts::classic::common::DepositParameters'})
   for decl in d['declarations']:
    if decl['name']=='DedustLiquidityDepositInstall':
     for field in decl['fields']:
      if field['name']=='extraParams':field['ty_idx']=arguments;field['description']='Fixed initialized-program arguments: minLP, targets, fulfillment/rejection refs, then source-ignored suffix. See classic_initializers.test.tolk.'
  # Known proof/reference bodies are typed even where catalog leaves a cell.
  if cn in ['DedustVaultJettonV1','DedustVaultJettonV2','DedustVaultNativeV2']:
   generation='v1'if cn.endswith('V1')else'v2'
   own_proof=cn+'ProofData'
   for decl in d['declarations']:
    target=None
    if decl['name'].endswith('PayOutFromPool'):target='crate::contracts::classic::liquidity_deposit_'+generation+'::DedustPoolProof'
    elif decl['name'].endswith('PayOutFromDeposit'):target='crate::contracts::classic::liquidity_deposit_'+generation+'::DedustLiquidityDepositProof'
    elif decl['name'].endswith('SwapExternal')or decl['name'].endswith('CreateLiquidityDeposit'):target=own_proof
    if target:
     inner=len(ts);ts.append({'kind':'StructRef','struct_name':target})
     reference=len(ts);ts.append({'kind':'cellOf','inner_ty_idx':inner})
     for field in decl.get('fields',[]):
      if field['name']=='proof':field['ty_idx']=reference;field['description']='Typed classic child identity proof; canonical address commitment is checked separately by the contract.'
  if cn=='DedustVaultNativeV1':
   inner=len(ts);ts.append({'kind':'StructRef','struct_name':'crate::contracts::classic::pool_v1::DedustPoolV1SwapParams'})
   reference=len(ts);ts.append({'kind':'cellOf','inner_ty_idx':inner})
   step=next(i for i,t in enumerate(ts)if t['kind']=='StructRef'and t.get('struct_name')=='DedustVaultNativeV1SwapStep')
   step_ref=len(ts);ts.append({'kind':'cellOf','inner_ty_idx':step})
   step_optional=len(ts);ts.append({'kind':'nullable','inner_ty_idx':step_ref})
   for decl in d['declarations']:
    for field in decl.get('fields',[]):
     if field['name']=='swapParams':field['ty_idx']=reference;field['description']='Fixed shared classic swap constraints loaded by PoolV1 after native forwarding.'
     if decl['name']=='DedustVaultNativeV1SwapStepParams'and field['name']=='next':field['ty_idx']=step_optional;field['description']='Referenced next fixed native swap step; recursively typed.'
  if cn=='DedustV2Cpmm':
   config=len(ts);ts.append({'kind':'StructRef','struct_name':'crate::contracts::cpmm_v2::deposit::storage::DedustV2CpmmDepositConfig'})
   config_ref=len(ts);ts.append({'kind':'cellOf','inner_ty_idx':config})
   for decl in d['declarations']:
    if decl['name']in ['DedustV2CpmmJoinLiquidity','DedustV2CpmmRefundDeposit']:
     for field in decl['fields']:
      if field['name']=='config':field['ty_idx']=config_ref;field['description']='Fixed deposit configuration loaded by the pool/child path, not caller-defined payload. See position/deposit captured-code replay.'
  # Finite payment/forward schemas remain typed through their parent messages.
  if cn in ['DedustVaultJettonV1','DedustVaultJettonV2']:
   generation='v1'if cn.endswith('V1')else'v2'
   forward=len(ts);ts.append({'kind':'StructRef','struct_name':'ton::ton_core::types::tlb_core::TLBEitherRef<crate::contracts::classic::vault_jetton_'+generation+'::msgs::DedustVaultJetton'+generation.upper()+'ForwardMsg>'})
   for decl in d['declarations']:
    if decl['name']==cn+'TransferNotification':
     for field in decl['fields']:
      if field['name']=='forwardPayload':field['ty_idx']=forward;field['description']='Typed generation-specific swap/deposit dispatcher with exact Either inline/reference form and unknown-byte preservation.'
  if cn=='DedustV2Cpmm':
   payload=len(ts);ts.append({'kind':'StructRef','struct_name':'crate::contracts::cpmm_v2::pool::msgs::PayloadMsg'})
   payload_ref=len(ts);ts.append({'kind':'cellOf','inner_ty_idx':payload})
   notification=len(ts);ts.append({'kind':'StructRef','struct_name':'ton::ton_core::types::tlb_core::TLBEitherRef<crate::contracts::cpmm_v2::pool::msgs::JettonPayloadMsg>'})
   for decl in d['declarations']:
    if decl['name']in ['DedustV2CpmmPayNative','DedustV2CpmmPayJetton']:
     for field in decl['fields']:
      if field['name']=='paymentPayload':field['ty_idx']=payload_ref;field['description']='Referenced finite swap/deposit/reward operation. Known malformed bodies fail; unknown bytes remain preserved.'
    if decl['name']=='DedustV2CpmmTransferNotification':
     decl['fields']=[f for f in decl['fields']if f['name']!='isForwardPayloadRef']
     for field in decl['fields']:
      if field['name']=='forwardPayload':field['ty_idx']=notification;field['description']='Typed Either inline/reference CBC33949 payment frame; caller payloads inside its payout config remain opaque.'
  if cn=='DedustX1000WalletV2':
   operation=next(i for i,t in enumerate(ts)if t['kind']=='StructRef'and t.get('struct_name')=='DedustX1000CpmmSwapPayload')
   operation_ref=len(ts);ts.append({'kind':'cellOf','inner_ty_idx':operation})
   for decl in d['declarations']:
    if decl['name']=='DedustX1000CpmmPayJettonPayload':
     for field in decl['fields']:
      if field['name']=='operation':field['ty_idx']=operation_ref;field['description']='Fixed referenced swap record emitted by the supported X1000 protocol builder; not arbitrary operation code.'
  if cn=='DedustX1000WalletV2':
   runtime=len(ts);ts.append({'kind':'StructRef','struct_name':'crate::contracts::custom_wallet::v2::storage::RuntimeState'})
   runtime_ref=len(ts);ts.append({'kind':'cellOf','inner_ty_idx':runtime})
   signed=len(ts);ts.append({'kind':'StructRef','struct_name':'crate::contracts::custom_wallet::v2::msgs::SignedPayloadMsg'})
   callback_tail=len(ts);ts.append({'kind':'StructRef','struct_name':'crate::contracts::custom_wallet::v2::msgs::CallbackPayloadMsg'})
   address_index=next(i for i,t in enumerate(ts)if t['kind']=='address')
   for decl in d['declarations']:
    if decl['name']=='DedustX1000WalletV2ExecuteStoredMessage':
     for field in decl['fields']:
      if field['name']=='payload':field['name']='ignoredSuffix';field['description']='Source controller branch ignores all remaining bits/refs after request id; retained byte-exact, not an unresolved callback record.'
    if decl['name']=='DedustX1000WalletV2InternalCallback':
     decl['description']='Generic correlation envelope only. Use IncomingMessage for known fixed callbacks; parsing this envelope alone does not establish their schema or dispatch acceptance.'
     for field in decl['fields']:
      if field['name']=='payload':field['description']='Generic remainder preserved for explicit unknown program dispatch. Known fixed callback bodies are typed separately by IncomingMessage; see callbacks_v2 and source BLESS/CALLXARGS83527 boundary.'
    if decl['name']=='DedustX1000BidaskAdditionalData':
     optional=next(i for i,t in enumerate(ts)if t['kind']=='addressOpt')
     for field in decl['fields']:
      if field['name']=='referralAddress':field['ty_idx']=optional;field['description']='Optional referral; captured builder7 explicitly writes addr_none. Preserve null versus standard zero.'
    if decl['name']=='DedustX1000MoonClmmCallback':
     for field in decl['fields']:
      if field['name']=='payload':field['ty_idx']=callback_tail;field['description']='Preserve recipient-only compact form emitted by captured builder7 and explicitly marked optional payload. Peer acceptance of compact form is not assumed.'
    if decl['name']=='DedustX1000CoffeeNotification':
     decl['fields'].insert(0,{'name':'recipient','ty_idx':address_index,'description':'Mandatory receiver emitted by captured function7 STSTDADDR before notification gas/payload. Missing from the catalog schema.'})
    if decl['name']=='DedustX1000PayloadFa45056b':
     decl['fields'].insert(0,{'name':'address0','ty_idx':address_index,'description':'Leading standard address emitted after prefixfa45056b by captured function7. Broader application meaning is not guessed.'})
    if decl['name']=='DedustX1000WalletV2SignedRequest':
     for field in decl['fields']:
      if field['name']=='payload':field['ty_idx']=signed;field['description']='Fixed typed signed envelope and action; unresolved scalar meanings remain explicitly positional.'
    if decl['name']=='DedustX1000WalletV2Storage':
     for field in decl['fields']:
      if field['name']=='runtimeState':
       field['ty_idx']=runtime_ref;field['description']='Typed runtime dictionary envelopes and source-selected continuation contexts; see the independent branch replay evidence.'
      if field['name']=='implementationState':field['name']='implementationCode';field['description']='Executable implementation dispatch code invoked with BLESS/CALLXARGS, preserved as code. Not application state or a substitute for a missing record.'
  # Operator storage and upgrade are absent from the catalog but explicit in
  # captured dispatch/storage code (entrypoints 62/63/64).
  if cn=='DedustOperatorV2':
   def ti(kind,**attrs):
    found=next((i for i,t in enumerate(ts) if t['kind']==kind and all(t.get(k)==v for k,v in attrs.items())),None)
    if found is not None:return found
    index=len(ts);ts.append({'kind':kind,**attrs});return index
   st=len(ts);ts.append({'kind':'StructRef','struct_name':'DedustOperatorV2Storage'})
   d['storage']={'storage_ty_idx':st}
   d['declarations'].append({'kind':'struct','name':'DedustOperatorV2Storage','ty_idx':st,'description':'Exact storage read by captured Operator V2 code.',
    'fields':[{'name':'factory','ty_idx':ti('address')},{'name':'version','ty_idx':ti('uintN',n=16)},{'name':'role','ty_idx':ti('uintN',n=8)},{'name':'user','ty_idx':ti('address')},{'name':'canExecuteAfter','ty_idx':ti('uintN',n=32)}]})
   up=len(ts);ts.append({'kind':'StructRef','struct_name':'DedustOperatorV2Upgrade'})
   d['declarations'].append({'kind':'struct','name':'DedustOperatorV2Upgrade','ty_idx':up,'prefix':{'prefix_num':444,'prefix_len':32},
    'description':'Factory-authorized implementation upgrade found in captured dispatch.',
    'fields':[{'name':'queryId','ty_idx':ti('uintN',n=64)},{'name':'version','ty_idx':ti('uintN',n=16)},{'name':'code','ty_idx':ti('cell')}]})
   d['incoming_messages'].append({'body_ty_idx':up})
   reply=len(ts);ts.append({'kind':'StructRef','struct_name':'DedustOperatorV2Method79987Reply'})
   d['declarations'].append({'kind':'struct','name':'DedustOperatorV2Method79987Reply','ty_idx':reply,'fields':[{'name':'factory','ty_idx':ti('address')},{'name':'role','ty_idx':ti('uintN',n=8)},{'name':'user','ty_idx':ti('address')},{'name':'canExecuteAfter','ty_idx':ti('uintN',n=32)}]})
   d['get_methods'].append({'name':'get_method_79987','tvm_method_id':79987,'parameters':[],'return_ty_idx':reply,'description':'Exact numeric selector79987 returns factory, role, user and execution activation time; no synthetic-name hashing.'})
   d['get_methods'].append({'name':'get_version','tvm_method_id':82320,'parameters':[],'return_ty_idx':ti('uintN',n=16),'description':'Stored implementation version from captured selector82320.'})

  # Additional numeric exports discovered by complete executable dictionary scan.
  # Arguments stay unnamed where no defensible application meaning is proven.
  if cn in ['DedustVaultNativeV1','DedustVaultNativeV2','DedustVaultJettonV1','DedustVaultJettonV2','DedustPoolV1','DedustV2Cpmm']:
   def numeric_type(kind,**attrs):
    found=next((i for i,t in enumerate(ts)if t['kind']==kind and all(t.get(k)==v for k,v in attrs.items())),None)
    if found is not None:return found
    index=len(ts);ts.append({'kind':kind,**attrs});return index
   def add_numeric(selector,parameters,fields,description):
    name=cn+'Method'+str(selector)+'Reply';index=len(ts);ts.append({'kind':'StructRef','struct_name':name})
    d['declarations'].append({'kind':'struct','name':name,'ty_idx':index,'fields':[{'name':n,'ty_idx':t}for n,t in fields]})
    d['get_methods'].append({'name':'get_method_'+str(selector),'tvm_method_id':selector,'parameters':[{'name':n,'ty_idx':t}for n,t in parameters],'return_ty_idx':index,'description':description})
   integer=numeric_type('int')
   if 'Vault'in cn:
    add_numeric(109505,[('argument0',integer)],[('value',integer)],'Captured numeric gas-estimation export with one integer argument. V2 depends on the emulation blockchain configuration; no fixed gas policy is inferred.')
    add_numeric(117300,[('argument0',integer),('argument1',integer)],[('value',integer)],'Captured numeric gas-estimation export with two integer arguments. Source order and signed result are preserved; application parameter labels remain unproven.')
   if cn=='DedustVaultNativeV2':add_numeric(109867,[],[('nativeBalance',numeric_type('coins'))],'Captured numeric getter for the tracked native balance, distinct from account balance.')
   if cn=='DedustPoolV1':add_numeric(106049,[],[('decimalsX',numeric_type('uintN',n=8)),('decimalsY',numeric_type('uintN',n=8))],'Captured numeric export for both stored asset decimal counts.')
   if cn=='DedustV2Cpmm':add_numeric(112421,[('liquidity',integer)],[('amountX',integer),('amountY',integer)],'Captured nominal liquidity valuation. Zero pool liquidity returns zeroes; signed inputs and amounts above total are arithmetic getter values, not executable withdrawal authorization.')
  if cn=='DedustPoolV2':
   def ti(kind,**attrs):
    found=next((i for i,t in enumerate(ts) if t['kind']==kind and all(t.get(k)==v for k,v in attrs.items())),None)
    if found is not None:return found
    index=len(ts);ts.append({'kind':kind,**attrs});return index
   for selector,items in [(66722,[('provider0','addressOpt',None),('provider1','addressOpt',None)]),(106049,[('decimalsX','uintN',8),('decimalsY','uintN',8)]),(111021,[('factorX','uintN',128),('factorY','uintN',128)])]:
    name='DedustPoolV2Method'+str(selector)+'Reply';index=len(ts);ts.append({'kind':'StructRef','struct_name':name})
    fs=[{'name':n,'ty_idx':ti(k,**({'n':bits} if bits else {}))} for n,k,bits in items]
    d['declarations'].append({'kind':'struct','name':name,'ty_idx':index,'fields':fs})
    d['get_methods'].append({'name':'get_method_'+str(selector),'tvm_method_id':selector,'parameters':[],'return_ty_idx':index,'description':'Captured numeric selector; signature reconciled against exact getter dispatch.'})
  if cn=='DedustPoolV2':
   response=len(ts);ts.append({'kind':'StructRef','struct_name':'DedustPoolV2QuoteResponse'})
   context=len(ts);ts.append({'kind':'nullable','inner_ty_idx':ti('cell')})
   d['declarations'].append({'kind':'struct','name':'DedustPoolV2QuoteResponse','ty_idx':response,'prefix':{'prefix_num':0x0a420458,'prefix_len':32},'description':'Fixed quote-provider response omitted from the catalog. Captured function147 authenticates one configured provider, loads this complete body, then replaces normalization factors. Parsing proves no authority.',
    'fields':[{'name':'queryId','ty_idx':ti('uintN',n=64)},{'name':'numerator','ty_idx':ti('uintN',n=128)},{'name':'denominator','ty_idx':ti('uintN',n=128)},{'name':'ignoredContext','ty_idx':context,'description':'LDDICT/ENDS followed by DROP: the optional referenced cell is not inspected. Preserve its bytes without inventing a context ABI.'}]})
   d['incoming_messages'].append({'body_ty_idx':response})
  if cn=='DedustV2Cpmm':
   mapping={'walletsByAssets':('TonHash','MsgAddressInt',256),
            'assetsByWallets':('TonHash','DedustV2CpmmAsset',256),
            'walletsResolutions':('TonHash','DedustV2CpmmAsset',256),
            'rewards':('u8','DedustV2CpmmPoolReward',2)}
   for decl in d['declarations']:
    if decl['name']=='DedustV2CpmmPoolDataReply':
     for field in decl['fields']:
      if field['name'] in mapping:
       key,value,bits=mapping[field['name']];field['ty_idx']=len(ts)
       ts.append({'kind':'stackDict','key':key,'value':value,'bits':bits})
  # Classic stack slices have known Asset wire types; keep their typed identity
  # at provider boundaries rather than requiring callers to decode a raw cell.
  asset=next((x['name'] for x in d['declarations'] if x['kind']=='alias' and x['name'].endswith('Asset')),None)
  if asset:
   decl_by_name={x['name']:x for x in d['declarations']}
   for getter in d['get_methods']:
    rt=ts[getter['return_ty_idx']]
    output_fields=decl_by_name[rt['struct_name']]['fields'] if rt['kind']=='StructRef' else []
    for field in getter['parameters']+output_fields:
     if ts[field['ty_idx']]['kind']=='slice':
      field['ty_idx']=len(ts);ts.append({'kind':'stackSlice','rust_type':asset})
    if rt['kind']=='slice':
     getter['return_ty_idx']=len(ts);ts.append({'kind':'stackSlice','rust_type':asset})
  decls={x['name']:x for x in d['declarations']}
  # Classic generations share the complete native/jetton identity, including
  # the wire tags. Preserve catalog identities in metadata, not duplicate types.
  shared_assets = {}
  if mod.startswith('classic/') and asset:
   shared_assets[asset] = 'DedustClassicAsset'
   for variant in ts[decls[asset]['target_ty_idx']]['variants']:
    name = ts[variant['variant_ty_idx']]['struct_name']
    shared_assets[name] = {0: 'DedustClassicNativeAsset', 1: 'DedustClassicJettonAsset'}[decls[name]['prefix']['prefix_num']]
  def shared_type_path(name):
   if name=='crate::contracts::classic::common::method_results::VersionedCode':return 'VersionedCode'
   if name in shared_assets:return 'crate::contracts::classic::asset::'+shared_assets[name]
   if mod=='classic/vault_jetton_v2' and name in SHARED_VAULT_RECORDS:
    return 'crate::contracts::classic::vault_native_v2::storage::'+name
   return name
  def referenced(i):
   t=ts[i];k=t['kind']
   if k in ['StructRef','AliasRef']:return {t.get('struct_name',t.get('alias_name'))}
   if k in ['nullable','cellOf']:return referenced(t['inner_ty_idx'])
   if k=='union':return set().union(*(referenced(v['variant_ty_idx']) for v in t['variants']))
   return set()
  edges={name:set().union(*(referenced(f['ty_idx']) for f in x.get('fields',[]))) if x['kind']=='struct' else referenced(x['target_ty_idx']) if x['kind']=='alias' else set() for name,x in decls.items()}
  def cyclic(name,current,seen):
   if current in seen:return False
   return any(child==name or cyclic(name,child,seen|{current}) for child in edges.get(current,set()))
  recursive={name for name in edges if cyclic(name,name,set())}
  def schema_path(name):
   for old in existing:
    for symbol in old['symbols']:
     if symbol['module'] in (old['module'],old['module']+'/storage') and symbol.get('rust_kind') not in ('method','result'):
      source='crate::contracts::'+old['module'].replace('/','::')+'::'+symbol['declaration']
      name=name.replace(source,source.rsplit('::',1)[0]+'::storage::'+symbol['declaration'])
   return name
  def typ(i):
   t=ts[i];k=t['kind']
   if k in ['StructRef','AliasRef','EnumRef']:
    name=t.get('struct_name',t.get('alias_name',t.get('enum_name')))
    return 'VarLenBytes<U256, 5>' if name=='DedustV2CpmmQ120X120' else shared_type_path(schema_path(name))
   if k=='uintN':return number(t['n'])
   if k=='intN':return number(t['n'],True)
   if k=='bitsN':return 'TonHash' if t['n']==256 else 'U512'
   if k=='int':return 'I512'
   if k=='trailingStruct':return 'Option<'+t['rust_type']+'>'
   if k=='stackDict':return f'HashMap<{t["key"]},{t["value"]}>'
   if k=='stackSlice':return shared_type_path(t['rust_type'])
   if k=='bool':return 'bool'
   if k=='coins':return 'TLBCoins'
   if k=='address':return 'TonAddress'
   if k=='addressOpt':return 'Option<TonAddress>'
   if k=='addressAny':return 'MsgAddress' if mod.startswith('custom_wallet/') else 'Option<TonAddress>'
   if k=='cell':return 'TLBRef<TonCell>'
   if k in ['slice','remaining']:return 'TonCell'
   if k=='string':return 'TLBRef<SnakeData>'
   if k=='cellOf':
    inner='MsgAddressInt' if ts[t['inner_ty_idx']]['kind']=='address' else 'MsgAddress' if ts[t['inner_ty_idx']]['kind']=='addressOpt' else typ(t['inner_ty_idx']);return f'TLBRef<Box<{inner}>>' if inner.rsplit('::',1)[-1] in recursive else f'TLBRef<{inner}>'
   if k=='nullable':
    raw=f'Option<{typ(t["inner_ty_idx"])}>'
    if not mod.startswith('custom_wallet/'):
     return {'Option<TLBRef<MsgAddressInt>>':'Option<TonAddress>', 'Option<TLBRef<MsgAddress>>':'Option<Option<TonAddress>>', 'Option<MsgAddressInt>':'Option<TonAddress>'}.get(raw,raw)
    return raw
   if k=='varuintN':return 'VarLenBytes<U256, 5>'
   if k=='mapKV':
    kt=ts[t['key_ty_idx']]; key='TonHash' if kt.get('n')==256 else typ(t['key_ty_idx'])
    return f'HashMap<{key}, {typ(t["value_ty_idx"])}>'
   raise ValueError((cn,t))
  def expr(i,write=None):
   t=ts[i];k=t['kind'];T=typ(i)
   if k=='trailingStruct':return f'if p.data_bits_left()?>0 {{Some({t["rust_type"]}::read(p)?)}}else{{None}}'if write is None else f'if let Some(value)={write}{{value.write(b)?;}}'
   if k=='address':return 'InternalAddress.read(p)?' if write is None else f'InternalAddress.write(b, {write})?;'
   if k=='addressOpt' or (k=='addressAny' and not mod.startswith('custom_wallet/')):return 'NullableAddress.read(p)?' if write is None else f'NullableAddress.write(b, {write})?;'
   if k in ['cell','cellOf','string']:return f'<{T}>::read(p)?' if write is None else f'{write.removeprefix(chr(38))}.write(b)?;'
   if k=='bitsN' and t['n']==256:
    return 'TonHash::read(p)?' if write is None else f'{write.removeprefix(chr(38))}.write(b)?;'
   if k in ['uintN','intN','bitsN']:
    return f'p.read_num({t["n"]})?' if write is None else f'b.write_num({write}, {t["n"]})?;'
   if k in ['StructRef','AliasRef','EnumRef','bool','coins','address','addressOpt','addressAny','varuintN']:
    return f'<{T}>::read(p)?' if write is None else f'{write.removeprefix(chr(38))}.write(b)?;'
   if k in ['remaining','slice']:return 'p.read_remaining()?' if write is None else f'b.write_cell({write})?;'
   if k=='nullable':
    inner=t['inner_ty_idx']
    return f'if p.read_bit()? {{ Some({expr(inner)}) }} else {{ None }}' if write is None else f'if let Some(v) = {write} {{ b.write_bit(true)?; {expr(inner,"v")} }} else {{ b.write_bit(false)?; }}'
   if k=='mapKV':
    kt=ts[t['key_ty_idx']];ka='DictKeyAdapterTonHash' if kt.get('n')==256 else f'DictKeyAdapterUint<{typ(t["key_ty_idx"])}>'
    vt=ts[t['value_ty_idx']]; va=f'DictValAdapterNum<{typ(t["value_ty_idx"])}, {vt["n"]}>' if vt['kind'] in ['uintN','intN'] else f'DictValAdapterTLB<{typ(t["value_ty_idx"])}>'
    ad=f'TLBHashMapE::<{ka}, {va}>::new({kt["n"]})'
    return f'{ad}.read(p)?' if write is None else f'{ad}.write(b, {write})?;'
   raise ValueError(t)
  def fields(s):return s.get('fields',[])
  # Message ownership follows protocol role, not the mere presence of a prefix.
  message_names={x['name'] for x in d['declarations'] if x.get('prefix',{}).get('prefix_len')==32 or x.get('message_role')=='bounce' or x['name'].endswith('InternalCallback')}
  for role in ['incoming_messages','outgoing_messages','incoming_external','emitted_events']:
   for message in d[role]:
    t=ts[message.get('body_ty_idx',message.get('event_ty_idx'))]
    name=t.get('struct_name',t.get('alias_name'))
    if name:message_names.add(name)
  # Union envelopes composed solely of messages also belong with messages.
  for x in d['declarations']:
   if x['kind']=='alias' and ts[x['target_ty_idx']]['kind']=='union':
    variants=ts[x['target_ty_idx']]['variants']
    if all(ts[v['variant_ty_idx']].get('struct_name') in message_names for v in variants):message_names.add(x['name'])
  renames={name:re.sub(r'Message$','',name)+'Msg' for name in message_names}
  symbols=[]
  for x in d['declarations']:
   symbols.append({'declaration':x['name'],'rust_name':renames.get(x['name'],x['name']),'module':mod+('/msgs' if x['name'] in message_names else '/storage'),'owner':'generated','fields':[{'name':f['name'],'rust_type':typ(f['ty_idx']),'wire_kind':ts[f['ty_idx']]['kind'],'source_description':f.get('description','')} for f in x.get('fields',[])]})
  for symbol in symbols:
   if mod=='classic/vault_jetton_v2' and symbol['declaration'] in SHARED_VAULT_RECORDS:
    canonical=next(s for row in records if row['module']=='classic/vault_native_v2' for s in row['symbols'] if s['declaration']==symbol['declaration'])
    symbol.update(module=canonical['module'])
    if 'variants' in canonical:symbol['variants']=canonical['variants']
   if symbol['declaration'] in shared_assets:
    symbol.update(module='classic/asset', rust_name=shared_assets[symbol['declaration']], owner='handwritten')
    if symbol['declaration'] == asset:
     symbol['stack_traits'] = ['FromTVMStack', 'ToTVMStack']
     symbol['variants'] = [{'declaration':name, 'rust_name':rust_type.removeprefix('DedustClassic').removesuffix('Asset'), 'boxed':False}
                           for name,rust_type in shared_assets.items() if name != asset]
   if symbol['declaration']=='DedustV2CpmmQ120X120':
    symbol.update(rust_type='ton::ton_core::types::tlb_core::VarLenBytes<fastnum::U256, 5>', rust_name='VarLenBytes<U256, 5>', rust_kind='primitive', coverage_symbol={'module':mod+'/storage','rust_name':'DedustV2CpmmPoolFees'})
  # Stack-only declarations are not serialized as cells.
  stack_names=set()
  for g in d['get_methods']:
   rt=ts[g['return_ty_idx']]
   if rt['kind']=='StructRef':stack_names.add(rt['struct_name'])
  header=['//! Generated schema records from the pinned TON ABI catalog.\n//! Ownership and provenance: tools/reference/dedust/bindings-generation.json; guidance: tools/reference/dedust/README.md.\n']
  out=header.copy(); parent_out=out; message_out=header.copy()
  for s in d['declarations']:
   name=s['name'];kind=s['kind']
   out=message_out if name in message_names else parent_out
   if name in stack_names or name in shared_assets or name=='DedustV2CpmmQ120X120' or (mod=='classic/vault_jetton_v2' and name in SHARED_VAULT_RECORDS):continue
   if kind=='struct':
    derived=not any(ts[f['ty_idx']]['kind']=='trailingStruct' for f in fields(s))
    out += [doc(s.get('description')), '#[derive(Debug, Clone, PartialEq'+(', TLB' if derived else '')+')]\n']
    if derived and 'prefix' in s:out +=[f'#[tlb(prefix = {s["prefix"]["prefix_num"]}, bits_len = {s["prefix"]["prefix_len"]})]\n']
    out +=[f'pub struct {name} {{\n']
    for f in fields(s):
     t=ts[f['ty_idx']];k=t['kind'];attribute=None
     if k=='address':attribute='InternalAddress'
     if k=='addressOpt' or (k=='addressAny' and not mod.startswith('custom_wallet/')):attribute='NullableAddress'
     if k=='nullable' and not mod.startswith('custom_wallet/'):
      inner=ts[t['inner_ty_idx']]
      if inner['kind']=='cellOf':
       address_kind=ts[inner['inner_ty_idx']]['kind']
       if address_kind in ('address','addressOpt'):attribute='MaybeRefAddress' if address_kind=='address' else 'MaybeRefNullableAddress'
      elif inner['kind']=='address':attribute='MaybeAddress'
     if k=='mapKV':
      kt=ts[t['key_ty_idx']];vt=ts[t['value_ty_idx']]
      ka='DictKeyAdapterTonHash' if kt.get('n')==256 else f'DictKeyAdapterUint<{typ(t["key_ty_idx"])}>'
      va=f'DictValAdapterNum<{typ(t["value_ty_idx"])}, {vt["n"]}>' if vt['kind'] in ['uintN','intN'] else f'DictValAdapterTLB<{typ(t["value_ty_idx"])}>'
      if vt['kind']=='address':va='crate::contracts::utils::DictValueInternalAddress'
      attribute=f'TLBHashMapE::<{ka}, {va}>::new({kt["n"]})'
     note='Q120.120 numerator, encoded as varuint32.' if typ(f['ty_idx'])=='VarLenBytes<U256, 5>' and ts[f['ty_idx']]['kind']=='StructRef' else FIELD_NOTES.get((cn, name, f['name']))
     out +=[doc(note)]
     if derived and attribute:out +=[f'#[tlb(adapter = "{attribute}")]\n']
     if derived and k in ['uintN','intN','bitsN'] and not(k=='bitsN' and t['n']==256) and t['n'] != DEFAULT_WIDTHS.get(typ(f['ty_idx'])):out +=[f'#[tlb(bits_len = {t["n"]})]\n']
     out +=[f'pub {snake(f["name"])}: {typ(f["ty_idx"])},\n']
    out +=['}\n']
    if not derived:
     out +=[f'impl TLB for {name} {{\n']
     if 'prefix'in s:
      pr=s['prefix'];out +=[f'const PREFIX: TLBPrefix = TLBPrefix::new({pr["prefix_num"]}, {pr["prefix_len"]});\n']
     out+=['fn read_definition(p: &mut CellParser) -> Result<Self, TonCoreError> {\nOk(Self {\n']
     for f in fields(s):out +=[f'{snake(f["name"])}: {expr(f["ty_idx"])},\n']
     out+=['})\n}\nfn write_definition(&self, b: &mut CellBuilder) -> Result<(), TonCoreError> {\n']
     for f in fields(s):out +=[expr(f['ty_idx'],'&self.'+snake(f['name']))+'\n']
     out+=['Ok(())\n}\n}\n']
   elif kind=='enum':
    if name.endswith('Errors') or name=='Errors':
     out +=[doc(s.get('description')),'#[derive(Debug, Clone, Copy, PartialEq, Eq)]\n#[repr(i32)]\n',f'pub enum {name} {{\n']
     for m in s['members']:out +=[doc(m.get('description')),f'{m["name"]}={m["value"]},\n']
     out+=['}\n'];continue
    enc=ts[s['encoded_as_ty_idx']];n=enc['n']
    out +=[doc(s.get('description')),'#[derive(Debug, Clone, Copy, PartialEq, Eq)]\n',f'pub enum {name} {{\n']
    for m in s['members']:out +=[doc(m.get('description')),f'{m["name"]},\n']
    out+=['}\n',f'impl TLB for {name} {{ fn read_definition(p: &mut CellParser) -> Result<Self,TonCoreError> {{ match p.read_num::<u64>({n})? {{\n']
    for m in s['members']:out +=[f'{m["value"]} => Ok(Self::{m["name"]}),\n']
    out +=[f'tag => Err(crate::contracts::utils::invalid_tag("{name}",tag)), }} }}\nfn write_definition(&self,b:&mut CellBuilder)->Result<(),TonCoreError>{{let tag=match self{{\n']
    for m in s['members']:out +=[f'Self::{m["name"]}=>{m["value"]}u64,\n']
    out +=[f'}}; b.write_num(&tag,{n}) }} }}\n']
   elif kind=='alias':
    target=ts[s['target_ty_idx']]
    if target['kind']!='union':continue
    out +=[doc(s.get('description')),'#[derive(Debug, Clone, PartialEq, TLB)]\n',f'pub enum {name} {{\n']
    variant_records=[]
    # These closed schema unions have small, nonrecursive payloads. Keep large
    # callback/message unions boxed; wire prefixes remain on the payload types.
    small=name not in recursive and name.endswith(('Asset','PoolType','SwapKind','Status','Activation'))
    for v in target['variants']:
     vn=typ(v['variant_ty_idx'])
     if name.endswith('Asset'):
      variant='Native' if vn.endswith(('NativeAsset','AssetNative')) else 'Jetton'
     elif name.endswith('PoolType'):
      variant='Stable' if 'Stable' in vn else 'Volatile'
     elif name.endswith('SwapKind'):
      variant='GivenIn' if vn.endswith('GivenIn') else 'GivenOut'
     elif name.endswith(('Status','Activation')):
      variant=vn.removeprefix(name)
     else:variant=vn
     variant_records.append({'declaration':vn,'rust_name':variant,'boxed':not small})
     payload=vn if small else f'Box<{vn}>'
     out +=[f'{variant}({payload}),\n']
    next(symbol for symbol in symbols if symbol['declaration']==name)['variants']=variant_records
    out +=['}\n']
  out=message_out
  # Distinct enum for incoming/outgoing dispatch; no error fallback on recognized opcodes.
  for role in ['incoming_messages','outgoing_messages','incoming_external','emitted_events']:
   variants=[]
   for m in d[role]:
    idx=m.get('body_ty_idx',m.get('event_ty_idx'))
    # A one-body outgoing alias is an actual known dispatch, not unknown bytes.
    while ts[idx]['kind']=='AliasRef'and ts[decls[typ(idx)]['target_ty_idx']]['kind']!='union':idx=decls[typ(idx)]['target_ty_idx']
    t=ts[idx];vn=typ(idx);s=decls.get(vn)
    if s and 'prefix'in s:variants.append((vn,s['prefix']))
   if not variants:continue
   if cn=='DedustX1000WalletV2'and role=='incoming_messages':
    callbacks=[('JettonNotification',0x7362d09c),('JettonExcesses',0xd53276db),('ClassicNativePayout',0x474f86cf),('CoffeePayout',0xc0ffee32),('PtonTransfer',0x01f3835d),('BidaskNotification',0x6edd65f0),('MoonClmmNativeResult',0x9d142643),('MoonClmmNativeRefund',0xff32da99),('DedustX1000MoonSwapSucceeded',0xcb7f38d6),('DedustX1000MoonSwapFailed',0xc47c1f57)]
    out+=['use crate::contracts::custom_wallet::v2::msgs::{'+','.join(name+'Msg' for name,_ in callbacks if not name.startswith('Dedust'))+'};\n']
    for name,tag in callbacks:variants.append((name+'Msg',{'prefix_num':tag,'prefix_len':32}))
   enum=cn+{'incoming_messages':'IncomingMsg','outgoing_messages':'OutgoingMsg','incoming_external':'ExternalMsg','emitted_events':'EventMsg'}[role]
   symbols.append({'declaration':{'incoming_messages':'IncomingMessage','outgoing_messages':'OutgoingMessage','incoming_external':'ExternalMessage','emitted_events':'Event'}[role],'rust_name':enum,'module':mod+'/msgs'})
   # Native derive treats nested TLBWrongPrefix as a top-level miss. Classic
   # pool proofs contain fixed tags, so commit to the matched opcode there.
   manual = cn in ('DedustPoolV1', 'DedustPoolV2')
   out += ['/// Typed dispatch with raw-body preservation; see the crate README for prefix-mismatch fallback.\n']
   if manual:out += ['/// Manual dispatch preserves errors from fixed-prefix nested proofs.\n']
   out += ['#[derive(Debug, Clone, PartialEq'+('' if manual else ', TLB')+')]\n', f'pub enum {enum} {{\n']
   def variant(vn):
    value=vn.removeprefix(cn).removeprefix('Dedust')
    return value.removesuffix('Message').removesuffix('Msg')
   for vn,pr in variants:out +=[f'{variant(vn)}(Box<{vn}>),\n']
   widths={prefix['prefix_len'] for _,prefix in variants}
   if len(widths)!=1:raise ValueError(f'Mixed dispatch prefix widths: {enum}')
   width=next(iter(widths))
   tags=[prefix['prefix_num'] for _,prefix in variants]
   if len(tags)!=len(set(tags)):raise ValueError(f'Duplicate dispatch prefix: {enum}')
   out+=['Unknown(TonCell),\n}\n']
   if manual:
    out+=[f'impl TLB for {enum} {{\nfn read_definition(p:&mut CellParser)->Result<Self,TonCoreError>{{\n',
          f'if p.data_bits_left()?<{width}{{return Ok(Self::Unknown(p.read_remaining()?));}}\n',
          f'match p.clone().read_num::<u64>({width})?{{\n']
    for vn,pr in variants:
     out +=[f'{pr["prefix_num"]:#x}=>Ok(Self::{variant(vn)}(Box::new(<{vn}>::read(p)?))),\n']
    out+=['_=>Ok(Self::Unknown(p.read_remaining()?)),\n}\n}\nfn write_definition(&self,b:&mut CellBuilder)->Result<(),TonCoreError>{match self {\n']
    for vn,_ in variants:out +=[f'Self::{variant(vn)}(v)=>v.write(b),\n']
    out+=['Self::Unknown(v)=>b.write_cell(v),\n}}}\n']
  def stacktype(i):
   t=ts[i];k=t['kind']
   if k in ['cell','cellOf','slice','remaining']:return 'TonCell'
   if k in ['coins','int']:return 'I512'
   if k=='nullable':return 'Option<'+stacktype(t['inner_ty_idx'])+'>'
   return typ(i)
  getter_records=[]
  for g in d['get_methods']:
   original=g['name']; name=GETTER_NAMES.get(g['tvm_method_id'],original) if original.startswith('get_method_') else original
   rt=ts[g['return_ty_idx']]
   fs=decls[rt['struct_name']]['fields'] if rt['kind']=='StructRef' else [{'name':'value','ty_idx':g['return_ty_idx']}]
   result=''.join(x.title() for x in name.split('_'))+'Result'
   scalar=len(fs)==1
   coin_fields=GETTER_COINS_FIELDS.get((mod,g['tvm_method_id']),())
   unsigned_fields=GETTER_U256_FIELDS.get((mod,g['tvm_method_id']),())
   getter_fields=[{'name':snake(f['name']),'rust_type':GETTER_NATIVE_FIELDS.get((mod,g['tvm_method_id'])) or ('Coins' if snake(f['name']) in coin_fields else 'U256' if snake(f['name']) in unsigned_fields else stacktype(f['ty_idx']))} for f in fs]
   getter_records.append({'name':original,'rust_name':name,'selector':g['tvm_method_id'],'module':mod,'owner':'handwritten',
    'result':None if scalar else result,'result_module':None if scalar else mod+'/method_results','return_type':getter_fields[0]['rust_type'] if scalar else result,
    'parameters':[{'name':snake(f['name']),'rust_type':stacktype(f['ty_idx']),'wire_kind':ts[f['ty_idx']]['kind']} for f in g['parameters']],
    'fields':getter_fields})
   if rt['kind']=='StructRef':
    for symbol in symbols:
     if symbol['declaration']==rt['struct_name']:
      symbol.update(module=mod if scalar else mod+'/method_results',rust_name=name if scalar else result,owner='handwritten',rust_kind='method' if scalar else 'result')
      for field in symbol.get('fields',[]):
       if (mod,g['tvm_method_id']) in GETTER_NATIVE_FIELDS:field['rust_type']=GETTER_NATIVE_FIELDS[(mod,g['tvm_method_id'])]
       elif snake(field['name']) in coin_fields:field['rust_type']='Coins'
       elif snake(field['name']) in unsigned_fields:field['rust_type']='U256'
  if cn in ('DedustFactoryV1','DedustFactoryV2'):
   for getter in getter_records:
    for param in getter['parameters']:
     if param['name']=='pool_type':param.update(name='is_stable',rust_type='bool',stack_encoding='integer_0_or_1',source_name='poolType')
   extra=[(79554,'get_pool_code','VersionedCode',[]),(120195,'get_liquidity_deposit_code','VersionedCode',[]),
          (129291,'get_vault_code','VersionedCode',[{'name':'asset_type','rust_type':'u8','wire_kind':'uintN'}])]
   if cn=='DedustFactoryV2':extra += [(78248,'get_operator_code','NullableVersionedCode',[]),
       (126188,'get_operator_address',None,[{'name':'role','rust_type':'u8','wire_kind':'uintN'}])]
   for selector,name,result,params in extra:
    getter_records.append({'name':'get_method_'+str(selector),'rust_name':name,'selector':selector,'module':mod,'owner':'handwritten',
     'result':result,'return_type':'MsgAddressInt' if result is None else 'crate::contracts::classic::common::method_results::'+result,'result_module':None if result is None else 'classic/common/method_results',
     'parameters':params,'fields':[{'name':'address','rust_type':'MsgAddressInt'}] if result is None else [
      {'name':'version','rust_type':'u16'},{'name':'code','rust_type':'Option<TonCell>' if result=='NullableVersionedCode' else 'TLBRef<TonCell>'}]})
  has_messages=cn!='DedustX1000WalletV1' and len(message_out)>len(header)
  if has_messages:
   parent_names=[x['name'] for x in d['declarations'] if x['name'] not in message_names and x['name'] not in stack_names and x['name']!='DedustV2CpmmQ120X120' and (x['kind']!='alias' or ts[x['target_ty_idx']]['kind']=='union')]
   syntax=re.sub(r'(?m)^\s*//[^\n]*','', ''.join(message_out))
   used=[n for n in parent_names if re.search(r'(?<![\w:])'+re.escape(n)+r'\b',syntax)]
   if used:message_out.insert(1,'use super::storage::{'+','.join(used)+'};\n')
  if cn in ['DedustVaultJettonV1','DedustVaultJettonV2']:
   forward=cn+'ForwardMsg';swap=cn+'SwapForwardPayload';deposit=cn+'DepositLiquidityForwardPayload'
   message_out +=[f"""/// TEP-74 forward body; short and unknown bodies retain their raw cells.
 #[derive(Debug, Clone, PartialEq, TLB)]
 pub enum {forward} {{ Swap(Box<{swap}>), DepositLiquidity(Box<{deposit}>), Unknown(TonCell) }}
 """]
  def renamed(text):
   return re.sub(r'\b(?:'+ '|'.join(map(re.escape,sorted(renames,key=len,reverse=True)))+r')\b',lambda m:renames[m[0]],text) if renames else text
  for symbol in symbols:
   for field in symbol.get('fields',[]):field['rust_type']=renamed(field['rust_type'])
  def render(parts):
   body=renamed(''.join(parts))
   body=body.replace('crate::contracts::classic::asset::DedustClassicAsset', 'DedustClassicAsset')
   # Width adapters are strings; retain those identifiers in dependency analysis.
   return body.split('\n',2)[0]+'\n'+body.split('\n',2)[1]+'\n'+imports(body)+body.split('\n',2)[2]
  dest=ROOT/'crates/dedust/src/contracts'/mod/'storage.rs'
  if dest in generated_paths():
   write_generated(dest,rename_source(render(parent_out), mod+'/storage'))
  outputs=([str(dest.relative_to(ROOT))] if dest in generated_paths() else [])+[f'crates/dedust/src/contracts/{mod.split(chr(47))[0]}.rs']
  if has_messages:
   msgs=dest.with_name('msgs.rs')
   if msgs in generated_paths():
    write_generated(msgs,rename_source(render(message_out), mod+'/msgs'));outputs.append(str(msgs.relative_to(ROOT)))
  for symbol in symbols:
   for field in symbol.get('fields',[]):
    for declaration in symbols:
     if declaration['module'].endswith('/storage'):
      old='crate::contracts::'+mod.replace('/', '::')+'::'+declaration['rust_name']
      field['rust_type']=field['rust_type'].replace(old,old.rsplit('::',1)[0]+'::storage::'+declaration['rust_name'])
  records.append({'contract':cn,'module':mod,'owner':'handwritten','schema_sha256':hashlib.sha256(path.read_bytes()).hexdigest(),
   'api_path':f'crates/dedust/src/contracts/{mod}.rs','generated_path':str(dest.relative_to(ROOT)) if dest in generated_paths() else None,'generated_paths':outputs,
   'symbols':symbols,'getters':getter_records,'wrapper':cn+'Contract','handwritten_children':(['msgs'] if cn=='DedustX1000WalletV1' else ['storage','msgs'] if cn=='DedustX1000WalletV2' else [])})
  if excluded_declarations:
   records[-1]['excluded_declarations'] = excluded_declarations
 for group in sorted(set(x.split('/')[0] for x in MODS.values())):
  mods=sorted(x.split('/')[1] for x in MODS.values() if x.startswith(group+'/'))
  if group=='cpmm_v2':mods+=['position','deposit','deployment']
  if group=='uranus':mods+=['deployment','wallet_msgs']
  if group=='classic':mods+=['asset','lp_wallet','common','operator_pool']
  write_generated(ROOT/'crates/dedust/src/contracts'/f'{group}.rs','//! '+group+' contract generations.\n'+''.join('pub mod '+x+';\n' for x in mods))
 records.extend(handwritten_apis(records))
 # Public getter types follow the standard-address API; schema types retain provenance.
 for record in records:
  for getter in record['getters']:
   for field in getter['fields']:
    field['rust_type']={'MsgAddress':'Option<TonAddress>', 'MsgAddressInt':'TonAddress', 'HashMap<TonHash, MsgAddressInt>':'HashMap<TonHash, TonAddress>'}.get(field['rust_type'],field['rust_type'])
   if getter.get('return_type')=='MsgAddressInt':getter['return_type']='TonAddress'
 rename_metadata(records)
 write_generated(ROOT/'tools/reference/dedust/bindings-generation.json',json.dumps(records,indent=2)+'\n')
 print('Generated',len(MODS),'schema modules; mapped',len(records),'contract APIs')

if __name__=='__main__':
 main(sys.argv[1])

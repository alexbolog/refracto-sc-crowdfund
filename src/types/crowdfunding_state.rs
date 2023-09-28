multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(TopEncode, TopDecode, TypeAbi)]
pub struct CrowdfundingState<M: ManagedTypeApi> {
    pub project_id: u64,
    pub share_token_identifier: TokenIdentifier<M>,
    pub share_token_total_supply: BigUint<M>,
    pub share_price_per_unit: BigUint<M>,

    pub cf_start_timestamp: u64,
    pub cf_end_timestamp: u64,

    pub cf_target_min: BigUint<M>,
    pub cf_target_max: BigUint<M>,
    pub cf_progress: BigUint<M>,
}

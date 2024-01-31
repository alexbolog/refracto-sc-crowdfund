use super::crowdfunding_state::{CrowdfundingStateContext, ProjectFundingState};

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub const INTEREST_RATE_DENOMINATION: u64 = 1_000_000_000;

#[derive(TopEncode, TopDecode, TypeAbi, ManagedVecItem, NestedDecode, NestedEncode)]
pub struct CrowdfundingStateUiContext<M: ManagedTypeApi> {
    pub project_id: u64,
    pub project_name: ManagedBuffer<M>, // good to easier identify what's going on
    pub project_payment_token: TokenIdentifier<M>,

    pub daily_interest_rate: u64,
    pub daily_penalty_fee_rate: u64,
    pub developer_wallet: ManagedAddress<M>,

    pub share_token_nonce: u64,
    pub share_price_per_unit: BigUint<M>,

    pub cf_start_timestamp: u64,
    pub cf_end_timestamp: u64,

    pub cf_target_min: BigUint<M>,
    pub cf_target_max: BigUint<M>,
    pub cf_progress: BigUint<M>,

    pub loan_duration: u64,
    pub loan_start_timestamp: u64,
    pub repayment_contract_address: ManagedAddress<M>,

    pub is_cancelled: bool,
    pub is_loan_active: bool,
    pub is_repayed: bool,

    pub funding_state: usize,
}

impl<M: ManagedTypeApi> CrowdfundingStateUiContext<M> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        internal_project_context: CrowdfundingStateContext<M>,
        funding_state: ProjectFundingState,
    ) -> Self {
        CrowdfundingStateUiContext {
            project_id: internal_project_context.project_id,
            project_name: internal_project_context.project_name,
            project_payment_token: internal_project_context.project_payment_token,
            daily_interest_rate: internal_project_context.daily_interest_rate,
            daily_penalty_fee_rate: internal_project_context.daily_penalty_fee_rate,
            developer_wallet: internal_project_context.developer_wallet,
            share_token_nonce: internal_project_context.share_token_nonce,
            share_price_per_unit: internal_project_context.share_price_per_unit,
            cf_start_timestamp: internal_project_context.cf_start_timestamp,
            cf_end_timestamp: internal_project_context.cf_end_timestamp,
            cf_target_min: internal_project_context.cf_target_min,
            cf_target_max: internal_project_context.cf_target_max,
            cf_progress: internal_project_context.cf_progress,
            loan_duration: internal_project_context.loan_duration,
            loan_start_timestamp: internal_project_context.loan_start_timestamp,
            repayment_contract_address: internal_project_context.repayment_contract_address,
            is_cancelled: internal_project_context.is_cancelled,
            is_loan_active: internal_project_context.is_loan_active,
            is_repayed: internal_project_context.is_repayed,
            funding_state: funding_state as usize,
        }
    }
}

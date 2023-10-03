multiversx_sc::imports!();

use crate::types::crowdfunding_state::CrowdfundingStateContext;

#[multiversx_sc::module]
pub trait ConfigModule {
    #[view(getWithdrawFromLoanTimeSpan)]
    #[storage_mapper("withdraw_from_loan_timespan")]
    fn withdraw_from_loan_timespan(&self) -> SingleValueMapper<u64>;

    #[view(getLoanShareTokenIdentifiers)]
    #[storage_mapper("loan_share_token_identifiers")]
    fn loan_share_token_identifiers(&self) -> SetMapper<TokenIdentifier>;

    #[view(getCrowdfundingState)]
    #[storage_mapper("crowdfunding_state")]
    fn crowdfunding_state(
        &self,
        project_id: u64,
    ) -> SingleValueMapper<CrowdfundingStateContext<Self::Api>>;
}

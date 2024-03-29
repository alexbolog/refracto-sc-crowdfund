multiversx_sc::imports!();

use crate::types::crowdfunding_state::CrowdfundingStateContext;

#[multiversx_sc::module]
pub trait ConfigModule {
    #[view(getLoanShareTokenIdentifier)]
    #[storage_mapper("loan_share_token_identifier")]
    fn loan_share_token_identifier(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getCrowdfundingState)]
    #[storage_mapper("crowdfunding_state")]
    fn crowdfunding_state(
        &self,
        project_id: u64,
    ) -> SingleValueMapper<CrowdfundingStateContext<Self::Api>>;

    #[view(getProjectIdByLoanShareNonce)]
    #[storage_mapper("project_id_by_loan_share_nonce")]
    fn project_id_by_loan_share_nonce(&self, nonce: u64) -> SingleValueMapper<u64>;

    #[view(getSourceLoanRepaymentScAddress)]
    #[storage_mapper("source_loan_repayment_sc_address")]
    fn template_loan_repayment_sc_address(&self) -> SingleValueMapper<ManagedAddress>;
}

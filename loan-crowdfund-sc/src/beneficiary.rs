multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait BeneficiaryModule:
    crate::permissions::PermissionsModule + crate::storage::config::ConfigModule
{
    #[endpoint(claimLoanFunds)]
    fn claim_loan_funds(&self, project_id: u64) {
        // TODO: add check for caller == beneficiary (+ tests)
        let mut state = self.crowdfunding_state(project_id).get();
        state.loan_start_timestamp = self.blockchain().get_block_timestamp();
        state.is_loan_active = true;
        self.crowdfunding_state(project_id).set(&state);
    }
}

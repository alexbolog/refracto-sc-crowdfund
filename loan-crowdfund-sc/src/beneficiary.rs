multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait BeneficiaryModule: crate::permissions::PermissionsModule {
    #[endpoint(claimLoanFunds)]
    fn claim_loan_funds(&self, project_id: u64) {}
}

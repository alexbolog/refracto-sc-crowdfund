use crate::types::crowdfunding_state::ProjectFundingState;

multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait CommonModule {
    #[view(getExpectedInterest)]
    fn get_expected_interest(&self, project_id: u64) -> BigUint {
        todo!()
    }

    #[view(getExpectedLateFees)]
    fn get_expected_late_fees(&self, project_id: u64) -> BigUint {
        todo!()
    }

    #[view(getTotalAmount)]
    fn get_total_amount(&self, project_id: u64) -> BigUint {
        todo!()
    }

    #[view(getFundingState)]
    fn get_funding_state(&self, project_id: u64) -> ProjectFundingState {
        ProjectFundingState::Invalid
    }
}

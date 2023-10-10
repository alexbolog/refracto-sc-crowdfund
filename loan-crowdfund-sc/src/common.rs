use crate::{constants::COOL_OFF_PERIOD, types::crowdfunding_state::ProjectFundingState};

multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait CommonModule:
    crate::storage::config::ConfigModule + crate::storage::payments::PaymentsModule
{
    #[view(getExpectedInterest)]
    fn get_expected_interest(&self, project_id: u64) -> BigUint {
        let state = self.crowdfunding_state(project_id).get();
        state.get_accrued_interest(self.blockchain().get_block_timestamp())
    }

    #[view(getExpectedLateFees)]
    fn get_expected_late_fees(&self, project_id: u64) -> BigUint {
        let state = self.crowdfunding_state(project_id).get();
        state.get_accrued_penalty(self.blockchain().get_block_timestamp())
    }

    #[view(getTotalAmount)]
    fn get_total_amount(&self, project_id: u64) -> BigUint {
        todo!()
    }

    #[view(getFundingState)]
    fn get_funding_state(&self, project_id: u64) -> ProjectFundingState {
        if self.crowdfunding_state(project_id).is_empty() {
            return ProjectFundingState::Invalid;
        }

        self.crowdfunding_state(project_id).get().get_funding_state(
            &self.get_aggregated_cool_off_amount(project_id),
            self.blockchain().get_block_timestamp(),
        )
    }

    #[view(getAggregatedCoolOffAmount)]
    fn get_aggregated_cool_off_amount(&self, project_id: u64) -> BigUint {
        let block_timestamp = self.blockchain().get_block_timestamp();
        let last_timestamp_for_cool_off = block_timestamp - COOL_OFF_PERIOD;

        let mut aggregated_cool_off_amount = BigUint::zero();
        for (_, timestamp, amount) in self.recorded_payments(project_id).iter() {
            if timestamp >= last_timestamp_for_cool_off {
                aggregated_cool_off_amount += amount;
            }
        }

        aggregated_cool_off_amount
    }
}

use crate::{
    constants::{COOL_OFF_PERIOD, ERR_INSUFFICIENT_REPAYMENT_AMOUNT, ERR_REPAYMENT_DISTRIBUTED},
    types::crowdfunding_state::{CrowdfundingStateContext, ProjectFundingState},
};

multiversx_sc::imports!();
use loan_refund_escrow_sc::ProxyTrait as _;

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
        let state = self.crowdfunding_state(project_id).get();
        state.get_total_amount_due(self.blockchain().get_block_timestamp())
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

    fn process_payment_distribution(
        &self,
        cf_state: &mut CrowdfundingStateContext<Self::Api>,
        min_allowed_amount: &BigUint,
    ) {
        require!(
            self.repayment_rates(cf_state.project_id).is_empty(),
            ERR_REPAYMENT_DISTRIBUTED
        );

        let repayment_amount: BigUint = self
            .repayment_sc_proxy(cf_state.repayment_contract_address.clone())
            .withdraw_repayment_funds()
            .execute_on_dest_context();

        require!(
            min_allowed_amount <= &repayment_amount,
            ERR_INSUFFICIENT_REPAYMENT_AMOUNT
        );

        let repayment_rate = cf_state.get_repayment_rate(&repayment_amount);
        self.repayment_rates(cf_state.project_id)
            .set(&repayment_rate);
        cf_state.is_repayed = true;
        self.crowdfunding_state(cf_state.project_id).set(cf_state);
    }

    #[proxy]
    fn repayment_sc_proxy(
        &self,
        sc_address: ManagedAddress,
    ) -> loan_refund_escrow_sc::Proxy<Self::Api>;
}

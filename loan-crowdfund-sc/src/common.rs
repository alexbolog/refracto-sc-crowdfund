multiversx_sc::imports!();
use crate::{
    constants::{
        COOL_OFF_PERIOD, ERR_INSUFFICIENT_REPAYMENT_AMOUNT, ERR_REPAYMENT_DISTRIBUTED,
        ONE_SHARE_DENOMINATION,
    },
    types::crowdfunding_state::{CrowdfundingStateContext, ProjectFundingState},
};

#[multiversx_sc::module]
pub trait CommonModule:
    crate::storage::config::ConfigModule
    + crate::storage::payments::PaymentsModule
    + crate::interactors::loan_repayment_sc_interactor::LoanRepaymentScInteractor
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
        if state.is_repayed {
            let repayment_rate = self.repayment_rates(project_id).get();

            return state.get_repaid_amount(&repayment_rate);
        }

        state.get_total_amount_due(self.blockchain().get_block_timestamp())
    }

    #[view(getFundingState)]
    fn get_funding_state(&self, project_id: u64) -> ProjectFundingState {
        if self.crowdfunding_state(project_id).is_empty() {
            return ProjectFundingState::Invalid;
        }

        let cf_state = self.crowdfunding_state(project_id).get();
        let repayment_sc_balance =
            self.get_repayment_funds_balance(cf_state.repayment_contract_address.clone());

        cf_state.get_funding_state(
            &self.get_aggregated_cool_off_amount(project_id),
            self.blockchain().get_block_timestamp(),
            &repayment_sc_balance,
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

    #[view(getRepaymentRate)]
    fn get_repayment_rate(&self, project_id: u64) -> BigUint {
        self.repayment_rates(project_id).get()
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
        let repayment_amount: BigUint =
            self.withdraw_repayment_funds(cf_state.repayment_contract_address.clone());

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

    fn mint_project_shares(
        &self,
        cf_max_target: &BigUint,
        price_per_share: &BigUint,
        project_name: &ManagedBuffer,
    ) -> u64 {
        let token = self.loan_share_token_identifier().get();
        let amount = &self.get_max_shares_supply(cf_max_target, price_per_share)
            * &BigUint::from(ONE_SHARE_DENOMINATION);

        self.send()
            .esdt_nft_create_compact_named(&token, &amount, project_name, b"")
    }

    fn get_max_shares_supply(&self, cf_max_target: &BigUint, price_per_share: &BigUint) -> BigUint {
        cf_max_target / price_per_share
    }

    fn burn_project_shares(
        &self,
        token_identifier: &TokenIdentifier,
        nonce: u64,
        amount: &BigUint,
    ) {
        self.send().esdt_local_burn(token_identifier, nonce, amount);
    }
}

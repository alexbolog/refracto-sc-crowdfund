#![no_std]

use constants::ONE_SHARE_DENOMINATION;
use types::crowdfunding_state::CrowdfundingStateContext;

use crate::{
    constants::{
        CF_STATES_ALLOWING_CLAIMING, CF_STATES_ALLOWING_INVESTMENT, CF_STATES_ALLOWING_WITHDRAWAL,
        COOL_OFF_PERIOD, ERR_CANNOT_CLAIM_IN_CRT_STATE, ERR_CANNOT_INVEST_IN_CRT_STATE,
        ERR_CANNOT_OVER_FINANCE, ERR_CANNOT_WITHDRAW_IN_CRT_STATE, ERR_COOL_OFF_EXPIRED,
        ERR_INVALID_PAYMENT_NONCE, ERR_INVALID_PAYMENT_TOKEN, ERR_INVALID_PROJECT_ID,
        ERR_INVESTMENT_NOT_FOUND, ERR_NOTHING_TO_CLAIM,
    },
    types::crowdfunding_state::INTEREST_RATE_DENOMINATION,
};

multiversx_sc::imports!();
pub mod admin;
pub mod beneficiary;
pub mod common;
pub mod constants;
pub mod interactors;
pub mod kyc;
mod permissions;
pub mod storage;
pub mod types;

#[multiversx_sc::contract]
pub trait LoanCrowdfundScContract:
    permissions::PermissionsModule
    + storage::config::ConfigModule
    + storage::payments::PaymentsModule
    + kyc::KycModule
    + admin::AdminModule
    + beneficiary::BeneficiaryModule
    + common::CommonModule
    + interactors::loan_repayment_sc_interactor::LoanRepaymentScInteractor
{
    #[init]
    fn init(&self, source_loan_repayment_sc_address: ManagedAddress) {
        self.source_loan_repayment_sc_address()
            .set(&source_loan_repayment_sc_address);
    }

    #[payable("*")]
    #[endpoint(invest)]
    fn invest(&self, project_id: u64) {
        let caller = self.blockchain().get_caller();
        self.require_address_is_kyc_compliant(&caller);

        let mut cf_state = self.get_project_by_id_or_fail(project_id);
        self.require_can_invest_in_current_state(&cf_state);
        let payment = self.get_invest_payment_or_fail_if_invalid(&cf_state);

        let shares = self.get_loan_shares(&cf_state, &payment.amount);
        self.send()
            .direct_multi(&caller, &ManagedVec::from_single_item(shares));

        self.update_successful_investment(
            &mut cf_state,
            caller,
            payment.amount,
            self.blockchain().get_block_timestamp(),
        );
    }

    #[view(getDebugSourceRepaymentSc)]
    fn get_source_repayment_sc(&self) -> ManagedAddress {
        self.source_loan_repayment_sc_address().get()
    }

    #[payable("*")]
    #[endpoint(withdraw)]
    fn withdraw(&self) {
        let caller = self.blockchain().get_caller();
        let payment = self.get_loan_share_payment_or_fail();
        let mut cf_state = self.get_project_by_nonce_or_fail(payment.token_nonce);

        let investment_amount = self.get_investment_amount_by_shares_amount(
            &payment.amount,
            &cf_state.share_price_per_unit,
        );
        let recorded_payment =
            self.get_oldest_recorded_payment(cf_state.project_id, &caller, &investment_amount);

        self.require_withdraw_is_possible(&cf_state, recorded_payment.1);

        self.update_successful_cool_off_withdrawal(
            &mut cf_state,
            &investment_amount,
            recorded_payment,
        );

        self.send().direct_esdt(
            &caller,
            &cf_state.project_payment_token,
            0,
            &investment_amount,
        );
    }

    #[payable("*")]
    #[endpoint(claim)]
    fn claim(&self) {
        let payment = self.get_loan_share_payment_or_fail();
        let cf_state = self.get_project_by_nonce_or_fail(payment.token_nonce);

        self.require_can_claim_in_current_state(&cf_state);
        let refund_amount = match cf_state.is_cancelled {
            true => &payment.amount * &cf_state.share_price_per_unit,
            false => match self.repayment_rates(cf_state.project_id).is_empty() {
                true => BigUint::zero(),
                false => {
                    self.repayment_rates(cf_state.project_id).get() * &payment.amount
                        / INTEREST_RATE_DENOMINATION
                }
            },
        };

        require!(refund_amount > 0, ERR_NOTHING_TO_CLAIM);

        self.burn_project_shares(
            &payment.token_identifier,
            payment.token_nonce,
            &payment.amount,
        );
        self.send().direct_esdt(
            &self.blockchain().get_caller(),
            &cf_state.project_payment_token,
            0,
            &refund_amount,
        );
    }

    #[endpoint(distributeRepayment)]
    fn distribute_repayment(&self, project_id: u64) {
        let mut cf_state = self.crowdfunding_state(project_id).get();
        let expected_total_repayment_amount =
            cf_state.get_total_amount_due(self.blockchain().get_block_timestamp());

        self.process_payment_distribution(&mut cf_state, &expected_total_repayment_amount);
    }

    #[view(getProjectDetails)]
    fn get_project_details(
        &self,
        project_ids: MultiValueManagedVec<u64>,
    ) -> ManagedVec<CrowdfundingStateContext<Self::Api>> {
        let mut result = ManagedVec::new();
        for project_id in project_ids.iter() {
            if self.crowdfunding_state(project_id).is_empty() {
                continue;
            }
            let cf_state = self.crowdfunding_state(project_id).get();
            result.push(cf_state);
        }
        result
    }

    fn get_loan_shares(
        &self,
        state: &CrowdfundingStateContext<Self::Api>,
        invested_amount: &BigUint,
    ) -> EsdtTokenPayment {
        let share_token_id = self.loan_share_token_identifier().get();
        let nonce = state.share_token_nonce;
        let shares_amount = &(invested_amount / &state.share_price_per_unit)
            * &BigUint::from(ONE_SHARE_DENOMINATION);

        EsdtTokenPayment::new(share_token_id, nonce, shares_amount)
    }

    fn update_successful_investment(
        &self,
        state: &mut CrowdfundingStateContext<Self::Api>,
        caller: ManagedAddress,
        amount: BigUint,
        timestamp: u64,
    ) {
        state.cf_progress += &amount;
        self.recorded_payments(state.project_id)
            .insert((caller, timestamp, amount));

        self.crowdfunding_state(state.project_id).set(state);
    }

    fn update_successful_cool_off_withdrawal(
        &self,
        state: &mut CrowdfundingStateContext<Self::Api>,
        amount: &BigUint,
        recorded_payment: (ManagedAddress, u64, BigUint),
    ) {
        state.cf_progress -= amount;
        self.recorded_payments(state.project_id)
            .remove(&recorded_payment);
        self.crowdfunding_state(state.project_id).set(state);
    }

    fn get_investment_amount_by_shares_amount(
        &self,
        shares_amount: &BigUint,
        price_per_share: &BigUint,
    ) -> BigUint {
        shares_amount * price_per_share / &BigUint::from(ONE_SHARE_DENOMINATION)
    }

    fn get_oldest_recorded_payment(
        &self,
        project_id: u64,
        caller: &ManagedAddress,
        investment_token_amount: &BigUint,
    ) -> (ManagedAddress, u64, BigUint) {
        let mut oldest_ts = u64::MAX;
        let mut oldest_recorded_payment = (ManagedAddress::zero(), 0, BigUint::zero());

        for recorded_payment in self.recorded_payments(project_id).iter() {
            let (address, timestamp, recorded_amount) = recorded_payment;
            if &address == caller && &recorded_amount == investment_token_amount {
                // return (address, timestamp, recorded_amount);
                if oldest_ts > timestamp {
                    oldest_ts = timestamp;
                    oldest_recorded_payment = (address, timestamp, recorded_amount);
                }
            }
        }

        require!(oldest_ts != u64::MAX, ERR_INVESTMENT_NOT_FOUND);

        oldest_recorded_payment
    }

    fn get_project_by_id_or_fail(&self, project_id: u64) -> CrowdfundingStateContext<Self::Api> {
        let state_strg = self.crowdfunding_state(project_id);
        require!(!state_strg.is_empty(), ERR_INVALID_PROJECT_ID);

        state_strg.get()
    }

    fn get_project_by_nonce_or_fail(&self, nonce: u64) -> CrowdfundingStateContext<Self::Api> {
        let nonce_mapping_strg = self.project_id_by_loan_share_nonce(nonce);
        require!(!nonce_mapping_strg.is_empty(), ERR_INVALID_PAYMENT_NONCE);

        let project_id = nonce_mapping_strg.get();
        self.get_project_by_id_or_fail(project_id)
    }

    fn get_invest_payment_or_fail_if_invalid(
        &self,
        cf_state: &CrowdfundingStateContext<Self::Api>,
    ) -> EsdtTokenPayment {
        let payment = self.call_value().single_esdt();

        require!(
            payment.token_identifier == cf_state.project_payment_token,
            ERR_INVALID_PAYMENT_TOKEN
        );

        require!(
            payment.amount <= (&cf_state.cf_target_max - &cf_state.cf_progress),
            ERR_CANNOT_OVER_FINANCE
        );

        payment
    }

    fn get_loan_share_payment_or_fail(&self) -> EsdtTokenPayment {
        let payment = self.call_value().single_esdt();

        require!(
            payment.token_identifier == self.loan_share_token_identifier().get(),
            ERR_INVALID_PAYMENT_TOKEN
        );

        payment
    }

    fn require_can_invest_in_current_state(&self, cf_state: &CrowdfundingStateContext<Self::Api>) {
        let repayment_sc_balance =
            self.get_repayment_funds_balance(cf_state.repayment_contract_address.clone());
        let state = cf_state.get_funding_state(
            &self.get_aggregated_cool_off_amount(cf_state.project_id),
            self.blockchain().get_block_timestamp(),
            &repayment_sc_balance,
        );
        require!(
            CF_STATES_ALLOWING_INVESTMENT.contains(&state),
            ERR_CANNOT_INVEST_IN_CRT_STATE
        );
    }

    fn require_can_claim_in_current_state(&self, cf_state: &CrowdfundingStateContext<Self::Api>) {
        let repayment_sc_balance =
            self.get_repayment_funds_balance(cf_state.repayment_contract_address.clone());
        let state = cf_state.get_funding_state(
            &self.get_aggregated_cool_off_amount(cf_state.project_id),
            self.blockchain().get_block_timestamp(),
            &repayment_sc_balance,
        );
        require!(
            CF_STATES_ALLOWING_CLAIMING.contains(&state),
            ERR_CANNOT_CLAIM_IN_CRT_STATE
        );
    }

    fn require_withdraw_is_possible(
        &self,
        cf_state: &CrowdfundingStateContext<Self::Api>,
        investment_timestamp: u64,
    ) {
        let block_timestamp = self.blockchain().get_block_timestamp();
        let repayment_sc_balance =
            self.get_repayment_funds_balance(cf_state.repayment_contract_address.clone());
        let state = cf_state.get_funding_state(
            &self.get_aggregated_cool_off_amount(cf_state.project_id),
            block_timestamp,
            &repayment_sc_balance,
        );
        require!(
            CF_STATES_ALLOWING_WITHDRAWAL.contains(&state),
            ERR_CANNOT_WITHDRAW_IN_CRT_STATE
        );
        require!(
            investment_timestamp + COOL_OFF_PERIOD > block_timestamp,
            ERR_COOL_OFF_EXPIRED
        );
    }
}

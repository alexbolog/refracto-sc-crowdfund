// Specs
// Admin side
// - Create a loan: project id, crowdfunding start/end and target etc
// - cancel crowdfund and return funds (maybe)
// - approve crowdfund and proceed with funding -> might not be needed, if we make the developer claim the funds
// - distribute repayments to users

// Bot side
// - send loan shares to users
// - distribute repayments to users

// User side
// - invest in a loan
// - withdraw from investing in a loan (MiCa compliance)
// - claim loan repayments
// - claim unsuccessful crowdfunding funds back

// Beneficiary side (developer)
// - claim funds from successful crowdfunding
// - repay loan
// - interest rate calculator + penalty fee calculator

// Add expected repayment amount check in repayment distribution

// !!! Single, full claiming + interest

#![no_std]

use types::crowdfunding_state::CrowdfundingStateContext;

use crate::{
    constants::{
        ERR_CANNOT_INVEST_IN_CRT_STATE, ERR_INVALID_PAYMENT_TOKEN, ERR_INVALID_PROJECT_ID,
    },
    types::crowdfunding_state::ProjectFundingState,
};

multiversx_sc::imports!();
pub mod admin;
pub mod beneficiary;
pub mod common;
pub mod constants;
pub mod kyc;
mod permissions;
mod storage;
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
{
    #[init]
    fn init(&self) {}

    #[payable("*")]
    #[endpoint(invest)]
    fn invest(&self, project_id: u64) {
        let caller = self.blockchain().get_caller();
        self.require_address_is_kyc_compliant(&caller);

        let mut cf_state = self.get_project_by_id_or_fail(project_id);
        self.require_state_is_active(&cf_state);
        let payment = self.get_payment_or_fail_if_invalid(&cf_state);

        cf_state.cf_progress += payment.amount;

        self.crowdfunding_state(project_id).set(cf_state);
    }

    #[endpoint(withdraw)]
    fn withdraw(&self) {}

    #[endpoint(claim)]
    fn claim(&self) {}

    #[endpoint(distributeRepayment)]
    fn distribute_repayment(&self) {}

    fn mint_loan_shares(&self) -> EsdtTokenPayment {
        todo!()
    }

    fn get_project_by_id_or_fail(&self, project_id: u64) -> CrowdfundingStateContext<Self::Api> {
        let state_strg = self.crowdfunding_state(project_id);
        require!(!state_strg.is_empty(), ERR_INVALID_PROJECT_ID);

        state_strg.get()
    }

    fn get_payment_or_fail_if_invalid(
        &self,
        cf_state: &CrowdfundingStateContext<Self::Api>,
    ) -> EsdtTokenPayment {
        let payment = self.call_value().single_esdt();

        require!(
            &payment.token_identifier == &cf_state.project_payment_token,
            ERR_INVALID_PAYMENT_TOKEN
        );

        payment
    }

    fn require_state_is_active(&self, cf_state: &CrowdfundingStateContext<Self::Api>) {
        let state = cf_state.get_funding_state(
            &self.get_aggregated_cool_off_amount(cf_state.project_id),
            self.blockchain().get_block_timestamp(),
        );
        require!(
            &state == &ProjectFundingState::CFActive,
            ERR_CANNOT_INVEST_IN_CRT_STATE
        );
    }
}

use loan_crowdfund_sc::{
    constants::COOL_OFF_PERIOD, types::crowdfunding_state::ProjectFundingState,
};

use super::{LoanCfTestState, INVESTOR_1_ADDRESS_EXPR, USDC_TOKEN_ID};

pub const MOCKUP_CF_START_TIMESTAMP: u64 = 100;
pub const MOCKUP_CF_END_TIMESTAMP: u64 = 100000 + COOL_OFF_PERIOD;
pub const MOCKUP_CF_TIMESTAMP_AFTER_START: u64 = MOCKUP_CF_START_TIMESTAMP + 1;
pub const MOCKUP_CF_TIMESTAMP_AFTER_END: u64 = MOCKUP_CF_END_TIMESTAMP + 1;
pub const MOCKUP_CF_TIMESTAMP_BEFORE_END: u64 = MOCKUP_CF_END_TIMESTAMP - 1;

pub const MOCKUP_CF_DEFAULT_MIN_PRINCIPAL: u64 = 9000;
pub const MOCKUP_CF_DEFAULT_MAX_PRINCIPAL: u64 = 10000;
pub const MOCKUP_CF_DEFAULT_COVER_MIN_PRINCIPAL: u64 = 9000;
pub const MOCKUP_CF_DEFAULT_COVER_MIN_REPAYMENT: u64 = 9000 * 120 / 100;
pub const MOCKUP_CF_SMALL_INVESTMENT: u64 = MOCKUP_CF_DEFAULT_MIN_PRINCIPAL / 10;

pub const MOCKUP_CF_DEFAULT_LOAN_DURATION: u64 = 12 * 30 * 24 * 60 * 60; // one year

impl LoanCfTestState {
    pub fn create_fully_mocked_project(&mut self) -> String {
        self.create_mocked_project_explicit_proj_id(1)
    }

    pub fn create_mocked_project_explicit_proj_id(&mut self, project_id: u64) -> String {
        self.create_mocked_project_explicit_financing_details(
            project_id,
            MOCKUP_CF_DEFAULT_MIN_PRINCIPAL,
            MOCKUP_CF_DEFAULT_MAX_PRINCIPAL,
            100,
            100,
            MOCKUP_CF_DEFAULT_LOAN_DURATION,
        )
    }

    pub fn create_mocked_project_explicit_financing_details(
        &mut self,
        project_id: u64,
        principal_min: u64,
        principal_max: u64,
        daily_interest_rate: u64,
        daily_penalty_fee: u64,
        loan_duration: u64,
    ) -> String {
        self.create_project(
            project_id,
            "TEST PROJ",
            USDC_TOKEN_ID,
            daily_interest_rate,
            daily_penalty_fee,
            self.beneficiary_address.clone(),
            1,
            MOCKUP_CF_START_TIMESTAMP + COOL_OFF_PERIOD,
            MOCKUP_CF_END_TIMESTAMP + COOL_OFF_PERIOD,
            principal_min,
            principal_max,
            loan_duration,
        )
    }

    pub fn create_default_mockup_in_state(
        &mut self,
        project_id: u64,
        state: &ProjectFundingState,
    ) -> String {
        let repayment_sc_address = self.create_mocked_project_explicit_proj_id(project_id);
        self.whitelist_address(INVESTOR_1_ADDRESS_EXPR);

        match state {
            ProjectFundingState::Pending => {}
            ProjectFundingState::Invalid => {}
            ProjectFundingState::CFActive => {
                self.set_block_timestamp(MOCKUP_CF_TIMESTAMP_AFTER_START);
            }
            ProjectFundingState::CFWaitingCooloff => {
                self.set_block_timestamp(MOCKUP_CF_TIMESTAMP_BEFORE_END);
                self.invest(
                    INVESTOR_1_ADDRESS_EXPR,
                    MOCKUP_CF_DEFAULT_COVER_MIN_PRINCIPAL,
                    project_id,
                );
                self.set_block_timestamp(MOCKUP_CF_TIMESTAMP_AFTER_END);
            }
            ProjectFundingState::CFSuccessful => {
                self.set_block_timestamp(MOCKUP_CF_TIMESTAMP_AFTER_START);
                self.invest(
                    INVESTOR_1_ADDRESS_EXPR,
                    MOCKUP_CF_DEFAULT_COVER_MIN_PRINCIPAL,
                    project_id,
                );
                self.set_block_timestamp(MOCKUP_CF_TIMESTAMP_AFTER_END);
            }
            ProjectFundingState::CFFailed => {
                self.set_block_timestamp(MOCKUP_CF_TIMESTAMP_AFTER_END);
            }
            ProjectFundingState::CFCancelled => {
                self.set_block_timestamp(MOCKUP_CF_TIMESTAMP_AFTER_START);
                self.invest(
                    INVESTOR_1_ADDRESS_EXPR,
                    MOCKUP_CF_SMALL_INVESTMENT,
                    project_id,
                );
                self.cancel_project(project_id);
            }
            ProjectFundingState::LoanActive => {
                self.set_block_timestamp(MOCKUP_CF_TIMESTAMP_AFTER_START);
                self.invest(
                    INVESTOR_1_ADDRESS_EXPR,
                    MOCKUP_CF_DEFAULT_COVER_MIN_PRINCIPAL,
                    project_id,
                );
                self.set_block_timestamp(MOCKUP_CF_TIMESTAMP_AFTER_END);
                self.claim_loan_funds(project_id);
            }
            ProjectFundingState::LoanRepaymentRunningLate => {
                todo!()
            }
            ProjectFundingState::Completed => {
                self.set_block_timestamp(MOCKUP_CF_TIMESTAMP_AFTER_START);
                self.invest(
                    INVESTOR_1_ADDRESS_EXPR,
                    MOCKUP_CF_DEFAULT_COVER_MIN_PRINCIPAL,
                    1,
                );
                self.set_block_timestamp(MOCKUP_CF_TIMESTAMP_AFTER_END);
                self.claim_loan_funds(project_id);
                self.set_block_timestamp(
                    MOCKUP_CF_TIMESTAMP_AFTER_END + MOCKUP_CF_DEFAULT_LOAN_DURATION + 1,
                );
                self.repay_loan(&repayment_sc_address, MOCKUP_CF_DEFAULT_COVER_MIN_REPAYMENT);
                self.admin_distribute_repayment(project_id);
            }
        }

        repayment_sc_address
    }
}

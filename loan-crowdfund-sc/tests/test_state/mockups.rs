use loan_crowdfund_sc::{
    admin::ProxyTrait as _, beneficiary::ProxyTrait as _, common::ProxyTrait,
    constants::COOL_OFF_PERIOD, types::crowdfunding_state::ProjectFundingState, ProxyTrait as _,
};
use multiversx_sc::{storage::mappers::SingleValue, types::Address};
use multiversx_sc_scenario::{
    api::StaticApi,
    managed_buffer, managed_token_id,
    scenario_model::{
        Account, AddressValue, CheckAccount, CheckStateStep, ScCallStep, ScDeployStep, ScQueryStep,
        SetStateStep,
    },
    ContractInfo, ScenarioWorld,
};

use super::{
    world, LoanCfContract, LoanCfTestState, ACCOUNT_BALANCE_EXPR, BENEFICIARY_ADDRESS_EXPR,
    INVESTOR_1_ADDRESS_EXPR, INVESTOR_2_ADDRESS_EXPR, LOAN_CF_ADDRESS_EXPR, LOAN_SHARES_ID_EXPR,
    OWNER_ADDRESS_EXPR, USDC_TOKEN_ID, USDC_TOKEN_ID_EXPR,
};

pub const MOCKUP_CF_START_TIMESTAMP: u64 = 100;
pub const MOCKUP_CF_END_TIMESTAMP: u64 = 100000 + COOL_OFF_PERIOD;
pub const MOCKUP_CF_TIMESTAMP_AFTER_START: u64 = MOCKUP_CF_START_TIMESTAMP + 1;
pub const MOCKUP_CF_TIMESTAMP_AFTER_END: u64 = MOCKUP_CF_END_TIMESTAMP + 1;
pub const MOCKUP_CF_TIMESTAMP_BEFORE_END: u64 = MOCKUP_CF_END_TIMESTAMP - 1;

pub const MOCKUP_CF_DEFAULT_MIN_PRINCIPAL: u64 = 9000;
pub const MOCKUP_CF_DEFAULT_MAX_PRINCIPAL: u64 = 10000;
pub const MOCKUP_CF_DEFAULT_COVER_MIN_PRINCIPAL: u64 = 9001;

impl LoanCfTestState {
    pub fn create_fully_mocked_project(&mut self) {
        self.create_mocked_project_explicit_proj_id(1);
    }

    pub fn create_mocked_project_explicit_proj_id(&mut self, project_id: u64) {
        self.create_mocked_project_explicit_financing_details(
            project_id,
            MOCKUP_CF_DEFAULT_MIN_PRINCIPAL,
            MOCKUP_CF_DEFAULT_MAX_PRINCIPAL,
            100,
            100,
            12 * 30 * 24 * 60 * 60, // one year
        );
    }

    pub fn create_mocked_project_explicit_financing_details(
        &mut self,
        project_id: u64,
        principal_min: u64,
        principal_max: u64,
        daily_interest_rate: u64,
        daily_penalty_fee: u64,
        loan_duration: u64,
    ) {
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
        );
    }

    pub fn create_default_mockup_in_state(&mut self, project_id: u64, state: &ProjectFundingState) {
        self.create_mocked_project_explicit_proj_id(project_id);
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
            ProjectFundingState::Completed => {
                todo!();
            }
        }
    }
}

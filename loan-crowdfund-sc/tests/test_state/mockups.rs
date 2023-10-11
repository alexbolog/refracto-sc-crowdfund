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

impl LoanCfTestState {
    pub fn create_fully_mocked_project(&mut self) {
        self.create_mocked_project_explicit_proj_id(1);
    }

    pub fn create_mocked_project_explicit_proj_id(&mut self, project_id: u64) {
        self.create_mocked_project_explicit_financing_details(
            project_id,
            9000,
            10000,
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
            100 + COOL_OFF_PERIOD,
            10000 + COOL_OFF_PERIOD,
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
                self.set_block_timestamp(101);
            }
            ProjectFundingState::CFWaitingCooloff => {
                self.set_block_timestamp(9999);
                self.invest(INVESTOR_1_ADDRESS_EXPR, 9001, project_id);
                self.set_block_timestamp(100001);
            }
            ProjectFundingState::CFSuccessful => {
                self.set_block_timestamp(101);
                self.invest(INVESTOR_1_ADDRESS_EXPR, 9001, project_id);
                self.set_block_timestamp(100001);
            }
            ProjectFundingState::CFFailed => {
                self.set_block_timestamp(100001);
            }
            ProjectFundingState::CFCancelled => {
                self.cancel_project(project_id);
            }
            ProjectFundingState::LoanActive => {
                self.set_block_timestamp(101);
                self.invest(INVESTOR_1_ADDRESS_EXPR, 9001, project_id);
                self.set_block_timestamp(100001);
                self.claim_loan_funds(project_id);
            }
            ProjectFundingState::Completed => {
                todo!();
            }
        }
    }
}

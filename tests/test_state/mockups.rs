use loan_crowdfund_sc::{
    admin::ProxyTrait as _, beneficiary::ProxyTrait as _, common::ProxyTrait,
    types::crowdfunding_state::ProjectFundingState, ProxyTrait as _,
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
    INVESTOR_1_ADDRESS_EXPR, INVESTOR_2_ADDRESS_EXPR, LOAN_CF_ADDRESS_EXPR, LOAN_SHARES_ID,
    OWNER_ADDRESS_EXPR, USDC_TOKEN_ID,
};

impl LoanCfTestState {
    pub fn create_fully_mocked_project(&mut self) {
        self.create_mocked_project_explicit_proj_id(1);
    }

    pub fn create_mocked_project_explicit_proj_id(&mut self, project_id: u64) {
        self.create_mocked_project_explicit_financing_details(project_id, 90000, 10000, 100, 100);
    }

    pub fn create_mocked_project_explicit_financing_details(
        &mut self,
        project_id: u64,
        principal_min: u64,
        principal_max: u64,
        daily_interest_rate: u64,
        daily_penalty_fee: u64,
    ) {
        self.create_project(
            project_id,
            "TEST PROJ",
            USDC_TOKEN_ID,
            daily_interest_rate,
            daily_penalty_fee,
            self.beneficiary_address.clone(),
            1,
            100,
            10000,
            principal_min,
            principal_max,
        );
    }
}

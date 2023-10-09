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
    pub fn set_block_timestamp(&mut self, block_timestamp_expr: &str) {
        self.world
            .set_state_step(SetStateStep::new().block_timestamp(block_timestamp_expr));
    }
}
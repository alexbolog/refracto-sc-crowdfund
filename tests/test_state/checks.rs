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
    pub fn check_funding_state(&mut self, project_id: u64, expected_state: ProjectFundingState) {
        self.world.sc_query(
            ScQueryStep::new()
                .call(self.contract.get_funding_state(project_id))
                .expect_value(expected_state),
        );
    }

    pub fn check_investor_usdc_balance(&mut self, address_expr: &str, expected_balance_expr: &str) {
        self.world
            .check_state_step(CheckStateStep::new().put_account(
                address_expr,
                CheckAccount::new().esdt_balance(USDC_TOKEN_ID, expected_balance_expr),
            ));
    }

    pub fn check_investor_share_balance(
        &mut self,
        address_expr: &str,
        nonce_expr: &str,
        expected_balance_expr: &str,
    ) {
        self.world
            .check_state_step(CheckStateStep::new().put_account(
                address_expr,
                CheckAccount::new().esdt_nft_balance_and_attributes(
                    LOAN_SHARES_ID,
                    nonce_expr,
                    expected_balance_expr,
                    Some(""),
                ),
            ));
    }
}

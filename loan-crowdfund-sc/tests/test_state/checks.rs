use loan_crowdfund_sc::{
    admin::ProxyTrait as _, beneficiary::ProxyTrait as _, common::ProxyTrait,
    types::crowdfunding_state::ProjectFundingState, ProxyTrait as _,
};
use multiversx_sc::{storage::mappers::SingleValue, types::Address};
use multiversx_sc_scenario::{
    api::StaticApi,
    managed_biguint, managed_buffer, managed_token_id,
    scenario_model::{
        Account, AddressValue, CheckAccount, CheckStateStep, ScCallStep, ScDeployStep, ScQueryStep,
        SetStateStep,
    },
    ContractInfo, ScenarioWorld,
};

use super::{
    world, LoanCfContract, LoanCfTestState, ACCOUNT_BALANCE_EXPR, BENEFICIARY_ADDRESS_EXPR,
    INVESTOR_1_ADDRESS_EXPR, INVESTOR_2_ADDRESS_EXPR, LOAN_CF_ADDRESS_EXPR, LOAN_SHARES_ID_EXPR,
    OWNER_ADDRESS_EXPR, USDC_TOKEN_ID_EXPR,
};

impl LoanCfTestState {
    pub fn check_funding_state(&mut self, project_id: u64, expected_state: ProjectFundingState) {
        self.world.sc_query(
            ScQueryStep::new()
                .call(self.contract.get_funding_state(project_id))
                .expect_value(expected_state),
        );
    }

    pub fn check_address_usdc_balance(&mut self, address_expr: &str, expected_balance_expr: &str) {
        self.world
            .check_state_step(CheckStateStep::new().put_account(
                address_expr,
                CheckAccount::new().esdt_balance(USDC_TOKEN_ID_EXPR, expected_balance_expr),
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
                    LOAN_SHARES_ID_EXPR,
                    nonce_expr,
                    expected_balance_expr,
                    Some(""),
                ),
            ));
    }

    pub fn check_expected_interest(&mut self, project_id: u64, expected_interest: u64) {
        self.world.sc_query(
            ScQueryStep::new()
                .call(self.contract.get_expected_interest(project_id))
                .expect_value(managed_biguint!(expected_interest)),
        );
    }

    pub fn check_expected_late_fees(&mut self, project_id: u64, expected_late_fees: u64) {
        self.world.sc_query(
            ScQueryStep::new()
                .call(self.contract.get_expected_late_fees(project_id))
                .expect_value(managed_biguint!(expected_late_fees)),
        );
    }

    pub fn check_total_repayment_amount(&mut self, project_id: u64, expected_total_amount: u64) {
        self.world.sc_query(
            ScQueryStep::new()
                .call(self.contract.get_total_amount(project_id))
                .expect_value(managed_biguint!(expected_total_amount)),
        );
    }
}

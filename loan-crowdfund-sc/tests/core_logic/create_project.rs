use loan_crowdfund_sc::{
    constants::ONE_SHARE_DENOMINATION, types::crowdfunding_state::INTEREST_RATE_DENOMINATION,
};
use multiversx_sc_scenario::{rust_biguint, scenario_model::AddressValue};

use crate::test_state::{
    LoanCfTestState, BENEFICIARY_ADDRESS_EXPR, LOAN_CF_ADDRESS_EXPR, USDC_TOKEN_ID,
};

#[test]
fn create_project_issues_expected_number_of_shares() {
    let min_target = 90000;
    let max_target = 100000;

    let mut state = LoanCfTestState::new();
    state.deploy_contract();
    state.create_project(
        1,
        "Test Project",
        USDC_TOKEN_ID,
        1,
        1,
        AddressValue::from(BENEFICIARY_ADDRESS_EXPR).to_address(),
        1,
        1,
        1,
        min_target,
        max_target,
        1000,
    );

    let expected_shares = rust_biguint!(max_target) * rust_biguint!(ONE_SHARE_DENOMINATION);
    state.check_address_share_balance(LOAN_CF_ADDRESS_EXPR, "1", &(expected_shares.to_string()));
}

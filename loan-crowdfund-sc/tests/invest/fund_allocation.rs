use loan_crowdfund_sc::{
    constants::ONE_SHARE_DENOMINATION, types::crowdfunding_state::ProjectFundingState,
};
use multiversx_sc_scenario::rust_biguint;

use crate::test_state::{LoanCfTestState, INVESTOR_1_ADDRESS_EXPR};

#[test]
fn invest_yields_correct_number_of_shares() {
    let project_id = 1;
    let mut state = LoanCfTestState::new();
    state.deploy_contract();
    state.create_default_mockup_in_state(project_id, &ProjectFundingState::CFActive);
    state.invest(INVESTOR_1_ADDRESS_EXPR, 1000, project_id);
    let expected_share_balance = rust_biguint!(1000) * rust_biguint!(ONE_SHARE_DENOMINATION);

    state.check_address_share_balance(
        INVESTOR_1_ADDRESS_EXPR,
        &project_id.to_string(),
        &expected_share_balance.to_string(),
    );
}

#[test]
fn invest_yields_correct_share_nonce() {
    let mut state = LoanCfTestState::new();
    state.deploy_contract();
    state.create_default_mockup_in_state(1, &ProjectFundingState::CFActive);
    state.create_default_mockup_in_state(2, &ProjectFundingState::CFActive);

    state.invest(INVESTOR_1_ADDRESS_EXPR, 1000, 2);

    let expected_proj_2_shares_amount = rust_biguint!(1000) * rust_biguint!(ONE_SHARE_DENOMINATION);

    state.check_address_share_balance(INVESTOR_1_ADDRESS_EXPR, "1", "0");
    state.check_address_share_balance(
        INVESTOR_1_ADDRESS_EXPR,
        "2",
        &expected_proj_2_shares_amount.to_string(),
    );
}

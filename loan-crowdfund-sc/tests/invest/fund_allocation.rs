use loan_crowdfund_sc::types::crowdfunding_state::ProjectFundingState;

use crate::test_state::{LoanCfTestState, INVESTOR_1_ADDRESS_EXPR};

#[test]
fn invest_yields_correct_number_of_shares() {
    let project_id = 1;
    let mut state = LoanCfTestState::new();
    state.deploy_contract();
    state.create_default_mockup_in_state(project_id, &ProjectFundingState::CFActive);
    state.invest(INVESTOR_1_ADDRESS_EXPR, 1000, project_id);

    state.check_address_share_balance(INVESTOR_1_ADDRESS_EXPR, &project_id.to_string(), "1000");
}

#[test]
fn invest_yields_correct_share_nonce() {
    let mut state = LoanCfTestState::new();
    state.deploy_contract();
    state.create_default_mockup_in_state(1, &ProjectFundingState::CFActive);
    state.create_default_mockup_in_state(2, &ProjectFundingState::CFActive);

    state.invest(INVESTOR_1_ADDRESS_EXPR, 1000, 2);

    state.check_address_share_balance(INVESTOR_1_ADDRESS_EXPR, "1", "0");
    state.check_address_share_balance(INVESTOR_1_ADDRESS_EXPR, "2", "1000");
}

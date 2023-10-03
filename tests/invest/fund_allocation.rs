use crate::test_state::{LoanCfTestState, INVESTOR_1_ADDRESS_EXPR};

#[test]
fn invest_yields_correct_number_of_shares() {
    let mut state = LoanCfTestState::new();
    state.deploy_contract();
    state.create_valid_mockup_project();

    state.invest(INVESTOR_1_ADDRESS_EXPR, 1000, 1);

    state.check_investor_share_balance(INVESTOR_1_ADDRESS_EXPR, "1", "1000");
}

#[test]
fn invest_yields_correct_share_nonce() {
    let mut state = LoanCfTestState::new();
    state.deploy_contract();
    state.create_valid_mockup_project();
    state.create_valid_mockup_project_explicit_project_id(2);

    state.invest(INVESTOR_1_ADDRESS_EXPR, 1000, 2);
    state.check_investor_share_balance(INVESTOR_1_ADDRESS_EXPR, "1", "0");
    state.check_investor_share_balance(INVESTOR_1_ADDRESS_EXPR, "2", "1000");
}

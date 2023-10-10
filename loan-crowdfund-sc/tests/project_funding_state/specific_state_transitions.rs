use crate::test_state::{LoanCfTestState, INVESTOR_1_ADDRESS_EXPR};
use loan_crowdfund_sc::types::crowdfunding_state::ProjectFundingState;

#[test]
fn funding_state_cool_off_to_failed() {
    let mut state = LoanCfTestState::new();
    state.deploy_contract();
    state.create_fully_mocked_project();
    state.whitelist_address(INVESTOR_1_ADDRESS_EXPR);

    state.set_block_timestamp(9999);
    state.invest(INVESTOR_1_ADDRESS_EXPR, 9001, 1);
    state.set_block_timestamp(10001);

    state.check_funding_state(1, ProjectFundingState::CFWaitingCooloff);
}

use crate::test_state::{LoanCfTestState, INVESTOR_1_ADDRESS_EXPR};
use loan_crowdfund_sc::types::crowdfunding_state::ProjectFundingState;

#[test]
fn funding_state_cool_off_to_failed() {
    let mut state = LoanCfTestState::new();
    state.deploy_contract();
    state.create_fully_mocked_project();
    state.set_block_timestamp("101");
    state.invest(INVESTOR_1_ADDRESS_EXPR, 90001, 1);
    state.set_block_timestamp("102");

    state.check_funding_state(1, ProjectFundingState::CFWaitingCooloff);
    state.withdraw(INVESTOR_1_ADDRESS_EXPR, 1, 90001);

    state.set_block_timestamp("10001");
    state.check_funding_state(1, ProjectFundingState::CFFailed);
}

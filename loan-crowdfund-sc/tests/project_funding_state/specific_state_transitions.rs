use crate::test_state::{
    mockups::{
        MOCKUP_CF_DEFAULT_COVER_MIN_PRINCIPAL, MOCKUP_CF_TIMESTAMP_AFTER_END,
        MOCKUP_CF_TIMESTAMP_BEFORE_END,
    },
    LoanCfTestState, INVESTOR_1_ADDRESS_EXPR,
};
use loan_crowdfund_sc::types::crowdfunding_state::ProjectFundingState;

#[test]
fn funding_state_cool_off_to_failed() {
    let mut state = LoanCfTestState::new();
    state.deploy_contract();
    state.create_fully_mocked_project();
    state.whitelist_address(INVESTOR_1_ADDRESS_EXPR);

    state.set_block_timestamp(MOCKUP_CF_TIMESTAMP_BEFORE_END);
    state.invest(
        INVESTOR_1_ADDRESS_EXPR,
        MOCKUP_CF_DEFAULT_COVER_MIN_PRINCIPAL,
        1,
    );
    state.set_block_timestamp(MOCKUP_CF_TIMESTAMP_AFTER_END);

    state.check_funding_state(1, ProjectFundingState::CFWaitingCooloff);
}
